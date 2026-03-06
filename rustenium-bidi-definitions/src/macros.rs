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
