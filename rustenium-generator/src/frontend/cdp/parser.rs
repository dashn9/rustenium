//! Parser for the chrome pdl files
//!
//! All regexp's are copied from pdl.py in the chromium source tree.

use std::sync::OnceLock;
use regex::Regex;
use crate::backend::base_types::{Command, CommandResult, Event, Module, Param, Protocol, Type, TypeDef};
use crate::frontend::cdp::dep::is_circular_dep;
use crate::frontend::cdp::error::*;
use crate::frontend::cdp::*;
use std::borrow::Cow;

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

/// Parse the input into a [`Protocol`].
///
/// Rewrite of the Python script from the Chromium source tree.
///
///  See: https://chromium.googlesource.com/deps/inspector_protocol/+/refs/heads/master/pdl.py
pub fn parse_pdl(input: &str) -> Result<Protocol<'_>, Error> {
    let mut protocol = Protocol::default();
    let mut description: Option<String> = None;
    let mut version = None;

    let mut mod_prop = None;

    for (idx, line) in input.lines().enumerate() {
        let line_num = idx + 1;

        let trim_line = line.trim();
        if trim_line.starts_with('#') {
            if let Some(desc) = description.as_mut() {
                desc.push('\n');
                desc.extend(trim_line[1..].trim_start().to_string());
            } else {
                description = Some(
                    trim_line[1..].trim_start().to_string()
                );
            }
            continue;
        }

        if trim_line.is_empty() {
            continue;
        }

        if let Some(caps) = regex!("^(experimental )?(deprecated )?module (.*)").captures(line) {
            if let Some(module) = protocol.modules.last_mut() {
                if let Some(mut element) = element.take() {
                    if let Some(mod_prop) = mod_prop.take() {
                        element.add_member(mod_prop)?;
                    }
                    element.consume(module);
                }
            }

            let module = Module {
                description: description.take().map(Cow::Owned),
                experimental: caps.get(1).is_some(),
                deprecated: caps.get(2).is_some(),
                name: borrowed!(caps.get(3), "line {}: No name for module", line_num)?,
                dependencies: vec![],
                types: vec![],
                commands: vec![],
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

            if let Some(mut el) = element.take() {
                if let Some(mod_prop) = mod_prop.take() {
                    el.add_member(mod_prop)?;
                }
                el.consume(module);
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
                item: None,
            };
            element = Some(Element::Type(ty));
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
            if let Some(mut el) = element.take() {
                if let Some(mod_prop) = mod_prop.take() {
                    el.add_member(mod_prop)?;
                }
                el.consume(module);
            }
            let name = borrowed!(caps.get(4)).unwrap();
            if Some("command") == caps.get(3).map(|m| m.as_str()) {
                let cmd = Command {
                    description: description.take().map(Cow::Owned),
                    experimental: caps.get(1).is_some(),
                    deprecated: caps.get(2).is_some(),
                    parameters: vec![],
                    returns: vec![],
                    redirect: None,
                    raw_name: Cow::Owned(format!("{}.{}", module.name, name)),
                    is_circular_dep: is_circular_dep(&module.name, name.as_ref()),
                    name,
                };
                element = Some(Element::Commnad(cmd));
            } else {
                let ev = Event {
                    description: description.take().map(Cow::Owned),
                    experimental: caps.get(1).is_some(),
                    deprecated: caps.get(2).is_some(),
                    parameters: vec![],
                    raw_name: Cow::Owned(format!("{}.{}", module.name, name)),
                    is_circular_dep: is_circular_dep(&module.name, name.as_ref()),
                    name,
                };
                element = Some(Element::Event(ev));
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
            };
            match mod_prop.as_mut().ok_or_else(|| {
                format_err!(
                    "line {}: parameter {} has no declared mod_prop section",
                    line_num,
                    param.name
                )
            })? {
                Member::Parameters(params) => params.push(param),
                Member::Returns(params) => params.push(param),
                Member::Properties(params) => params.push(param),
            };
            if Some("enum") == caps.get(5).map(|m| m.as_str()) {
                member_enum = true;
            }
            continue;
        }

        // parameters, returns, properties definition
        if let Some(caps) = regex!("^    (parameters|returns|properties)").captures(line) {
            // if let Some(mod_prop) = mod_prop.take() {
            //     element
            //         .as_mut()
            //         .ok_or_else(|| format_err!("line {}: mod_prop has no parent item", line_num))?
            //         .add_member(mod_prop)?;
            // }
            // match caps.get(1).unwrap().as_str() {
            //     "parameters" => mod_prop = Some(Member::Parameters(vec![])),
            //     "returns" => mod_prop = Some(Member::Returns(vec![])),
            //     "properties" => mod_prop = Some(Member::Properties(vec![])),
            //     _ => unreachable!(),
            // }
            continue;
        }

        // enum
        if line.starts_with("    enum") {
            member_enum = false;
            if let Some(Element::Type(ref mut ty)) = element.as_mut() {
                if ty.item.is_none() {
                    ty.item = Some(Item::Enum(vec![]));
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
            match element
                .as_mut()
                .ok_or_else(|| format_err!("line {}: missing item declaration", line_num))?
            {
                Element::Commnad(cmd) => {
                    cmd.redirect = Some(redirect);
                }
                _ => bail!("line {}: can't add redirect here", line_num),
            }
            continue;
        }

        // enum literal
        if regex!("^      (  )?[^\\n\\t]+$").is_match(line) {
            if member_enum {
                let param = match mod_prop
                    .as_mut()
                    .ok_or_else(|| format_err!("line {}: missing mod_prop declaration", line_num))?
                {
                    Member::Parameters(params) => params.last_mut(),
                    Member::Returns(params) => params.last_mut(),
                    Member::Properties(params) => params.last_mut(),
                }
                .ok_or_else(|| format_err!("line {}: missing parameter declaration", line_num))?;

                if let Type::Enum(ref mut vars) = param.r#type {
                    vars.push(Variant {
                        description: description.take().map(Cow::Owned),
                        name: Cow::Borrowed(trim_line),
                    });
                } else {
                    bail!("line {}: missing enum declaration", line_num)
                }
            } else {
                match element
                    .as_mut()
                    .ok_or_else(|| format_err!("line {}: missing item declaration", line_num))?
                {
                    Element::Type(ty) => {
                        if let Some(Item::Enum(vars)) = ty.item.as_mut() {
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
        if let Some(mut element) = element.take() {
            if let Some(mod_prop) = mod_prop.take() {
                element.add_member(mod_prop)?;
            }
            element.consume(module);
        }
    }

    protocol.version = version.ok_or_else(|| format_err!("Missing version"))?;
    Ok(protocol)
}
