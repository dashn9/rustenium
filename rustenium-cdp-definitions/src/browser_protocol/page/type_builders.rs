use super::types::*;
impl AdFrameStatus {
    pub fn builder() -> AdFrameStatusBuilder {
        AdFrameStatusBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AdFrameStatusBuilder {
    ad_frame_type: Option<AdFrameType>,
    explanations: Option<Vec<AdFrameExplanation>>,
}
impl AdFrameStatusBuilder {
    pub fn ad_frame_type(mut self, ad_frame_type: impl Into<AdFrameType>) -> Self {
        self.ad_frame_type = Some(ad_frame_type.into());
        self
    }
    pub fn explanation(mut self, explanation: impl Into<AdFrameExplanation>) -> Self {
        let v = self.explanations.get_or_insert(Vec::new());
        v.push(explanation.into());
        self
    }
    pub fn explanations<I, S>(mut self, explanations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AdFrameExplanation>,
    {
        let v = self.explanations.get_or_insert(Vec::new());
        for val in explanations {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<AdFrameStatus, String> {
        Ok(AdFrameStatus {
            ad_frame_type: self.ad_frame_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(ad_frame_type))
            })?,
            explanations: self.explanations,
        })
    }
}
impl AdScriptId {
    pub fn builder() -> AdScriptIdBuilder {
        AdScriptIdBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AdScriptIdBuilder {
    script_id: Option<super::super::super::js_protocol::runtime::types::ScriptId>,
    debugger_id: Option<super::super::super::js_protocol::runtime::types::UniqueDebuggerId>,
}
impl AdScriptIdBuilder {
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
    pub fn build(self) -> Result<AdScriptId, String> {
        Ok(AdScriptId {
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            debugger_id: self
                .debugger_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(debugger_id)))?,
        })
    }
}
impl AdScriptAncestry {
    pub fn builder() -> AdScriptAncestryBuilder {
        AdScriptAncestryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AdScriptAncestryBuilder {
    ancestry_chain: Option<Vec<AdScriptId>>,
    root_script_filterlist_rule: Option<String>,
}
impl AdScriptAncestryBuilder {
    pub fn ancestry_chain(mut self, ancestry_chain: impl Into<AdScriptId>) -> Self {
        let v = self.ancestry_chain.get_or_insert(Vec::new());
        v.push(ancestry_chain.into());
        self
    }
    pub fn ancestry_chains<I, S>(mut self, ancestry_chains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<AdScriptId>,
    {
        let v = self.ancestry_chain.get_or_insert(Vec::new());
        for val in ancestry_chains {
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
    pub fn build(self) -> Result<AdScriptAncestry, String> {
        Ok(AdScriptAncestry {
            ancestry_chain: self.ancestry_chain.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(ancestry_chain))
            })?,
            root_script_filterlist_rule: self.root_script_filterlist_rule,
        })
    }
}
impl PermissionsPolicyBlockLocator {
    pub fn builder() -> PermissionsPolicyBlockLocatorBuilder {
        PermissionsPolicyBlockLocatorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PermissionsPolicyBlockLocatorBuilder {
    frame_id: Option<FrameId>,
    block_reason: Option<PermissionsPolicyBlockReason>,
}
impl PermissionsPolicyBlockLocatorBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn block_reason(mut self, block_reason: impl Into<PermissionsPolicyBlockReason>) -> Self {
        self.block_reason = Some(block_reason.into());
        self
    }
    pub fn build(self) -> Result<PermissionsPolicyBlockLocator, String> {
        Ok(PermissionsPolicyBlockLocator {
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
            block_reason: self.block_reason.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(block_reason))
            })?,
        })
    }
}
impl PermissionsPolicyFeatureState {
    pub fn builder() -> PermissionsPolicyFeatureStateBuilder {
        PermissionsPolicyFeatureStateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PermissionsPolicyFeatureStateBuilder {
    feature: Option<PermissionsPolicyFeature>,
    allowed: Option<bool>,
    locator: Option<PermissionsPolicyBlockLocator>,
}
impl PermissionsPolicyFeatureStateBuilder {
    pub fn feature(mut self, feature: impl Into<PermissionsPolicyFeature>) -> Self {
        self.feature = Some(feature.into());
        self
    }
    pub fn allowed(mut self, allowed: impl Into<bool>) -> Self {
        self.allowed = Some(allowed.into());
        self
    }
    pub fn locator(mut self, locator: impl Into<PermissionsPolicyBlockLocator>) -> Self {
        self.locator = Some(locator.into());
        self
    }
    pub fn build(self) -> Result<PermissionsPolicyFeatureState, String> {
        Ok(PermissionsPolicyFeatureState {
            feature: self
                .feature
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(feature)))?,
            allowed: self
                .allowed
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(allowed)))?,
            locator: self.locator,
        })
    }
}
impl OriginTrialToken {
    pub fn builder() -> OriginTrialTokenBuilder {
        OriginTrialTokenBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct OriginTrialTokenBuilder {
    origin: Option<String>,
    match_sub_domains: Option<bool>,
    trial_name: Option<String>,
    expiry_time: Option<super::super::network::types::TimeSinceEpoch>,
    is_third_party: Option<bool>,
    usage_restriction: Option<OriginTrialUsageRestriction>,
}
impl OriginTrialTokenBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn match_sub_domains(mut self, match_sub_domains: impl Into<bool>) -> Self {
        self.match_sub_domains = Some(match_sub_domains.into());
        self
    }
    pub fn trial_name(mut self, trial_name: impl Into<String>) -> Self {
        self.trial_name = Some(trial_name.into());
        self
    }
    pub fn expiry_time(
        mut self,
        expiry_time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.expiry_time = Some(expiry_time.into());
        self
    }
    pub fn is_third_party(mut self, is_third_party: impl Into<bool>) -> Self {
        self.is_third_party = Some(is_third_party.into());
        self
    }
    pub fn usage_restriction(
        mut self,
        usage_restriction: impl Into<OriginTrialUsageRestriction>,
    ) -> Self {
        self.usage_restriction = Some(usage_restriction.into());
        self
    }
    pub fn build(self) -> Result<OriginTrialToken, String> {
        Ok(OriginTrialToken {
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            match_sub_domains: self.match_sub_domains.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(match_sub_domains)
                )
            })?,
            trial_name: self
                .trial_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(trial_name)))?,
            expiry_time: self
                .expiry_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expiry_time)))?,
            is_third_party: self.is_third_party.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(is_third_party))
            })?,
            usage_restriction: self.usage_restriction.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(usage_restriction)
                )
            })?,
        })
    }
}
impl OriginTrialTokenWithStatus {
    pub fn builder() -> OriginTrialTokenWithStatusBuilder {
        OriginTrialTokenWithStatusBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct OriginTrialTokenWithStatusBuilder {
    raw_token_text: Option<String>,
    parsed_token: Option<OriginTrialToken>,
    status: Option<OriginTrialTokenStatus>,
}
impl OriginTrialTokenWithStatusBuilder {
    pub fn raw_token_text(mut self, raw_token_text: impl Into<String>) -> Self {
        self.raw_token_text = Some(raw_token_text.into());
        self
    }
    pub fn parsed_token(mut self, parsed_token: impl Into<OriginTrialToken>) -> Self {
        self.parsed_token = Some(parsed_token.into());
        self
    }
    pub fn status(mut self, status: impl Into<OriginTrialTokenStatus>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn build(self) -> Result<OriginTrialTokenWithStatus, String> {
        Ok(OriginTrialTokenWithStatus {
            raw_token_text: self.raw_token_text.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(raw_token_text))
            })?,
            parsed_token: self.parsed_token,
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
        })
    }
}
impl OriginTrial {
    pub fn builder() -> OriginTrialBuilder {
        OriginTrialBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct OriginTrialBuilder {
    trial_name: Option<String>,
    status: Option<OriginTrialStatus>,
    tokens_with_status: Option<Vec<OriginTrialTokenWithStatus>>,
}
impl OriginTrialBuilder {
    pub fn trial_name(mut self, trial_name: impl Into<String>) -> Self {
        self.trial_name = Some(trial_name.into());
        self
    }
    pub fn status(mut self, status: impl Into<OriginTrialStatus>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn tokens_with_statu(
        mut self,
        tokens_with_statu: impl Into<OriginTrialTokenWithStatus>,
    ) -> Self {
        let v = self.tokens_with_status.get_or_insert(Vec::new());
        v.push(tokens_with_statu.into());
        self
    }
    pub fn tokens_with_status<I, S>(mut self, tokens_with_status: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OriginTrialTokenWithStatus>,
    {
        let v = self.tokens_with_status.get_or_insert(Vec::new());
        for val in tokens_with_status {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<OriginTrial, String> {
        Ok(OriginTrial {
            trial_name: self
                .trial_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(trial_name)))?,
            status: self
                .status
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(status)))?,
            tokens_with_status: self.tokens_with_status.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(tokens_with_status)
                )
            })?,
        })
    }
}
impl SecurityOriginDetails {
    pub fn builder() -> SecurityOriginDetailsBuilder {
        SecurityOriginDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SecurityOriginDetailsBuilder {
    is_localhost: Option<bool>,
}
impl SecurityOriginDetailsBuilder {
    pub fn is_localhost(mut self, is_localhost: impl Into<bool>) -> Self {
        self.is_localhost = Some(is_localhost.into());
        self
    }
    pub fn build(self) -> Result<SecurityOriginDetails, String> {
        Ok(SecurityOriginDetails {
            is_localhost: self.is_localhost.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(is_localhost))
            })?,
        })
    }
}
impl Frame {
    pub fn builder() -> FrameBuilder {
        FrameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FrameBuilder {
    id: Option<FrameId>,
    parent_id: Option<FrameId>,
    loader_id: Option<super::super::network::types::LoaderId>,
    name: Option<String>,
    url: Option<String>,
    url_fragment: Option<String>,
    domain_and_registry: Option<String>,
    security_origin: Option<String>,
    security_origin_details: Option<SecurityOriginDetails>,
    mime_type: Option<String>,
    unreachable_url: Option<String>,
    ad_frame_status: Option<AdFrameStatus>,
    secure_context_type: Option<SecureContextType>,
    cross_origin_isolated_context_type: Option<CrossOriginIsolatedContextType>,
    gated_api_features: Option<Vec<GatedApiFeatures>>,
}
impl FrameBuilder {
    pub fn id(mut self, id: impl Into<FrameId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn parent_id(mut self, parent_id: impl Into<FrameId>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    pub fn loader_id(
        mut self,
        loader_id: impl Into<super::super::network::types::LoaderId>,
    ) -> Self {
        self.loader_id = Some(loader_id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn url_fragment(mut self, url_fragment: impl Into<String>) -> Self {
        self.url_fragment = Some(url_fragment.into());
        self
    }
    pub fn domain_and_registry(mut self, domain_and_registry: impl Into<String>) -> Self {
        self.domain_and_registry = Some(domain_and_registry.into());
        self
    }
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn security_origin_details(
        mut self,
        security_origin_details: impl Into<SecurityOriginDetails>,
    ) -> Self {
        self.security_origin_details = Some(security_origin_details.into());
        self
    }
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
    pub fn unreachable_url(mut self, unreachable_url: impl Into<String>) -> Self {
        self.unreachable_url = Some(unreachable_url.into());
        self
    }
    pub fn ad_frame_status(mut self, ad_frame_status: impl Into<AdFrameStatus>) -> Self {
        self.ad_frame_status = Some(ad_frame_status.into());
        self
    }
    pub fn secure_context_type(
        mut self,
        secure_context_type: impl Into<SecureContextType>,
    ) -> Self {
        self.secure_context_type = Some(secure_context_type.into());
        self
    }
    pub fn cross_origin_isolated_context_type(
        mut self,
        cross_origin_isolated_context_type: impl Into<CrossOriginIsolatedContextType>,
    ) -> Self {
        self.cross_origin_isolated_context_type = Some(cross_origin_isolated_context_type.into());
        self
    }
    pub fn gated_api_feature(mut self, gated_api_feature: impl Into<GatedApiFeatures>) -> Self {
        let v = self.gated_api_features.get_or_insert(Vec::new());
        v.push(gated_api_feature.into());
        self
    }
    pub fn gated_api_features<I, S>(mut self, gated_api_features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<GatedApiFeatures>,
    {
        let v = self.gated_api_features.get_or_insert(Vec::new());
        for val in gated_api_features {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Frame, String> {
        Ok(Frame {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            parent_id: self.parent_id,
            loader_id: self
                .loader_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(loader_id)))?,
            name: self.name,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            url_fragment: self.url_fragment,
            domain_and_registry: self.domain_and_registry.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(domain_and_registry)
                )
            })?,
            security_origin: self.security_origin.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(security_origin))
            })?,
            security_origin_details: self.security_origin_details,
            mime_type: self
                .mime_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mime_type)))?,
            unreachable_url: self.unreachable_url,
            ad_frame_status: self.ad_frame_status,
            secure_context_type: self.secure_context_type.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(secure_context_type)
                )
            })?,
            cross_origin_isolated_context_type: self
                .cross_origin_isolated_context_type
                .ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(cross_origin_isolated_context_type)
                    )
                })?,
            gated_api_features: self.gated_api_features.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(gated_api_features)
                )
            })?,
        })
    }
}
impl FrameResource {
    pub fn builder() -> FrameResourceBuilder {
        FrameResourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FrameResourceBuilder {
    url: Option<String>,
    r#type: Option<super::super::network::types::ResourceType>,
    mime_type: Option<String>,
    last_modified: Option<super::super::network::types::TimeSinceEpoch>,
    content_size: Option<f64>,
    failed: Option<bool>,
    canceled: Option<bool>,
}
impl FrameResourceBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::super::network::types::ResourceType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
    pub fn last_modified(
        mut self,
        last_modified: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.last_modified = Some(last_modified.into());
        self
    }
    pub fn content_size(mut self, content_size: impl Into<f64>) -> Self {
        self.content_size = Some(content_size.into());
        self
    }
    pub fn failed(mut self, failed: impl Into<bool>) -> Self {
        self.failed = Some(failed.into());
        self
    }
    pub fn canceled(mut self, canceled: impl Into<bool>) -> Self {
        self.canceled = Some(canceled.into());
        self
    }
    pub fn build(self) -> Result<FrameResource, String> {
        Ok(FrameResource {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mime_type)))?,
            last_modified: self.last_modified,
            content_size: self.content_size,
            failed: self.failed,
            canceled: self.canceled,
        })
    }
}
impl FrameResourceTree {
    pub fn builder() -> FrameResourceTreeBuilder {
        FrameResourceTreeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FrameResourceTreeBuilder {
    frame: Option<Frame>,
    child_frames: Option<Vec<FrameResourceTree>>,
    resources: Option<Vec<FrameResource>>,
}
impl FrameResourceTreeBuilder {
    pub fn frame(mut self, frame: impl Into<Frame>) -> Self {
        self.frame = Some(frame.into());
        self
    }
    pub fn child_frame(mut self, child_frame: impl Into<FrameResourceTree>) -> Self {
        let v = self.child_frames.get_or_insert(Vec::new());
        v.push(child_frame.into());
        self
    }
    pub fn child_frames<I, S>(mut self, child_frames: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FrameResourceTree>,
    {
        let v = self.child_frames.get_or_insert(Vec::new());
        for val in child_frames {
            v.push(val.into());
        }
        self
    }
    pub fn resource(mut self, resource: impl Into<FrameResource>) -> Self {
        let v = self.resources.get_or_insert(Vec::new());
        v.push(resource.into());
        self
    }
    pub fn resources<I, S>(mut self, resources: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FrameResource>,
    {
        let v = self.resources.get_or_insert(Vec::new());
        for val in resources {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<FrameResourceTree, String> {
        Ok(FrameResourceTree {
            frame: self
                .frame
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame)))?,
            child_frames: self.child_frames,
            resources: self
                .resources
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(resources)))?,
        })
    }
}
impl FrameTree {
    pub fn builder() -> FrameTreeBuilder {
        FrameTreeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FrameTreeBuilder {
    frame: Option<Frame>,
    child_frames: Option<Vec<FrameTree>>,
}
impl FrameTreeBuilder {
    pub fn frame(mut self, frame: impl Into<Frame>) -> Self {
        self.frame = Some(frame.into());
        self
    }
    pub fn child_frame(mut self, child_frame: impl Into<FrameTree>) -> Self {
        let v = self.child_frames.get_or_insert(Vec::new());
        v.push(child_frame.into());
        self
    }
    pub fn child_frames<I, S>(mut self, child_frames: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FrameTree>,
    {
        let v = self.child_frames.get_or_insert(Vec::new());
        for val in child_frames {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<FrameTree, String> {
        Ok(FrameTree {
            frame: self
                .frame
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame)))?,
            child_frames: self.child_frames,
        })
    }
}
impl NavigationEntry {
    pub fn builder() -> NavigationEntryBuilder {
        NavigationEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct NavigationEntryBuilder {
    id: Option<i64>,
    url: Option<String>,
    user_typed_url: Option<String>,
    title: Option<String>,
    transition_type: Option<TransitionType>,
}
impl NavigationEntryBuilder {
    pub fn id(mut self, id: impl Into<i64>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn user_typed_url(mut self, user_typed_url: impl Into<String>) -> Self {
        self.user_typed_url = Some(user_typed_url.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn transition_type(mut self, transition_type: impl Into<TransitionType>) -> Self {
        self.transition_type = Some(transition_type.into());
        self
    }
    pub fn build(self) -> Result<NavigationEntry, String> {
        Ok(NavigationEntry {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            user_typed_url: self.user_typed_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(user_typed_url))
            })?,
            title: self
                .title
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(title)))?,
            transition_type: self.transition_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(transition_type))
            })?,
        })
    }
}
impl ScreencastFrameMetadata {
    pub fn builder() -> ScreencastFrameMetadataBuilder {
        ScreencastFrameMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreencastFrameMetadataBuilder {
    offset_top: Option<f64>,
    page_scale_factor: Option<f64>,
    device_width: Option<f64>,
    device_height: Option<f64>,
    scroll_offset_x: Option<f64>,
    scroll_offset_y: Option<f64>,
    timestamp: Option<super::super::network::types::TimeSinceEpoch>,
}
impl ScreencastFrameMetadataBuilder {
    pub fn offset_top(mut self, offset_top: impl Into<f64>) -> Self {
        self.offset_top = Some(offset_top.into());
        self
    }
    pub fn page_scale_factor(mut self, page_scale_factor: impl Into<f64>) -> Self {
        self.page_scale_factor = Some(page_scale_factor.into());
        self
    }
    pub fn device_width(mut self, device_width: impl Into<f64>) -> Self {
        self.device_width = Some(device_width.into());
        self
    }
    pub fn device_height(mut self, device_height: impl Into<f64>) -> Self {
        self.device_height = Some(device_height.into());
        self
    }
    pub fn scroll_offset_x(mut self, scroll_offset_x: impl Into<f64>) -> Self {
        self.scroll_offset_x = Some(scroll_offset_x.into());
        self
    }
    pub fn scroll_offset_y(mut self, scroll_offset_y: impl Into<f64>) -> Self {
        self.scroll_offset_y = Some(scroll_offset_y.into());
        self
    }
    pub fn timestamp(
        mut self,
        timestamp: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn build(self) -> Result<ScreencastFrameMetadata, String> {
        Ok(ScreencastFrameMetadata {
            offset_top: self
                .offset_top
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset_top)))?,
            page_scale_factor: self.page_scale_factor.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(page_scale_factor)
                )
            })?,
            device_width: self.device_width.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(device_width))
            })?,
            device_height: self.device_height.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(device_height))
            })?,
            scroll_offset_x: self.scroll_offset_x.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(scroll_offset_x))
            })?,
            scroll_offset_y: self.scroll_offset_y.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(scroll_offset_y))
            })?,
            timestamp: self.timestamp,
        })
    }
}
impl AppManifestError {
    pub fn builder() -> AppManifestErrorBuilder {
        AppManifestErrorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AppManifestErrorBuilder {
    message: Option<String>,
    critical: Option<i64>,
    line: Option<i64>,
    column: Option<i64>,
}
impl AppManifestErrorBuilder {
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn critical(mut self, critical: impl Into<i64>) -> Self {
        self.critical = Some(critical.into());
        self
    }
    pub fn line(mut self, line: impl Into<i64>) -> Self {
        self.line = Some(line.into());
        self
    }
    pub fn column(mut self, column: impl Into<i64>) -> Self {
        self.column = Some(column.into());
        self
    }
    pub fn build(self) -> Result<AppManifestError, String> {
        Ok(AppManifestError {
            message: self
                .message
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(message)))?,
            critical: self
                .critical
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(critical)))?,
            line: self
                .line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line)))?,
            column: self
                .column
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(column)))?,
        })
    }
}
impl AppManifestParsedProperties {
    pub fn builder() -> AppManifestParsedPropertiesBuilder {
        AppManifestParsedPropertiesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AppManifestParsedPropertiesBuilder {
    scope: Option<String>,
}
impl AppManifestParsedPropertiesBuilder {
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }
    pub fn build(self) -> Result<AppManifestParsedProperties, String> {
        Ok(AppManifestParsedProperties {
            scope: self
                .scope
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scope)))?,
        })
    }
}
impl LayoutViewport {
    pub fn builder() -> LayoutViewportBuilder {
        LayoutViewportBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LayoutViewportBuilder {
    page_x: Option<i64>,
    page_y: Option<i64>,
    client_width: Option<i64>,
    client_height: Option<i64>,
}
impl LayoutViewportBuilder {
    pub fn page_x(mut self, page_x: impl Into<i64>) -> Self {
        self.page_x = Some(page_x.into());
        self
    }
    pub fn page_y(mut self, page_y: impl Into<i64>) -> Self {
        self.page_y = Some(page_y.into());
        self
    }
    pub fn client_width(mut self, client_width: impl Into<i64>) -> Self {
        self.client_width = Some(client_width.into());
        self
    }
    pub fn client_height(mut self, client_height: impl Into<i64>) -> Self {
        self.client_height = Some(client_height.into());
        self
    }
    pub fn build(self) -> Result<LayoutViewport, String> {
        Ok(LayoutViewport {
            page_x: self
                .page_x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(page_x)))?,
            page_y: self
                .page_y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(page_y)))?,
            client_width: self.client_width.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(client_width))
            })?,
            client_height: self.client_height.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(client_height))
            })?,
        })
    }
}
impl VisualViewport {
    pub fn builder() -> VisualViewportBuilder {
        VisualViewportBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct VisualViewportBuilder {
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    page_x: Option<f64>,
    page_y: Option<f64>,
    client_width: Option<f64>,
    client_height: Option<f64>,
    scale: Option<f64>,
    zoom: Option<f64>,
}
impl VisualViewportBuilder {
    pub fn offset_x(mut self, offset_x: impl Into<f64>) -> Self {
        self.offset_x = Some(offset_x.into());
        self
    }
    pub fn offset_y(mut self, offset_y: impl Into<f64>) -> Self {
        self.offset_y = Some(offset_y.into());
        self
    }
    pub fn page_x(mut self, page_x: impl Into<f64>) -> Self {
        self.page_x = Some(page_x.into());
        self
    }
    pub fn page_y(mut self, page_y: impl Into<f64>) -> Self {
        self.page_y = Some(page_y.into());
        self
    }
    pub fn client_width(mut self, client_width: impl Into<f64>) -> Self {
        self.client_width = Some(client_width.into());
        self
    }
    pub fn client_height(mut self, client_height: impl Into<f64>) -> Self {
        self.client_height = Some(client_height.into());
        self
    }
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn zoom(mut self, zoom: impl Into<f64>) -> Self {
        self.zoom = Some(zoom.into());
        self
    }
    pub fn build(self) -> Result<VisualViewport, String> {
        Ok(VisualViewport {
            offset_x: self
                .offset_x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset_x)))?,
            offset_y: self
                .offset_y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset_y)))?,
            page_x: self
                .page_x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(page_x)))?,
            page_y: self
                .page_y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(page_y)))?,
            client_width: self.client_width.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(client_width))
            })?,
            client_height: self.client_height.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(client_height))
            })?,
            scale: self
                .scale
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scale)))?,
            zoom: self.zoom,
        })
    }
}
impl Viewport {
    pub fn builder() -> ViewportBuilder {
        ViewportBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ViewportBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    scale: Option<f64>,
}
impl ViewportBuilder {
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
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn build(self) -> Result<Viewport, String> {
        Ok(Viewport {
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
            scale: self
                .scale
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scale)))?,
        })
    }
}
impl FontFamilies {
    pub fn builder() -> FontFamiliesBuilder {
        FontFamiliesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FontFamiliesBuilder {
    standard: Option<String>,
    fixed: Option<String>,
    serif: Option<String>,
    sans_serif: Option<String>,
    cursive: Option<String>,
    fantasy: Option<String>,
    math: Option<String>,
}
impl FontFamiliesBuilder {
    pub fn standard(mut self, standard: impl Into<String>) -> Self {
        self.standard = Some(standard.into());
        self
    }
    pub fn fixed(mut self, fixed: impl Into<String>) -> Self {
        self.fixed = Some(fixed.into());
        self
    }
    pub fn serif(mut self, serif: impl Into<String>) -> Self {
        self.serif = Some(serif.into());
        self
    }
    pub fn sans_serif(mut self, sans_serif: impl Into<String>) -> Self {
        self.sans_serif = Some(sans_serif.into());
        self
    }
    pub fn cursive(mut self, cursive: impl Into<String>) -> Self {
        self.cursive = Some(cursive.into());
        self
    }
    pub fn fantasy(mut self, fantasy: impl Into<String>) -> Self {
        self.fantasy = Some(fantasy.into());
        self
    }
    pub fn math(mut self, math: impl Into<String>) -> Self {
        self.math = Some(math.into());
        self
    }
    pub fn build(self) -> FontFamilies {
        FontFamilies {
            standard: self.standard,
            fixed: self.fixed,
            serif: self.serif,
            sans_serif: self.sans_serif,
            cursive: self.cursive,
            fantasy: self.fantasy,
            math: self.math,
        }
    }
}
impl ScriptFontFamilies {
    pub fn builder() -> ScriptFontFamiliesBuilder {
        ScriptFontFamiliesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScriptFontFamiliesBuilder {
    script: Option<String>,
    font_families: Option<FontFamilies>,
}
impl ScriptFontFamiliesBuilder {
    pub fn script(mut self, script: impl Into<String>) -> Self {
        self.script = Some(script.into());
        self
    }
    pub fn font_families(mut self, font_families: impl Into<FontFamilies>) -> Self {
        self.font_families = Some(font_families.into());
        self
    }
    pub fn build(self) -> Result<ScriptFontFamilies, String> {
        Ok(ScriptFontFamilies {
            script: self
                .script
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script)))?,
            font_families: self.font_families.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(font_families))
            })?,
        })
    }
}
impl FontSizes {
    pub fn builder() -> FontSizesBuilder {
        FontSizesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FontSizesBuilder {
    standard: Option<i64>,
    fixed: Option<i64>,
}
impl FontSizesBuilder {
    pub fn standard(mut self, standard: impl Into<i64>) -> Self {
        self.standard = Some(standard.into());
        self
    }
    pub fn fixed(mut self, fixed: impl Into<i64>) -> Self {
        self.fixed = Some(fixed.into());
        self
    }
    pub fn build(self) -> FontSizes {
        FontSizes {
            standard: self.standard,
            fixed: self.fixed,
        }
    }
}
impl InstallabilityErrorArgument {
    pub fn builder() -> InstallabilityErrorArgumentBuilder {
        InstallabilityErrorArgumentBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InstallabilityErrorArgumentBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl InstallabilityErrorArgumentBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<InstallabilityErrorArgument, String> {
        Ok(InstallabilityErrorArgument {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl InstallabilityError {
    pub fn builder() -> InstallabilityErrorBuilder {
        InstallabilityErrorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InstallabilityErrorBuilder {
    error_id: Option<String>,
    error_arguments: Option<Vec<InstallabilityErrorArgument>>,
}
impl InstallabilityErrorBuilder {
    pub fn error_id(mut self, error_id: impl Into<String>) -> Self {
        self.error_id = Some(error_id.into());
        self
    }
    pub fn error_argument(
        mut self,
        error_argument: impl Into<InstallabilityErrorArgument>,
    ) -> Self {
        let v = self.error_arguments.get_or_insert(Vec::new());
        v.push(error_argument.into());
        self
    }
    pub fn error_arguments<I, S>(mut self, error_arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<InstallabilityErrorArgument>,
    {
        let v = self.error_arguments.get_or_insert(Vec::new());
        for val in error_arguments {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<InstallabilityError, String> {
        Ok(InstallabilityError {
            error_id: self
                .error_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error_id)))?,
            error_arguments: self.error_arguments.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(error_arguments))
            })?,
        })
    }
}
impl CompilationCacheParams {
    pub fn builder() -> CompilationCacheParamsBuilder {
        CompilationCacheParamsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CompilationCacheParamsBuilder {
    url: Option<String>,
    eager: Option<bool>,
}
impl CompilationCacheParamsBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn eager(mut self, eager: impl Into<bool>) -> Self {
        self.eager = Some(eager.into());
        self
    }
    pub fn build(self) -> Result<CompilationCacheParams, String> {
        Ok(CompilationCacheParams {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            eager: self.eager,
        })
    }
}
impl FileFilter {
    pub fn builder() -> FileFilterBuilder {
        FileFilterBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FileFilterBuilder {
    name: Option<String>,
    accepts: Option<Vec<String>>,
}
impl FileFilterBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        let v = self.accepts.get_or_insert(Vec::new());
        v.push(accept.into());
        self
    }
    pub fn accepts<I, S>(mut self, accepts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.accepts.get_or_insert(Vec::new());
        for val in accepts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> FileFilter {
        FileFilter {
            name: self.name,
            accepts: self.accepts,
        }
    }
}
impl FileHandler {
    pub fn builder() -> FileHandlerBuilder {
        FileHandlerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct FileHandlerBuilder {
    action: Option<String>,
    name: Option<String>,
    icons: Option<Vec<ImageResource>>,
    accepts: Option<Vec<FileFilter>>,
    launch_type: Option<String>,
}
impl FileHandlerBuilder {
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn icon(mut self, icon: impl Into<ImageResource>) -> Self {
        let v = self.icons.get_or_insert(Vec::new());
        v.push(icon.into());
        self
    }
    pub fn icons<I, S>(mut self, icons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ImageResource>,
    {
        let v = self.icons.get_or_insert(Vec::new());
        for val in icons {
            v.push(val.into());
        }
        self
    }
    pub fn accept(mut self, accept: impl Into<FileFilter>) -> Self {
        let v = self.accepts.get_or_insert(Vec::new());
        v.push(accept.into());
        self
    }
    pub fn accepts<I, S>(mut self, accepts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FileFilter>,
    {
        let v = self.accepts.get_or_insert(Vec::new());
        for val in accepts {
            v.push(val.into());
        }
        self
    }
    pub fn launch_type(mut self, launch_type: impl Into<String>) -> Self {
        self.launch_type = Some(launch_type.into());
        self
    }
    pub fn build(self) -> Result<FileHandler, String> {
        Ok(FileHandler {
            action: self
                .action
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(action)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            icons: self.icons,
            accepts: self.accepts,
            launch_type: self
                .launch_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(launch_type)))?,
        })
    }
}
impl ImageResource {
    pub fn builder() -> ImageResourceBuilder {
        ImageResourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ImageResourceBuilder {
    url: Option<String>,
    sizes: Option<String>,
    r#type: Option<String>,
}
impl ImageResourceBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn sizes(mut self, sizes: impl Into<String>) -> Self {
        self.sizes = Some(sizes.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<ImageResource, String> {
        Ok(ImageResource {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            sizes: self.sizes,
            r#type: self.r#type,
        })
    }
}
impl LaunchHandler {
    pub fn builder() -> LaunchHandlerBuilder {
        LaunchHandlerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LaunchHandlerBuilder {
    client_mode: Option<String>,
}
impl LaunchHandlerBuilder {
    pub fn client_mode(mut self, client_mode: impl Into<String>) -> Self {
        self.client_mode = Some(client_mode.into());
        self
    }
    pub fn build(self) -> Result<LaunchHandler, String> {
        Ok(LaunchHandler {
            client_mode: self
                .client_mode
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(client_mode)))?,
        })
    }
}
impl ProtocolHandler {
    pub fn builder() -> ProtocolHandlerBuilder {
        ProtocolHandlerBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ProtocolHandlerBuilder {
    protocol: Option<String>,
    url: Option<String>,
}
impl ProtocolHandlerBuilder {
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<ProtocolHandler, String> {
        Ok(ProtocolHandler {
            protocol: self
                .protocol
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(protocol)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
        })
    }
}
impl RelatedApplication {
    pub fn builder() -> RelatedApplicationBuilder {
        RelatedApplicationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RelatedApplicationBuilder {
    id: Option<String>,
    url: Option<String>,
}
impl RelatedApplicationBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<RelatedApplication, String> {
        Ok(RelatedApplication {
            id: self.id,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
        })
    }
}
impl ScopeExtension {
    pub fn builder() -> ScopeExtensionBuilder {
        ScopeExtensionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScopeExtensionBuilder {
    origin: Option<String>,
    has_origin_wildcard: Option<bool>,
}
impl ScopeExtensionBuilder {
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn has_origin_wildcard(mut self, has_origin_wildcard: impl Into<bool>) -> Self {
        self.has_origin_wildcard = Some(has_origin_wildcard.into());
        self
    }
    pub fn build(self) -> Result<ScopeExtension, String> {
        Ok(ScopeExtension {
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            has_origin_wildcard: self.has_origin_wildcard.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(has_origin_wildcard)
                )
            })?,
        })
    }
}
impl Screenshot {
    pub fn builder() -> ScreenshotBuilder {
        ScreenshotBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreenshotBuilder {
    image: Option<ImageResource>,
    form_factor: Option<String>,
    label: Option<String>,
}
impl ScreenshotBuilder {
    pub fn image(mut self, image: impl Into<ImageResource>) -> Self {
        self.image = Some(image.into());
        self
    }
    pub fn form_factor(mut self, form_factor: impl Into<String>) -> Self {
        self.form_factor = Some(form_factor.into());
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn build(self) -> Result<Screenshot, String> {
        Ok(Screenshot {
            image: self
                .image
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(image)))?,
            form_factor: self
                .form_factor
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(form_factor)))?,
            label: self.label,
        })
    }
}
impl ShareTarget {
    pub fn builder() -> ShareTargetBuilder {
        ShareTargetBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ShareTargetBuilder {
    action: Option<String>,
    method: Option<String>,
    enctype: Option<String>,
    title: Option<String>,
    text: Option<String>,
    url: Option<String>,
    files: Option<Vec<FileFilter>>,
}
impl ShareTargetBuilder {
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn enctype(mut self, enctype: impl Into<String>) -> Self {
        self.enctype = Some(enctype.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn file(mut self, file: impl Into<FileFilter>) -> Self {
        let v = self.files.get_or_insert(Vec::new());
        v.push(file.into());
        self
    }
    pub fn files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FileFilter>,
    {
        let v = self.files.get_or_insert(Vec::new());
        for val in files {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ShareTarget, String> {
        Ok(ShareTarget {
            action: self
                .action
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(action)))?,
            method: self
                .method
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(method)))?,
            enctype: self
                .enctype
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enctype)))?,
            title: self.title,
            text: self.text,
            url: self.url,
            files: self.files,
        })
    }
}
impl Shortcut {
    pub fn builder() -> ShortcutBuilder {
        ShortcutBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ShortcutBuilder {
    name: Option<String>,
    url: Option<String>,
}
impl ShortcutBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<Shortcut, String> {
        Ok(Shortcut {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
        })
    }
}
impl WebAppManifest {
    pub fn builder() -> WebAppManifestBuilder {
        WebAppManifestBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct WebAppManifestBuilder {
    background_color: Option<String>,
    description: Option<String>,
    dir: Option<String>,
    display: Option<String>,
    display_overrides: Option<Vec<String>>,
    file_handlers: Option<Vec<FileHandler>>,
    icons: Option<Vec<ImageResource>>,
    id: Option<String>,
    lang: Option<String>,
    launch_handler: Option<LaunchHandler>,
    name: Option<String>,
    orientation: Option<String>,
    prefer_related_applications: Option<bool>,
    protocol_handlers: Option<Vec<ProtocolHandler>>,
    related_applications: Option<Vec<RelatedApplication>>,
    scope: Option<String>,
    scope_extensions: Option<Vec<ScopeExtension>>,
    screenshots: Option<Vec<Screenshot>>,
    share_target: Option<ShareTarget>,
    short_name: Option<String>,
    shortcuts: Option<Vec<Shortcut>>,
    start_url: Option<String>,
    theme_color: Option<String>,
}
impl WebAppManifestBuilder {
    pub fn background_color(mut self, background_color: impl Into<String>) -> Self {
        self.background_color = Some(background_color.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn dir(mut self, dir: impl Into<String>) -> Self {
        self.dir = Some(dir.into());
        self
    }
    pub fn display(mut self, display: impl Into<String>) -> Self {
        self.display = Some(display.into());
        self
    }
    pub fn display_override(mut self, display_override: impl Into<String>) -> Self {
        let v = self.display_overrides.get_or_insert(Vec::new());
        v.push(display_override.into());
        self
    }
    pub fn display_overrides<I, S>(mut self, display_overrides: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.display_overrides.get_or_insert(Vec::new());
        for val in display_overrides {
            v.push(val.into());
        }
        self
    }
    pub fn file_handler(mut self, file_handler: impl Into<FileHandler>) -> Self {
        let v = self.file_handlers.get_or_insert(Vec::new());
        v.push(file_handler.into());
        self
    }
    pub fn file_handlers<I, S>(mut self, file_handlers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FileHandler>,
    {
        let v = self.file_handlers.get_or_insert(Vec::new());
        for val in file_handlers {
            v.push(val.into());
        }
        self
    }
    pub fn icon(mut self, icon: impl Into<ImageResource>) -> Self {
        let v = self.icons.get_or_insert(Vec::new());
        v.push(icon.into());
        self
    }
    pub fn icons<I, S>(mut self, icons: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ImageResource>,
    {
        let v = self.icons.get_or_insert(Vec::new());
        for val in icons {
            v.push(val.into());
        }
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }
    pub fn launch_handler(mut self, launch_handler: impl Into<LaunchHandler>) -> Self {
        self.launch_handler = Some(launch_handler.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn orientation(mut self, orientation: impl Into<String>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
    pub fn prefer_related_applications(
        mut self,
        prefer_related_applications: impl Into<bool>,
    ) -> Self {
        self.prefer_related_applications = Some(prefer_related_applications.into());
        self
    }
    pub fn protocol_handler(mut self, protocol_handler: impl Into<ProtocolHandler>) -> Self {
        let v = self.protocol_handlers.get_or_insert(Vec::new());
        v.push(protocol_handler.into());
        self
    }
    pub fn protocol_handlers<I, S>(mut self, protocol_handlers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ProtocolHandler>,
    {
        let v = self.protocol_handlers.get_or_insert(Vec::new());
        for val in protocol_handlers {
            v.push(val.into());
        }
        self
    }
    pub fn related_application(
        mut self,
        related_application: impl Into<RelatedApplication>,
    ) -> Self {
        let v = self.related_applications.get_or_insert(Vec::new());
        v.push(related_application.into());
        self
    }
    pub fn related_applications<I, S>(mut self, related_applications: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<RelatedApplication>,
    {
        let v = self.related_applications.get_or_insert(Vec::new());
        for val in related_applications {
            v.push(val.into());
        }
        self
    }
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }
    pub fn scope_extension(mut self, scope_extension: impl Into<ScopeExtension>) -> Self {
        let v = self.scope_extensions.get_or_insert(Vec::new());
        v.push(scope_extension.into());
        self
    }
    pub fn scope_extensions<I, S>(mut self, scope_extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ScopeExtension>,
    {
        let v = self.scope_extensions.get_or_insert(Vec::new());
        for val in scope_extensions {
            v.push(val.into());
        }
        self
    }
    pub fn screenshot(mut self, screenshot: impl Into<Screenshot>) -> Self {
        let v = self.screenshots.get_or_insert(Vec::new());
        v.push(screenshot.into());
        self
    }
    pub fn screenshots<I, S>(mut self, screenshots: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Screenshot>,
    {
        let v = self.screenshots.get_or_insert(Vec::new());
        for val in screenshots {
            v.push(val.into());
        }
        self
    }
    pub fn share_target(mut self, share_target: impl Into<ShareTarget>) -> Self {
        self.share_target = Some(share_target.into());
        self
    }
    pub fn short_name(mut self, short_name: impl Into<String>) -> Self {
        self.short_name = Some(short_name.into());
        self
    }
    pub fn shortcut(mut self, shortcut: impl Into<Shortcut>) -> Self {
        let v = self.shortcuts.get_or_insert(Vec::new());
        v.push(shortcut.into());
        self
    }
    pub fn shortcuts<I, S>(mut self, shortcuts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Shortcut>,
    {
        let v = self.shortcuts.get_or_insert(Vec::new());
        for val in shortcuts {
            v.push(val.into());
        }
        self
    }
    pub fn start_url(mut self, start_url: impl Into<String>) -> Self {
        self.start_url = Some(start_url.into());
        self
    }
    pub fn theme_color(mut self, theme_color: impl Into<String>) -> Self {
        self.theme_color = Some(theme_color.into());
        self
    }
    pub fn build(self) -> WebAppManifest {
        WebAppManifest {
            background_color: self.background_color,
            description: self.description,
            dir: self.dir,
            display: self.display,
            display_overrides: self.display_overrides,
            file_handlers: self.file_handlers,
            icons: self.icons,
            id: self.id,
            lang: self.lang,
            launch_handler: self.launch_handler,
            name: self.name,
            orientation: self.orientation,
            prefer_related_applications: self.prefer_related_applications,
            protocol_handlers: self.protocol_handlers,
            related_applications: self.related_applications,
            scope: self.scope,
            scope_extensions: self.scope_extensions,
            screenshots: self.screenshots,
            share_target: self.share_target,
            short_name: self.short_name,
            shortcuts: self.shortcuts,
            start_url: self.start_url,
            theme_color: self.theme_color,
        }
    }
}
impl BackForwardCacheBlockingDetails {
    pub fn builder() -> BackForwardCacheBlockingDetailsBuilder {
        BackForwardCacheBlockingDetailsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BackForwardCacheBlockingDetailsBuilder {
    url: Option<String>,
    function: Option<String>,
    line_number: Option<i64>,
    column_number: Option<i64>,
}
impl BackForwardCacheBlockingDetailsBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn function(mut self, function: impl Into<String>) -> Self {
        self.function = Some(function.into());
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
    pub fn build(self) -> Result<BackForwardCacheBlockingDetails, String> {
        Ok(BackForwardCacheBlockingDetails {
            url: self.url,
            function: self.function,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
        })
    }
}
impl BackForwardCacheNotRestoredExplanation {
    pub fn builder() -> BackForwardCacheNotRestoredExplanationBuilder {
        BackForwardCacheNotRestoredExplanationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BackForwardCacheNotRestoredExplanationBuilder {
    r#type: Option<BackForwardCacheNotRestoredReasonType>,
    reason: Option<BackForwardCacheNotRestoredReason>,
    context: Option<String>,
    details: Option<Vec<BackForwardCacheBlockingDetails>>,
}
impl BackForwardCacheNotRestoredExplanationBuilder {
    pub fn r#type(mut self, r#type: impl Into<BackForwardCacheNotRestoredReasonType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn reason(mut self, reason: impl Into<BackForwardCacheNotRestoredReason>) -> Self {
        self.reason = Some(reason.into());
        self
    }
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn detail(mut self, detail: impl Into<BackForwardCacheBlockingDetails>) -> Self {
        let v = self.details.get_or_insert(Vec::new());
        v.push(detail.into());
        self
    }
    pub fn details<I, S>(mut self, details: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<BackForwardCacheBlockingDetails>,
    {
        let v = self.details.get_or_insert(Vec::new());
        for val in details {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<BackForwardCacheNotRestoredExplanation, String> {
        Ok(BackForwardCacheNotRestoredExplanation {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            reason: self
                .reason
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(reason)))?,
            context: self.context,
            details: self.details,
        })
    }
}
impl BackForwardCacheNotRestoredExplanationTree {
    pub fn builder() -> BackForwardCacheNotRestoredExplanationTreeBuilder {
        BackForwardCacheNotRestoredExplanationTreeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BackForwardCacheNotRestoredExplanationTreeBuilder {
    url: Option<String>,
    explanations: Option<Vec<BackForwardCacheNotRestoredExplanation>>,
    children: Option<Vec<BackForwardCacheNotRestoredExplanationTree>>,
}
impl BackForwardCacheNotRestoredExplanationTreeBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn explanation(
        mut self,
        explanation: impl Into<BackForwardCacheNotRestoredExplanation>,
    ) -> Self {
        let v = self.explanations.get_or_insert(Vec::new());
        v.push(explanation.into());
        self
    }
    pub fn explanations<I, S>(mut self, explanations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<BackForwardCacheNotRestoredExplanation>,
    {
        let v = self.explanations.get_or_insert(Vec::new());
        for val in explanations {
            v.push(val.into());
        }
        self
    }
    pub fn children(
        mut self,
        children: impl Into<BackForwardCacheNotRestoredExplanationTree>,
    ) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<BackForwardCacheNotRestoredExplanationTree>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<BackForwardCacheNotRestoredExplanationTree, String> {
        Ok(BackForwardCacheNotRestoredExplanationTree {
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            explanations: self.explanations.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(explanations))
            })?,
            children: self
                .children
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(children)))?,
        })
    }
}
