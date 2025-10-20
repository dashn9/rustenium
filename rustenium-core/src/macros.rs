#[macro_export]
macro_rules! impl_has_context {
    ($($enum_name:ident)?, [$($variant_name:ident),+]) => {
        impl HasContext for $enum_name {
            fn get_context(&self) -> Option<&str> {
                match self {
                    $($enum_name::$variant_name => Some($variant_name.params.context.clone())),+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_has_method {
    ($enum_name:ident, [$($variant_name:ident),+]) => {
        impl HasMethod for $enum_name {
            fn get_method(&self) -> String {
                match self {
                    $($enum_name::$variant_name(ev) => serde_json::to_string(&ev.method).unwrap_or("".to_string())),+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_has_method_getter {
    ($enum_name:ident, [$($variant_name:ident),+]) => {
        impl HasMethodGetter for $enum_name {
            fn get_method(&self) -> String {
                match self {
                    $($enum_name::$variant_name(ev) => ev.get_method()),+
                }
            }
        }
    }
}
