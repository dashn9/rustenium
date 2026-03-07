use crate::backend::base_types::{Command, Constraint, DomainDirection, Event, Item, Module, Param, Type, TypeDef, TypeRef, Variant};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::slice::Iter;

pub struct ModuleDataTypeIter<'a> {
    types: Iter<'a, TypeDef<'a>>,
    commands: Iter<'a, Command<'a>>,
    events: Iter<'a, Event<'a>>,
}

impl<'a> Iterator for ModuleDataTypeIter<'a> {
    type Item = ModuleDatatype<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ty) = self.types.next() {
            return Some(ModuleDatatype::Type(ty));
        }
        if let Some(cmd) = self.commands.next() {
            return Some(ModuleDatatype::Command(cmd));
        }
        if let Some(ev) = self.events.next() {
            return Some(ModuleDatatype::Event(ev));
        }
        None
    }
}

impl<'a> IntoIterator for &'a Module<'a> {
    type Item = ModuleDatatype<'a>;
    type IntoIter = ModuleDataTypeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ModuleDataTypeIter {
            types: self.types.iter(),
            commands: self.commands.iter(),
            events: self.events.iter(),
        }
    }
}

pub enum ModuleDatatype<'a> {
    Type(&'a TypeDef<'a>),
    Command(&'a Command<'a>),
    Event(&'a Event<'a>),
}

impl<'a> ModuleDatatype<'a> {
    pub fn is_type(&self) -> bool {
        matches!(self, ModuleDatatype::Type(_))
    }

    pub fn is_command(&self) -> bool {
        matches!(self, ModuleDatatype::Command(_))
    }

    pub fn is_event(&self) -> bool {
        matches!(self, ModuleDatatype::Event(_))
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn type_description_tokens(&self, domain_name: &str) -> TokenStream {
        let base_url = "https://chromedevtools.github.io/devtools-protocol/tot/";

        let url = match self {
            ModuleDatatype::Type(ty) => format!("{}{}/#type-{}", base_url, domain_name, ty.name),
            ModuleDatatype::Command(cmd) => {
                format!("{}{}/#method-{}", base_url, domain_name, cmd.method.name)
            }
            ModuleDatatype::Event(ev) => {
                format!("{}{}/#event-{}", base_url, domain_name, ev.name)
            }
        };

        if let Some(desc) = self.description() {
            let desc = format!("{}\n[{}]({})", desc, self.name(), url);
            quote! {
                #[doc = #desc]
            }
        } else {
            TokenStream::default()
        }
    }

    pub fn ident_name(&self) -> String {
        match self {
            ModuleDatatype::Type(_ty) => self.name().to_upper_camel_case(),
            ModuleDatatype::Command(cmd) => format!("{}Params", cmd.method.name.to_upper_camel_case()),
            ModuleDatatype::Event(event) => format!("{}Params", event.name.to_upper_camel_case()),
        }
    }

    /// For commands/events: the method struct name (e.g. `EvaluateMethod`)
    pub fn method_ident_name(&self) -> Option<String> {
        match self {
            ModuleDatatype::Command(cmd) => Some(format!("{}Method", cmd.method.name.to_upper_camel_case())),
            ModuleDatatype::Event(event) => Some(format!("{}Method", event.name.to_upper_camel_case())),
            _ => None,
        }
    }

    /// For commands: the definition name (e.g. `Evaluate`) — wraps method + params.
    pub fn definition_name(&self) -> String {
        self.name().to_upper_camel_case()
    }

    pub fn params(&self) -> impl Iterator<Item = &'a Param<'a>> + 'a {
        match self {
            ModuleDatatype::Type(ty) => {
                if let Some(Item::Properties(ref params)) = ty.parameters {
                    params.iter()
                } else {
                    [].iter()
                }
            }
            ModuleDatatype::Command(cmd) => cmd.params.iter(),
            ModuleDatatype::Event(ev) => ev.parameters.iter(),
        }
    }

