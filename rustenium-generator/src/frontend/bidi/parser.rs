use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use heck::ToUpperCamelCase;

use crate::backend::base_types::*;

fn o(s: impl Into<String>) -> Cow<'static, str> {
    Cow::Owned(s.into())
}

fn type_ref(s: &str) -> TypeRef<'static> {
    if let Some((module, name)) = s.split_once('.') {
        TypeRef { module: Some(o(module)), name: o(name) }
    } else {
        TypeRef { module: None, name: o(s) }
    }
}

/// Check if a variant name will produce a valid Rust identifier after case conversion.
fn is_valid_variant_name(name: &str) -> bool {
    let camel = name.to_upper_camel_case();
    !camel.is_empty() && camel.starts_with(|c: char| c.is_ascii_alphabetic())
}

struct RawRule {
    name: String,
    body: String,
    origin: DomainDirection,
}

/// Parse labeled CDDL inputs into a single Protocol.
/// Each input is `(content, origin)`.
pub fn parse_cddl(inputs: &[(&str, DomainDirection)]) -> Protocol<'static> {
    let mut rules: Vec<RawRule> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();

    for (content, origin) in inputs {
        for (name, body) in split_rules(content) {
            if let Some(&idx) = seen.get(&name) {
                // Seen in another file → mark as Both
                rules[idx].origin = DomainDirection::All;
            } else {
                seen.insert(name.clone(), rules.len());
                rules.push(RawRule {
                    name,
                    body,
                    origin: *origin,
                });
            }
        }
    }

    let rule_map: HashMap<&str, &str> = rules
        .iter()
        .map(|r| (r.name.as_str(), r.body.as_str()))
        .collect();


    // Classify using aggregate groups + name/body heuristics
    let mut commands: HashSet<String> = collect_group_members(&rule_map, "Command");
    let mut events: HashSet<String> = collect_group_members(&rule_map, "Event");
    let mut results: HashSet<String> = collect_group_members(&rule_map, "Result");

    for rule in &rules {
        let name = rule.name.as_str();
        if !name.contains('.') {
            continue;
        }

        // Name ends with Result → result
        if name.ends_with("Result") {
            results.insert(name.to_string());
            continue;
        }

        // Has method: + params: → command or event
        if rule.body.contains("method:") && rule.body.contains("params:") {
            if commands.contains(name) || events.contains(name) {
                continue;
            }
            // Remote-origin rules with method → command; local-origin → event
            match rule.origin {
                DomainDirection::Local => {
                    events.insert(name.to_string());
                }
                _ => {
                    commands.insert(name.to_string());
                }
            }
        }
    }

    // Collect param type names from commands and events to exclude from types.
    // Only exclude types whose body is a struct (starts with '{') AND are not
    // referenced by any other type (i.e., only used as inline params).
    let mut param_types: HashSet<String> = HashSet::new();

    for name in commands.iter().chain(events.iter()) {
        if let Some(body) = rule_map.get(name.as_str()) {
            if let Some(pt) = extract_value(body, "params:") {
                // Only exclude if it's a struct body and not referenced elsewhere as a type
                if let Some(pt_body) = rule_map.get(pt.as_str()) {
                    if pt_body.trim().starts_with('{') {
                        // Check if this param type is referenced by non-command/event rules
                        let referenced_elsewhere = rules.iter().any(|r| {
                            !commands.contains(r.name.as_str())
                                && !events.contains(r.name.as_str())
                                && r.name != pt
                                && r.body.contains(&pt)
                        });
                        if !referenced_elsewhere {
                            param_types.insert(pt);
                        }
                    }
                } else {
                    param_types.insert(pt);
                }
            }
        }
    }

    // Group rules by module
    let mut modules: HashMap<String, Module<'static>> = HashMap::new();

    for rule in &rules {
        let Some((mod_name, type_name)) = rule.name.split_once('.') else {
            continue;
        };

        let module = modules
            .entry(mod_name.to_string())
            .or_insert_with(|| Module {
                name: o(mod_name),
                ..Default::default()
            });

        let name = rule.name.as_str();

        let dir = Some(rule.origin);

        if commands.contains(name) {
            if let Some((mut cmd, synthetic)) = parse_command(name, type_name, &rule.body, &rule_map) {
                cmd.direction = dir;
                module.commands.push(cmd);
                module.types.extend(synthetic);
            }
        } else if events.contains(name) {
            if let Some((mut evt, synthetic)) = parse_event(name, type_name, &rule.body, &rule_map) {
                evt.direction = dir;
                module.events.push(evt);
                module.types.extend(synthetic);
            }
        } else if results.contains(name) {
            if let Some((cr, synthetic)) = parse_result(name, type_name, &rule.body, &rule_map) {
                module.command_results.push(cr);
                module.types.extend(synthetic);
            }
        } else if !param_types.contains(name) {
            if let Some((mut td, synthetic)) = parse_typedef(name, type_name, &rule.body, &rule_map) {
                td.direction = dir;
                module.types.push(td);
                module.types.extend(synthetic);
            }
        }
    }

    // Upsert overrides: add/replace params on existing typedefs, or insert new ones
    apply_overrides(&mut modules);

    // Ensure every command has a corresponding result
    for module in modules.values_mut() {
        let existing: HashSet<String> = module
            .command_results
            .iter()
            .map(|cr| cr.name.to_string())
            .collect();
        for cmd in &module.commands {
            let result_name = format!("{}Result", cmd.method.name);
            if !existing.contains(&result_name) {
                module.command_results.push(CommandResult {
                    description: Some(o(format!("Result for {} command", cmd.method.name))),
                    name: o(result_name),
                    parameters: vec![],
                    raw_name: o(""),
                    type_choice: None,
                });
            }
        }
    }

    let mut module_list: Vec<_> = modules.into_values().collect();
    module_list.sort_by(|a, b| a.name.cmp(&b.name));

    // Detect recursive types and mark circular deps
    mark_circular_deps(&mut module_list);

    Protocol {
        name: None,
        description: Some(o("WebDriver BiDi Protocol")),
        version: Version { major: 1, minor: 0 },
        modules: module_list,
    }
}

