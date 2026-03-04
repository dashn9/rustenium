use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// The `group_enum!` macro definition, emitted into the generated output's `macros.rs`.
///
/// Written as a raw string because `quote!` cannot produce `$` tokens for macro_rules patterns.
///
/// Provides two forms:
/// - **Closed**: `group_enum!(EnumName { Var1(Type1), Var2(Type2) });`
/// - **Open**: `group_enum!(EnumName { Var1(Type1), Var2(Type2) } + other);`
///   adds an `Other(serde_json::Value)` catch-all variant.
///
/// Both forms auto-generate `From<Inner> for Enum` (upward) and
/// `TryFrom<Enum> for Inner` (downward) for every variant.
pub const GROUP_ENUM_MACRO: &str = r#"
macro_rules! group_enum {
    ($name:ident { $( $variant:ident($ty:ty) ),* $(,)? } + other) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $( $variant($ty), )*
            Other(serde_json::Value),
        }

        $(
            impl From<$ty> for $name {
                fn from(v: $ty) -> Self {
                    $name::$variant(v)
                }
            }

            impl TryFrom<$name> for $ty {
                type Error = $name;

                #[allow(unreachable_patterns)]
                fn try_from(e: $name) -> Result<Self, <$ty as TryFrom<$name>>::Error> {
                    match e {
                        $name::$variant(inner) => Ok(inner),
                        other => Err(other),
                    }
                }
            }
        )*
    };

    ($name:ident { $( $variant:ident($ty:ty) ),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $( $variant($ty), )*
        }

        $(
            impl From<$ty> for $name {
                fn from(v: $ty) -> Self {
                    $name::$variant(v)
                }
            }

            impl TryFrom<$name> for $ty {
                type Error = $name;

                #[allow(unreachable_patterns)]
                fn try_from(e: $name) -> Result<Self, <$ty as TryFrom<$name>>::Error> {
                    match e {
                        $name::$variant(inner) => Ok(inner),
                        other => Err(other),
                    }
                }
            }
        )*
    };
}
"#;

/// Generate a closed `group_enum!` invocation (no catch-all variant).
///
/// Used at module level for Type, Command, and Event groups.
/// Returns empty TokenStream if `entries` is empty.
pub fn group_enum_closed(enum_name: &Ident, entries: &[(Ident, TokenStream)]) -> TokenStream {
    if entries.is_empty() {
        return TokenStream::default();
    }

    let variants: Vec<_> = entries
        .iter()
        .map(|(var, ty)| quote! { #var(#ty) })
        .collect();

    quote! {
        group_enum!(#enum_name { #(#variants),* });
    }
}

/// Generate an open `group_enum!` invocation (with `Other(serde_json::Value)` catch-all).
///
/// Used at protocol and top level for Event groups.
/// Returns empty TokenStream if `entries` is empty.
pub fn group_enum_open(enum_name: &Ident, entries: &[(Ident, TokenStream)]) -> TokenStream {
    if entries.is_empty() {
        return TokenStream::default();
    }

    let variants: Vec<_> = entries
        .iter()
        .map(|(var, ty)| quote! { #var(#ty) })
        .collect();

    quote! {
        group_enum!(#enum_name { #(#variants),* } + other);
    }
}
