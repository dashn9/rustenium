use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::backend::base_types::Module;

/// Builds a protocol-level Event enum for a single protocol's mod.rs.
///
/// Generates:
/// ```ignore
/// pub enum Event {
///     Dom(dom::EventDomContentLoaded),
///     Network(network::EventRequestWillBeSent),
///     ...
///     Other(serde_json::Value),
/// }
/// ```
pub fn generate_protocol_event_enum(
    modules: &[&Module],
    with_deprecated: bool,
    with_experimental: bool,
) -> TokenStream {
    let mut variants = TokenStream::default();
    let mut var_idents = Vec::new();
    let mut mod_idents = Vec::new();
    let mut ty_idents = Vec::new();

    for module in modules {
        let mod_name = format_ident!("{}", module.name.to_snake_case());
        for event in module
            .events
            .iter()
            .filter(|ev| with_deprecated || !ev.deprecated)
            .filter(|ev| with_experimental || !ev.experimental)
        {
            let var_ident = format_ident!(
                "{}{}",
                module.name.to_upper_camel_case(),
                event.name.to_upper_camel_case()
            );
            let ty_ident = format_ident!("Event{}", event.name.to_upper_camel_case());

            let deprecated = if event.deprecated {
                quote! { #[deprecated] }
            } else {
                TokenStream::default()
            };

            variants.extend(quote! {
                #deprecated
                #var_ident(#mod_name::#ty_ident),
            });

            var_idents.push(var_ident);
            mod_idents.push(mod_name.clone());
            ty_idents.push(ty_ident);
        }
    }

    if var_idents.is_empty() {
        return TokenStream::default();
    }

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Event {
            #variants
            Other(serde_json::Value),
        }

        impl Event {
            pub fn other(other: serde_json::Value) -> Self {
                Event::Other(other)
            }

            pub fn into_json(self) -> serde_json::Result<serde_json::Value> {
                match self {
                    #(Event::#var_idents(inner) => serde_json::to_value(inner),)*
                    Event::Other(val) => Ok(val),
                }
            }
        }
    }
}

/// Builds the top-level Event enum that wraps protocol-level Event enums.
///
/// Generates:
/// ```ignore
/// pub enum Event {
///     JsProtocol(js_protocol::Event),
///     BrowserProtocol(browser_protocol::Event),
///     Other(serde_json::Value),
/// }
/// ```
pub fn generate_top_event_enum(protocol_names: &[String]) -> TokenStream {
    let mut variants = TokenStream::default();
    let mut var_idents = Vec::new();
    let mut mod_idents = Vec::new();

    for name in protocol_names {
        let mod_name = format_ident!("{}", name.to_snake_case());
        let var_name = format_ident!("{}", name.to_upper_camel_case());
        variants.extend(quote! {
            #var_name(#mod_name::Event),
        });
        var_idents.push(var_name);
        mod_idents.push(mod_name);
    }

    if var_idents.is_empty() {
        return TokenStream::default();
    }

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Event {
            #variants
            Other(serde_json::Value),
        }

        impl Event {
            pub fn other(other: serde_json::Value) -> Self {
                Event::Other(other)
            }

            pub fn into_json(self) -> serde_json::Result<serde_json::Value> {
                match self {
                    #(Event::#var_idents(inner) => inner.into_json(),)*
                    Event::Other(val) => Ok(val),
                }
            }
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct EventMessage {
            pub method: String,
            pub session_id: Option<String>,
            pub params: Event,
        }
    }
}
