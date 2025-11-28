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

#[macro_export]
macro_rules! xpath {
    ($value:expr) => {
        $crate::rustenium_bidi_commands::browsing_context::types::Locator::XPathLocator(
            $crate::rustenium_bidi_commands::browsing_context::types::XPathLocator {
                r#type: $crate::rustenium_bidi_commands::browsing_context::types::XpathEnum::Xpath,
                value: $value.to_string(),
            }
        )
    };
}