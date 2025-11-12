#[macro_export]
macro_rules! css {
    ($value:expr) => {
        $crate::rustenium_bidi_commands::browsing_context::types::Locator::CssLocator(
            $crate::rustenium_bidi_commands::browsing_context::types::CssLocator {
                r#type: $crate::rustenium_bidi_commands::browsing_context::types::CssEnum::Css,
                value: $value.to_string(),
            }
        )
    };
}