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