/// Split CDDL input into `(rule_name, rule_body)` pairs.
fn split_rules(input: &str) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_body = String::new();

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with(';') {
            continue;
        }

        if !trimmed.is_empty() {
            if let Some(eq_pos) = trimmed.find(" = ") {
                let candidate_name = &trimmed[..eq_pos];
                if !candidate_name.contains(' ') {
                    if let Some(name) = current_name.take() {
                        rules.push((name, current_body.trim().to_string()));
                    }
                    current_name = Some(candidate_name.to_string());
                    current_body = trimmed[eq_pos + 3..].to_string();
                    current_body.push('\n');
                    continue;
                }
            }
        }

        if current_name.is_some() && !trimmed.is_empty() {
            current_body.push_str(trimmed);
            current_body.push('\n');
        }
    }

    if let Some(name) = current_name {
        rules.push((name, current_body.trim().to_string()));
    }

    rules
}

/// Collect module-qualified member names from aggregate group rules.
fn collect_group_members(rule_map: &HashMap<&str, &str>, suffix: &str) -> HashSet<String> {
    let mut members = HashSet::new();
    for (name, body) in rule_map {
        if name.ends_with(suffix) && !name.contains('.') {
            for part in body.split('/') {
                let member = part
                    .trim()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .trim();
                if !member.is_empty() && member.contains('.') {
                    members.insert(member.to_string());
                }
            }
        }
    }
    members
}

fn extract_quoted(body: &str, key: &str) -> Option<String> {
    let idx = body.find(key)?;
    let after = &body[idx + key.len()..];
    let start = after.find('"')? + 1;
    let end = after[start..].find('"')? + start;
    Some(after[start..end].to_string())
}

fn extract_value(body: &str, key: &str) -> Option<String> {
    let idx = body.find(key)?;
    let after = body[idx + key.len()..].trim_start();
    let end = after
        .find(|c: char| c == ',' || c == ')' || c == '}' || c == '\n')
        .unwrap_or(after.len());
    let val = after[..end].trim();
    if val.is_empty() {
        None
    } else {
        Some(val.to_string())
    }
}

fn parse_command(
    _full_name: &str,
    type_name: &str,
    body: &str,
    rule_map: &HashMap<&str, &str>,
) -> Option<(Command<'static>, Vec<TypeDef<'static>>)> {
    let method_str = extract_quoted(body, "method:")?;
    let params_type = extract_value(body, "params:");
    let (params, synthetic) = resolve_params(&params_type, rule_map);

    Some((Command {
        method: CommandMethod {
            description: None,
            experimental: false,
            deprecated: false,
            name: o(type_name),
            redirect: None,
            raw_name: o(&method_str),
        },
        params,
        returns: vec![],
        is_circular_dep: false,
        direction: None,
    }, synthetic))
}

