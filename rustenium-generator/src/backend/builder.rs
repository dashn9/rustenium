use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use heck::*;

use crate::backend::{generator::generate_field_name, types::FieldDefinition};
pub struct Builder {
    pub fields: Vec<(TokenStream, FieldDefinition)>,
    pub name: Ident,
}

impl Builder {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            fields: vec![],
        }
    }

    pub fn has_mandatory_types(&self) -> bool {
        self.mandatory().any(|f| !f.optional)
    }

    fn mandatory(&self) -> impl Iterator<Item = &FieldDefinition> + '_ {
        self.fields
            .iter()
            .filter(|(_, f)| !f.optional)
            .map(|(_, f)| f)
    }

    pub fn generate_struct_def(&self) -> TokenStream {
        let name = &self.name;
        let field_definitions = self.fields.iter().map(|(def, _)| def);
        quote! {
             pub struct #name {
                #(#field_definitions),*
             }
        }
    }

    /// Generates `new()` constructor and `From<String>` impl.
    /// Stays with the struct definition file.
    pub fn generate_constructors(&self) -> TokenStream {
        let mut stream = TokenStream::default();
        if self.fields.is_empty() {
            return stream;
        }

        let name = &self.name;

        let optionals = self
            .fields
            .iter()
            .filter(|(_, f)| f.optional)
            .map(|(_, f)| &f.name_ident);

        let mut mandatory_param_name = vec![];
        let mut mandatory_param_ty = vec![];
        let mut assign = vec![];

        for field in self.mandatory() {
            let field_name = &field.name_ident;
            mandatory_param_name.push(field_name);
            mandatory_param_ty.push(field.ty.param_type_def());
            if field.ty.is_vec {
                assign.push(quote! {#field_name});
            } else if field.ty.needs_box {
                assign.push(quote! {#field_name : Box::new(#field_name.into())});
            } else {
                assign.push(quote! {#field_name : #field_name.into()});
            }
        }

        // clippy allows up to 7 arguments
        // But let's limit this to 4, because a builder will also be implemented
        if !mandatory_param_name.is_empty() && mandatory_param_name.len() <= 4 {
            stream.extend(quote! {
                impl #name {
                    pub fn new(#(#mandatory_param_name : #mandatory_param_ty),*) -> Self {
                        Self {
                          #(#assign,)*
                          #(#optionals : None),*
                        }
                    }
                }
            })
        }

        // impl From<String> for types that only have a single mandatory field of type string
        if mandatory_param_name.len() == 1 {
            let f = self.mandatory().next().unwrap();
            if !f.ty.is_vec && f.ty.ty.to_string() == "String" {
                stream.extend(quote! {
                    impl<T: Into<String>> From<T> for #name {
                          fn from(url: T) -> Self {
                              #name::new(url)
                          }
                    }
                });
            }
        }

        stream
    }

    /// Generates a builder that targets a command definition struct wrapping method + params.
    ///
    /// The builder collects individual parameter fields, then `build()` constructs:
    /// ```ignore
    /// CommandDef { method: FooMethod, params: FooParams { field1, field2, ... } }
    /// ```
    pub fn generate_command_builder(
        &self,
        def_name: &Ident,
        method_ident: &Ident,
        variant_ident: &Ident,
        params_ident: &Ident,
    ) -> TokenStream {
        if self.fields.is_empty() {
            return TokenStream::default();
        }

        let builder = format_ident!("{}Builder", def_name);

        let mut mandatory_param_name = vec![];
        for field in self.mandatory() {
            mandatory_param_name.push(&field.name_ident);
        }

        let mut setters = TokenStream::default();
        let mut builder_type_defs = TokenStream::default();
        let mut build_fn_assigns = TokenStream::default();

        for field in self.fields.iter().map(|(_, f)| f) {
            let field_name = &field.name_ident;
            let builder_ty = field.ty.builder_type();

            builder_type_defs.extend(quote! {
                #field_name: Option<#builder_ty>,
            });

            let ty_param = field.ty.param_type_def();
            let assign = if field.ty.is_vec {
                quote! { #field_name }
            } else {
                quote! { #field_name.into() }
            };

            if field.ty.is_vec {
                let ty = &field.ty.ty;
                let s = field_name.to_string().to_snake_case();
                let (iter_setter_name, single_setter_name) = if s.ends_with('s') {
                    (
                        field_name.clone(),
                        format_ident!("{}", generate_field_name(&s[..s.len() - 1])),
                    )
                } else {
                    (format_ident!("{}s", s), field_name.clone())
                };

                setters.extend(quote! {
                    pub fn #single_setter_name(mut self, #single_setter_name: impl Into<#ty>) -> Self {
                        let v = self.#field_name.get_or_insert(Vec::new());
                        v.push(#single_setter_name.into());
                        self
                    }

                    pub fn #iter_setter_name<I, S>(mut self, #iter_setter_name: I) -> Self
                    where
                        I: IntoIterator<Item = S>,
                        S: Into<#ty>,
                    {
                        let v = self.#field_name.get_or_insert(Vec::new());
                        for val in #iter_setter_name {
                            v.push(val.into());
                        }
                        self
                    }
                });
            } else {
                setters.extend(quote! {
                    pub fn #field_name(mut self, #field_name: #ty_param) -> Self {
                        self.#field_name = Some(#assign);
                        self
                    }
                });
            }

            if field.optional {
                if field.ty.needs_box {
                    build_fn_assigns.extend(quote! {
                        #field_name: self.#field_name.map(Box::new),
                    })
                } else {
                    build_fn_assigns.extend(quote! {
                        #field_name: self.#field_name,
                    })
                }
            } else if field.ty.needs_box {
                build_fn_assigns.extend(quote! {
                    #field_name: Box::new(self.#field_name.ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(#field_name)))?),
                })
            } else {
                build_fn_assigns.extend(quote! {
                    #field_name: self.#field_name.ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(#field_name)))?,
                })
            }
        }

        let build_fn = if mandatory_param_name.is_empty() {
            quote! {
                pub fn build(self) -> #def_name {
                    #def_name {
                        method: #method_ident::#variant_ident,
                        params: #params_ident {
                            #build_fn_assigns
                        },
                    }
                }
            }
        } else {
            quote! {
                pub fn build(self) -> Result<#def_name, String> {
                    Ok(#def_name {
                        method: #method_ident::#variant_ident,
                        params: #params_ident {
                            #build_fn_assigns
                        },
                    })
                }
            }
        };

        quote! {
            impl #def_name {
                pub fn builder() -> #builder {
                    #builder::default()
                }
            }

            #[derive(Default, Clone)]
            pub struct #builder {
                #builder_type_defs
            }

            impl #builder {
                #setters

                #build_fn
            }
        }
    }

    /// Generates the builder struct, setters, build fn, and the `builder()` method.
    /// Goes into a separate builder file.
    pub fn generate_builder(&self) -> TokenStream {
        if self.fields.is_empty() {
            return TokenStream::default();
        }

        let name = &self.name;
        let builder = format_ident!("{}Builder", self.name);

        let mut mandatory_param_name = vec![];
        for field in self.mandatory() {
            mandatory_param_name.push(&field.name_ident);
        }

        let mut setters = TokenStream::default();
        let mut names = vec![];
        let mut builder_type_defs = TokenStream::default();
        let mut build_fn_assigns = TokenStream::default();

        for field in self.fields.iter().map(|(_, f)| f) {
            let field_name = &field.name_ident;
            names.push(field_name);
            let builder_ty = field.ty.builder_type();
            builder_type_defs.extend(quote! {
                #field_name: Option<#builder_ty>,
            });

            let ty_param = field.ty.param_type_def();
            let assign = if field.ty.is_vec {
                quote! {#field_name}
            } else {
                quote! {#field_name.into()}
            };

            if field.ty.is_vec {
                let ty = &field.ty.ty;
                let s = field_name.to_string().to_snake_case();
                let (iter_setter_name, single_setter_name) = if s.ends_with('s') {
                    (
                        field_name.clone(),
                        format_ident!("{}", generate_field_name(&s[..s.len() - 1])),
                    )
                } else {
                    (format_ident!("{}s", s), field_name.clone())
                };

                setters.extend(
                  quote! {
                     pub fn #single_setter_name(mut self, #single_setter_name: impl Into<#ty>) -> Self {
                        let v = self.#field_name.get_or_insert(Vec::new());
                        v.push(#single_setter_name.into());
                        self
                     }

                    pub fn #iter_setter_name<I, S>(mut self, #iter_setter_name: I) -> Self
                    where
                        I: IntoIterator<Item = S>,
                        S: Into<#ty>,
                    {
                        let v = self.#field_name.get_or_insert(Vec::new());
                        for val in #iter_setter_name {
                            v.push(val.into());
                        }
                        self
                    }
                  }
                );
            } else {
                setters.extend(quote! {
                    pub fn #field_name(mut self, #field_name : #ty_param ) -> Self {
                        self.#field_name = Some(#assign);
                        self
                    }
                });
            }

            // mappings for the `build` fn
            if field.optional {
                if field.ty.needs_box {
                    build_fn_assigns.extend(quote! {
                        #field_name : self.#field_name.map(Box::new),
                    })
                } else {
                    build_fn_assigns.extend(quote! {
                        #field_name : self.#field_name,
                    })
                }
            } else if field.ty.needs_box {
                build_fn_assigns.extend(
                        quote!{
                            #field_name : Box::new(self.#field_name.ok_or_else(||std::stringify!("Field `{}` is mandatory.", std::stringify!(#field_name))))?,
                        }
                    )
            } else {
                build_fn_assigns.extend(
                        quote!{
                            #field_name : self.#field_name.ok_or_else(||format!("Field `{}` is mandatory.", std::stringify!(#field_name)))?,
                        }
                    )
            }
        }

        let build_fn = if mandatory_param_name.is_empty() {
            quote! {
                pub fn build(self) -> #name {
                    #name {
                        #build_fn_assigns
                    }
                }
            }
        } else {
            quote! {
                pub fn build(self) -> Result<#name, String> {
                    Ok(#name {
                        #build_fn_assigns
                    })
                }
            }
        };

        quote! {
               impl #name {
                    pub fn builder() -> #builder {
                        #builder::default()
                    }
               }

               #[derive(Default, Clone)]
               pub struct #builder {
                    #builder_type_defs
               }

               impl #builder {
                    #setters
                    #build_fn
               }
        }
    }
}
