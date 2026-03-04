use super::types::*;
impl AffectedCookie {
    pub fn builder() -> AffectedCookieBuilder {
        AffectedCookieBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AffectedCookieBuilder {
    name: Option<String>,
    path: Option<String>,
    domain: Option<String>,
}
impl AffectedCookieBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn build(self) -> Result<AffectedCookie, String> {
        Ok(AffectedCookie {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            path: self
                .path
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
            domain: self
                .domain
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(domain)))?,
        })
    }
}
impl AffectedRequest {
    pub fn builder() -> AffectedRequestBuilder {
        AffectedRequestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AffectedRequestBuilder {
    request_id: Option<super::super::network::types::RequestId>,
    url: Option<String>,
}
impl AffectedRequestBuilder {
    pub fn request_id(
        mut self,
        request_id: impl Into<super::super::network::types::RequestId>,
    ) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<AffectedRequest, String> {
        Ok(AffectedRequest {
            request_id: self.request_id,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
        })
    }
}
impl AffectedFrame {
    pub fn builder() -> AffectedFrameBuilder {
        AffectedFrameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AffectedFrameBuilder {
    frame_id: Option<super::super::page::types::FrameId>,
}
impl AffectedFrameBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<AffectedFrame, String> {
        Ok(AffectedFrame {
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
        })
    }
}
impl CookieIssueInsight {
    pub fn builder() -> CookieIssueInsightBuilder {
        CookieIssueInsightBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieIssueInsightBuilder {
    r#type: Option<InsightType>,
    table_entry_url: Option<String>,
}
impl CookieIssueInsightBuilder {
    pub fn r#type(mut self, r#type: impl Into<InsightType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn table_entry_url(mut self, table_entry_url: impl Into<String>) -> Self {
        self.table_entry_url = Some(table_entry_url.into());
        self
    }
    pub fn build(self) -> Result<CookieIssueInsight, String> {
        Ok(CookieIssueInsight {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            table_entry_url: self.table_entry_url,
        })
    }
}
impl CookieIssueDetails {
    pub fn builder() -> CookieIssueDetailsBuilder {
        CookieIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieIssueDetailsBuilder {
    cookie: Option<AffectedCookie>,
    raw_cookie_line: Option<String>,
    cookie_warning_reasons: Option<Vec<CookieWarningReason>>,
    cookie_exclusion_reasons: Option<Vec<CookieExclusionReason>>,
    operation: Option<CookieOperation>,
    site_for_cookies: Option<String>,
    cookie_url: Option<String>,
    request: Option<AffectedRequest>,
    insight: Option<CookieIssueInsight>,
}
impl CookieIssueDetailsBuilder {
    pub fn cookie(mut self, cookie: impl Into<AffectedCookie>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
    pub fn raw_cookie_line(mut self, raw_cookie_line: impl Into<String>) -> Self {
        self.raw_cookie_line = Some(raw_cookie_line.into());
        self
    }
    pub fn cookie_warning_reason(
        mut self,
        cookie_warning_reason: impl Into<CookieWarningReason>,
    ) -> Self {
        let v = self.cookie_warning_reasons.get_or_insert(Vec::new());
        v.push(cookie_warning_reason.into());
        self
    }
    pub fn cookie_warning_reasons<I, S>(mut self, cookie_warning_reasons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CookieWarningReason>,
    {
        let v = self.cookie_warning_reasons.get_or_insert(Vec::new());
        for val in cookie_warning_reasons {
            v.push(val.into());
        }
        self
    }
    pub fn cookie_exclusion_reason(
        mut self,
        cookie_exclusion_reason: impl Into<CookieExclusionReason>,
    ) -> Self {
        let v = self.cookie_exclusion_reasons.get_or_insert(Vec::new());
        v.push(cookie_exclusion_reason.into());
        self
    }
    pub fn cookie_exclusion_reasons<I, S>(mut self, cookie_exclusion_reasons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CookieExclusionReason>,
    {
        let v = self.cookie_exclusion_reasons.get_or_insert(Vec::new());
        for val in cookie_exclusion_reasons {
            v.push(val.into());
        }
        self
    }
    pub fn operation(mut self, operation: impl Into<CookieOperation>) -> Self {
        self.operation = Some(operation.into());
        self
    }
    pub fn site_for_cookies(mut self, site_for_cookies: impl Into<String>) -> Self {
        self.site_for_cookies = Some(site_for_cookies.into());
        self
    }
    pub fn cookie_url(mut self, cookie_url: impl Into<String>) -> Self {
        self.cookie_url = Some(cookie_url.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn insight(mut self, insight: impl Into<CookieIssueInsight>) -> Self {
        self.insight = Some(insight.into());
        self
    }
    pub fn build(self) -> Result<CookieIssueDetails, String> {
        Ok(CookieIssueDetails {
            cookie: self.cookie,
            raw_cookie_line: self.raw_cookie_line,
            cookie_warning_reasons: self.cookie_warning_reasons.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(cookie_warning_reasons)
                )
            })?,
            cookie_exclusion_reasons: self.cookie_exclusion_reasons.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(cookie_exclusion_reasons)
                )
            })?,
            operation: self
                .operation
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(operation)))?,
            site_for_cookies: self.site_for_cookies,
            cookie_url: self.cookie_url,
            request: self.request,
            insight: self.insight,
        })
    }
}
impl PerformanceIssueDetails {
    pub fn builder() -> PerformanceIssueDetailsBuilder {
        PerformanceIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PerformanceIssueDetailsBuilder {
    performance_issue_type: Option<PerformanceIssueType>,
    source_code_location: Option<SourceCodeLocation>,
}
impl PerformanceIssueDetailsBuilder {
    pub fn performance_issue_type(
        mut self,
        performance_issue_type: impl Into<PerformanceIssueType>,
    ) -> Self {
        self.performance_issue_type = Some(performance_issue_type.into());
        self
    }
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn build(self) -> Result<PerformanceIssueDetails, String> {
        Ok(PerformanceIssueDetails {
            performance_issue_type: self.performance_issue_type.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(performance_issue_type)
                )
            })?,
            source_code_location: self.source_code_location,
        })
    }
}
impl MixedContentIssueDetails {
    pub fn builder() -> MixedContentIssueDetailsBuilder {
        MixedContentIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct MixedContentIssueDetailsBuilder {
    resource_type: Option<MixedContentResourceType>,
    resolution_status: Option<MixedContentResolutionStatus>,
    insecure_url: Option<String>,
    main_resource_url: Option<String>,
    request: Option<AffectedRequest>,
    frame: Option<AffectedFrame>,
}
impl MixedContentIssueDetailsBuilder {
    pub fn resource_type(mut self, resource_type: impl Into<MixedContentResourceType>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }
    pub fn resolution_status(
        mut self,
        resolution_status: impl Into<MixedContentResolutionStatus>,
    ) -> Self {
        self.resolution_status = Some(resolution_status.into());
        self
    }
    pub fn insecure_url(mut self, insecure_url: impl Into<String>) -> Self {
        self.insecure_url = Some(insecure_url.into());
        self
    }
    pub fn main_resource_url(mut self, main_resource_url: impl Into<String>) -> Self {
        self.main_resource_url = Some(main_resource_url.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn frame(mut self, frame: impl Into<AffectedFrame>) -> Self {
        self.frame = Some(frame.into());
        self
    }
    pub fn build(self) -> Result<MixedContentIssueDetails, String> {
        Ok(MixedContentIssueDetails {
            resource_type: self.resource_type,
            resolution_status: self.resolution_status.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(resolution_status)
                )
            })?,
            insecure_url: self.insecure_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(insecure_url))
            })?,
            main_resource_url: self.main_resource_url.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(main_resource_url)
                )
            })?,
            request: self.request,
            frame: self.frame,
        })
    }
}
impl BlockedByResponseIssueDetails {
    pub fn builder() -> BlockedByResponseIssueDetailsBuilder {
        BlockedByResponseIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BlockedByResponseIssueDetailsBuilder {
    request: Option<AffectedRequest>,
    parent_frame: Option<AffectedFrame>,
    blocked_frame: Option<AffectedFrame>,
    reason: Option<BlockedByResponseReason>,
}
impl BlockedByResponseIssueDetailsBuilder {
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn parent_frame(mut self, parent_frame: impl Into<AffectedFrame>) -> Self {
        self.parent_frame = Some(parent_frame.into());
        self
    }
    pub fn blocked_frame(mut self, blocked_frame: impl Into<AffectedFrame>) -> Self {
        self.blocked_frame = Some(blocked_frame.into());
        self
    }
    pub fn reason(mut self, reason: impl Into<BlockedByResponseReason>) -> Self {
        self.reason = Some(reason.into());
        self
    }
    pub fn build(self) -> Result<BlockedByResponseIssueDetails, String> {
        Ok(BlockedByResponseIssueDetails {
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            parent_frame: self.parent_frame,
            blocked_frame: self.blocked_frame,
            reason: self
                .reason
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(reason)))?,
        })
    }
}
impl HeavyAdIssueDetails {
    pub fn builder() -> HeavyAdIssueDetailsBuilder {
        HeavyAdIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HeavyAdIssueDetailsBuilder {
    resolution: Option<HeavyAdResolutionStatus>,
    reason: Option<HeavyAdReason>,
    frame: Option<AffectedFrame>,
}
impl HeavyAdIssueDetailsBuilder {
    pub fn resolution(mut self, resolution: impl Into<HeavyAdResolutionStatus>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }
    pub fn reason(mut self, reason: impl Into<HeavyAdReason>) -> Self {
        self.reason = Some(reason.into());
        self
    }
    pub fn frame(mut self, frame: impl Into<AffectedFrame>) -> Self {
        self.frame = Some(frame.into());
        self
    }
    pub fn build(self) -> Result<HeavyAdIssueDetails, String> {
        Ok(HeavyAdIssueDetails {
            resolution: self
                .resolution
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(resolution)))?,
            reason: self
                .reason
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(reason)))?,
            frame: self
                .frame
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame)))?,
        })
    }
}
impl SourceCodeLocation {
    pub fn builder() -> SourceCodeLocationBuilder {
        SourceCodeLocationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SourceCodeLocationBuilder {
    script_id: Option<super::super::super::js_protocol::runtime::types::ScriptId>,
    url: Option<String>,
    line_number: Option<i64>,
    column_number: Option<i64>,
}
impl SourceCodeLocationBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::super::js_protocol::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn build(self) -> Result<SourceCodeLocation, String> {
        Ok(SourceCodeLocation {
            script_id: self.script_id,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
        })
    }
}
impl ContentSecurityPolicyIssueDetails {
    pub fn builder() -> ContentSecurityPolicyIssueDetailsBuilder {
        ContentSecurityPolicyIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContentSecurityPolicyIssueDetailsBuilder {
    blocked_url: Option<String>,
    violated_directive: Option<String>,
    is_report_only: Option<bool>,
    content_security_policy_violation_type: Option<ContentSecurityPolicyViolationType>,
    frame_ancestor: Option<AffectedFrame>,
    source_code_location: Option<SourceCodeLocation>,
    violating_node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl ContentSecurityPolicyIssueDetailsBuilder {
    pub fn blocked_url(mut self, blocked_url: impl Into<String>) -> Self {
        self.blocked_url = Some(blocked_url.into());
        self
    }
    pub fn violated_directive(mut self, violated_directive: impl Into<String>) -> Self {
        self.violated_directive = Some(violated_directive.into());
        self
    }
    pub fn is_report_only(mut self, is_report_only: impl Into<bool>) -> Self {
        self.is_report_only = Some(is_report_only.into());
        self
    }
    pub fn content_security_policy_violation_type(
        mut self,
        content_security_policy_violation_type: impl Into<ContentSecurityPolicyViolationType>,
    ) -> Self {
        self.content_security_policy_violation_type =
            Some(content_security_policy_violation_type.into());
        self
    }
    pub fn frame_ancestor(mut self, frame_ancestor: impl Into<AffectedFrame>) -> Self {
        self.frame_ancestor = Some(frame_ancestor.into());
        self
    }
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn violating_node_id(
        mut self,
        violating_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.violating_node_id = Some(violating_node_id.into());
        self
    }
    pub fn build(self) -> Result<ContentSecurityPolicyIssueDetails, String> {
        Ok(ContentSecurityPolicyIssueDetails {
            blocked_url: self.blocked_url,
            violated_directive: self.violated_directive.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(violated_directive)
                )
            })?,
            is_report_only: self.is_report_only.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(is_report_only))
            })?,
            content_security_policy_violation_type: self
                .content_security_policy_violation_type
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(content_security_policy_violation_type)
                    )
                })?,
            frame_ancestor: self.frame_ancestor,
            source_code_location: self.source_code_location,
            violating_node_id: self.violating_node_id,
        })
    }
}
impl SharedArrayBufferIssueDetails {
    pub fn builder() -> SharedArrayBufferIssueDetailsBuilder {
        SharedArrayBufferIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedArrayBufferIssueDetailsBuilder {
    source_code_location: Option<SourceCodeLocation>,
    is_warning: Option<bool>,
    r#type: Option<SharedArrayBufferIssueType>,
}
impl SharedArrayBufferIssueDetailsBuilder {
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn is_warning(mut self, is_warning: impl Into<bool>) -> Self {
        self.is_warning = Some(is_warning.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<SharedArrayBufferIssueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<SharedArrayBufferIssueDetails, String> {
        Ok(SharedArrayBufferIssueDetails {
            source_code_location: self.source_code_location.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(source_code_location)
                )
            })?,
            is_warning: self
                .is_warning
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_warning)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl LowTextContrastIssueDetails {
    pub fn builder() -> LowTextContrastIssueDetailsBuilder {
        LowTextContrastIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LowTextContrastIssueDetailsBuilder {
    violating_node_id: Option<super::super::dom::types::BackendNodeId>,
    violating_node_selector: Option<String>,
    contrast_ratio: Option<f64>,
    threshold_aa: Option<f64>,
    threshold_aaa: Option<f64>,
    font_size: Option<String>,
    font_weight: Option<String>,
}
impl LowTextContrastIssueDetailsBuilder {
    pub fn violating_node_id(
        mut self,
        violating_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.violating_node_id = Some(violating_node_id.into());
        self
    }
    pub fn violating_node_selector(mut self, violating_node_selector: impl Into<String>) -> Self {
        self.violating_node_selector = Some(violating_node_selector.into());
        self
    }
    pub fn contrast_ratio(mut self, contrast_ratio: impl Into<f64>) -> Self {
        self.contrast_ratio = Some(contrast_ratio.into());
        self
    }
    pub fn threshold_aa(mut self, threshold_aa: impl Into<f64>) -> Self {
        self.threshold_aa = Some(threshold_aa.into());
        self
    }
    pub fn threshold_aaa(mut self, threshold_aaa: impl Into<f64>) -> Self {
        self.threshold_aaa = Some(threshold_aaa.into());
        self
    }
    pub fn font_size(mut self, font_size: impl Into<String>) -> Self {
        self.font_size = Some(font_size.into());
        self
    }
    pub fn font_weight(mut self, font_weight: impl Into<String>) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }
    pub fn build(self) -> Result<LowTextContrastIssueDetails, String> {
        Ok(LowTextContrastIssueDetails {
            violating_node_id: self.violating_node_id.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(violating_node_id)
                )
            })?,
            violating_node_selector: self.violating_node_selector.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(violating_node_selector)
                )
            })?,
            contrast_ratio: self.contrast_ratio.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(contrast_ratio))
            })?,
            threshold_aa: self.threshold_aa.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(threshold_aa))
            })?,
            threshold_aaa: self.threshold_aaa.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(threshold_aaa))
            })?,
            font_size: self
                .font_size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(font_size)))?,
            font_weight: self
                .font_weight
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(font_weight)))?,
        })
    }
}
impl CorsIssueDetails {
    pub fn builder() -> CorsIssueDetailsBuilder {
        CorsIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CorsIssueDetailsBuilder {
    cors_error_status: Option<super::super::network::types::CorsErrorStatus>,
    is_warning: Option<bool>,
    request: Option<AffectedRequest>,
    location: Option<SourceCodeLocation>,
    initiator_origin: Option<String>,
    resource_ip_address_space: Option<super::super::network::types::IpAddressSpace>,
    client_security_state: Option<super::super::network::types::ClientSecurityState>,
}
impl CorsIssueDetailsBuilder {
    pub fn cors_error_status(
        mut self,
        cors_error_status: impl Into<super::super::network::types::CorsErrorStatus>,
    ) -> Self {
        self.cors_error_status = Some(cors_error_status.into());
        self
    }
    pub fn is_warning(mut self, is_warning: impl Into<bool>) -> Self {
        self.is_warning = Some(is_warning.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn location(mut self, location: impl Into<SourceCodeLocation>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn initiator_origin(mut self, initiator_origin: impl Into<String>) -> Self {
        self.initiator_origin = Some(initiator_origin.into());
        self
    }
    pub fn resource_ip_address_space(
        mut self,
        resource_ip_address_space: impl Into<super::super::network::types::IpAddressSpace>,
    ) -> Self {
        self.resource_ip_address_space = Some(resource_ip_address_space.into());
        self
    }
    pub fn client_security_state(
        mut self,
        client_security_state: impl Into<super::super::network::types::ClientSecurityState>,
    ) -> Self {
        self.client_security_state = Some(client_security_state.into());
        self
    }
    pub fn build(self) -> Result<CorsIssueDetails, String> {
        Ok(CorsIssueDetails {
            cors_error_status: self.cors_error_status.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(cors_error_status)
                )
            })?,
            is_warning: self
                .is_warning
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_warning)))?,
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
            location: self.location,
            initiator_origin: self.initiator_origin,
            resource_ip_address_space: self.resource_ip_address_space,
            client_security_state: self.client_security_state,
        })
    }
}
impl AttributionReportingIssueDetails {
    pub fn builder() -> AttributionReportingIssueDetailsBuilder {
        AttributionReportingIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AttributionReportingIssueDetailsBuilder {
    violation_type: Option<AttributionReportingIssueType>,
    request: Option<AffectedRequest>,
    violating_node_id: Option<super::super::dom::types::BackendNodeId>,
    invalid_parameter: Option<String>,
}
impl AttributionReportingIssueDetailsBuilder {
    pub fn violation_type(
        mut self,
        violation_type: impl Into<AttributionReportingIssueType>,
    ) -> Self {
        self.violation_type = Some(violation_type.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn violating_node_id(
        mut self,
        violating_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.violating_node_id = Some(violating_node_id.into());
        self
    }
    pub fn invalid_parameter(mut self, invalid_parameter: impl Into<String>) -> Self {
        self.invalid_parameter = Some(invalid_parameter.into());
        self
    }
    pub fn build(self) -> Result<AttributionReportingIssueDetails, String> {
        Ok(AttributionReportingIssueDetails {
            violation_type: self.violation_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(violation_type))
            })?,
            request: self.request,
            violating_node_id: self.violating_node_id,
            invalid_parameter: self.invalid_parameter,
        })
    }
}
impl QuirksModeIssueDetails {
    pub fn builder() -> QuirksModeIssueDetailsBuilder {
        QuirksModeIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct QuirksModeIssueDetailsBuilder {
    is_limited_quirks_mode: Option<bool>,
    document_node_id: Option<super::super::dom::types::BackendNodeId>,
    url: Option<String>,
    frame_id: Option<super::super::page::types::FrameId>,
    loader_id: Option<super::super::network::types::LoaderId>,
}
impl QuirksModeIssueDetailsBuilder {
    pub fn is_limited_quirks_mode(mut self, is_limited_quirks_mode: impl Into<bool>) -> Self {
        self.is_limited_quirks_mode = Some(is_limited_quirks_mode.into());
        self
    }
    pub fn document_node_id(
        mut self,
        document_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.document_node_id = Some(document_node_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn loader_id(
        mut self,
        loader_id: impl Into<super::super::network::types::LoaderId>,
    ) -> Self {
        self.loader_id = Some(loader_id.into());
        self
    }
    pub fn build(self) -> Result<QuirksModeIssueDetails, String> {
        Ok(QuirksModeIssueDetails {
            is_limited_quirks_mode: self.is_limited_quirks_mode.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(is_limited_quirks_mode)
                )
            })?,
            document_node_id: self.document_node_id.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(document_node_id)
                )
            })?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
            loader_id: self
                .loader_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(loader_id)))?,
        })
    }
}
impl SharedDictionaryIssueDetails {
    pub fn builder() -> SharedDictionaryIssueDetailsBuilder {
        SharedDictionaryIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SharedDictionaryIssueDetailsBuilder {
    shared_dictionary_error: Option<SharedDictionaryError>,
    request: Option<AffectedRequest>,
}
impl SharedDictionaryIssueDetailsBuilder {
    pub fn shared_dictionary_error(
        mut self,
        shared_dictionary_error: impl Into<SharedDictionaryError>,
    ) -> Self {
        self.shared_dictionary_error = Some(shared_dictionary_error.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<SharedDictionaryIssueDetails, String> {
        Ok(SharedDictionaryIssueDetails {
            shared_dictionary_error: self.shared_dictionary_error.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(shared_dictionary_error)
                )
            })?,
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
        })
    }
}
impl SriMessageSignatureIssueDetails {
    pub fn builder() -> SriMessageSignatureIssueDetailsBuilder {
        SriMessageSignatureIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SriMessageSignatureIssueDetailsBuilder {
    error: Option<SriMessageSignatureError>,
    signature_base: Option<String>,
    integrity_assertions: Option<Vec<String>>,
    request: Option<AffectedRequest>,
}
impl SriMessageSignatureIssueDetailsBuilder {
    pub fn error(mut self, error: impl Into<SriMessageSignatureError>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn signature_base(mut self, signature_base: impl Into<String>) -> Self {
        self.signature_base = Some(signature_base.into());
        self
    }
    pub fn integrity_assertion(mut self, integrity_assertion: impl Into<String>) -> Self {
        let v = self.integrity_assertions.get_or_insert(Vec::new());
        v.push(integrity_assertion.into());
        self
    }
    pub fn integrity_assertions<I, S>(mut self, integrity_assertions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.integrity_assertions.get_or_insert(Vec::new());
        for val in integrity_assertions {
            v.push(val.into());
        }
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<SriMessageSignatureIssueDetails, String> {
        Ok(SriMessageSignatureIssueDetails {
            error: self
                .error
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error)))?,
            signature_base: self.signature_base.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(signature_base))
            })?,
            integrity_assertions: self.integrity_assertions.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(integrity_assertions)
                )
            })?,
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
        })
    }
}
impl UnencodedDigestIssueDetails {
    pub fn builder() -> UnencodedDigestIssueDetailsBuilder {
        UnencodedDigestIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct UnencodedDigestIssueDetailsBuilder {
    error: Option<UnencodedDigestError>,
    request: Option<AffectedRequest>,
}
impl UnencodedDigestIssueDetailsBuilder {
    pub fn error(mut self, error: impl Into<UnencodedDigestError>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<UnencodedDigestIssueDetails, String> {
        Ok(UnencodedDigestIssueDetails {
            error: self
                .error
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error)))?,
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
        })
    }
}
impl ConnectionAllowlistIssueDetails {
    pub fn builder() -> ConnectionAllowlistIssueDetailsBuilder {
        ConnectionAllowlistIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ConnectionAllowlistIssueDetailsBuilder {
    error: Option<ConnectionAllowlistError>,
    request: Option<AffectedRequest>,
}
impl ConnectionAllowlistIssueDetailsBuilder {
    pub fn error(mut self, error: impl Into<ConnectionAllowlistError>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<ConnectionAllowlistIssueDetails, String> {
        Ok(ConnectionAllowlistIssueDetails {
            error: self
                .error
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error)))?,
            request: self
                .request
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(request)))?,
        })
    }
}
impl GenericIssueDetails {
    pub fn builder() -> GenericIssueDetailsBuilder {
        GenericIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GenericIssueDetailsBuilder {
    error_type: Option<GenericIssueErrorType>,
    frame_id: Option<super::super::page::types::FrameId>,
    violating_node_id: Option<super::super::dom::types::BackendNodeId>,
    violating_node_attribute: Option<String>,
    request: Option<AffectedRequest>,
}
impl GenericIssueDetailsBuilder {
    pub fn error_type(mut self, error_type: impl Into<GenericIssueErrorType>) -> Self {
        self.error_type = Some(error_type.into());
        self
    }
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn violating_node_id(
        mut self,
        violating_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.violating_node_id = Some(violating_node_id.into());
        self
    }
    pub fn violating_node_attribute(mut self, violating_node_attribute: impl Into<String>) -> Self {
        self.violating_node_attribute = Some(violating_node_attribute.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn build(self) -> Result<GenericIssueDetails, String> {
        Ok(GenericIssueDetails {
            error_type: self
                .error_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error_type)))?,
            frame_id: self.frame_id,
            violating_node_id: self.violating_node_id,
            violating_node_attribute: self.violating_node_attribute,
            request: self.request,
        })
    }
}
impl DeprecationIssueDetails {
    pub fn builder() -> DeprecationIssueDetailsBuilder {
        DeprecationIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DeprecationIssueDetailsBuilder {
    affected_frame: Option<AffectedFrame>,
    source_code_location: Option<SourceCodeLocation>,
    r#type: Option<String>,
}
impl DeprecationIssueDetailsBuilder {
    pub fn affected_frame(mut self, affected_frame: impl Into<AffectedFrame>) -> Self {
        self.affected_frame = Some(affected_frame.into());
        self
    }
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<DeprecationIssueDetails, String> {
        Ok(DeprecationIssueDetails {
            affected_frame: self.affected_frame,
            source_code_location: self.source_code_location.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(source_code_location)
                )
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl BounceTrackingIssueDetails {
    pub fn builder() -> BounceTrackingIssueDetailsBuilder {
        BounceTrackingIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BounceTrackingIssueDetailsBuilder {
    tracking_sites: Option<Vec<String>>,
}
impl BounceTrackingIssueDetailsBuilder {
    pub fn tracking_site(mut self, tracking_site: impl Into<String>) -> Self {
        let v = self.tracking_sites.get_or_insert(Vec::new());
        v.push(tracking_site.into());
        self
    }
    pub fn tracking_sites<I, S>(mut self, tracking_sites: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.tracking_sites.get_or_insert(Vec::new());
        for val in tracking_sites {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<BounceTrackingIssueDetails, String> {
        Ok(BounceTrackingIssueDetails {
            tracking_sites: self.tracking_sites.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(tracking_sites))
            })?,
        })
    }
}
impl CookieDeprecationMetadataIssueDetails {
    pub fn builder() -> CookieDeprecationMetadataIssueDetailsBuilder {
        CookieDeprecationMetadataIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CookieDeprecationMetadataIssueDetailsBuilder {
    allowed_sites: Option<Vec<String>>,
    opt_out_percentage: Option<f64>,
    is_opt_out_top_level: Option<bool>,
    operation: Option<CookieOperation>,
}
impl CookieDeprecationMetadataIssueDetailsBuilder {
    pub fn allowed_site(mut self, allowed_site: impl Into<String>) -> Self {
        let v = self.allowed_sites.get_or_insert(Vec::new());
        v.push(allowed_site.into());
        self
    }
    pub fn allowed_sites<I, S>(mut self, allowed_sites: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.allowed_sites.get_or_insert(Vec::new());
        for val in allowed_sites {
            v.push(val.into());
        }
        self
    }
    pub fn opt_out_percentage(mut self, opt_out_percentage: impl Into<f64>) -> Self {
        self.opt_out_percentage = Some(opt_out_percentage.into());
        self
    }
    pub fn is_opt_out_top_level(mut self, is_opt_out_top_level: impl Into<bool>) -> Self {
        self.is_opt_out_top_level = Some(is_opt_out_top_level.into());
        self
    }
    pub fn operation(mut self, operation: impl Into<CookieOperation>) -> Self {
        self.operation = Some(operation.into());
        self
    }
    pub fn build(self) -> Result<CookieDeprecationMetadataIssueDetails, String> {
        Ok(CookieDeprecationMetadataIssueDetails {
            allowed_sites: self.allowed_sites.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(allowed_sites))
            })?,
            opt_out_percentage: self.opt_out_percentage.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(opt_out_percentage)
                )
            })?,
            is_opt_out_top_level: self.is_opt_out_top_level.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(is_opt_out_top_level)
                )
            })?,
            operation: self
                .operation
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(operation)))?,
        })
    }
}
impl FederatedAuthRequestIssueDetails {
    pub fn builder() -> FederatedAuthRequestIssueDetailsBuilder {
        FederatedAuthRequestIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FederatedAuthRequestIssueDetailsBuilder {
    federated_auth_request_issue_reason: Option<FederatedAuthRequestIssueReason>,
}
impl FederatedAuthRequestIssueDetailsBuilder {
    pub fn federated_auth_request_issue_reason(
        mut self,
        federated_auth_request_issue_reason: impl Into<FederatedAuthRequestIssueReason>,
    ) -> Self {
        self.federated_auth_request_issue_reason = Some(federated_auth_request_issue_reason.into());
        self
    }
    pub fn build(self) -> Result<FederatedAuthRequestIssueDetails, String> {
        Ok(FederatedAuthRequestIssueDetails {
            federated_auth_request_issue_reason: self
                .federated_auth_request_issue_reason
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(federated_auth_request_issue_reason)
                    )
                })?,
        })
    }
}
impl FederatedAuthUserInfoRequestIssueDetails {
    pub fn builder() -> FederatedAuthUserInfoRequestIssueDetailsBuilder {
        FederatedAuthUserInfoRequestIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FederatedAuthUserInfoRequestIssueDetailsBuilder {
    federated_auth_user_info_request_issue_reason: Option<FederatedAuthUserInfoRequestIssueReason>,
}
impl FederatedAuthUserInfoRequestIssueDetailsBuilder {
    pub fn federated_auth_user_info_request_issue_reason(
        mut self,
        federated_auth_user_info_request_issue_reason: impl Into<
            FederatedAuthUserInfoRequestIssueReason,
        >,
    ) -> Self {
        self.federated_auth_user_info_request_issue_reason =
            Some(federated_auth_user_info_request_issue_reason.into());
        self
    }
    pub fn build(self) -> Result<FederatedAuthUserInfoRequestIssueDetails, String> {
        Ok(FederatedAuthUserInfoRequestIssueDetails {
            federated_auth_user_info_request_issue_reason: self
                .federated_auth_user_info_request_issue_reason
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(federated_auth_user_info_request_issue_reason)
                    )
                })?,
        })
    }
}
impl ClientHintIssueDetails {
    pub fn builder() -> ClientHintIssueDetailsBuilder {
        ClientHintIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ClientHintIssueDetailsBuilder {
    source_code_location: Option<SourceCodeLocation>,
    client_hint_issue_reason: Option<ClientHintIssueReason>,
}
impl ClientHintIssueDetailsBuilder {
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn client_hint_issue_reason(
        mut self,
        client_hint_issue_reason: impl Into<ClientHintIssueReason>,
    ) -> Self {
        self.client_hint_issue_reason = Some(client_hint_issue_reason.into());
        self
    }
    pub fn build(self) -> Result<ClientHintIssueDetails, String> {
        Ok(ClientHintIssueDetails {
            source_code_location: self.source_code_location.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(source_code_location)
                )
            })?,
            client_hint_issue_reason: self.client_hint_issue_reason.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(client_hint_issue_reason)
                )
            })?,
        })
    }
}
impl FailedRequestInfo {
    pub fn builder() -> FailedRequestInfoBuilder {
        FailedRequestInfoBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FailedRequestInfoBuilder {
    url: Option<String>,
    failure_message: Option<String>,
    request_id: Option<super::super::network::types::RequestId>,
}
impl FailedRequestInfoBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn failure_message(mut self, failure_message: impl Into<String>) -> Self {
        self.failure_message = Some(failure_message.into());
        self
    }
    pub fn request_id(
        mut self,
        request_id: impl Into<super::super::network::types::RequestId>,
    ) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    pub fn build(self) -> Result<FailedRequestInfo, String> {
        Ok(FailedRequestInfo {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            failure_message: self.failure_message.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(failure_message))
            })?,
            request_id: self.request_id,
        })
    }
}
impl PartitioningBlobUrlIssueDetails {
    pub fn builder() -> PartitioningBlobUrlIssueDetailsBuilder {
        PartitioningBlobUrlIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PartitioningBlobUrlIssueDetailsBuilder {
    url: Option<String>,
    partitioning_blob_url_info: Option<PartitioningBlobUrlInfo>,
}
impl PartitioningBlobUrlIssueDetailsBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn partitioning_blob_url_info(
        mut self,
        partitioning_blob_url_info: impl Into<PartitioningBlobUrlInfo>,
    ) -> Self {
        self.partitioning_blob_url_info = Some(partitioning_blob_url_info.into());
        self
    }
    pub fn build(self) -> Result<PartitioningBlobUrlIssueDetails, String> {
        Ok(PartitioningBlobUrlIssueDetails {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            partitioning_blob_url_info: self.partitioning_blob_url_info.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(partitioning_blob_url_info)
                )
            })?,
        })
    }
}
impl ElementAccessibilityIssueDetails {
    pub fn builder() -> ElementAccessibilityIssueDetailsBuilder {
        ElementAccessibilityIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ElementAccessibilityIssueDetailsBuilder {
    node_id: Option<super::super::dom::types::BackendNodeId>,
    element_accessibility_issue_reason: Option<ElementAccessibilityIssueReason>,
    has_disallowed_attributes: Option<bool>,
}
impl ElementAccessibilityIssueDetailsBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn element_accessibility_issue_reason(
        mut self,
        element_accessibility_issue_reason: impl Into<ElementAccessibilityIssueReason>,
    ) -> Self {
        self.element_accessibility_issue_reason = Some(element_accessibility_issue_reason.into());
        self
    }
    pub fn has_disallowed_attributes(mut self, has_disallowed_attributes: impl Into<bool>) -> Self {
        self.has_disallowed_attributes = Some(has_disallowed_attributes.into());
        self
    }
    pub fn build(self) -> Result<ElementAccessibilityIssueDetails, String> {
        Ok(ElementAccessibilityIssueDetails {
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            element_accessibility_issue_reason: self
                .element_accessibility_issue_reason
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(element_accessibility_issue_reason)
                    )
                })?,
            has_disallowed_attributes: self.has_disallowed_attributes.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(has_disallowed_attributes)
                )
            })?,
        })
    }
}
impl StylesheetLoadingIssueDetails {
    pub fn builder() -> StylesheetLoadingIssueDetailsBuilder {
        StylesheetLoadingIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StylesheetLoadingIssueDetailsBuilder {
    source_code_location: Option<SourceCodeLocation>,
    style_sheet_loading_issue_reason: Option<StyleSheetLoadingIssueReason>,
    failed_request_info: Option<FailedRequestInfo>,
}
impl StylesheetLoadingIssueDetailsBuilder {
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn style_sheet_loading_issue_reason(
        mut self,
        style_sheet_loading_issue_reason: impl Into<StyleSheetLoadingIssueReason>,
    ) -> Self {
        self.style_sheet_loading_issue_reason = Some(style_sheet_loading_issue_reason.into());
        self
    }
    pub fn failed_request_info(
        mut self,
        failed_request_info: impl Into<FailedRequestInfo>,
    ) -> Self {
        self.failed_request_info = Some(failed_request_info.into());
        self
    }
    pub fn build(self) -> Result<StylesheetLoadingIssueDetails, String> {
        Ok(StylesheetLoadingIssueDetails {
            source_code_location: self.source_code_location.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(source_code_location)
                )
            })?,
            style_sheet_loading_issue_reason: self.style_sheet_loading_issue_reason.ok_or_else(
                || {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(style_sheet_loading_issue_reason)
                    )
                },
            )?,
            failed_request_info: self.failed_request_info,
        })
    }
}
impl PropertyRuleIssueDetails {
    pub fn builder() -> PropertyRuleIssueDetailsBuilder {
        PropertyRuleIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PropertyRuleIssueDetailsBuilder {
    source_code_location: Option<SourceCodeLocation>,
    property_rule_issue_reason: Option<PropertyRuleIssueReason>,
    property_value: Option<String>,
}
impl PropertyRuleIssueDetailsBuilder {
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn property_rule_issue_reason(
        mut self,
        property_rule_issue_reason: impl Into<PropertyRuleIssueReason>,
    ) -> Self {
        self.property_rule_issue_reason = Some(property_rule_issue_reason.into());
        self
    }
    pub fn property_value(mut self, property_value: impl Into<String>) -> Self {
        self.property_value = Some(property_value.into());
        self
    }
    pub fn build(self) -> Result<PropertyRuleIssueDetails, String> {
        Ok(PropertyRuleIssueDetails {
            source_code_location: self.source_code_location.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(source_code_location)
                )
            })?,
            property_rule_issue_reason: self.property_rule_issue_reason.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(property_rule_issue_reason)
                )
            })?,
            property_value: self.property_value,
        })
    }
}
impl UserReidentificationIssueDetails {
    pub fn builder() -> UserReidentificationIssueDetailsBuilder {
        UserReidentificationIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct UserReidentificationIssueDetailsBuilder {
    r#type: Option<UserReidentificationIssueType>,
    request: Option<AffectedRequest>,
    source_code_location: Option<SourceCodeLocation>,
}
impl UserReidentificationIssueDetailsBuilder {
    pub fn r#type(mut self, r#type: impl Into<UserReidentificationIssueType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn request(mut self, request: impl Into<AffectedRequest>) -> Self {
        self.request = Some(request.into());
        self
    }
    pub fn source_code_location(
        mut self,
        source_code_location: impl Into<SourceCodeLocation>,
    ) -> Self {
        self.source_code_location = Some(source_code_location.into());
        self
    }
    pub fn build(self) -> Result<UserReidentificationIssueDetails, String> {
        Ok(UserReidentificationIssueDetails {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            request: self.request,
            source_code_location: self.source_code_location,
        })
    }
}
impl PermissionElementIssueDetails {
    pub fn builder() -> PermissionElementIssueDetailsBuilder {
        PermissionElementIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PermissionElementIssueDetailsBuilder {
    issue_type: Option<PermissionElementIssueType>,
    r#type: Option<String>,
    node_id: Option<super::super::dom::types::BackendNodeId>,
    is_warning: Option<bool>,
    permission_name: Option<String>,
    occluder_node_info: Option<String>,
    occluder_parent_node_info: Option<String>,
    disable_reason: Option<String>,
}
impl PermissionElementIssueDetailsBuilder {
    pub fn issue_type(mut self, issue_type: impl Into<PermissionElementIssueType>) -> Self {
        self.issue_type = Some(issue_type.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn is_warning(mut self, is_warning: impl Into<bool>) -> Self {
        self.is_warning = Some(is_warning.into());
        self
    }
    pub fn permission_name(mut self, permission_name: impl Into<String>) -> Self {
        self.permission_name = Some(permission_name.into());
        self
    }
    pub fn occluder_node_info(mut self, occluder_node_info: impl Into<String>) -> Self {
        self.occluder_node_info = Some(occluder_node_info.into());
        self
    }
    pub fn occluder_parent_node_info(
        mut self,
        occluder_parent_node_info: impl Into<String>,
    ) -> Self {
        self.occluder_parent_node_info = Some(occluder_parent_node_info.into());
        self
    }
    pub fn disable_reason(mut self, disable_reason: impl Into<String>) -> Self {
        self.disable_reason = Some(disable_reason.into());
        self
    }
    pub fn build(self) -> Result<PermissionElementIssueDetails, String> {
        Ok(PermissionElementIssueDetails {
            issue_type: self
                .issue_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(issue_type)))?,
            r#type: self.r#type,
            node_id: self.node_id,
            is_warning: self.is_warning,
            permission_name: self.permission_name,
            occluder_node_info: self.occluder_node_info,
            occluder_parent_node_info: self.occluder_parent_node_info,
            disable_reason: self.disable_reason,
        })
    }
}
impl AdScriptIdentifier {
    pub fn builder() -> AdScriptIdentifierBuilder {
        AdScriptIdentifierBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AdScriptIdentifierBuilder {
    script_id: Option<super::super::super::js_protocol::runtime::types::ScriptId>,
    debugger_id: Option<super::super::super::js_protocol::runtime::types::UniqueDebuggerId>,
    name: Option<String>,
}
impl AdScriptIdentifierBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::super::js_protocol::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn debugger_id(
        mut self,
        debugger_id: impl Into<super::super::super::js_protocol::runtime::types::UniqueDebuggerId>,
    ) -> Self {
        self.debugger_id = Some(debugger_id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<AdScriptIdentifier, String> {
        Ok(AdScriptIdentifier {
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            debugger_id: self
                .debugger_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(debugger_id)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
        })
    }
}
impl AdAncestry {
    pub fn builder() -> AdAncestryBuilder {
        AdAncestryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AdAncestryBuilder {
    ad_ancestry_chain: Option<Vec<AdScriptIdentifier>>,
    root_script_filterlist_rule: Option<String>,
}
impl AdAncestryBuilder {
    pub fn ad_ancestry_chain(mut self, ad_ancestry_chain: impl Into<AdScriptIdentifier>) -> Self {
        let v = self.ad_ancestry_chain.get_or_insert(Vec::new());
        v.push(ad_ancestry_chain.into());
        self
    }
    pub fn ad_ancestry_chains<I, S>(mut self, ad_ancestry_chains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AdScriptIdentifier>,
    {
        let v = self.ad_ancestry_chain.get_or_insert(Vec::new());
        for val in ad_ancestry_chains {
            v.push(val.into());
        }
        self
    }
    pub fn root_script_filterlist_rule(
        mut self,
        root_script_filterlist_rule: impl Into<String>,
    ) -> Self {
        self.root_script_filterlist_rule = Some(root_script_filterlist_rule.into());
        self
    }
    pub fn build(self) -> Result<AdAncestry, String> {
        Ok(AdAncestry {
            ad_ancestry_chain: self.ad_ancestry_chain.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(ad_ancestry_chain)
                )
            })?,
            root_script_filterlist_rule: self.root_script_filterlist_rule,
        })
    }
}
impl SelectivePermissionsInterventionIssueDetails {
    pub fn builder() -> SelectivePermissionsInterventionIssueDetailsBuilder {
        SelectivePermissionsInterventionIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SelectivePermissionsInterventionIssueDetailsBuilder {
    api_name: Option<String>,
    ad_ancestry: Option<AdAncestry>,
    stack_trace: Option<super::super::super::js_protocol::runtime::types::StackTrace>,
}
impl SelectivePermissionsInterventionIssueDetailsBuilder {
    pub fn api_name(mut self, api_name: impl Into<String>) -> Self {
        self.api_name = Some(api_name.into());
        self
    }
    pub fn ad_ancestry(mut self, ad_ancestry: impl Into<AdAncestry>) -> Self {
        self.ad_ancestry = Some(ad_ancestry.into());
        self
    }
    pub fn stack_trace(
        mut self,
        stack_trace: impl Into<super::super::super::js_protocol::runtime::types::StackTrace>,
    ) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    pub fn build(self) -> Result<SelectivePermissionsInterventionIssueDetails, String> {
        Ok(SelectivePermissionsInterventionIssueDetails {
            api_name: self
                .api_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(api_name)))?,
            ad_ancestry: self
                .ad_ancestry
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ad_ancestry)))?,
            stack_trace: self.stack_trace,
        })
    }
}
impl InspectorIssueDetails {
    pub fn builder() -> InspectorIssueDetailsBuilder {
        InspectorIssueDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InspectorIssueDetailsBuilder {
    cookie_issue_details: Option<CookieIssueDetails>,
    mixed_content_issue_details: Option<MixedContentIssueDetails>,
    blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
    heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
    content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
    shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails>,
    low_text_contrast_issue_details: Option<LowTextContrastIssueDetails>,
    cors_issue_details: Option<CorsIssueDetails>,
    attribution_reporting_issue_details: Option<AttributionReportingIssueDetails>,
    quirks_mode_issue_details: Option<QuirksModeIssueDetails>,
    partitioning_blob_url_issue_details: Option<PartitioningBlobUrlIssueDetails>,
    generic_issue_details: Option<GenericIssueDetails>,
    deprecation_issue_details: Option<DeprecationIssueDetails>,
    client_hint_issue_details: Option<ClientHintIssueDetails>,
    federated_auth_request_issue_details: Option<FederatedAuthRequestIssueDetails>,
    bounce_tracking_issue_details: Option<BounceTrackingIssueDetails>,
    cookie_deprecation_metadata_issue_details: Option<CookieDeprecationMetadataIssueDetails>,
    stylesheet_loading_issue_details: Option<StylesheetLoadingIssueDetails>,
    property_rule_issue_details: Option<PropertyRuleIssueDetails>,
    federated_auth_user_info_request_issue_details:
        Option<FederatedAuthUserInfoRequestIssueDetails>,
    shared_dictionary_issue_details: Option<SharedDictionaryIssueDetails>,
    element_accessibility_issue_details: Option<ElementAccessibilityIssueDetails>,
    sri_message_signature_issue_details: Option<SriMessageSignatureIssueDetails>,
    unencoded_digest_issue_details: Option<UnencodedDigestIssueDetails>,
    connection_allowlist_issue_details: Option<ConnectionAllowlistIssueDetails>,
    user_reidentification_issue_details: Option<UserReidentificationIssueDetails>,
    permission_element_issue_details: Option<PermissionElementIssueDetails>,
    performance_issue_details: Option<PerformanceIssueDetails>,
    selective_permissions_intervention_issue_details:
        Option<SelectivePermissionsInterventionIssueDetails>,
}
impl InspectorIssueDetailsBuilder {
    pub fn cookie_issue_details(
        mut self,
        cookie_issue_details: impl Into<CookieIssueDetails>,
    ) -> Self {
        self.cookie_issue_details = Some(cookie_issue_details.into());
        self
    }
    pub fn mixed_content_issue_details(
        mut self,
        mixed_content_issue_details: impl Into<MixedContentIssueDetails>,
    ) -> Self {
        self.mixed_content_issue_details = Some(mixed_content_issue_details.into());
        self
    }
    pub fn blocked_by_response_issue_details(
        mut self,
        blocked_by_response_issue_details: impl Into<BlockedByResponseIssueDetails>,
    ) -> Self {
        self.blocked_by_response_issue_details = Some(blocked_by_response_issue_details.into());
        self
    }
    pub fn heavy_ad_issue_details(
        mut self,
        heavy_ad_issue_details: impl Into<HeavyAdIssueDetails>,
    ) -> Self {
        self.heavy_ad_issue_details = Some(heavy_ad_issue_details.into());
        self
    }
    pub fn content_security_policy_issue_details(
        mut self,
        content_security_policy_issue_details: impl Into<ContentSecurityPolicyIssueDetails>,
    ) -> Self {
        self.content_security_policy_issue_details =
            Some(content_security_policy_issue_details.into());
        self
    }
    pub fn shared_array_buffer_issue_details(
        mut self,
        shared_array_buffer_issue_details: impl Into<SharedArrayBufferIssueDetails>,
    ) -> Self {
        self.shared_array_buffer_issue_details = Some(shared_array_buffer_issue_details.into());
        self
    }
    pub fn low_text_contrast_issue_details(
        mut self,
        low_text_contrast_issue_details: impl Into<LowTextContrastIssueDetails>,
    ) -> Self {
        self.low_text_contrast_issue_details = Some(low_text_contrast_issue_details.into());
        self
    }
    pub fn cors_issue_details(mut self, cors_issue_details: impl Into<CorsIssueDetails>) -> Self {
        self.cors_issue_details = Some(cors_issue_details.into());
        self
    }
    pub fn attribution_reporting_issue_details(
        mut self,
        attribution_reporting_issue_details: impl Into<AttributionReportingIssueDetails>,
    ) -> Self {
        self.attribution_reporting_issue_details = Some(attribution_reporting_issue_details.into());
        self
    }
    pub fn quirks_mode_issue_details(
        mut self,
        quirks_mode_issue_details: impl Into<QuirksModeIssueDetails>,
    ) -> Self {
        self.quirks_mode_issue_details = Some(quirks_mode_issue_details.into());
        self
    }
    pub fn partitioning_blob_url_issue_details(
        mut self,
        partitioning_blob_url_issue_details: impl Into<PartitioningBlobUrlIssueDetails>,
    ) -> Self {
        self.partitioning_blob_url_issue_details = Some(partitioning_blob_url_issue_details.into());
        self
    }
    pub fn generic_issue_details(
        mut self,
        generic_issue_details: impl Into<GenericIssueDetails>,
    ) -> Self {
        self.generic_issue_details = Some(generic_issue_details.into());
        self
    }
    pub fn deprecation_issue_details(
        mut self,
        deprecation_issue_details: impl Into<DeprecationIssueDetails>,
    ) -> Self {
        self.deprecation_issue_details = Some(deprecation_issue_details.into());
        self
    }
    pub fn client_hint_issue_details(
        mut self,
        client_hint_issue_details: impl Into<ClientHintIssueDetails>,
    ) -> Self {
        self.client_hint_issue_details = Some(client_hint_issue_details.into());
        self
    }
    pub fn federated_auth_request_issue_details(
        mut self,
        federated_auth_request_issue_details: impl Into<FederatedAuthRequestIssueDetails>,
    ) -> Self {
        self.federated_auth_request_issue_details =
            Some(federated_auth_request_issue_details.into());
        self
    }
    pub fn bounce_tracking_issue_details(
        mut self,
        bounce_tracking_issue_details: impl Into<BounceTrackingIssueDetails>,
    ) -> Self {
        self.bounce_tracking_issue_details = Some(bounce_tracking_issue_details.into());
        self
    }
    pub fn cookie_deprecation_metadata_issue_details(
        mut self,
        cookie_deprecation_metadata_issue_details: impl Into<CookieDeprecationMetadataIssueDetails>,
    ) -> Self {
        self.cookie_deprecation_metadata_issue_details =
            Some(cookie_deprecation_metadata_issue_details.into());
        self
    }
    pub fn stylesheet_loading_issue_details(
        mut self,
        stylesheet_loading_issue_details: impl Into<StylesheetLoadingIssueDetails>,
    ) -> Self {
        self.stylesheet_loading_issue_details = Some(stylesheet_loading_issue_details.into());
        self
    }
    pub fn property_rule_issue_details(
        mut self,
        property_rule_issue_details: impl Into<PropertyRuleIssueDetails>,
    ) -> Self {
        self.property_rule_issue_details = Some(property_rule_issue_details.into());
        self
    }
    pub fn federated_auth_user_info_request_issue_details(
        mut self,
        federated_auth_user_info_request_issue_details: impl Into<
            FederatedAuthUserInfoRequestIssueDetails,
        >,
    ) -> Self {
        self.federated_auth_user_info_request_issue_details =
            Some(federated_auth_user_info_request_issue_details.into());
        self
    }
    pub fn shared_dictionary_issue_details(
        mut self,
        shared_dictionary_issue_details: impl Into<SharedDictionaryIssueDetails>,
    ) -> Self {
        self.shared_dictionary_issue_details = Some(shared_dictionary_issue_details.into());
        self
    }
    pub fn element_accessibility_issue_details(
        mut self,
        element_accessibility_issue_details: impl Into<ElementAccessibilityIssueDetails>,
    ) -> Self {
        self.element_accessibility_issue_details = Some(element_accessibility_issue_details.into());
        self
    }
    pub fn sri_message_signature_issue_details(
        mut self,
        sri_message_signature_issue_details: impl Into<SriMessageSignatureIssueDetails>,
    ) -> Self {
        self.sri_message_signature_issue_details = Some(sri_message_signature_issue_details.into());
        self
    }
    pub fn unencoded_digest_issue_details(
        mut self,
        unencoded_digest_issue_details: impl Into<UnencodedDigestIssueDetails>,
    ) -> Self {
        self.unencoded_digest_issue_details = Some(unencoded_digest_issue_details.into());
        self
    }
    pub fn connection_allowlist_issue_details(
        mut self,
        connection_allowlist_issue_details: impl Into<ConnectionAllowlistIssueDetails>,
    ) -> Self {
        self.connection_allowlist_issue_details = Some(connection_allowlist_issue_details.into());
        self
    }
    pub fn user_reidentification_issue_details(
        mut self,
        user_reidentification_issue_details: impl Into<UserReidentificationIssueDetails>,
    ) -> Self {
        self.user_reidentification_issue_details = Some(user_reidentification_issue_details.into());
        self
    }
    pub fn permission_element_issue_details(
        mut self,
        permission_element_issue_details: impl Into<PermissionElementIssueDetails>,
    ) -> Self {
        self.permission_element_issue_details = Some(permission_element_issue_details.into());
        self
    }
    pub fn performance_issue_details(
        mut self,
        performance_issue_details: impl Into<PerformanceIssueDetails>,
    ) -> Self {
        self.performance_issue_details = Some(performance_issue_details.into());
        self
    }
    pub fn selective_permissions_intervention_issue_details(
        mut self,
        selective_permissions_intervention_issue_details: impl Into<
            SelectivePermissionsInterventionIssueDetails,
        >,
    ) -> Self {
        self.selective_permissions_intervention_issue_details =
            Some(selective_permissions_intervention_issue_details.into());
        self
    }
    pub fn build(self) -> InspectorIssueDetails {
        InspectorIssueDetails {
            cookie_issue_details: self.cookie_issue_details,
            mixed_content_issue_details: self.mixed_content_issue_details,
            blocked_by_response_issue_details: self.blocked_by_response_issue_details,
            heavy_ad_issue_details: self.heavy_ad_issue_details,
            content_security_policy_issue_details: self.content_security_policy_issue_details,
            shared_array_buffer_issue_details: self.shared_array_buffer_issue_details,
            low_text_contrast_issue_details: self.low_text_contrast_issue_details,
            cors_issue_details: self.cors_issue_details,
            attribution_reporting_issue_details: self.attribution_reporting_issue_details,
            quirks_mode_issue_details: self.quirks_mode_issue_details,
            partitioning_blob_url_issue_details: self.partitioning_blob_url_issue_details,
            generic_issue_details: self.generic_issue_details,
            deprecation_issue_details: self.deprecation_issue_details,
            client_hint_issue_details: self.client_hint_issue_details,
            federated_auth_request_issue_details: self.federated_auth_request_issue_details,
            bounce_tracking_issue_details: self.bounce_tracking_issue_details,
            cookie_deprecation_metadata_issue_details: self
                .cookie_deprecation_metadata_issue_details,
            stylesheet_loading_issue_details: self.stylesheet_loading_issue_details,
            property_rule_issue_details: self.property_rule_issue_details,
            federated_auth_user_info_request_issue_details: self
                .federated_auth_user_info_request_issue_details,
            shared_dictionary_issue_details: self.shared_dictionary_issue_details,
            element_accessibility_issue_details: self.element_accessibility_issue_details,
            sri_message_signature_issue_details: self.sri_message_signature_issue_details,
            unencoded_digest_issue_details: self.unencoded_digest_issue_details,
            connection_allowlist_issue_details: self.connection_allowlist_issue_details,
            user_reidentification_issue_details: self.user_reidentification_issue_details,
            permission_element_issue_details: self.permission_element_issue_details,
            performance_issue_details: self.performance_issue_details,
            selective_permissions_intervention_issue_details: self
                .selective_permissions_intervention_issue_details,
        }
    }
}
impl InspectorIssue {
    pub fn builder() -> InspectorIssueBuilder {
        InspectorIssueBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InspectorIssueBuilder {
    code: Option<InspectorIssueCode>,
    details: Option<InspectorIssueDetails>,
    issue_id: Option<IssueId>,
}
impl InspectorIssueBuilder {
    pub fn code(mut self, code: impl Into<InspectorIssueCode>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn details(mut self, details: impl Into<InspectorIssueDetails>) -> Self {
        self.details = Some(details.into());
        self
    }
    pub fn issue_id(mut self, issue_id: impl Into<IssueId>) -> Self {
        self.issue_id = Some(issue_id.into());
        self
    }
    pub fn build(self) -> Result<InspectorIssue, String> {
        Ok(InspectorIssue {
            code: self
                .code
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(code)))?,
            details: self
                .details
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(details)))?,
            issue_id: self.issue_id,
        })
    }
}