fn parse_event(
    _full_name: &str,
    type_name: &str,
    body: &str,
    rule_map: &HashMap<&str, &str>,
) -> Option<(Event<'static>, Vec<TypeDef<'static>>)> {
    let method_str = extract_quoted(body, "method:")?;
    let params_type = extract_value(body, "params:");
    let (parameters, synthetic) = resolve_params(&params_type, rule_map);

    Some((Event {
        description: None,
        experimental: false,
        deprecated: false,
        name: o(type_name),
        parameters,
        raw_name: o(&method_str),
        is_circular_dep: false,
        direction: None,
    }, synthetic))
}

fn parse_result(
    full_name: &str,
    type_name: &str,
    body: &str,
    rule_map: &HashMap<&str, &str>,
) -> Option<(CommandResult<'static>, Vec<TypeDef<'static>>)> {
    let body = body.trim().trim_end_matches(';');
    // Union-body result → TypeChoice enum in results.rs
    if let Some(refs) = parse_union_refs(body) {
        return Some((CommandResult {
            description: None, name: o(type_name), parameters: vec![],
            raw_name: o(full_name), type_choice: Some(refs),
        }, vec![]));
    }
    // Type alias result (e.g. CallFunctionResult = EvaluateResult) → type alias
    if !body.contains('{') && !body.contains('/') && !body.contains(':') && body.contains('.') {
        return Some((CommandResult {
            description: None, name: o(type_name), parameters: vec![],
            raw_name: o(full_name), type_choice: Some(vec![type_ref(body)]),
        }, vec![]));
    }
    let (params, synthetic) = resolve_result_fields(body, rule_map);
    Some((CommandResult {
        description: None, name: o(type_name), parameters: params,
        raw_name: o(full_name), type_choice: None,
    }, synthetic))
}

/// Try to parse a body as a type union, returning TypeRefs if successful.
fn parse_union_refs(body: &str) -> Option<Vec<TypeRef<'static>>> {
    let inner = if body.starts_with('(') {
        &body[1..body.rfind(')')?]
    } else if body.contains('/') && !body.starts_with('{') {
        body
    } else {
        return None;
    };
    let sep = if inner.contains("//") { "//" } else { "/" };
    let parts: Vec<&str> = inner.split(sep).map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| p.strip_prefix('{').and_then(|p| p.strip_suffix('}')).map(|p| p.trim()).unwrap_or(p))
        .collect();
    (parts.len() >= 2 && parts.iter().all(|p| !p.contains(':') && !p.contains('{'))).then(|| {
        parts.into_iter().map(type_ref).collect()
    })
}

/// Resolve a result body to struct fields, following aliases.
fn resolve_result_fields(body: &str, rule_map: &HashMap<&str, &str>) -> (Vec<Param<'static>>, Vec<TypeDef<'static>>) {
    if body.starts_with('{') {
        return parse_struct_fields_resolved(body, rule_map);
    }
    if let Some(aliased_body) = rule_map.get(body) {
        return resolve_result_fields(aliased_body.trim(), rule_map);
    }
    Default::default()
}

