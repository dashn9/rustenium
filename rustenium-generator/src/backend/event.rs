use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::backend::base_types::Module;

/// Builds a module-level Event enum (placed in `{module}/events.rs`).
///
/// Wraps individual event structs within a single module:
/// ```ignore
/// pub enum Event {
///     DomContentLoaded(EventDomContentLoaded),
///     RequestWillBeSent(EventRequestWillBeSent),
///     ...
/// }
/// ```
///
/// Generates `From<EventFoo> for Event` for each variant.
pub fn generate_module_event_enum(
    module: &Module,
    with_deprecated: bool,
    with_experimental: bool,
) -> TokenStream {
    let filtered: Vec<_> = module
        .events
        .iter()
        .filter(|ev| with_deprecated || !ev.deprecated)
        .filter(|ev| with_experimental || !ev.experimental)
        .collect();

    if filtered.is_empty() {
        return TokenStream::default();
    }

    let mut variants = TokenStream::default();
    let mut from_impls = TokenStream::default();

    for event in &filtered {
        let var_ident = format_ident!("{}", event.name.to_upper_camel_case());
        let ty_ident = format_ident!("Event{}", event.name.to_upper_camel_case());

        let deprecated = if event.deprecated {
            quote! { #[deprecated] }
        } else {
            TokenStream::default()
        };

        variants.extend(quote! {
            #deprecated
            #var_ident(#ty_ident),
        });

        from_impls.extend(quote! {
            impl From<#ty_ident> for Event {
                fn from(ev: #ty_ident) -> Self {
                    Event::#var_ident(ev)
                }
            }
        });
    }

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Event {
            #variants
        }

        #from_impls
    }
}

/// Builds a protocol-level Event enum (placed in `{protocol}/mod.rs`).
///
/// Wraps module-level Event enums:
/// ```ignore
/// pub enum Event {
///     Dom(dom::Event),
///     Network(network::Event),
///     ...
///     Other(serde_json::Value),
/// }
/// ```
///
/// Generates:
/// - `From<mod::Event> for Event` (upward)
/// - `TryFrom<Event> for mod::Event` (downward)
pub fn generate_protocol_event_enum(
    modules: &[&Module],
    with_deprecated: bool,
    with_experimental: bool,
) -> TokenStream {
    // Only include modules that have events
    let modules_with_events: Vec<_> = modules
        .iter()
        .filter(|m| {
            m.events
                .iter()
                .any(|ev| (with_deprecated || !ev.deprecated) && (with_experimental || !ev.experimental))
        })
        .collect();

    if modules_with_events.is_empty() {
        return TokenStream::default();
    }

    let mut variants = TokenStream::default();
    let mut from_impls = TokenStream::default();
    let mut try_from_impls = TokenStream::default();

    for module in &modules_with_events {
        let mod_name = format_ident!("{}", module.name.to_snake_case());
        let var_ident = format_ident!("{}", module.name.to_upper_camel_case());

        let deprecated = if module.deprecated {
            quote! { #[deprecated] }
        } else {
            TokenStream::default()
        };

        variants.extend(quote! {
            #deprecated
            #var_ident(#mod_name::Event),
        });

        from_impls.extend(quote! {
            impl From<#mod_name::Event> for Event {
                fn from(ev: #mod_name::Event) -> Self {
                    Event::#var_ident(ev)
                }
            }
        });

        try_from_impls.extend(quote! {
            impl TryFrom<Event> for #mod_name::Event {
                type Error = Event;

                fn try_from(ev: Event) -> Result<Self, Self::Error> {
                    match ev {
                        Event::#var_ident(inner) => Ok(inner),
                        other => Err(other),
                    }
                }
            }
        });
    }

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Event {
            #variants
            Other(serde_json::Value),
        }

        #from_impls
        #try_from_impls
    }
}

/// Builds the top-level Event enum (placed in root `mod.rs`).
///
/// Wraps protocol-level Event enums:
/// ```ignore
/// pub enum Event {
///     JsProtocol(js_protocol::Event),
///     BrowserProtocol(browser_protocol::Event),
///     Other(serde_json::Value),
/// }
/// ```
///
/// Generates:
/// - `From<protocol::Event> for Event` (upward)
/// - `TryFrom<Event> for protocol::Event` (downward)
pub fn generate_top_event_enum(protocol_names: &[String]) -> TokenStream {
    if protocol_names.is_empty() {
        return TokenStream::default();
    }

    let mut variants = TokenStream::default();
    let mut from_impls = TokenStream::default();
    let mut try_from_impls = TokenStream::default();

    for name in protocol_names {
        let mod_name = format_ident!("{}", name.to_snake_case());
        let var_name = format_ident!("{}", name.to_upper_camel_case());

        variants.extend(quote! {
            #var_name(#mod_name::Event),
        });

        from_impls.extend(quote! {
            impl From<#mod_name::Event> for Event {
                fn from(ev: #mod_name::Event) -> Self {
                    Event::#var_name(ev)
                }
            }
        });

        try_from_impls.extend(quote! {
            impl TryFrom<Event> for #mod_name::Event {
                type Error = Event;

                fn try_from(ev: Event) -> Result<Self, Self::Error> {
                    match ev {
                        Event::#var_name(inner) => Ok(inner),
                        other => Err(other),
                    }
                }
            }
        });
    }

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Event {
            #variants
            Other(serde_json::Value),
        }

        #from_impls
        #try_from_impls

        #[derive(Debug, PartialEq, Clone)]
        pub struct EventMessage {
            pub method: String,
            pub session_id: Option<String>,
            pub params: Event,
        }
    }
}
