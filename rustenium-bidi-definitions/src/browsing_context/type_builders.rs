use super::types::*;
impl AccessibilityLocator {
    pub fn builder() -> AccessibilityLocatorBuilder {
        <AccessibilityLocatorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AccessibilityLocatorBuilder {
    r#type: Option<AccessibilityLocatorType>,
    value: Option<AccessibilityLocatorValue>,
}
impl AccessibilityLocatorBuilder {
    pub fn r#type(mut self, r#type: impl Into<AccessibilityLocatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<AccessibilityLocatorValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<AccessibilityLocator, String> {
        Ok(AccessibilityLocator {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl AccessibilityLocatorValue {
    pub fn builder() -> AccessibilityLocatorValueBuilder {
        <AccessibilityLocatorValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AccessibilityLocatorValueBuilder {
    name: Option<String>,
    role: Option<String>,
}
impl AccessibilityLocatorValueBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }
    pub fn build(self) -> AccessibilityLocatorValue {
        AccessibilityLocatorValue {
            name: self.name,
            role: self.role,
        }
    }
}
impl CssLocator {
    pub fn builder() -> CssLocatorBuilder {
        <CssLocatorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssLocatorBuilder {
    r#type: Option<CssLocatorType>,
    value: Option<String>,
}
impl CssLocatorBuilder {
    pub fn r#type(mut self, r#type: impl Into<CssLocatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<CssLocator, String> {
        Ok(CssLocator {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl ContextLocator {
    pub fn builder() -> ContextLocatorBuilder {
        <ContextLocatorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContextLocatorBuilder {
    r#type: Option<ContextLocatorType>,
    value: Option<ContextLocatorValue>,
}
impl ContextLocatorBuilder {
    pub fn r#type(mut self, r#type: impl Into<ContextLocatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<ContextLocatorValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<ContextLocator, String> {
        Ok(ContextLocator {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl ContextLocatorValue {
    pub fn builder() -> ContextLocatorValueBuilder {
        <ContextLocatorValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContextLocatorValueBuilder {
    context: Option<BrowsingContext>,
}
impl ContextLocatorValueBuilder {
    pub fn context(mut self, context: impl Into<BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn build(self) -> Result<ContextLocatorValue, String> {
        Ok(ContextLocatorValue {
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
        })
    }
}
impl InnerTextLocator {
    pub fn builder() -> InnerTextLocatorBuilder {
        <InnerTextLocatorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InnerTextLocatorBuilder {
    r#type: Option<InnerTextLocatorType>,
    value: Option<String>,
    ignore_case: Option<bool>,
    match_type: Option<InnerTextLocatorMatchType>,
    max_depth: Option<u64>,
}
impl InnerTextLocatorBuilder {
    pub fn r#type(mut self, r#type: impl Into<InnerTextLocatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn ignore_case(mut self, ignore_case: impl Into<bool>) -> Self {
        self.ignore_case = Some(ignore_case.into());
        self
    }
    pub fn match_type(mut self, match_type: impl Into<InnerTextLocatorMatchType>) -> Self {
        self.match_type = Some(match_type.into());
        self
    }
    pub fn max_depth(mut self, max_depth: impl Into<u64>) -> Self {
        self.max_depth = Some(max_depth.into());
        self
    }
    pub fn build(self) -> Result<InnerTextLocator, String> {
        Ok(InnerTextLocator {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            ignore_case: self.ignore_case,
            match_type: self.match_type,
            max_depth: self.max_depth,
        })
    }
}
impl XPathLocator {
    pub fn builder() -> XPathLocatorBuilder {
        <XPathLocatorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct XPathLocatorBuilder {
    r#type: Option<XPathLocatorType>,
    value: Option<String>,
}
impl XPathLocatorBuilder {
    pub fn r#type(mut self, r#type: impl Into<XPathLocatorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<XPathLocator, String> {
        Ok(XPathLocator {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl ImageFormat {
    pub fn builder() -> ImageFormatBuilder {
        <ImageFormatBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ImageFormatBuilder {
    r#type: Option<String>,
    quality: Option<f64>,
}
impl ImageFormatBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn quality(mut self, quality: impl Into<f64>) -> Self {
        self.quality = Some(quality.into());
        self
    }
    pub fn build(self) -> Result<ImageFormat, String> {
        Ok(ImageFormat {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            quality: self.quality,
        })
    }
}
impl ElementClipRectangle {
    pub fn builder() -> ElementClipRectangleBuilder {
        <ElementClipRectangleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ElementClipRectangleBuilder {
    r#type: Option<ElementClipRectangleType>,
    element: Option<crate::script::types::SharedReference>,
}
impl ElementClipRectangleBuilder {
    pub fn r#type(mut self, r#type: impl Into<ElementClipRectangleType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn element(mut self, element: impl Into<crate::script::types::SharedReference>) -> Self {
        self.element = Some(element.into());
        self
    }
    pub fn build(self) -> Result<ElementClipRectangle, String> {
        Ok(ElementClipRectangle {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            element: self
                .element
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(element)))?,
        })
    }
}
impl BoxClipRectangle {
    pub fn builder() -> BoxClipRectangleBuilder {
        <BoxClipRectangleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BoxClipRectangleBuilder {
    r#type: Option<BoxClipRectangleType>,
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
}
impl BoxClipRectangleBuilder {
    pub fn r#type(mut self, r#type: impl Into<BoxClipRectangleType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn width(mut self, width: impl Into<f64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<f64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn build(self) -> Result<BoxClipRectangle, String> {
        Ok(BoxClipRectangle {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
        })
    }
}
impl PrintMarginParameters {
    pub fn builder() -> PrintMarginParametersBuilder {
        <PrintMarginParametersBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PrintMarginParametersBuilder {
    bottom: Option<f64>,
    left: Option<f64>,
    right: Option<f64>,
    top: Option<f64>,
}
impl PrintMarginParametersBuilder {
    pub fn bottom(mut self, bottom: impl Into<f64>) -> Self {
        self.bottom = Some(bottom.into());
        self
    }
    pub fn left(mut self, left: impl Into<f64>) -> Self {
        self.left = Some(left.into());
        self
    }
    pub fn right(mut self, right: impl Into<f64>) -> Self {
        self.right = Some(right.into());
        self
    }
    pub fn top(mut self, top: impl Into<f64>) -> Self {
        self.top = Some(top.into());
        self
    }
    pub fn build(self) -> PrintMarginParameters {
        PrintMarginParameters {
            bottom: self.bottom,
            left: self.left,
            right: self.right,
            top: self.top,
        }
    }
}
impl PrintPageParameters {
    pub fn builder() -> PrintPageParametersBuilder {
        <PrintPageParametersBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PrintPageParametersBuilder {
    height: Option<f64>,
    width: Option<f64>,
}
impl PrintPageParametersBuilder {
    pub fn height(mut self, height: impl Into<f64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn width(mut self, width: impl Into<f64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn build(self) -> PrintPageParameters {
        PrintPageParameters {
            height: self.height,
            width: self.width,
        }
    }
}
impl Viewport {
    pub fn builder() -> ViewportBuilder {
        <ViewportBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ViewportBuilder {
    width: Option<u64>,
    height: Option<u64>,
}
impl ViewportBuilder {
    pub fn width(mut self, width: impl Into<u64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<u64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn build(self) -> Result<Viewport, String> {
        Ok(Viewport {
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
        })
    }
}
impl Info {
    pub fn builder() -> InfoBuilder {
        <InfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InfoBuilder {
    children: Option<InfoList>,
    client_window: Option<crate::browser::types::ClientWindow>,
    context: Option<BrowsingContext>,
    original_opener: Option<BrowsingContext>,
    url: Option<String>,
    user_context: Option<crate::browser::types::UserContext>,
    parent: Option<BrowsingContext>,
}
impl InfoBuilder {
    pub fn children(mut self, children: impl Into<InfoList>) -> Self {
        self.children = Some(children.into());
        self
    }
    pub fn client_window(
        mut self,
        client_window: impl Into<crate::browser::types::ClientWindow>,
    ) -> Self {
        self.client_window = Some(client_window.into());
        self
    }
    pub fn context(mut self, context: impl Into<BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn original_opener(mut self, original_opener: impl Into<BrowsingContext>) -> Self {
        self.original_opener = Some(original_opener.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        self.user_context = Some(user_context.into());
        self
    }
    pub fn parent(mut self, parent: impl Into<BrowsingContext>) -> Self {
        self.parent = Some(parent.into());
        self
    }
    pub fn build(self) -> Result<Info, String> {
        Ok(Info {
            children: self.children,
            client_window: self.client_window.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(client_window))
            })?,
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
            original_opener: self.original_opener,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            user_context: self.user_context.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(user_context))
            })?,
            parent: self.parent,
        })
    }
}
impl BaseNavigationInfo {
    pub fn builder() -> BaseNavigationInfoBuilder {
        <BaseNavigationInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BaseNavigationInfoBuilder {
    context: Option<BrowsingContext>,
    navigation: Option<Navigation>,
    timestamp: Option<u64>,
    url: Option<String>,
}
impl BaseNavigationInfoBuilder {
    pub fn context(mut self, context: impl Into<BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn navigation(mut self, navigation: impl Into<Navigation>) -> Self {
        self.navigation = Some(navigation.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<u64>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<BaseNavigationInfo, String> {
        Ok(BaseNavigationInfo {
            context: self
                .context
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
            navigation: self.navigation,
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
        })
    }
}
impl DownloadCanceledParams {
    pub fn builder() -> DownloadCanceledParamsBuilder {
        <DownloadCanceledParamsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DownloadCanceledParamsBuilder {
    status: Option<DownloadCanceledParamsStatus>,
    base_navigation_info: Option<BaseNavigationInfo>,
}
impl DownloadCanceledParamsBuilder {
    pub fn status(mut self, status: impl Into<DownloadCanceledParamsStatus>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn base_navigation_info(
        mut self,
        base_navigation_info: impl Into<BaseNavigationInfo>,
    ) -> Self {
        self.base_navigation_info = Some(base_navigation_info.into());
        self
    }
    pub fn build(self) -> Result<DownloadCanceledParams, String> {
        Ok(DownloadCanceledParams {
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            base_navigation_info: self.base_navigation_info.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(base_navigation_info)
                )
            })?,
        })
    }
}
impl DownloadCompleteParams {
    pub fn builder() -> DownloadCompleteParamsBuilder {
        <DownloadCompleteParamsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DownloadCompleteParamsBuilder {
    status: Option<DownloadCompleteParamsStatus>,
    filepath: Option<String>,
    base_navigation_info: Option<BaseNavigationInfo>,
}
impl DownloadCompleteParamsBuilder {
    pub fn status(mut self, status: impl Into<DownloadCompleteParamsStatus>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn filepath(mut self, filepath: impl Into<String>) -> Self {
        self.filepath = Some(filepath.into());
        self
    }
    pub fn base_navigation_info(
        mut self,
        base_navigation_info: impl Into<BaseNavigationInfo>,
    ) -> Self {
        self.base_navigation_info = Some(base_navigation_info.into());
        self
    }
    pub fn build(self) -> Result<DownloadCompleteParams, String> {
        Ok(DownloadCompleteParams {
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            filepath: self.filepath,
            base_navigation_info: self.base_navigation_info.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(base_navigation_info)
                )
            })?,
        })
    }
}