fn parse_typedef(full_name: &str, type_name: &str, body: &str, rule_map: &HashMap<&str, &str>) -> Option<(TypeDef<'static>, Vec<TypeDef<'static>>)> {
    let body = body.trim().trim_end_matches(';');
    let td = |extends: Type<'static>, parameters: Option<Item<'static>>| -> (TypeDef<'static>, Vec<TypeDef<'static>>) {
        (TypeDef {
            description: None, experimental: false, deprecated: false,
            name: o(type_name), extends, parameters,
            raw_name: o(full_name), is_circular_dep: false, direction: None,
        }, vec![])
    };

    // text alias: module.Name = text
    if body == "text" {
        return Some(td(Type::String, None));
    }
    if body.starts_with('"')
        && body.ends_with('"')
        && !body[1..body.len() - 1].contains('"')
    {
        return Some(td(Type::String, None));
    }

    // String enum: "a" / "b" / "c" (must have / and at least 2 valid variants)
    if body.contains('"') && body.contains('/') && !body.starts_with('{') && !body.starts_with('(')
    {
        let variants = parse_string_enum(body);
        if variants.len() >= 2 {
            return Some(td(Type::String, Some(Item::Enum(variants))));
        }
        return Some(td(Type::String, None));
    }

    // Struct with inner type choice: { ( A // B ) } → untagged enum
    if body.starts_with('{') {
        if let Some(type_refs) = parse_inner_type_choice(body) {
            return Some(td(Type::Object, Some(Item::TypeChoice(type_refs))));
        }

        let (params, synthetic) = parse_struct_fields_resolved(body, rule_map);
        return Some((TypeDef {
            description: None,
            experimental: false,
            deprecated: false,
            name: o(type_name),
            extends: Type::Object,
            parameters: if params.is_empty() {
                None
            } else {
                Some(Item::Properties(params))
            },
            raw_name: o(full_name),
            is_circular_dep: false,
            direction: None,
        }, synthetic));
    }

    // Array type: [*module.Type]
    if body.starts_with('[') {
        return Some(td(parse_type(body), None));
    }

    // Group: ( ... )
    if body.starts_with('(') {
        let inner = &body[1..body.rfind(')').unwrap_or(body.len())];
        let inner = inner.trim();

        // Treat as struct if it has named fields (contains ':')
        if inner.contains(':') {
            let wrapped = format!("{{\n{}\n}}", inner);
            let (params, synthetic) = parse_struct_fields_resolved(&wrapped, rule_map);
            return Some((TypeDef {
                description: None,
                experimental: false,
                deprecated: false,
                name: o(type_name),
                extends: Type::Object,
                parameters: if params.is_empty() { None } else { Some(Item::Properties(params)) },
                raw_name: o(full_name),
                is_circular_dep: false,
                direction: None,
            }, synthetic));
        }

        // Type union: ( TypeA / TypeB / ... ) or ( TypeA // TypeB // ... )
        let separator = if inner.contains("//") { "//" } else { "/" };
        if inner.contains(separator) {
            let parts: Vec<&str> = inner
                .split(separator)
                .map(|p| p.trim())
                .filter(|p| !p.is_empty())
                // Strip { } wrappers: CDDL `{ Type }` means the same as `Type` in a union context
                .map(|p| p.strip_prefix('{').and_then(|p| p.strip_suffix('}')).map(|p| p.trim()).unwrap_or(p))
                .collect();
            if parts.len() >= 2 && parts.iter().all(|p| !p.contains(':') && !p.contains('{')) {
                let refs: Vec<TypeRef<'static>> = parts.into_iter().map(type_ref).collect();
                return Some(td(Type::Object, Some(Item::TypeChoice(refs))));
            }
        }

        return Some(td(Type::Object, None));
    }

    // Bare type union: A / B or A // B (without parens or braces)
    if body.contains('/') && !body.contains('"') {
        let separator = if body.contains("//") { "//" } else { "/" };
        let parts: Vec<&str> = body.split(separator)
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| p.strip_prefix('{').and_then(|p| p.strip_suffix('}')).map(|p| p.trim()).unwrap_or(p))
            .collect();
        if parts.len() >= 2 && parts.iter().all(|p| !p.contains(':') && !p.contains('{')) {
            let refs: Vec<TypeRef<'static>> = parts.into_iter().map(type_ref).collect();
            return Some(td(Type::Object, Some(Item::TypeChoice(refs))));
        }
    }

    // Type alias
    Some(td(parse_type(body), None))
}

fn resolve_params(
    params_type: &Option<String>,
    rule_map: &HashMap<&str, &str>,
) -> (Vec<Param<'static>>, Vec<TypeDef<'static>>) {
    match params_type {
        Some(pt) if pt != "EmptyParams" => {
            if let Some(body) = rule_map.get(pt.as_str()) {
                let body = body.trim();
                if body.starts_with('{') || body.starts_with('(') {
                    parse_struct_fields_resolved(body, rule_map)
                } else {
                    // Non-struct params (e.g., union type alias) → single flattened param
                    let ty = parse_type(pt);
                    let field_name = pt.rsplit('.').next().unwrap_or(pt);
                    let field_snake = heck::ToSnakeCase::to_snake_case(field_name);
                    (vec![Param {
                        description: None, experimental: false, deprecated: false,
                        optional: false, r#type: ty, name: o(&field_snake),
                        raw_name: o(pt.as_str()), is_circular_dep: false,
                        default_value: None, validation: None, flatten: true,
                    }], vec![])
                }
            } else {
                Default::default()
            }
        }
        _ => Default::default(),
    }
}

fn parse_struct_fields_resolved(body: &str, rule_map: &HashMap<&str, &str>) -> (Vec<Param<'static>>, Vec<TypeDef<'static>>) {
    parse_struct_fields_with(body, Some(rule_map))
}

