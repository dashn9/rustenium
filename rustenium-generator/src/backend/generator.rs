use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::backend::base_types::{Module, Param, Protocol, Type, TypeRef, Variant};
use crate::backend::builder::Builder;
use crate::backend::event;
use crate::backend::types::{FieldDefinition, FieldType, ModuleDatatype};

#[derive(Debug, Clone)]
pub struct Generator {
    with_experimental: bool,
    with_deprecated: bool,
    out_dir: Option<PathBuf>,
    /// All enums across modules with qualified names `<module>.<name>`
    enums: HashSet<String>,
    /// Maps module name (original case, e.g. "Runtime") to protocol snake name (e.g. "js_protocol")
    module_protocol_map: HashMap<String, String>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            with_experimental: true,
            with_deprecated: false,
            out_dir: None,
            enums: HashSet::new(),
            module_protocol_map: HashMap::new(),
        }
    }
}

pub struct ModuleParts {
    pub types: TokenStream,
    pub commands: TokenStream,
    pub command_builders: TokenStream,
    pub type_builders: TokenStream,
    pub results: TokenStream,
    pub events: TokenStream,
    pub type_leaves: Vec<Ident>,
    pub command_leaves: Vec<Ident>,
    pub event_leaves: Vec<Ident>,
}

impl Generator {
    pub fn out_dir<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.out_dir = Some(path.into());
        self
    }

    pub fn compile_protocols(&mut self, protocols: &[Protocol]) -> io::Result<()> {
        let target_base: PathBuf = self.out_dir.clone().map(Ok).unwrap_or_else(|| {
            std::env::var_os("OUT_DIR")
                .ok_or_else(|| Error::new(ErrorKind::Other, "OUT_DIR environment variable is not set"))
                .map(Into::into)
        })?;

        fs::create_dir_all(&target_base)
            .unwrap_or_else(|e| panic!("Unable to create directory {}: {}", target_base.display(), e));

        let macros_path = target_base.join("macros.rs");
        fs::write(&macros_path, event::GROUP_ENUM_MACRO)
            .unwrap_or_else(|e| panic!("Unable to write {}: {}", macros_path.display(), e));

        // Build module_protocol_map
        self.module_protocol_map.clear();
        for protocol in protocols.iter() {
            let ps = protocol.name.unwrap_or("").to_snake_case();
            for module in &protocol.modules {
                self.module_protocol_map.insert(module.name.to_string(), ps.clone());
            }
        }

        // (proto_snake, proto_camel, modules, protocol_mod_content)
        let mut protocol_infos: Vec<(String, String, Vec<(String, String, Vec<Ident>, Vec<Ident>, Vec<Ident>)>, TokenStream)> = Vec::new();

        let flat = protocols.len() == 1 && protocols[0].name.is_none();

        for protocol in protocols.iter() {
            let version = format!("{}.{}", protocol.version.major, protocol.version.minor);
            let protocol_snake = protocol.name.unwrap_or("").to_snake_case();

            let protocol_dir = if flat {
                target_base.clone()
            } else {
                let dir = target_base.join(&protocol_snake);
                fs::create_dir_all(&dir)
                    .unwrap_or_else(|e| panic!("Unable to create directory {}: {}", dir.display(), e));
                dir
            };

            let mut module_infos: Vec<(String, String, Vec<Ident>, Vec<Ident>, Vec<Ident>)> = Vec::new();

            let with_deprecated = self.with_deprecated;
            let with_experimental = self.with_experimental;
            let modules_to_process: Vec<_> = protocol.modules.iter()
                .filter(|d| with_deprecated || !d.deprecated)
                .filter(|d| with_experimental || !d.experimental)
                .collect();

            for module in &modules_to_process {
                let mod_name = module.name.to_snake_case();
                let parts = self.generate_module_files(module, &protocol_snake);

                let module_dir = protocol_dir.join(&mod_name);
                fs::create_dir_all(&module_dir)
                    .unwrap_or_else(|e| panic!("Unable to create directory {}: {}", module_dir.display(), e));

                let mut sub_mods = Vec::new();
                for (file_name, content) in [
                    ("types", parts.types),
                    ("commands", parts.commands),
                    ("command_builders", parts.command_builders),
                    ("type_builders", parts.type_builders),
                    ("results", parts.results),
                    ("events", parts.events),
                ] {
                    if !content.is_empty() {
                        let file_path = module_dir.join(format!("{file_name}.rs"));
                        fs::write(&file_path, content.to_string())
                            .unwrap_or_else(|e| panic!("Unable to write {}: {}", file_path.display(), e));
                        sub_mods.push(file_name.to_string());
                    }
                }

                let sub_mod_decls: Vec<_> = sub_mods.iter().map(|name| {
                    let mod_ident = format_ident!("{}", name);
                    quote! { pub mod #mod_ident; }
                }).collect();

                let module_mod_content = quote! { #(#sub_mod_decls)* };
                let module_mod_path = module_dir.join("mod.rs");
                fs::write(&module_mod_path, module_mod_content.to_string())
                    .unwrap_or_else(|e| panic!("Unable to write {}: {}", module_mod_path.display(), e));

                module_infos.push((mod_name, module.name.to_upper_camel_case(), parts.type_leaves, parts.command_leaves, parts.event_leaves));
            }

            let type_entries: Vec<_> = module_infos.iter()
                .filter(|(_, _, tl, _, _)| !tl.is_empty())
                .map(|(name, mcamel, _, _, _)| {
                    let mod_id = format_ident!("{}", name);
                    let var_id = format_ident!("{}", mcamel);
                    let enum_id = format_ident!("{}Types", mcamel);
                    (var_id, quote!(#mod_id::types::#enum_id))
                })
                .collect();

            let cmd_entries: Vec<_> = module_infos.iter()
                .filter(|(_, _, _, cl, _)| !cl.is_empty())
                .map(|(name, mcamel, _, _, _)| {
                    let mod_id = format_ident!("{}", name);
                    let var_id = format_ident!("{}", mcamel);
                    let enum_id = format_ident!("{}Commands", mcamel);
                    (var_id, quote!(#mod_id::commands::#enum_id))
                })
                .collect();

            let evt_entries: Vec<_> = module_infos.iter()
                .filter(|(_, _, _, _, el)| !el.is_empty())
                .map(|(name, mcamel, _, _, _)| {
                    let mod_id = format_ident!("{}", name);
                    let var_id = format_ident!("{}", mcamel);
                    let enum_id = format_ident!("{}Events", mcamel);
                    (var_id, quote!(#mod_id::events::#enum_id))
                })
                .collect();

            let protocol_camel = protocol.name.unwrap_or("unknown").to_upper_camel_case();

            // Build transitive entries: leaf => protocol_enum via module_enum
            let mut proto_transitive: Vec<(TokenStream, TokenStream, TokenStream)> = Vec::new();
            let proto_type_name = format_ident!("{}Types", protocol_camel);
            let proto_cmd_name = format_ident!("{}Commands", protocol_camel);
            let proto_evt_name = format_ident!("{}Events", protocol_camel);
            for (mod_name, mcamel, type_leaves, cmd_leaves, evt_leaves) in &module_infos {
                let mod_id = format_ident!("{}", mod_name);
                for leaf in type_leaves {
                    let me = format_ident!("{}Types", mcamel);
                    proto_transitive.push((quote!(#mod_id::types::#leaf), quote!(#mod_id::types::#me), quote!(#proto_type_name)));
                }
                for leaf in cmd_leaves {
                    let me = format_ident!("{}Commands", mcamel);
                    proto_transitive.push((quote!(#mod_id::commands::#leaf), quote!(#mod_id::commands::#me), quote!(#proto_cmd_name)));
                }
                for leaf in evt_leaves {
                    let me = format_ident!("{}Events", mcamel);
                    proto_transitive.push((quote!(#mod_id::events::#leaf), quote!(#mod_id::events::#me), quote!(#proto_evt_name)));
                }
            }

            let protocol_type_group = event::group_enum_closed(&proto_type_name, &type_entries);
            let protocol_cmd_group = event::group_enum_closed(&proto_cmd_name, &cmd_entries);
            let protocol_evt_group = event::group_enum_open(&proto_evt_name, &evt_entries);
            let protocol_transitive = event::impl_from_transitive(&proto_transitive);

            let mod_decls: Vec<_> = module_infos.iter().map(|(name, _, _, _, _)| {
                let mod_ident = format_ident!("{}", name);
                quote! { pub mod #mod_ident; }
            }).collect();

            let protocol_mod_content = quote! {
                #(#mod_decls)*

                pub const VERSION: &str = #version;

                #protocol_type_group
                #protocol_cmd_group
                #protocol_evt_group
                #protocol_transitive
            };

            if !flat {
                let protocol_mod_path = protocol_dir.join("mod.rs");
                fs::write(&protocol_mod_path, protocol_mod_content.to_string())
                    .unwrap_or_else(|e| panic!("Unable to write {}: {}", protocol_mod_path.display(), e));
            }

            protocol_infos.push((protocol_snake, protocol_camel, module_infos, protocol_mod_content));
        }

        let top_mod_content = if flat {
            // Single unnamed protocol: merge protocol content directly into lib.rs
            let (_, _, _, proto_content) = &protocol_infos[0];
            let serde_import = quote! { use serde::{Serialize, Deserialize}; };
            quote! {
                #serde_import

                #[macro_use]
                mod macros;

                #[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
                pub struct Binary(String);

                impl AsRef<str> for Binary { fn as_ref(&self) -> &str { self.0.as_str() } }
                impl AsRef<[u8]> for Binary { fn as_ref(&self) -> &[u8] { self.0.as_bytes() } }
                impl From<Binary> for String { fn from(b: Binary) -> String { b.0 } }
                impl From<String> for Binary { fn from(expr: String) -> Self { Self(expr) } }

                pub trait CommandResult {
                    type Result: serde::de::DeserializeOwned + std::fmt::Debug;
                    fn result_from_value(result: serde_json::Value) -> serde_json::Result<Self::Result> {
                        serde_json::from_value(result)
                    }
                }

                #proto_content
            }
        } else {
            let has_types = |modules: &Vec<(String, String, Vec<Ident>, Vec<Ident>, Vec<Ident>)>| modules.iter().any(|(_, _, tl, _, _)| !tl.is_empty());
            let has_cmds = |modules: &Vec<(String, String, Vec<Ident>, Vec<Ident>, Vec<Ident>)>| modules.iter().any(|(_, _, _, cl, _)| !cl.is_empty());
            let has_evts = |modules: &Vec<(String, String, Vec<Ident>, Vec<Ident>, Vec<Ident>)>| modules.iter().any(|(_, _, _, _, el)| !el.is_empty());

            let top_type_entries: Vec<_> = protocol_infos.iter()
                .filter(|(_, _, modules, _)| has_types(modules))
                .map(|(name, pcamel, _, _)| {
                    let mod_id = format_ident!("{}", name);
                    let var_id = format_ident!("{}", pcamel);
                    let enum_id = format_ident!("{}Types", pcamel);
                    (var_id, quote!(#mod_id::#enum_id))
                })
                .collect();

            let top_cmd_entries: Vec<_> = protocol_infos.iter()
                .filter(|(_, _, modules, _)| has_cmds(modules))
                .map(|(name, pcamel, _, _)| {
                    let mod_id = format_ident!("{}", name);
                    let var_id = format_ident!("{}", pcamel);
                    let enum_id = format_ident!("{}Commands", pcamel);
                    (var_id, quote!(#mod_id::#enum_id))
                })
                .collect();

            let top_evt_entries: Vec<_> = protocol_infos.iter()
                .filter(|(_, _, modules, _)| has_evts(modules))
                .map(|(name, pcamel, _, _)| {
                    let mod_id = format_ident!("{}", name);
                    let var_id = format_ident!("{}", pcamel);
                    let enum_id = format_ident!("{}Events", pcamel);
                    (var_id, quote!(#mod_id::#enum_id))
                })
                .collect();

            let mut top_transitive: Vec<(TokenStream, TokenStream, TokenStream)> = Vec::new();
            for (proto_name, pcamel, modules, _) in &protocol_infos {
                let proto_id = format_ident!("{}", proto_name);
                let proto_type_enum = format_ident!("{}Types", pcamel);
                let proto_cmd_enum = format_ident!("{}Commands", pcamel);
                let proto_evt_enum = format_ident!("{}Events", pcamel);

                for (mod_name, mcamel, type_leaves, cmd_leaves, evt_leaves) in modules {
                    let mod_id = format_ident!("{}", mod_name);
                    if !type_leaves.is_empty() {
                        let me = format_ident!("{}Types", mcamel);
                        top_transitive.push((quote!(#proto_id::#mod_id::types::#me), quote!(#proto_id::#proto_type_enum), quote!(Type)));
                        for leaf in type_leaves { top_transitive.push((quote!(#proto_id::#mod_id::types::#leaf), quote!(#proto_id::#proto_type_enum), quote!(Type))); }
                    }
                    if !cmd_leaves.is_empty() {
                        let me = format_ident!("{}Commands", mcamel);
                        top_transitive.push((quote!(#proto_id::#mod_id::commands::#me), quote!(#proto_id::#proto_cmd_enum), quote!(Command)));
                        for leaf in cmd_leaves { top_transitive.push((quote!(#proto_id::#mod_id::commands::#leaf), quote!(#proto_id::#proto_cmd_enum), quote!(Command))); }
                    }
                    if !evt_leaves.is_empty() {
                        let me = format_ident!("{}Events", mcamel);
                        top_transitive.push((quote!(#proto_id::#mod_id::events::#me), quote!(#proto_id::#proto_evt_enum), quote!(Event)));
                        for leaf in evt_leaves { top_transitive.push((quote!(#proto_id::#mod_id::events::#leaf), quote!(#proto_id::#proto_evt_enum), quote!(Event))); }
                    }
                }
            }

            let top_type_group = event::group_enum_closed(&format_ident!("Type"), &top_type_entries);
            let top_cmd_group = event::group_enum_closed(&format_ident!("Command"), &top_cmd_entries);
            let top_evt_group = event::group_enum_open(&format_ident!("Event"), &top_evt_entries);
            let top_transitive_impls = event::impl_from_transitive(&top_transitive);

            let event_message = if !top_evt_entries.is_empty() {
                quote! {
                    #[derive(Debug, PartialEq, Clone)]
                    pub struct EventMessage {
                        pub method: String,
                        pub session_id: Option<String>,
                        pub params: Event,
                    }
                }
            } else {
                TokenStream::default()
            };

            let top_mod_decls: Vec<_> = protocol_infos.iter().map(|(name, _, _, _)| {
                let mod_ident = format_ident!("{}", name);
                quote! { pub mod #mod_ident; }
            }).collect();

            quote! {
                use serde::{Serialize, Deserialize};

                #[macro_use]
                mod macros;

                #(#top_mod_decls)*

                #[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
                pub struct Binary(String);

                impl AsRef<str> for Binary { fn as_ref(&self) -> &str { self.0.as_str() } }
                impl AsRef<[u8]> for Binary { fn as_ref(&self) -> &[u8] { self.0.as_bytes() } }
                impl From<Binary> for String { fn from(b: Binary) -> String { b.0 } }
                impl From<String> for Binary { fn from(expr: String) -> Self { Self(expr) } }

                pub trait CommandResult {
                    type Result: serde::de::DeserializeOwned + std::fmt::Debug;
                    fn result_from_value(result: serde_json::Value) -> serde_json::Result<Self::Result> {
                        serde_json::from_value(result)
                    }
                }

                #top_type_group
                #top_cmd_group
                #top_evt_group
                #top_transitive_impls

                #event_message
            }
        };

        let top_mod_path = target_base.join("lib.rs");
        fs::write(&top_mod_path, top_mod_content.to_string())
            .unwrap_or_else(|e| panic!("Unable to write {}: {}", top_mod_path.display(), e));

        fmt(&target_base);
        Ok(())
    }

    pub fn generate_module_files(&mut self, module: &Module, current_protocol: &str) -> ModuleParts {
        let with_deprecated = self.with_deprecated;
        let with_experimental = self.with_experimental;

        let mut types_stream = TokenStream::default();
        let mut commands_stream = TokenStream::default();
        let mut command_builders_stream = TokenStream::default();
        let mut type_builders_stream = TokenStream::default();
        let mut events_stream = TokenStream::default();
        let mut results_stream = TokenStream::default();

        let mut type_idents: Vec<(Ident, Ident)> = Vec::new();
        let mut command_idents: Vec<(Ident, Ident)> = Vec::new();
        let mut event_idents: Vec<(Ident, Ident)> = Vec::new();

        let datatypes: Vec<_> = module
            .into_iter()
            .filter(|dt| with_deprecated || !dt.is_deprecated())
            .filter(|dt| with_experimental || !dt.is_experimental())
            .collect();

        for dt in datatypes {
            let is_type = dt.is_type();
            let is_command = dt.is_command();
            let camel_name = dt.name().to_upper_camel_case();

            let local_same_file = is_type;
            let (def, builder_tokens) = self.generate_type(module, &dt, current_protocol, local_same_file);
            if is_type {
                types_stream.extend(def);
                type_builders_stream.extend(builder_tokens);
                let ident = format_ident!("{}", camel_name);
                type_idents.push((ident.clone(), ident));
            } else if is_command {
                commands_stream.extend(def);
                command_builders_stream.extend(builder_tokens);

                // Link command to its result type
                let def_ident = format_ident!("{}", camel_name);
                let returns_ident = format_ident!("{}Result", camel_name);
                commands_stream.extend(quote! {
                    impl crate::CommandResult for #def_ident {
                        type Result = super::results::#returns_ident;
                    }
                });

                let ident = format_ident!("{}", camel_name);
                command_idents.push((ident.clone(), ident));
            } else {
                events_stream.extend(def);
                let ident = format_ident!("{}", camel_name);
                event_idents.push((ident.clone(), ident));
            }
        }

        // Generate CommandResult structs
        for cr in &module.command_results {
            let returns_name = cr.name.to_upper_camel_case();
            let params = cr.parameters.iter()
                .filter(|p| with_deprecated || !p.deprecated)
                .filter(|p| with_experimental || !p.experimental);

            let name = format_ident!("{}", returns_name);
            let mut builder = Builder::new(name.clone());
            let mut default_fns = TokenStream::default();

            for param in params {
                if let Type::Enum(vars) = &param.r#type {
                    let enum_ident = Variant {
                        description: param.description.as_deref().map(Cow::Borrowed),
                        name: Cow::Owned(subenum_name(cr.name.as_ref(), param.name.as_ref())),
                    };
                    results_stream.extend(self.generate_enum(&enum_ident, vars));
                }

                let field_name = format_ident!("{}", generate_field_name(param.name.as_ref()));
                let mut ty = self.generate_field_type(module, cr.name.as_ref(), param.name.as_ref(), &param.r#type, current_protocol, false);
                ty.needs_box = ty.needs_box || param.is_circular_dep;

                let is_enum = if let Type::Ref(type_ref) = &param.r#type {
                    self.enums.contains(type_ref.name.as_ref())
                        || self.enums.contains(&format!("{}.{}", module.name, type_ref.name.as_ref()))
                } else {
                    param.r#type.is_enum()
                };

                let field = FieldDefinition {
                    name: param.name.to_string(),
                    name_ident: field_name,
                    ty,
                    optional: param.optional,
                    deprecated: param.deprecated,
                    is_enum,
                    serde_skip: false,
                };

                if let Some(default_fn) = field.generate_default_fn(param, &returns_name) {
                    default_fns.extend(default_fn);
                }

                builder.fields.push((field.generate_meta(param, &returns_name), field));
            }

            if builder.fields.is_empty() {
                results_stream.extend(quote! {
                    #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
                    pub struct #name {}
                });
            } else {
                let struct_def = builder.generate_struct_def();
                let derives = if !builder.has_mandatory_types() {
                    quote! { #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)] }
                } else {
                    quote! { #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)] }
                };
                results_stream.extend(quote! {
                    #derives
                    #struct_def
                    #default_fns
                });
            }
        }

        // Group enums
        let type_entries: Vec<_> = type_idents.iter().map(|(v, t)| (v.clone(), quote!(#t))).collect();
        let cmd_entries: Vec<_> = command_idents.iter().map(|(v, t)| (v.clone(), quote!(#t))).collect();
        let evt_entries: Vec<_> = event_idents.iter().map(|(v, t)| (v.clone(), quote!(#t))).collect();

        let module_camel = module.name.to_upper_camel_case();

        if !type_entries.is_empty() {
            types_stream.extend(event::group_enum_closed(&format_ident!("{}Types", module_camel), &type_entries));
        }
        if !cmd_entries.is_empty() {
            commands_stream.extend(event::group_enum_closed(&format_ident!("{}Commands", module_camel), &cmd_entries));
        }
        if !evt_entries.is_empty() {
            events_stream.extend(event::group_enum_closed(&format_ident!("{}Events", module_camel), &evt_entries));
        }

        let serde_import = quote! { use serde::{Serialize, Deserialize}; };

        let wrap = |stream: TokenStream| -> TokenStream {
            if stream.is_empty() { stream }
            else { let imp = serde_import.clone(); quote! { #imp #stream } }
        };

        let wrap_builder = |stream: TokenStream, source_mod: &str| -> TokenStream {
            if stream.is_empty() { stream }
            else {
                let source = format_ident!("{}", source_mod);
                quote! { use super::#source::*; #stream }
            }
        };

        ModuleParts {
            types: wrap(types_stream),
            commands: wrap(commands_stream),
            command_builders: wrap_builder(command_builders_stream, "commands"),
            type_builders: wrap_builder(type_builders_stream, "types"),
            results: wrap(results_stream),
            events: wrap(events_stream),
            type_leaves: type_idents.into_iter().map(|(_, t)| t).collect(),
            command_leaves: command_idents.into_iter().map(|(_, t)| t).collect(),
            event_leaves: event_idents.into_iter().map(|(_, t)| t).collect(),
        }
    }

    fn generate_type(&mut self, module: &Module, dt: &ModuleDatatype, current_protocol: &str, local_same_file: bool) -> (TokenStream, TokenStream) {
        let (def, builder) = if let Some(type_refs) = dt.as_type_choice() {
            (self.generate_type_choice_enum(module, dt, type_refs, current_protocol, local_same_file), TokenStream::default())
        } else if let Some(vars) = dt.as_enum() {
            (self.generate_enum(&Variant::from(dt), vars), TokenStream::default())
        } else {
            let with_deprecated = self.with_deprecated;
            let with_experimental = self.with_experimental;
            let params = dt.params()
                .filter(|dt| with_deprecated || !dt.deprecated)
                .filter(|dt| with_experimental || !dt.experimental);

            let (mut stream, params_builder) = self.generate_struct(module, dt, dt.ident_name(), params, current_protocol, local_same_file);

            if let Some(method_name) = dt.method_ident_name() {
                let method_ident = format_ident!("{}", method_name);
                let params_ident = format_ident!("{}", dt.ident_name());
                let def_ident = format_ident!("{}", dt.definition_name());
                let variant_ident = def_ident.clone();
                let identifier = dt.raw_name();
                let desc = dt.type_description_tokens(module.name.as_ref());

                stream.extend(quote! {

                    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
                    pub enum #method_ident {
                        #[serde(rename = #identifier)]
                        #variant_ident,
                    }

                    impl #method_ident {
                        pub const IDENTIFIER: &'static str = #identifier;
                    }

                    #desc
                    #[derive(Debug, Clone, PartialEq)]
                    pub struct #def_ident {
                        pub method: #method_ident,
                        pub params: #params_ident,
                    }

                });

                let builder = params_builder.generate_command_builder(
                    &def_ident,
                    &method_ident,
                    &variant_ident,
                    &params_ident,
                );

                (stream, builder)
            } else {
                let identifier = dt.raw_name();
                let name = format_ident!("{}", dt.ident_name());

                stream.extend(quote! {

                    impl #name {
                        pub const IDENTIFIER: &'static str = #identifier;
                    }

                });

                let builder = if dt.is_type() {
                    params_builder.generate_builder()
                } else {
                    TokenStream::default()
                };

                (stream, builder)
            }
        };

        if dt.is_deprecated() {
            (quote! { #[deprecated] #def }, builder)
        } else {
            (def, builder)
        }
    }

    fn generate_struct<'a, T>(
        &mut self,
        module: &Module,
        dt: &ModuleDatatype,
        struct_ident: String,
        params: T,
        current_protocol: &str,
        local_same_file: bool,
    ) -> (TokenStream, Builder)
    where
        T: Iterator<Item = &'a Param<'a>> + 'a,
    {
        let name = format_ident!("{}", struct_ident);
        let mut enum_definitions = TokenStream::default();
        let mut default_fns = TokenStream::default();
        let mut builder = Builder::new(name.clone());
        let mut has_validation = false;

        for param in params {
            if param.validation.is_some() {
                has_validation = true;
            }
            if let Type::Enum(vars) = &param.r#type {
                let enum_ident = Variant {
                    description: param.description.as_deref().map(Cow::Borrowed),
                    name: Cow::Owned(subenum_name(dt.name(), param.name.as_ref())),
                };
                if param.deprecated {
                    enum_definitions.extend(quote! {#[deprecated]});
                }
                enum_definitions.extend(self.generate_enum(&enum_ident, vars));
            }

            let field_name = format_ident!("{}", generate_field_name(param.name.as_ref()));
            let mut ty = self.generate_field_type(module, dt.name(), param.name.as_ref(), &param.r#type, current_protocol, local_same_file);
            ty.needs_box = ty.needs_box || param.is_circular_dep;

            let is_enum = if let Type::Ref(type_ref) = &param.r#type {
                self.enums.contains(type_ref.name.as_ref())
                    || self.enums.contains(&format!("{}.{}", module.name, type_ref.name.as_ref()))
            } else {
                param.r#type.is_enum()
            };

            let field = FieldDefinition {
                name: param.name.to_string(),
                name_ident: field_name,
                ty,
                optional: param.optional,
                deprecated: param.deprecated,
                is_enum,
                serde_skip: false,
            };

            if let Some(default_fn) = field.generate_default_fn(param, &struct_ident) {
                default_fns.extend(default_fn);
            }

            builder.fields.push((field.generate_meta(param, &struct_ident), field));
        }

        let serde_derives = if has_validation {
            quote! { #[derive(Serialize, Deserialize, serde_valid::Validate)] }
        } else {
            quote! { #[derive(Serialize, Deserialize)] }
        };
        let desc = dt.type_description_tokens(module.name.as_ref());

        if builder.fields.is_empty() {
            if let ModuleDatatype::Type(tydef) = dt {
                // Newtypes: only add Default for primitive/string inner types
                let derives = if tydef.extends.is_integer() || tydef.extends.is_string()
                    || matches!(tydef.extends, Type::Number | Type::Boolean | Type::Object | Type::Any) {
                    quote! { #[derive(Debug, Clone, PartialEq, Default)] }
                } else {
                    quote! { #[derive(Debug, Clone, PartialEq)] }
                };

                let mut stream = quote! {
                    #desc
                    #derives
                    #serde_derives
                };
                let wrapped_ty = self.generate_field_type(module, dt.name(), dt.name(), &tydef.extends, current_protocol, local_same_file);

                let struct_def = quote! {
                    pub struct #name(#wrapped_ty);

                    impl #name {
                        pub fn new(val: impl Into<#wrapped_ty>) -> Self {
                            #name(val.into())
                        }

                        pub fn inner(&self) -> &#wrapped_ty {
                            &self.0
                        }
                    }
                };

                if tydef.extends.is_integer() {
                    stream.extend(quote! {
                        #[derive(Eq, Copy, Hash)]
                        #struct_def
                    });
                } else if tydef.extends.is_string() {
                    stream.extend(quote! {
                        #[derive(Eq, Hash)]
                        #struct_def

                        impl AsRef<str> for #name {
                            fn as_ref(&self) -> &str {
                                self.0.as_str()
                            }
                        }

                        impl From<#name> for String {
                            fn from(el: #name) -> String {
                                el.0
                            }
                        }

                        impl From<String> for #name {
                            fn from(expr: String) -> Self {
                                #name(expr)
                            }
                        }
                    });

                    if struct_ident.ends_with("Id") {
                        stream.extend(quote! {
                            impl std::borrow::Borrow<str> for #name {
                                fn borrow(&self) -> &str {
                                    &self.0
                                }
                            }
                        })
                    }
                } else {
                    stream.extend(struct_def);
                }
                return (stream, builder);
            } else {
                let mut stream = quote! { #desc #[derive(Debug, Clone, PartialEq)] #serde_derives };
                stream.extend(quote! {
                    pub struct #name {}
                });
                return (stream, builder);
            }
        }

        // Struct with fields
        let derives = if !builder.has_mandatory_types() {
            quote! { #[derive(Debug, Clone, PartialEq, Default)] }
        } else {
            quote! { #[derive(Debug, Clone, PartialEq)] }
        };

        let mut stream = quote! { #desc #derives #serde_derives };

        let struct_def = builder.generate_struct_def();
        stream.extend(quote! {
            #struct_def

            #default_fns

            #enum_definitions
        });

        if dt.is_command() || dt.is_type() {
            stream.extend(builder.generate_constructors());
        }

        (stream, builder)
    }

    fn generate_enum(&mut self, ident: &Variant, variants: &[Variant]) -> TokenStream {
        let enum_name = ident.name.as_ref().rsplit('.').next().unwrap().to_upper_camel_case();
        let name = format_ident!("{}", enum_name);

        let vars = variants.iter().map(|v| {
            let v_ident = format_ident!("{}", generate_enum_field_name(&v.name));
            let rename = {
                let r = v.name.as_ref();
                quote! { #[serde(rename = #r)] }
            };
            if let Some(desc) = v.description.as_ref() {
                quote! { #[doc = #desc] #rename #v_ident }
            } else {
                quote! { #rename #v_ident }
            }
        });

        let desc = if let Some(desc) = ident.description.as_ref() {
            quote! { #[doc = #desc] }
        } else {
            TokenStream::default()
        };

        quote! {
            #desc
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
            pub enum #name {
                #(#vars),*
            }
        }
    }

    fn generate_type_choice_enum(
        &self,
        module: &Module,
        dt: &ModuleDatatype,
        type_refs: &[TypeRef],
        current_protocol: &str,
        local_same_file: bool,
    ) -> TokenStream {
        let enum_name = format_ident!("{}", dt.ident_name());
        let desc = dt.type_description_tokens(module.name.as_ref());

        let variants: Vec<TokenStream> = type_refs.iter().map(|tr| {
            let variant_name = format_ident!("{}", tr.name.to_upper_camel_case());
            let ty = self.projected_type(module, tr, current_protocol, local_same_file);
            quote! { #variant_name(#ty) }
        }).collect();

        quote! {
            #desc
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[serde(untagged)]
            pub enum #enum_name {
                #(#variants),*
            }
        }
    }

    fn generate_field_type(
        &self,
        module: &Module,
        parent: &str,
        param_name: &str,
        ty: &Type,
        current_protocol: &str,
        local_same_file: bool,
    ) -> FieldType {
        match ty {
            Type::Integer => FieldType::new(quote! { i64 }),
            Type::UnsignedInteger => FieldType::new(quote! { u64 }),
            Type::Number => FieldType::new(quote! { f64 }),
            Type::Boolean => FieldType::new(quote! { bool }),
            Type::String => FieldType::new(quote! { String }),
            Type::Object | Type::Any => FieldType::new(quote! { serde_json::Value }),
            Type::Extensible => FieldType::new(quote! { std::collections::HashMap<String, serde_json::Value> }),
            Type::Binary => FieldType::new(quote! { crate::Binary }),
            Type::Enum(_) => {
                let ty = format_ident!("{}", subenum_name(parent, param_name));
                FieldType::new(quote! { #ty })
            }
            Type::ArrayOf(ty) => {
                let ty = if let Type::Ref(type_ref) = ty.deref() {
                    self.projected_type(module, type_ref, current_protocol, local_same_file)
                } else {
                    let ty = self.generate_field_type(module, parent, param_name, ty, current_protocol, local_same_file);
                    quote! { #ty }
                };
                FieldType::new_vec(ty)
            }
            Type::Ref(type_ref) => {
                if type_ref.name == parent && type_ref.module.is_none() {
                    let ident = format_ident!("{}", type_ref.name.to_upper_camel_case());
                    FieldType::new_box(quote! { #ident })
                } else {
                    FieldType::new(self.projected_type(module, type_ref, current_protocol, local_same_file))
                }
            }
        }
    }

    fn projected_type(&self, module: &Module, type_ref: &TypeRef, current_protocol: &str, local_same_file: bool) -> TokenStream {
        let ident = format_ident!("{}", type_ref.name.to_upper_camel_case());

        let is_local = match &type_ref.module {
            None => true,
            Some(m) => m.as_ref() == module.name.as_ref(),
        };

        if is_local {
            if local_same_file {
                quote! { #ident }
            } else {
                quote! { super::types::#ident }
            }
        } else {
            let ref_module = type_ref.module.as_ref().unwrap();
            let ref_module_snake = format_ident!("{}", ref_module.to_snake_case());
            let ref_protocol = self.module_protocol_map.get(ref_module.as_ref());
            if let Some(ref_proto) = ref_protocol {
                if ref_proto.is_empty() || ref_proto == current_protocol {
                    // Same protocol or flat mode
                    quote! { crate::#ref_module_snake::types::#ident }
                } else {
                    let proto_ident = format_ident!("{}", ref_proto);
                    quote! { crate::#proto_ident::#ref_module_snake::types::#ident }
                }
            } else {
                quote! { super::types::#ident }
            }
        }
    }
}

pub(crate) fn generate_field_name(name: &str) -> String {
    let name = name.to_snake_case();
    match name.as_str() {
        "type" => "r#type".to_string(),
        "mod" => "r#mod".to_string(),
        "override" => "r#override".to_string(),
        "default" => "r#default".to_string(),
        _ => name,
    }
}

pub(crate) fn generate_enum_field_name(name: &str) -> String {
    match name {
        "Self" => "KSelf".to_string(),
        _ => name.to_upper_camel_case(),
    }
}

fn subenum_name(parent: &str, inner: &str) -> String {
    format!("{}{}", parent.to_upper_camel_case(), generate_enum_field_name(inner))
}

pub fn fmt(out_dir: &Path) {
    use std::io::Write;
    use std::process::Command;

    let entries = match fs::read_dir(out_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("error reading directory {}: {e:?}", out_dir.display());
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            fmt(&path);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let result = Command::new("rustfmt")
                .arg("--emit")
                .arg("files")
                .arg("--edition")
                .arg("2024")
                .arg(&path)
                .output();

            match result {
                Err(e) => {
                    eprintln!("error running rustfmt on {}: {e:?}", path.display());
                }
                Ok(output) => {
                    if !output.status.success() {
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                }
            }
        }
    }
}
