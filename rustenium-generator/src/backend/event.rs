use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// The `group_enum!` macro and `impl_from!` macro, emitted into the generated output's `macros.rs`.
pub const GROUP_ENUM_MACRO: &str = r#"
macro_rules! group_enum {
    ($name:ident { $( $variant:ident($ty:ty) ),* $(,)? } + other) => {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(untagged)]
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

    ($name:ident { $( $variant:ident($ty:ty) ),* $(,)? } + identifiable) => {
        group_enum!($name { $( $variant($ty) ),* });

        impl $name {
            pub fn identifier(&self) -> &'static str {
                match self {
                    $( $name::$variant(inner) => inner.identifier(), )*
                }
            }
        }
    };

    ($name:ident { $( $variant:ident($ty:ty) ),* $(,)? } + other + identifiable) => {
        group_enum!($name { $( $variant($ty) ),* } + other);

        impl $name {
            pub fn identifier(&self) -> &'static str {
                match self {
                    $( $name::$variant(inner) => inner.identifier(), )*
                    $name::Other(_) => "unknown",
                }
            }
        }
    };

    ($name:ident { $( $variant:ident($ty:ty) ),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(untagged)]
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

/// Generates transitive `From` and `TryFrom` impls.
/// `impl_from!(LeafType => IntermediateEnum => TargetEnum, ...);`
macro_rules! impl_from {
    ($( $from:ty => $via:ty => $to:ty ),* $(,)?) => {
        $(
            impl From<$from> for $to {
                fn from(v: $from) -> Self {
                    Self::from(<$via>::from(v))
                }
            }

            impl TryFrom<$to> for $from {
                type Error = $to;

                fn try_from(e: $to) -> Result<Self, <$from as TryFrom<$to>>::Error> {
                    let inner = <$via>::try_from(e)?;
                    Self::try_from(inner).map_err(<$to>::from)
                }
            }
        )*
    };
}
"#;

/// Generate a closed `group_enum!` invocation (no catch-all variant).
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

/// Generate a closed `group_enum!` with `identifier()` method.
pub fn group_enum_closed_identifiable(enum_name: &Ident, entries: &[(Ident, TokenStream)]) -> TokenStream {
    if entries.is_empty() {
        return TokenStream::default();
    }

    let variants: Vec<_> = entries
        .iter()
        .map(|(var, ty)| quote! { #var(#ty) })
        .collect();

    quote! {
        group_enum!(#enum_name { #(#variants),* } + identifiable);
    }
}

/// Generate an open `group_enum!` invocation (with `Other(serde_json::Value)` catch-all).
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

/// Generate an open `group_enum!` with `identifier()` method.
pub fn group_enum_open_identifiable(enum_name: &Ident, entries: &[(Ident, TokenStream)]) -> TokenStream {
    if entries.is_empty() {
        return TokenStream::default();
    }

    let variants: Vec<_> = entries
        .iter()
        .map(|(var, ty)| quote! { #var(#ty) })
        .collect();

    quote! {
        group_enum!(#enum_name { #(#variants),* } + other + identifiable);
    }
}

/// Generate an `impl_from!` invocation for transitive From/TryFrom.
/// Each entry is `(from, via, to)`.
pub fn impl_from_transitive(entries: &[(TokenStream, TokenStream, TokenStream)]) -> TokenStream {
    if entries.is_empty() {
        return TokenStream::default();
    }

    let stmts: Vec<_> = entries
        .iter()
        .map(|(from, via, to)| quote! { #from => #via => #to })
        .collect();

    quote! {
        impl_from!(#(#stmts),*);
    }
}