fn parse_struct_fields_with(body: &str, rule_map: Option<&HashMap<&str, &str>>) -> (Vec<Param<'static>>, Vec<TypeDef<'static>>) {
    let mut params = Vec::new();
    let mut synthetic_types = Vec::new();
    let mut depth: i32 = 0;
    // For collecting nested inline struct bodies
    let mut nested_field: Option<(String, bool)> = None; // (field_name, optional)
    let mut nested_body = String::new();
    // For collecting inline choice groups: (A // B) possibly spanning multiple lines
    let mut choice_buf: Option<String> = None;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let prev_depth = depth;
        for ch in trimmed.chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }

        // Collecting nested inline struct body
        if nested_field.is_some() {
            nested_body.push_str(trimmed);
            nested_body.push('\n');
            if depth <= 1 {
                // Nested block closed — flatten inner fields into parent
                let (_name, _optional) = nested_field.take().unwrap();
                let (inner_params, inner_types) = parse_struct_fields_with(&nested_body, rule_map);
                nested_body.clear();
                params.extend(inner_params);
                synthetic_types.extend(inner_types);
                continue;
            }
            continue;
        }

        // Collecting multi-line inline choice group
        if let Some(ref mut buf) = choice_buf {
            buf.push(' ');
            buf.push_str(trimmed);
            if trimmed.contains(')') {
                let full = buf.clone();
                choice_buf = None;
                if let Some((p, td)) = parse_inline_choice(&full) {
                    params.push(p);
                    synthetic_types.push(td);
                }
            }
            continue;
        }

        if prev_depth != 1 {
            continue;
        }

        let field_str = trimmed.trim_end_matches(',').trim_end_matches(';');
        if field_str.is_empty() {
            continue;
        }

        if field_str == "Extensible" {
            params.push(Param {
                description: None,
                experimental: false,
                deprecated: false,
                optional: false,
                r#type: Type::Extensible,
                name: o("extensible"),
                raw_name: o("extensible"),
                is_circular_dep: false,
                default_value: None,
                validation: None,
                flatten: true,
            });
            continue;
        }

        // Inline choice group: (A // B) or multi-line ( \n A // \n B \n )
        if field_str.starts_with('(') && field_str.contains("//") {
            if field_str.contains(')') {
                if let Some((p, td)) = parse_inline_choice(field_str) {
                    params.push(p);
                    synthetic_types.push(td);
                }
            } else {
                choice_buf = Some(field_str.to_string());
            }
            continue;
        }
        if field_str.starts_with('(') && !field_str.contains(':') && !field_str.contains(')') {
            // Possibly multi-line choice group starting
            choice_buf = Some(field_str.to_string());
            continue;
        }

        // Group inclusion: bare type reference without ':' → flattened field
        if !field_str.contains(':') {
            let ref_name = field_str.trim();
            if ref_name.is_empty() || !ref_name.chars().next().unwrap().is_ascii_alphabetic() {
                continue;
            }
            let ty = parse_type(ref_name);
            // Derive field name from the type name (e.g. "network.BaseParameters" → "base_parameters")
            let field_name = ref_name
                .rsplit('.')
                .next()
                .unwrap_or(ref_name);
            let field_name_snake = heck::ToSnakeCase::to_snake_case(field_name);
            params.push(Param {
                description: None,
                experimental: false,
                deprecated: false,
                optional: false,
                r#type: ty,
                name: o(&field_name_snake),
                raw_name: o(ref_name),
                is_circular_dep: false,
                default_value: None,
                validation: None,
                flatten: true,
            });
            continue;
        }

        let mut optional = field_str.starts_with('?');
        let field_str = if optional {
            field_str[1..].trim()
        } else {
            field_str
        };

        if let Some((name, type_str)) = field_str.split_once(':') {
            let name = name.trim();
            let type_str = type_str.trim();

            // Inline nested struct: field: {
            if type_str == "{" || type_str.starts_with('{') {
                if depth > 1 {
                    // Opening brace pushed depth > 1, start collecting
                    nested_field = Some((name.to_string(), optional));
                    nested_body.clear();
                    nested_body.push_str(trimmed);
                    nested_body.push('\n');
                    continue;
                }
            }

            let (default_value, validation) = extract_constraints(type_str);
            let (type_str, nullable) = strip_null_from_str(type_str);
            optional = optional || nullable || default_value.is_some();

            let ty = if type_str.is_empty() {
                Type::Object
            } else {
                parse_type(&type_str)
            };

            params.push(Param {
                description: None,
                experimental: false,
                deprecated: false,
                optional,
                r#type: ty,
                name: o(name),
                raw_name: o(name),
                is_circular_dep: false,
                default_value,
                validation,
                flatten: false,
            });
        }
    }

    (params, synthetic_types)
}

