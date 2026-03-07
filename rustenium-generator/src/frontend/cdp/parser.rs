//! Parser for the chrome pdl files
//!
//! All regexp's are copied from pdl.py in the chromium source tree.

use crate::backend::base_types::{
    Command, CommandMethod, CommandResult, Event, Item, Module, Param, Protocol, Redirect, Type,
    TypeDef, Variant, Version,
};
use crate::frontend::cdp::dep::is_circular_dep;
use crate::frontend::cdp::error::*;
use regex::Regex;
use std::borrow::Cow;
use std::sync::OnceLock;

/// Helper macro to create `&'static Regex`
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: OnceLock<Regex> = OnceLock::new();
        #[allow(clippy::regex_creation_in_loops)]
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug)]
enum ModuleProperty<'a> {
    Type(TypeDef<'a>),
    Command(Command<'a>),
    CommandResult(CommandResult<'a>),
    Event(Event<'a>),
}

fn add_module_property_to_module<'a>(mod_prop: ModuleProperty<'a>, module: &mut Module<'a>) {
    match mod_prop {
        ModuleProperty::Type(t) => module.types.push(t),
        ModuleProperty::Command(c) => {
            let name = c.method.name.clone();
            module.commands.push(c);
            // Ensure every command has a corresponding result entry
            let result_name = Cow::Owned(format!("{}Result", name));
            if !module.command_results.iter().any(|cr| cr.name == result_name) {
                module.command_results.push(CommandResult {
                    description: Some(Cow::Owned(format!("Result for {} command", name))),
                    name: result_name,
                    parameters: vec![],
                    raw_name: Cow::Owned(String::new()),
                });
            }
        }
        ModuleProperty::CommandResult(cr) => {
            if let Some(existing) = module.command_results.iter_mut().find(|r| r.name == cr.name) {
                *existing = cr;
            } else {
                module.command_results.push(cr);
            }
        }
        ModuleProperty::Event(e) => module.events.push(e),
    }
}
fn add_param_to_module_property<'a>(mod_prop: &mut ModuleProperty<'a>, param: Param<'a>) {
    match mod_prop {
        ModuleProperty::Command(c) => c.params.push(param),
        ModuleProperty::CommandResult(cr) => cr.parameters.push(param),
        ModuleProperty::Event(e) => e.parameters.push(param),
        ModuleProperty::Type(t) => {
            if let Some(Item::Properties(props)) = t.parameters.as_mut() {
                props.push(param)
            }
        }
    }
}

/// Parse the input into a [`Protocol`].
///
/// Rewrite of the Python script from the Chromium source tree.
///
///  See: https://chromium.googlesource.com/deps/inspector_protocol/+/refs/heads/master/pdl.py
pub fn parse_pdl<'a>(input: &'a str) -> Result<Protocol<'a>, Error> {
    let mut protocol = Protocol::default();
    let mut description: Option<String> = None;
    let mut version = None;

    let mut mod_prop = None;
    let mut has_enum_member = false;

    for (idx, line) in input.lines().enumerate() {
        let line_num = idx + 1;

        let trim_line = line.trim();
        if trim_line.starts_with('#') {
            if let Some(desc) = description.as_mut() {
                desc.push('\n');
                desc.push_str(trim_line[1..].trim_start());
            } else {
                description = Some(trim_line[1..].trim_start().to_string());
            }
            continue;
        }

        if trim_line.is_empty() {
            continue;
        }

        if let Some(caps) = regex!("^(experimental )?(deprecated )?domain (.*)").captures(line) {
            if let Some(module) = protocol.modules.last_mut() {
                if let Some(mod_prop) = mod_prop.take() {
                    add_module_property_to_module(mod_prop, module);
                }
            }

            let module = Module {
                description: description.take().map(Cow::Owned),
                experimental: caps.get(1).is_some(),
                deprecated: caps.get(2).is_some(),
                name: borrowed!(caps.get(3), "line {}: No name for domain", line_num)?,
                dependencies: vec![],
                types: vec![],
                commands: vec![],
                command_results: vec![],
                events: vec![],
            };
            protocol.modules.push(module);
            continue;
        }

        if let Some(caps) = regex!("^  depends on ([^\\s]+)").captures(line) {
            protocol
                .modules
                .last_mut()
                .ok_or_else(|| format_err!("line {}: missing module declaration", line_num))?
                .dependencies
                .push(borrowed!(caps.get(1)).unwrap());
            continue;
        }

        // type
        if let Some(caps) =
            regex!("^  (experimental )?(deprecated )?type (.*) extends (array of )?([^\\s]+)")
                .captures(line)
        {
            let module = protocol
                .modules
                .last_mut()
                .ok_or_else(|| format_err!("line {}: missing module declaration", line_num))?;

            if let Some(mod_prop) = mod_prop.take() {
                add_module_property_to_module(mod_prop, module);
            }
            let name = borrowed!(caps.get(3)).unwrap();
            let ty = TypeDef {
                description: description.take().map(Cow::Owned),
                experimental: caps.get(1).is_some(),
                deprecated: caps.get(2).is_some(),
                raw_name: Cow::Owned(format!("{}.{}", module.name, name)),
                is_circular_dep: is_circular_dep(&module.name, name.as_ref()),
                name,
                extends: Type::new(caps.get(5).unwrap().as_str(), caps.get(4).is_some()),
                parameters: None,
                direction: None,
            };
            mod_prop = Some(ModuleProperty::Type(ty));
            continue;
        }

        // cmd or event
        if let Some(caps) =
            regex!("^  (experimental )?(deprecated )?(command|event) (.*)").captures(line)
        {
            let module = protocol
                .modules
                .last_mut()
                .ok_or_else(|| format_err!("line {}: missing module declaration", line_num))?;
            if let Some(mod_prop) = mod_prop.take() {
                add_module_property_to_module(mod_prop, module);
            }
            let name = borrowed!(caps.get(4)).unwrap();
            if Some("command") == caps.get(3).map(|m| m.as_str()) {
                let is_circ = is_circular_dep(&module.name, name.as_ref());
                let cmd = Command {
                    method: CommandMethod {
                        description: description.take().map(Cow::Owned),
                        experimental: caps.get(1).is_some(),
                        deprecated: caps.get(2).is_some(),
                        redirect: None,
                        raw_name: Cow::Owned(format!("{}.{}", module.name, name)),
                        name,
                    },
                    params: vec![],
                    returns: vec![],
                    is_circular_dep: is_circ,
                    direction: None,
                };
                mod_prop = Some(ModuleProperty::Command(cmd));
            } else {
                let ev = Event {
                    description: description.take().map(Cow::Owned),
                    experimental: caps.get(1).is_some(),
                    deprecated: caps.get(2).is_some(),
                    parameters: vec![],
                    raw_name: Cow::Owned(format!("{}.{}", module.name, name)),
                    is_circular_dep: is_circular_dep(&module.name, name.as_ref()),
                    name,
                    direction: None,
                };
                mod_prop = Some(ModuleProperty::Event(ev));
            };
            continue;
        }

        // mod_prop to params / returns / properties
        if let Some(caps) = regex!(
            "^      (experimental )?(deprecated )?(optional )?(array of )?([^\\s]+) ([^\\s]+)"
        )
        .captures(line)
        {
            let module = protocol
                .modules
                .last_mut()
                .ok_or_else(|| format_err!("line {}: missing module declaration", line_num))?;
            let name = borrowed!(caps.get(6)).unwrap();
            let param = Param {
                description: description.take().map(Cow::Owned),
                experimental: caps.get(1).is_some(),
                deprecated: caps.get(2).is_some(),
                optional: caps.get(3).is_some(),
                raw_name: Cow::Owned(format!("{}.{}", module.name, name)),
                is_circular_dep: is_circular_dep(&module.name, name.as_ref()),
                name,
                r#type: Type::new(caps.get(5).unwrap().as_str(), caps.get(4).is_some()),
                default_value: None,
                validation: None,
            };
            add_param_to_module_property(
                mod_prop.as_mut().ok_or_else(|| {
                    format_err!(
                        "line {}: parameter {} has no declared mod_prop section",
                        line_num,
                        param.name
                    )
                })?,
                param,
            );
            if Some("enum") == caps.get(5).map(|m| m.as_str()) {
                has_enum_member = true;
            }
            continue;
        }

        // parameters, returns, properties definition
        if let Some(caps) = regex!("^    (parameters|returns|properties)").captures(line) {
            match caps.get(1).unwrap().as_str() {
                "returns" => {
                    let mut command_name = Cow::Borrowed("");
                    if let Some(mod_prop) = mod_prop.take() {
                        if let ModuleProperty::Command(ref mod_prop) = mod_prop {
                            command_name = mod_prop.method.name.clone();
                        }
                        add_module_property_to_module(
                            mod_prop,
                            protocol.modules.last_mut().ok_or_else(|| {
                                format_err!("line {}: mod_prop has no parent item", line_num)
                            })?,
                        );
                    }
                    mod_prop = Some(ModuleProperty::CommandResult(CommandResult {
                        description: Some(Cow::Owned(format!(
                            "Return object for {} command",
                            command_name
                        ))),
                        name: Cow::Owned(format!("{}Result", command_name)),
                        parameters: vec![],
                        raw_name: Cow::Owned(String::new()),
                    }))
                }
                "parameters" | "properties" => {
                    match mod_prop {
                        // typedef is the only one with the optional parameters
                        Some(ModuleProperty::Type(ref mut mod_prop)) => {
                            mod_prop.parameters = Some(Item::Properties(vec![]))
                        }
                        _ => (),
                    }
                }
                _ => unreachable!(),
            }
            continue;
        }

        // enum
        if line.starts_with("    enum") {
            has_enum_member = false;
            if let Some(ModuleProperty::Type(ty)) = mod_prop.as_mut() {
                if ty.parameters.is_none() {
                    ty.parameters = Some(Item::Enum(vec![]));
                    continue;
                } else {
                    bail!("line {}: enum declaration not allowed", line_num);
                }
            } else {
                bail!("line {}: enum declaration not allowed", line_num);
            }
        }

        // version
        if line.starts_with("version") {
            protocol.description = description.take().map(Cow::Owned);
            version = Some(Version::default());
            continue;
        }

        if let Some(caps) = regex!("^  major (\\d+)").captures(line) {
            let v = version
                .as_mut()
                .ok_or_else(|| format_err!("line {}: version must be declared first", line_num))?;
            v.major = caps.get(1).unwrap().as_str().parse().unwrap();
            continue;
        }

        if let Some(caps) = regex!("^  minor (\\d+)").captures(line) {
            let v = version
                .as_mut()
                .ok_or_else(|| format_err!("line {}: missing version declaration", line_num))?;
            v.minor = caps.get(1).unwrap().as_str().parse().unwrap();
            continue;
        }

        // redirect
        if let Some(caps) = regex!("^    redirect ([^\\s]+)").captures(line) {
            let mut redirect = Redirect {
                description: description.take().map(Cow::Owned),
                module: borrowed!(caps.get(1)).unwrap(),
                name: None,
            };
            if let Some(desc) = description.as_ref() {
                if let Some(caps) = regex!("^Use '([^']+)' instead$").captures(desc) {
                    let name = caps.get(1).unwrap().as_str();
                    redirect.name = name.rsplit('.').next().map(str::to_string).map(Cow::Owned);
                }
            }
            match mod_prop
                .as_mut()
                .ok_or_else(|| format_err!("line {}: missing item declaration", line_num))?
            {
                ModuleProperty::Command(cmd) => {
                    cmd.method.redirect = Some(redirect);
                }
                _ => bail!("line {}: can't add redirect here", line_num),
            }
            continue;
        }

        // enum literal
        // the difference between param being an enum vs typedef parameters-enum is param enum is a list of potential variations
        // typedef parameters-enum indicates the typedef itself is an enum with variations.
        // e.g. a SessionType of multiple variations and SessionParameters with url param as multiple variations
        if regex!("^      (  )?[^\\n\\t]+$").is_match(line) {
            if has_enum_member {
                let mut handle_enum_member = |param: Option<&mut Param<'a>>| {
                    let param = param.ok_or_else(|| {
                        format_err!("line {}: missing parameter declaration", line_num)
                    })?;
                    if let Type::Enum(ref mut vars) = param.r#type {
                        vars.push(Variant {
                            description: description.take().map(Cow::Owned),
                            name: Cow::Borrowed(trim_line),
                        });
                    } else {
                        bail!("line {}: missing enum declaration", line_num)
                    }
                    Ok(())
                };
                match mod_prop
                    .as_mut()
                    .ok_or_else(|| format_err!("line {}: missing mod_prop declaration", line_num))?
                {
                    ModuleProperty::Command(c) => handle_enum_member(c.params.last_mut()),
                    ModuleProperty::CommandResult(cr) => {
                        handle_enum_member(cr.parameters.last_mut())
                    }
                    ModuleProperty::Event(e) => handle_enum_member(e.parameters.last_mut()),
                    ModuleProperty::Type(t) => {
                        if let Some(Item::Properties(props)) = t.parameters.as_mut() {
                            handle_enum_member(props.last_mut())
                        } else {
                            Err(format_err!(
                                "line {}: invalid - typedef is of type enum",
                                line_num
                            ))
                        }
                    }
                }?;
            } else {
                match mod_prop
                    .as_mut()
                    .ok_or_else(|| format_err!("line {}: missing item declaration", line_num))?
                {
                    ModuleProperty::Type(ty) => {
                        if let Some(Item::Enum(vars)) = ty.parameters.as_mut() {
                            vars.push(Variant {
                                description: description.take().map(Cow::Owned),
                                name: Cow::Borrowed(trim_line),
                            });
                        } else {
                            bail!("line {}: missing enum declaration", line_num)
                        }
                    }
                    _ => bail!("line {}: missing enum declaration", line_num),
                }
            }
            continue;
        }
        bail!("line {}: unknown token `{}`", line_num, line)
    }

    if let Some(module) = protocol.modules.last_mut() {
        if let Some(mod_prop) = mod_prop.take() {
            add_module_property_to_module(mod_prop, module);
        }
    }
    protocol.version = version.ok_or_else(|| format_err!("Missing version"))?;
    Ok(protocol)
}
