use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::backend::base_types::{Module, Param, Protocol, Type, Variant};
use crate::backend::builder::Builder;
use crate::backend::event;
use crate::backend::types::{FieldDefinition, FieldType, ModuleDatatype};

/// Compile `.protocol` files into Rust files during a Cargo build.
///
/// The generated `.rs` files are written to the Cargo `OUT_DIR` directory,
/// suitable for use with
///
/// This function should be called in a project's `build.rs`.
///
/// # Arguments
///
/// **`protocols`** - Paths to `.protocol` files to compile.
///
/// # Errors
///
/// This function can fail for a number of reasons:
///
///   - Failure to locate `protocol` files.
///   - Failure to parse the `.protocol`s.
///
/// It's expected that this function call be `unwrap`ed in a `build.rs`; there
/// is typically no reason to gracefully recover from errors during a build.
///
/// # Example `build.rs`
///
/// ```rust,no_run
/// # use std::io::Result;
/// fn main() -> Result<()> {
///   chromiumoxide_pdl::build::compile_protocols(&["src/js.protocol", "src/browser.protocol"])?;
///   Ok(())
/// }
/// ```
pub fn compile_protocols(protocols: &[Protocol]) -> io::Result<()> {
    Generator::default().compile_protocols(protocols)
}

/// Generates rust code for the Chrome DevTools Protocol
#[derive(Debug, Clone)]
pub struct Generator {
    serde_support: SerdeSupport,
    with_experimental: bool,
    with_deprecated: bool,
    allowed_deprecated_type: HashSet<String>,
    out_dir: Option<PathBuf>,
    target_mod: Option<String>,
    /// This contains a list of all enums of all modules with their qualified
    /// names <module>.<name>
    ///
    /// This is a fix in order to check in struct definitions whether the
    /// targeted type is an enum
    enums: HashSet<String>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            serde_support: Default::default(),
            with_experimental: true,
            with_deprecated: false,
            allowed_deprecated_type: HashSet::new(),
            out_dir: None,
            target_mod: Default::default(),
            enums: Default::default(),
        }
    }
}

/// The token streams generated for a single protocol module, one per output file.
pub struct ModuleParts {
    pub types: TokenStream,
    pub commands: TokenStream,
    pub command_builders: TokenStream,
    pub type_builders: TokenStream,
    pub results: TokenStream,
    pub events: TokenStream,
}