/// Extract `.default`, `.ge`, `.gt`, `.le`, `.lt` constraints from a type string.
fn extract_constraints(s: &str) -> (Option<String>, Option<Vec<Constraint>>) {
    let mut default_value = None;
    let mut constraints = Vec::new();

    // Scan for constraint markers
    let mut remaining = s;
    while let Some(idx) = remaining.find(" .") {
        let after = &remaining[idx + 2..];
        if let Some(rest) = after.strip_prefix("default ") {
            let val = rest
                .split(|c: char| c == ',' || c == ')' || c == '}')
                .next()
                .unwrap_or("")
                .trim();
            if !val.is_empty() {
                default_value = Some(val.to_string());
            }
        } else if let Some((tag, rest)) = parse_constraint_tag(after) {
            let val = rest
                .split(|c: char| c == ',' || c == ')' || c == '}' || c == ' ')
                .next()
                .unwrap_or("")
                .trim();
            if let Ok(n) = val.parse::<f64>() {
                let c = match tag {
                    "ge" => Constraint::Ge(n),
                    "gt" => Constraint::Gt(n),
                    "le" => Constraint::Le(n),
                    "lt" => Constraint::Lt(n),
                    _ => unreachable!(),
                };
                constraints.push(c);
            }
        }
        if idx + 2 < remaining.len() {
            remaining = &remaining[idx + 2..];
        } else {
            break;
        }
    }

    let validation = if constraints.is_empty() {
        None
    } else {
        Some(constraints)
    };
    (default_value, validation)
}

fn parse_constraint_tag(s: &str) -> Option<(&str, &str)> {
    for tag in ["ge ", "gt ", "le ", "lt "] {
        if let Some(rest) = s.strip_prefix(tag) {
            return Some((tag.trim(), rest));
        }
    }
    None
}

fn strip_null_from_str(s: &str) -> (String, bool) {
    let s = s.trim();
    // Find "/ null" at paren depth 0
    let mut depth = 0i32;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'/' if depth == 0 => {
                let rest = &s[i..];
                if rest.starts_with("/ null")
                    && (rest.len() == 6
                        || !rest.as_bytes()[6].is_ascii_alphanumeric())
                {
                    let before = s[..i].trim();
                    return (before.to_string(), true);
                }
            }
            _ => {}
        }
    }
    // Check "null / ..." prefix
    if s.starts_with("null /") {
        return (s[6..].trim().to_string(), true);
    }
    (s.to_string(), false)
}

fn parse_type(raw: &str) -> Type<'static> {
    let s = raw.trim().trim_end_matches([',', ';']);
    let s = clean_type_str(s);

    if s.is_empty() || s == "null" {
        return Type::Any;
    }

    // CDDL map: {*key => value} → treat as Object (HashMap<String, Value>)
    if s.starts_with('{') && s.contains("=>") {
        return Type::Extensible;
    }

    // Array
    if s.starts_with('[') && s.ends_with(']') {
        let inner = s[1..s.len() - 1]
            .trim()
            .trim_start_matches(['*', '+'])
            .trim();
        if inner.is_empty() {
            return Type::ArrayOf(Box::new(Type::Any));
        }
        return Type::ArrayOf(Box::new(parse_type(inner)));
    }

    // Range
    if s.contains("..") {
        let cleaned = s.replace("..", "");
        return if cleaned.contains('.') {
            Type::Number
        } else if s.starts_with('-') {
            Type::Integer
        } else {
            Type::UnsignedInteger
        };
    }

    // Nullable
    if s.ends_with("/ null") {
        return parse_type(s[..s.len() - 6].trim());
    }
    if s.starts_with("null /") {
        return parse_type(s[6..].trim());
    }

    // Contains / → string enum or type union
    if s.contains('/') {
        let parts: Vec<&str> = s.split('/').map(|p| p.trim()).filter(|p| !p.is_empty()).collect();
        let string_parts: Vec<&str> = parts
            .iter()
            .filter(|p| p.starts_with('"') && p.ends_with('"'))
            .map(|p| &p[1..p.len() - 1])
            .collect();
        if !string_parts.is_empty()
            && string_parts.len() == parts.len()
            && string_parts.len() >= 2
            && string_parts.iter().all(|v| is_valid_variant_name(v))
        {
            return Type::Enum(
                string_parts
                    .into_iter()
                    .map(|v| Variant {
                        description: None,
                        name: o(v),
                    })
                    .collect(),
            );
        }
        if parts.iter().all(|p| p.starts_with('"')) {
            return Type::String;
        }
        return Type::Object;
    }

    match s {
        "text" => Type::String,
        "bool" | "true" | "false" => Type::Boolean,
        "number" | "float" => Type::Number,
        "js-uint" => Type::UnsignedInteger,
        "js-int" => Type::Integer,
        "any" => Type::Any,
        _ if s.starts_with('"') && s.ends_with('"') => {
            let val = &s[1..s.len() - 1];
            if is_valid_variant_name(val) {
                Type::Enum(vec![Variant { description: None, name: o(val) }])
            } else {
                Type::String
            }
        }
        _ => Type::Ref(type_ref(s)),
    }
}

