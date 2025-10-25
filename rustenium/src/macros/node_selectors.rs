#[macro_export]
macro_rules! css {
    ($value:expr) => {
        Locator::CssLocator(CssLocator {
            r#type: CssEnum::Css,
            value: $value.to_string(),
        })
    };
}