    pub fn as_enum(&self) -> Option<&Vec<Variant<'_>>> {
        match self {
            ModuleDatatype::Type(ty) => {
                if let Some(Item::Enum(ref vars)) = ty.parameters {
                    Some(vars)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_type_choice(&self) -> Option<&Vec<TypeRef<'_>>> {
        match self {
            ModuleDatatype::Type(ty) => {
                if let Some(Item::TypeChoice(ref refs)) = ty.parameters {
                    Some(refs)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn raw_name(&self) -> &'a str {
        match self {
            ModuleDatatype::Type(ty) => ty.raw_name.as_ref(),
            ModuleDatatype::Command(cmd) => cmd.method.raw_name.as_ref(),
            ModuleDatatype::Event(ev) => ev.raw_name.as_ref(),
        }
    }
}

impl<'a> ModuleDatatype<'a> {
    pub fn is_circular_dep(&self) -> bool {
        match self {
            ModuleDatatype::Type(inner) => inner.is_circular_dep,
            ModuleDatatype::Command(inner) => inner.is_circular_dep,
            ModuleDatatype::Event(inner) => inner.is_circular_dep,
        }
    }

    pub fn is_experimental(&self) -> bool {
        match self {
            ModuleDatatype::Type(inner) => inner.experimental,
            ModuleDatatype::Command(inner) => inner.method.experimental,
            ModuleDatatype::Event(inner) => inner.experimental,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            ModuleDatatype::Type(inner) => inner.description.as_deref(),
            ModuleDatatype::Command(inner) => inner.method.description.as_deref(),
            ModuleDatatype::Event(inner) => inner.description.as_deref(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ModuleDatatype::Type(inner) => inner.name.as_ref(),
            ModuleDatatype::Command(inner) => inner.method.name.as_ref(),
            ModuleDatatype::Event(inner) => inner.name.as_ref(),
        }
    }

    pub fn is_deprecated(&self) -> bool {
        match self {
            ModuleDatatype::Type(inner) => inner.deprecated,
            ModuleDatatype::Command(inner) => inner.method.deprecated,
            ModuleDatatype::Event(inner) => inner.deprecated,
        }
    }

    pub fn direction(&self) -> Option<DomainDirection> {
        match self {
            ModuleDatatype::Type(inner) => inner.direction,
            ModuleDatatype::Command(inner) => inner.direction,
            ModuleDatatype::Event(inner) => inner.direction,
        }
    }
}

pub struct FieldType {
    pub needs_box: bool,
    pub is_vec: bool,
    pub ty: TokenStream,
}

impl FieldType {
    pub fn new(ty: TokenStream) -> Self {
        Self {
            needs_box: false,
            is_vec: false,
            ty,
        }
    }

    pub fn new_box(ty: TokenStream) -> Self {
        Self {
            needs_box: true,
            is_vec: false,
            ty,
        }
    }

    pub fn new_vec(ty: TokenStream) -> Self {
        Self {
            needs_box: false,
            is_vec: true,
            ty,
        }
    }

    pub fn param_type_def(&self) -> TokenStream {
        let ty = &self.ty;
        if self.is_vec {
            quote! {
                Vec<#ty>
            }
        } else {
            quote! { impl Into<#ty>}
        }
    }

    pub fn builder_type(&self) -> TokenStream {
        let ty = &self.ty;
        if self.is_vec {
            quote! {
                Vec<#ty>
            }
        } else {
            quote! { #ty}
        }
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.ty;
        if self.needs_box {
            tokens.extend(quote! {
                Box<#ty>
            })
        } else if self.is_vec {
            tokens.extend(quote! {
                Vec<#ty>
            })
        } else {
            tokens.extend(quote! {#ty})
        }
    }
}

pub struct FieldDefinition {
    pub name: String,
    pub name_ident: Ident,
    pub ty: FieldType,
    pub optional: bool,
    pub deprecated: bool,
    pub serde_skip: bool,
}

impl FieldDefinition {
    /// Generate meta attributes: desc, serde. `struct_name` prefixes default fns.
    pub fn generate_meta(&self, param: &Param, struct_name: &str) -> TokenStream {
        let mut desc = if let Some(desc) = param.description.as_deref() {
            quote! {
                #[doc = #desc]
            }
        } else {
            TokenStream::default()
        };

        if self.deprecated {
            desc.extend(quote! {#[deprecated]});
        }

        let mut serde_attr = TokenStream::default();

        if self.serde_skip {
            serde_attr.extend(quote! {#[serde(skip)]})
        } else if matches!(&param.r#type, Type::Extensible) && self.name == "extensible" {
            serde_attr.extend(quote! {
                #[serde(flatten)]
                #[serde(default)]
            });
        } else if matches!(&param.r#type, Type::Extensible) {
            let name = &self.name;
            serde_attr.extend(quote! {
                #[serde(rename = #name)]
                #[serde(default)]
            });
        } else {
            let name = &self.name;
            serde_attr.extend(quote! {
                #[serde(rename = #name)]
            });
            if self.optional {
                serde_attr.extend(quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                });
                if param.default_value.is_none() {
                    serde_attr.extend(quote! {
                        #[serde(default)]
                    });
                }
            } else if let Type::ArrayOf(_) = &param.r#type {
                serde_attr.extend(quote! {
                    #[serde(skip_serializing_if = "Vec::is_empty")]
                });
            }
        }

        // Default value
        let mut default_attr = TokenStream::default();
        if param.default_value.is_some() {
            // Only use named default fn if we can generate it; otherwise plain default
            if self.generate_default_fn(param, struct_name).is_some() {
                let fn_name = self.default_fn_name(struct_name);
                default_attr.extend(quote! {
                    #[serde(default = #fn_name)]
                });
            } else {
                default_attr.extend(quote! {
                    #[serde(default)]
                });
            }
        }

        // Validation constraints — literal type must match the field type
        let mut validation_attr = TokenStream::default();
        if let Some(ref constraints) = param.validation {
            let ty_str = self.ty.ty.to_string();
            let val_lit = |v: f64| -> TokenStream {
                if ty_str.contains("u64") {
                    let vi = v as u64;
                    quote! { #vi }
                } else if ty_str.contains("i64") {
                    let vi = v as i64;
                    quote! { #vi }
                } else {
                    quote! { #v }
                }
            };
            for c in constraints {
                match c {
                    Constraint::Ge(v) => { let lit = val_lit(*v); validation_attr.extend(quote! { #[validate(minimum = #lit)] }); }
                    Constraint::Gt(v) => { let lit = val_lit(*v); validation_attr.extend(quote! { #[validate(exclusive_minimum = #lit)] }); }
                    Constraint::Le(v) => { let lit = val_lit(*v); validation_attr.extend(quote! { #[validate(maximum = #lit)] }); }
                    Constraint::Lt(v) => { let lit = val_lit(*v); validation_attr.extend(quote! { #[validate(exclusive_maximum = #lit)] }); }
                }
            }
        }

        let def = self.field_definition();
        quote! {
            #desc
            #serde_attr
            #default_attr
            #validation_attr
            #def
        }
    }

    pub fn default_fn_name(&self, struct_name: &str) -> String {
        format!("default_{}_{}", struct_name.to_snake_case(), self.name_ident)
    }

    /// Generate a free function for serde default, prefixed with struct name to avoid collisions.
    pub fn generate_default_fn(&self, param: &Param, struct_name: &str) -> Option<TokenStream> {
        let val_str = param.default_value.as_deref()?;
        let fn_name = self.default_fn_name(struct_name);
        let fn_ident = proc_macro2::Ident::new(&fn_name, proc_macro2::Span::call_site());

        let ty = &self.ty;
        let ty_str = ty.ty.to_string();

        let literal: TokenStream = if ty_str.contains("bool") {
            match val_str {
                "true" => quote! { true },
                "false" => quote! { false },
                _ => return None,
            }
        } else if ty_str.contains("u64") {
            let v: u64 = val_str.parse().ok()?;
            quote! { #v }
        } else if ty_str.contains("i64") {
            let v: i64 = val_str.parse().ok()?;
            quote! { #v }
        } else if ty_str.contains("f64") {
            let v: f64 = val_str.parse().ok()?;
            quote! { #v }
        } else if ty_str.contains("String") {
            quote! { #val_str.to_string() }
        } else {
            // Can't generate default for this type (enum, ref, etc.)
            return None;
        };

        if self.optional {
            Some(quote! {
                fn #fn_ident() -> Option<#ty> {
                    Some(#literal)
                }
            })
        } else {
            Some(quote! {
                fn #fn_ident() -> #ty {
                    #literal
                }
            })
        }
    }

    pub fn field_definition(&self) -> TokenStream {
        let name = &self.name_ident;
        let ty = &self.ty;
        if self.optional {
            quote! {
                pub #name : Option<#ty>
            }
        } else {
            quote! {
                pub #name : #ty
            }
        }
    }
}