/// Repeatedly strip constraints and outer parens until stable.
fn clean_type_str(s: &str) -> &str {
    let mut s = s.trim();
    loop {
        let prev = s;
        s = strip_outer_parens(s).trim();
        s = strip_constraint(s).trim();
        if s == prev {
            break;
        }
    }
    s
}

fn strip_constraint(s: &str) -> &str {
    let mut depth = 0i32;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b' ' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b'.' => {
                let rest = &s[i..];
                for marker in [" .default", " .ge ", " .gt ", " .le ", " .lt "] {
                    if rest.starts_with(marker) {
                        return &s[..i];
                    }
                }
            }
            _ => {}
        }
    }
    s
}

fn strip_outer_parens(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        let mut depth = 0i32;
        let balanced = inner.chars().all(|c| {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        return false;
                    }
                }
                _ => {}
            }
            true
        }) && depth == 0;
        if balanced {
            return inner.trim();
        }
    }
    s
}

/// Parse `{ ( TypeA // TypeB // ... ) }` into TypeRef variants.
/// Returns None if the body isn't this pattern.
fn parse_inner_type_choice(body: &str) -> Option<Vec<TypeRef<'static>>> {
    let inner = body.trim()
        .strip_prefix('{')?
        .strip_suffix('}')?
        .trim()
        .strip_prefix('(')?
        .strip_suffix(')')?
        .trim();

    // Must contain // (group choice operator)
    if !inner.contains("//") {
        return None;
    }

    let parts: Vec<&str> = inner.split("//")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    if parts.len() < 2 {
        return None;
    }

    // Each part should be a type reference (no colons = no fields)
    if parts.iter().any(|p| p.contains(':')) {
        return None;
    }

    Some(parts.into_iter().map(type_ref).collect())
}

/// Parse inline choice group `(A // B)` inside a struct body.
/// Returns a flattened Param and a synthetic TypeDef (untagged enum).
fn parse_inline_choice(s: &str) -> Option<(Param<'static>, TypeDef<'static>)> {
    let inner = s.trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim();

    if !inner.contains("//") {
        return None;
    }

    let parts: Vec<&str> = inner.split("//")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    if parts.len() < 2 {
        return None;
    }

    // Check if parts are named field groups: (field: Type) // (field: Type)
    let has_named_fields = parts.iter().all(|p| {
        let p = p.trim_start_matches('(').trim_end_matches(')').trim();
        p.contains(':')
    });

    if has_named_fields {
        // Named field choice: ((coordinates: Type) // (error: Type))
        // Each part becomes a struct variant in the union
        let mut refs = Vec::new();
        for part in &parts {
            let inner_part = part.trim_start_matches('(').trim_end_matches(')').trim();
            if let Some((name, _)) = inner_part.split_once(':') {
                let name = name.trim().to_upper_camel_case();
                refs.push(TypeRef { module: None, name: o(&name) });
            }
        }
        // For named field choices, fall through to create union from field names
        // but these are harder — for now, just use the field names as variant names
        // and generate a TypeChoice typedef
        let union_name: String = refs.iter()
            .map(|r| r.name.as_ref().to_string())
            .collect::<Vec<_>>()
            .join("");
        let union_name = format!("{}Union", union_name);
        let field_snake = heck::ToSnakeCase::to_snake_case(union_name.as_str());

        let td = TypeDef {
            description: None,
            experimental: false,
            deprecated: false,
            name: o(&union_name),
            extends: Type::Object,
            parameters: Some(Item::TypeChoice(refs)),
            raw_name: o(""),
            is_circular_dep: false,
            direction: None,
        };

        let param = Param {
            description: None,
            experimental: false,
            deprecated: false,
            optional: false,
            r#type: Type::Ref(TypeRef { module: None, name: o(&union_name) }),
            name: o(&field_snake),
            raw_name: o(s),
            is_circular_dep: false,
            default_value: None,
            validation: None,
            flatten: true,
        };

        return Some((param, td));
    }

    // Simple type choice: (A // B) — type refs only
    let refs: Vec<TypeRef<'static>> = parts.into_iter().map(type_ref).collect();

    let union_name: String = refs.iter()
        .map(|r| r.name.as_ref().to_string())
        .collect::<Vec<_>>()
        .join("");
    let union_name = format!("{}Union", union_name);
    let field_snake = heck::ToSnakeCase::to_snake_case(union_name.as_str());

    let td = TypeDef {
        description: None,
        experimental: false,
        deprecated: false,
        name: o(&union_name),
        extends: Type::Object,
        parameters: Some(Item::TypeChoice(refs)),
        raw_name: o(""),
        is_circular_dep: false,
        direction: None,
    };

    let param = Param {
        description: None,
        experimental: false,
        deprecated: false,
        optional: false,
        r#type: Type::Ref(TypeRef { module: None, name: o(&union_name) }),
        name: o(&field_snake),
        raw_name: o(s),
        is_circular_dep: false,
        default_value: None,
        validation: None,
        flatten: true,
    };

    Some((param, td))
}