impl Generator {
    /// Configures the output directory where generated Rust files will be
    /// written.
    ///
    /// If unset, defaults to the `OUT_DIR` environment variable. `OUT_DIR` is
    /// set by Cargo when executing build scripts, so `out_dir` typically
    /// does not need to be configured.
    pub fn out_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.out_dir = Some(path.into());
        self
    }

    /// Configures the serde support that should be included for all the
    /// generated types.
    pub fn serde(&mut self, serde: SerdeSupport) -> &mut Self {
        self.serde_support = serde;
        self
    }

    /// Configures whether experimental types and fields should be included.
    ///
    /// Disabling experimental types may result in missing type definitions
    /// (E0412)
    pub fn experimental(&mut self, experimental: bool) -> &mut Self {
        self.with_experimental = experimental;
        self
    }

    /// Configures whether deprecated types and fields should be included.
    pub fn deprecated(&mut self, deprecated: bool) -> &mut Self {
        self.with_deprecated = deprecated;
        self
    }

    /// Configures a type name that is allowed to be deprecated.
    pub fn allowed_deprecated_type(&mut self, type_name: impl Into<String>) -> &mut Self {
        self.allowed_deprecated_type.insert(type_name.into());
        self
    }

    /// Configures the name of the module and file generated.
    pub fn target_mod(&mut self, mod_name: impl Into<String>) -> &mut Self {
        self.target_mod = Some(mod_name.into());
        self
    }

    /// Compile `.protocols` files into Rust files during a Cargo build with
    /// additional code generator configuration options.
    ///
    /// This method is like the `chromiumoxide_pdl::build::compile_protocols`
    /// function, with the added ability to specify non-default code
    /// generation options. See that function for more information about the
    /// arguments and generated outputs.
    ///
    /// # Example `build.rs`
    ///
    /// ```rust,no_run
    /// # use std::io::Result;
    /// fn main() -> Result<()> {
    ///   let mut protocol_build = chromiumoxide_pdl::build::Generator::default();
    ///   protocol_build.out_dir("some/path");
    ///   protocol_build.compile_protocols(&["src/frontend.pdl", "src/backend.pdl"])?;
    ///   Ok(())
    /// }
    /// ```
    pub fn compile_protocols(&mut self, protocols: &[Protocol]) -> io::Result<()> {
        let target_base: PathBuf = self.out_dir.clone().map(Ok).unwrap_or_else(|| {
            std::env::var_os("OUT_DIR")
                .ok_or_else(|| {
                    Error::new(ErrorKind::Other, "OUT_DIR environment variable is not set")
                })
                .map(Into::into)
        })?;

        fs::create_dir_all(&target_base)
            .unwrap_or_else(|e| panic!("Unable to create directory {}: {}", target_base.display(), e));

        let mut protocol_names = Vec::new();

        for protocol in protocols.iter() {
            let version = format!("{}.{}", protocol.version.major, protocol.version.minor);

            let protocol_name = protocol.name.unwrap_or("unknown");
            let protocol_snake = protocol_name.to_snake_case();
            protocol_names.push(protocol_snake.clone());

            let protocol_dir = target_base.join(&protocol_snake);
            fs::create_dir_all(&protocol_dir)
                .unwrap_or_else(|e| panic!("Unable to create directory {}: {}", protocol_dir.display(), e));

            let mut module_names = Vec::new();
            let mut filtered_modules: Vec<&Module> = Vec::new();

            let with_deprecated = self.with_deprecated;
            let with_experimental = self.with_experimental;
            let modules_to_process: Vec<_> = protocol.modules
                .iter()
                .filter(|d| with_deprecated || !d.deprecated)
                .filter(|d| with_experimental || !d.experimental)
                .collect();

            for module in &modules_to_process {
                let mod_name = module.name.to_snake_case();
                let parts = self.generate_module_files(module);

                let module_dir = protocol_dir.join(&mod_name);
                fs::create_dir_all(&module_dir)
                    .unwrap_or_else(|e| panic!("Unable to create directory {}: {}", module_dir.display(), e));

                // Write each part file, skipping empty ones
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
                        sub_mods.push(file_name);
                    }
                }

                // Write module mod.rs with sub-module declarations
                let sub_mod_decls: Vec<_> = sub_mods.iter().map(|name| {
                    let mod_ident = format_ident!("{}", name);
                    quote! {
                        pub mod #mod_ident;
                        pub use #mod_ident::*;
                    }
                }).collect();

                let module_mod_content = quote! { #(#sub_mod_decls)* };
                let module_mod_path = module_dir.join("mod.rs");
                fs::write(&module_mod_path, module_mod_content.to_string())
                    .unwrap_or_else(|e| panic!("Unable to write {}: {}", module_mod_path.display(), e));

                module_names.push(mod_name);
                filtered_modules.push(module);
            }

            // Generate protocol-level Event enum
            let protocol_event_enum = event::generate_protocol_event_enum(
                &filtered_modules,
                self.with_deprecated,
                self.with_experimental,
            );

            // Write protocol mod.rs
            let mod_decls: Vec<_> = module_names.iter().map(|name| {
                let mod_ident = format_ident!("{}", name);
                quote! { pub mod #mod_ident; }
            }).collect();

            let protocol_mod_content = quote! {
                #(#mod_decls)*

                /// The version of this protocol definition
                pub const VERSION: &str = #version;

                #protocol_event_enum
            };

            let protocol_mod_path = protocol_dir.join("mod.rs");
            fs::write(&protocol_mod_path, protocol_mod_content.to_string())
                .unwrap_or_else(|e| panic!("Unable to write {}: {}", protocol_mod_path.display(), e));
        }

        // Write top-level mod.rs
        let top_event_enum = event::generate_top_event_enum(&protocol_names);

        let top_mod_decls: Vec<_> = protocol_names.iter().map(|name| {
            let mod_ident = format_ident!("{}", name);
            quote! { pub mod #mod_ident; }
        }).collect();

        let top_mod_content = quote! {
            #(#top_mod_decls)*

            #top_event_enum
        };

        let top_mod_path = target_base.join("mod.rs");
        fs::write(&top_mod_path, top_mod_content.to_string())
            .unwrap_or_else(|e| panic!("Unable to write {}: {}", top_mod_path.display(), e));

        fmt(&target_base);
        Ok(())
    }

    /// Generates the separate token streams for a module, one per output file.
    pub fn generate_module_files(&mut self, module: &Module) -> ModuleParts {
        let serde_imports = self.serde_support.generate_serde_imports();
        let with_deprecated = self.with_deprecated;
        let with_experimental = self.with_experimental;

        let mut types_stream = TokenStream::default();
        let mut commands_stream = TokenStream::default();
        let mut command_builders_stream = TokenStream::default();
        let mut type_builders_stream = TokenStream::default();
        let mut events_stream = TokenStream::default();
        let mut results_stream = TokenStream::default();

        let allowed = &self.allowed_deprecated_type;
        let datatypes: Vec<_> = module
            .into_iter()
            .filter(|dt| {
                with_deprecated
                    || !dt.is_deprecated()
                    || allowed.contains(dt.name())
            })
            .filter(|dt| with_experimental || !dt.is_experimental())
            .collect();

        for dt in datatypes {
            let is_type = dt.is_type();
            let is_command = dt.is_command();

            let (def, builder_tokens) = self.generate_type(module, &dt);
            if is_type {
                types_stream.extend(def);
                type_builders_stream.extend(builder_tokens);
            } else if is_command {
                commands_stream.extend(def);
                command_builders_stream.extend(builder_tokens);
            } else {
                events_stream.extend(def);
            }
        }

        // Generate CommandResult (FooReturns) structs
        for cr in &module.command_results {
            let returns_name = format!("{}Returns", cr.name.to_upper_camel_case());
            let params = cr.parameters.iter()
                .filter(|p| with_deprecated || !p.deprecated)
                .filter(|p| with_experimental || !p.experimental);

            let name = format_ident!("{}", returns_name);
            let serde_derives = self.serde_support.generate_derives();
            let derives = quote! { #[derive(Debug, Clone, PartialEq, Default)] };

            let mut builder = Builder::new(name.clone());

            for param in params {
                if let Type::Enum(vars) = &param.r#type {
                    let enum_ident = Variant {
                        description: param.description.as_deref().map(Cow::Borrowed),
                        name: Cow::Owned(subenum_name(cr.name.as_ref(), param.name.as_ref())),
                    };
                    results_stream.extend(self.generate_enum(&enum_ident, vars));
                }

                let field_name = format_ident!("{}", generate_field_name(param.name.as_ref()));
                let ty = self.generate_field_type(module, cr.name.as_ref(), param.name.as_ref(), &param.r#type);

                let is_enum = if let Type::Ref(ref_name) = &param.r#type {
                    self.enums.contains(ref_name.as_ref())
                        || self.enums.contains(&format!("{}.{}", module.name, ref_name.as_ref()))
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

                builder.fields.push((field.generate_meta(&self.serde_support, param), field));
            }

            if builder.fields.is_empty() {
                results_stream.extend(quote! {
                    #derives
                    #serde_derives
                    pub struct #name {}
                });
            } else {
                let struct_def = builder.generate_struct_def();
                results_stream.extend(quote! {
                    #derives
                    #serde_derives
                    #struct_def
                });
            }
        }

        // Append module-level Event enum to events stream
        let module_event_enum = event::generate_module_event_enum(
            module,
            with_deprecated,
            with_experimental,
        );
        events_stream.extend(module_event_enum);

        // Prepend serde imports to non-empty streams
        let wrap = |stream: TokenStream| -> TokenStream {
            if stream.is_empty() {
                stream
            } else {
                let imports = serde_imports.clone();
                quote! { #imports #stream }
            }
        };

        // Builder files need `use super::*;` to access sibling module types
        let wrap_builder = |stream: TokenStream| -> TokenStream {
            if stream.is_empty() {
                stream
            } else {
                quote! { use super::*; #stream }
            }
        };

        ModuleParts {
            types: wrap(types_stream),
            commands: wrap(commands_stream),
            command_builders: wrap_builder(command_builders_stream),
            type_builders: wrap_builder(type_builders_stream),
            results: wrap(results_stream),
            events: wrap(events_stream),
        }
    }

    /// Generated output for a single datatype (command, event, or type).
    /// Returns `(definition, builder)` — the builder may be empty.
    fn generate_type(&mut self, module: &Module, dt: &ModuleDatatype) -> (TokenStream, TokenStream) {
        let (def, builder) = if let Some(vars) = dt.as_enum() {
            (self.generate_enum(&&Variant::from(dt), vars), TokenStream::default())
        } else {
            let with_deprecated = self.with_deprecated;
            let with_experimental = self.with_experimental;
            let params = dt
                .params()
                .filter(|dt| with_deprecated || !dt.deprecated)
                .filter(|dt| with_experimental || !dt.experimental);

            let (mut stream, builder) = self.generate_struct(module, dt, dt.ident_name(), params);
            let identifier = dt.raw_name();
            let name = format_ident!("{}", dt.ident_name());
            stream.extend(quote! {
              impl #name {
                  pub const IDENTIFIER : &'static str = #identifier;
              }
            });
            (stream, builder)
        };
        if dt.is_deprecated() {
            (quote! { #[deprecated] #def }, builder)
        } else {
            (def, builder)
        }
    }

    /// Entry point to modify the builder for a struct manually
    ///
    /// This is useful to add utility fields that should not be serialized by
    /// make things easier
    fn apply_struct_fixup(&self, builder: &mut Builder, dt: &ModuleDatatype) {
        if dt.raw_name() == "Runtime.evaluate" {
            let field = FieldDefinition {
                name: "eval_as_function_fallback".to_string(),
                name_ident: format_ident!("eval_as_function_fallback"),
                ty: FieldType {
                    needs_box: false,
                    is_vec: false,
                    ty: quote! {
                        bool
                    },
                },
                optional: true,
                deprecated: false,
                is_enum: false,
                serde_skip: true,
            };

            let def = field.field_definition();

            let meta = quote! {
                /// This is a manually added field that is not part of the protocol definition, hence ignored during serde operations.
                ///
                /// If set to true, this field indicates, that if the command resulted in a response value of type `function` this, `EvaluateParams` command should be executed as a `CallFunctionOnParams` instead.
                #[serde(skip)]
                #def
            };
            builder.fields.push((meta, field));
        }
    }

    /// Generates the struct definitions including enum definitions inner
    /// parameter enums.
    /// Returns `(definition, builder)`.
    fn generate_struct<'a, T>(
        &mut self,
        module: &Module,
        dt: &ModuleDatatype,
        struct_ident: String,
        params: T,
    ) -> (TokenStream, TokenStream)
    where
        T: Iterator<Item = &'a Param<'a>> + 'a,
    {
        let name = format_ident!("{}", struct_ident);
        // also generate enums for inner enums
        let mut enum_definitions = TokenStream::default();
        let mut builder = Builder::new(name.clone());

        for param in params {
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

            let ty =
                self.generate_field_type(module, dt.name(), param.name.as_ref(), &param.r#type);

            // check if the type of the param points to an enum
            let is_enum = if let Type::Ref(name) = &param.r#type {
                self.enums.contains(name.as_ref())
                    || self
                        .enums
                        .contains(&format!("{}.{}", module.name, name.as_ref()))
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

            builder
                .fields
                .push((field.generate_meta(&self.serde_support, param), field));
        }

        self.apply_struct_fixup(&mut builder, dt);

        let derives = if !builder.has_mandatory_types() {
            quote! { #[derive(Debug, Clone, PartialEq, Default)]}
        } else {
            quote! {#[derive(Debug, Clone, PartialEq)] }
        };

        let serde_derives = self.serde_support.generate_derives();

        let desc = dt.type_description_tokens(module.name.as_ref());

        let mut stream = quote! {
            #desc
            #derives
            #serde_derives
        };

        if builder.fields.is_empty() {
            if let ModuleDatatype::Type(tydef) = dt {
                // create wrapper types if no fields present
                let wrapped_ty =
                    self.generate_field_type(module, dt.name(), dt.name(), &tydef.extends);
                let struct_def = quote! {
                    pub struct #name( #wrapped_ty);

                    impl #name {

                        pub fn new(val: impl Into<#wrapped_ty>) -> Self {
                            #name(val.into())
                        }

                        pub fn inner(&self) -> &#wrapped_ty {
                            &self.0
                        }
                    }
                };

                // add Hash +  Eq for integer and string types
                if tydef.extends.is_integer() {
                    stream.extend(quote! {
                        #[derive(Eq, Copy, Hash)]
                        #struct_def
                    });
                } else if tydef.extends.is_string() {
                    // add string helpers
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
                    // Fixup specifically types used as keys
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
            } else {
                stream.extend(quote! {
                    pub struct #name {}
                })
            }
        } else {
            let struct_def = builder.generate_struct_def();
            stream.extend(quote! {
                #struct_def
                #enum_definitions
            });

            if dt.is_command() || dt.is_type() {
                stream.extend(builder.generate_constructors());
            }

            let builder_tokens = if dt.is_command() || dt.is_type() {
                builder.generate_builder()
            } else {
                TokenStream::default()
            };

            return (stream, builder_tokens);
        }
        (stream, TokenStream::default())
    }

    /// Generate enum type with `as_str` and `FromStr` methods
    fn generate_enum(&mut self, ident: &Variant, variants: &[Variant]) -> TokenStream {
        let enum_name = ident
            .name
            .as_ref()
            .rsplit('.')
            .next()
            .unwrap()
            .to_upper_camel_case();

        let name = format_ident!("{}", enum_name);

        let vars = variants
            .iter()
            .map(|v| self.serde_support.generate_variant(v));

        let desc = if let Some(desc) = ident.description.as_ref() {
            quote! {
                #[doc = #desc]
            }
        } else {
            TokenStream::default()
        };

        let attr = self.serde_support.generate_derives();

        let ty_def = quote! {
            #desc
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #attr
            pub enum #name {
                #(#vars),*
            }
        };

        // from str to string impl
        let vars: Vec<_> = variants
            .iter()
            .map(|s| format_ident!("{}", generate_enum_field_name(&s.name)))
            .collect();

        let str_values: Vec<_> = variants
            .iter()
            .map(|s| {
                let mut vars = vec![s.name.to_string()];
                let lc = s.name.to_lowercase();
                let cc = generate_enum_field_name(&s.name);
                if cc != lc && vars[0] != cc {
                    vars.push(cc);
                }
                if vars[0] != lc {
                    vars.push(lc);
                }
                vars
            })
            .collect();

        let str_fns = generate_enum_str_fns(&name, &vars, &str_values);

        quote! {
            #ty_def
            #str_fns
        }
    }

    /// Generates the Tokenstream for the field type (bool, f64, etc.)
    fn generate_field_type(
        &self,
        module: &Module,
        parent: &str,
        param_name: &str,
        ty: &Type,
    ) -> FieldType {
        match ty {
            Type::Integer => FieldType::new(quote! {
                i64
            }),
            Type::Number => FieldType::new(quote! {
                f64
            }),
            Type::Boolean => FieldType::new(quote! {
                bool
            }),
            Type::String => FieldType::new(quote! {
                String
            }),
            Type::Object | Type::Any => FieldType::new(quote! {serde_json::Value}),
            Type::Binary => FieldType::new(quote! {chromiumoxide_types::Binary}),
            Type::Enum(_) => {
                let ty = format_ident!("{}", subenum_name(parent, param_name));
                FieldType::new(quote! {#ty})
            }
            Type::ArrayOf(ty) => {
                // recursive types don't need to be boxed in vec
                let ty = if let Type::Ref(name) = ty.deref() {
                    self.projected_type(module, name)
                } else {
                    let ty = self.generate_field_type(module, parent, param_name, ty);
                    quote! {#ty}
                };
                FieldType::new_vec(ty)
            }
            Type::Ref(name) => {
                // consider recursive types
                if name == parent {
                    let ident = format_ident!("{}", name.to_upper_camel_case());
                    FieldType::new_box(quote! {
                       #ident
                    })
                } else {
                    FieldType::new(self.projected_type(module, name))
                }
            }
        }
    }

    /// Resolve projections: `Runtime.ScriptId` where `Runtime` is the
    /// referenced module where `ScriptId` is defined.
    ///
    /// In order to resolve cross protocol references a module check is necessary.
    /// If the referenced module is defined in another protocol than the `module`'s
    /// protocol, we need to move up an additional level (`super::super`)
    fn projected_type(&self, _module: &Module, name: &str) -> TokenStream {
        let mut iter = name.rsplitn(2, '.');
        let ty_name = iter.next().unwrap();
        let _path = iter.collect::<String>();
        let ident = format_ident!("{}", ty_name.to_upper_camel_case());
        quote! {
            #ident
        }
    }
}

fn generate_enum_str_fns(name: &Ident, vars: &[Ident], str_vals: &[Vec<String>]) -> TokenStream {
    assert_eq!(vars.len(), str_vals.len());
    let mut from_str_stream = TokenStream::default();
    let mut as_str_idents = Vec::new();
    for (var, strs) in vars.iter().zip(str_vals.iter()) {
        from_str_stream.extend(quote! {
                #(#strs)|* => Ok(#name::#var),
        });
        as_str_idents.push(&strs[0]);
    }

    quote! {
        impl AsRef<str> for #name {
            fn as_ref(&self) -> &str {
                match self {
                    #( #name::#vars => #as_str_idents ),*
                }
            }
        }

        impl ::std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #from_str_stream
                    _=> Err(s.to_string())
                }
            }
        }
    }
}

/// Escapes reserved rust keywords
pub(crate) fn generate_field_name(name: &str) -> String {
    let name = name.to_snake_case();
    match name.as_str() {
        "type" => "r#type".to_string(),
        "mod" => "r#mod".to_string(),
        "override" => "r#override".to_string(),
        _ => name,
    }
}

pub(crate) fn generate_enum_field_name(name: &str) -> String {
    match name {
        "Self" => "KSelf".to_string(),
        _ => name.to_upper_camel_case(),
    }
}

/// Creates the name for an enum defined inside a type
///
/// ```text
/// type Parent
///     enum type
/// ```
/// to `ParentType`
fn subenum_name(parent: &str, inner: &str) -> String {
    format!(
        "{}{}",
        parent.to_upper_camel_case(),
        generate_enum_field_name(inner)
    )
}

#[derive(Debug, Default, Clone)]
pub enum SerdeSupport {
    None,
    #[default]
    Default,
    Feature(String),
}

impl SerdeSupport {
    pub fn with_feature(feature: impl Into<String>) -> Self {
        SerdeSupport::Feature(feature.into())
    }

    fn generate_serde_import_deserialize(&self) -> TokenStream {
        match self {
            SerdeSupport::None => TokenStream::default(),
            SerdeSupport::Default => quote! {
                 use serde::Deserialize;
            },
            SerdeSupport::Feature(feature) => {
                quote! {
                    #[cfg(feature = #feature)]
                    use serde::Deserialize;
                }
            }
        }
    }

    fn generate_serde_imports(&self) -> TokenStream {
        match self {
            SerdeSupport::None => TokenStream::default(),
            SerdeSupport::Default => quote! {
                 use serde::{Serialize, Deserialize};
            },
            SerdeSupport::Feature(feature) => {
                quote! {
                    #[cfg(feature = #feature)]
                    use serde::{Serialize, Deserialize};
                }
            }
        }
    }

    fn generate_derives(&self) -> TokenStream {
        match self {
            SerdeSupport::None => TokenStream::default(),
            SerdeSupport::Default => quote! {
                #[derive(Serialize, Deserialize)]
            },
            SerdeSupport::Feature(feature) => {
                quote! {
                    #[cfg_attr(feature = #feature, derive(Serialize, Deserialize))]
                }
            }
        }
    }

    pub(crate) fn generate_opt_field_attr(&self) -> TokenStream {
        match self {
            SerdeSupport::None => TokenStream::default(),
            SerdeSupport::Default => quote! {
                 #[serde(skip_serializing_if = "Option::is_none")]
            },
            SerdeSupport::Feature(feature) => {
                quote! {
                     #[cfg_attr(feature = #feature, serde(skip_serializing_if = "Option::is_none"))]
                }
            }
        }
    }

    pub(crate) fn generate_vec_field_attr(&self) -> TokenStream {
        match self {
            SerdeSupport::None => TokenStream::default(),
            SerdeSupport::Default => quote! {
                 #[serde(skip_serializing_if = "Vec::is_empty")]
            },
            SerdeSupport::Feature(feature) => {
                quote! {
                     #[cfg_attr(feature = #feature, serde(skip_serializing_if = "Vec::is_empty"))]
                }
            }
        }
    }

    pub(crate) fn generate_enum_de_with(is_option: bool) -> TokenStream {
        if is_option {
            // NOTE: `#[serde(default)]` is needed here: https://stackoverflow.com/a/44303505/6242846
            quote! {
                 #[serde(default)]
                 #[serde(deserialize_with = "super::super::de::deserialize_from_str_optional")]
            }
        } else {
            quote! {
                 #[serde(deserialize_with = "super::super::de::deserialize_from_str")]
            }
        }
    }

    pub(crate) fn generate_rename(&self, rename: &str) -> TokenStream {
        match self {
            SerdeSupport::None => TokenStream::default(),
            SerdeSupport::Default => quote! {
                 #[serde(rename = #rename)]
            },
            SerdeSupport::Feature(feature) => {
                quote! {
                     #[cfg_attr(feature = #feature, serde(rename = #rename))]
                }
            }
        }
    }

    fn generate_variant(&self, var: &Variant) -> TokenStream {
        let v = format_ident!("{}", generate_enum_field_name(&var.name));
        let rename = self.generate_rename(var.name.as_ref());
        if let Some(desc) = var.description.as_ref() {
            quote! {
                #[doc = #desc]
                #rename
                #v
            }
        } else {
            quote! {
                #rename
                #v
            }
        }
    }
}

/// Recursively format all `.rs` files under the given directory with rustfmt.
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
                .arg("2021")
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