/// Hardcoded param overrides: (module, type_name) → params to upsert.
/// TODO: Suboptimal — ideally these would be detected at CDDL parsing level
/// rather than patched after the fact.
fn type_overrides() -> Vec<(&'static str, &'static str, Vec<Param<'static>>)> {
    vec![
        ("network", "RequestData", vec![
            Param {
                description: None,
                experimental: false,
                deprecated: false,
                optional: false,
                r#type: Type::Extensible,
                name: o("extensible"),
                raw_name: o("extensible"),
                is_circular_dep: false,
                default_value: None,
                validation: None,
                flatten: true,
            },
        ]),
    ]
}

fn apply_overrides(modules: &mut HashMap<String, Module<'static>>) {
    for (mod_name, type_name, override_params) in type_overrides() {
        let Some(module) = modules.get_mut(mod_name) else { continue };
        let td = module.types.iter_mut().find(|t| t.name.as_ref() == type_name);
        if let Some(td) = td {
            let params = td.parameters.get_or_insert_with(|| Item::Properties(vec![]));
            if let Item::Properties(existing) = params {
                for p in override_params {
                    if let Some(pos) = existing.iter().position(|e| e.name == p.name) {
                        existing[pos] = p;
                    } else {
                        existing.push(p);
                    }
                }
            }
        } else {
            module.types.push(TypeDef {
                description: None,
                experimental: false,
                deprecated: false,
                name: o(type_name),
                extends: Type::Object,
                parameters: Some(Item::Properties(override_params)),
                raw_name: o(format!("{}.{}", mod_name, type_name)),
                is_circular_dep: false,
                direction: None,
            });
        }
    }
}

/// Known types that cause circular dependency (infinite size) issues.
/// Format: "module.typename" (lowercase).
fn is_circular_dep(module: &str, type_name: &str) -> bool {
    matches!(
        (module, type_name),
        ("script", "NodeRemoteValue")
            | ("script", "NodeProperties")
    )
}

/// Walk all modules and mark params whose referenced type is circular.
fn mark_circular_deps(modules: &mut [Module<'static>]) {
    for module in modules.iter_mut() {
        let mod_name = module.name.to_string();
        let mark = |params: &mut [Param<'static>]| {
            for p in params.iter_mut() {
                if let Type::Ref(ref tr) = p.r#type {
                    let ref_mod = tr.module.as_deref().unwrap_or(&mod_name);
                    if is_circular_dep(ref_mod, tr.name.as_ref()) {
                        p.is_circular_dep = true;
                    }
                }
            }
        };
        for td in &mut module.types {
            if let Some(Item::Properties(ref mut params)) = td.parameters {
                mark(params);
            }
        }
        for cmd in &mut module.commands {
            mark(&mut cmd.params);
        }
        for ev in &mut module.events {
            mark(&mut ev.parameters);
        }
        for cr in &mut module.command_results {
            mark(&mut cr.parameters);
        }
    }
}

/// Parse string enum variants. Returns empty vec if any variant would be an invalid ident.
fn parse_string_enum(body: &str) -> Vec<Variant<'static>> {
    let variants: Vec<&str> = body
        .split('/')
        .map(|p| p.trim().trim_end_matches(';').trim())
        .filter(|p| p.starts_with('"') && p.ends_with('"'))
        .map(|p| &p[1..p.len() - 1])
        .collect();

    if variants.iter().any(|v| !is_valid_variant_name(v)) {
        return vec![];
    }

    variants
        .into_iter()
        .map(|v| Variant {
            description: None,
            name: o(v),
        })
        .collect()
}
