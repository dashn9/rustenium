use serde::{Deserialize, Serialize};
#[doc = "Unique id\n[RuleSetId](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-RuleSetId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct RuleSetId(String);
impl RuleSetId {
    pub fn new(val: impl Into<String>) -> Self {
        RuleSetId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for RuleSetId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<RuleSetId> for String {
    fn from(el: RuleSetId) -> String {
        el.0
    }
}
impl From<String> for RuleSetId {
    fn from(expr: String) -> Self {
        RuleSetId(expr)
    }
}
impl std::borrow::Borrow<str> for RuleSetId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl RuleSetId {
    pub const IDENTIFIER: &'static str = "Preload.RuleSetId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Corresponds to SpeculationRuleSet\n[RuleSet](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-RuleSet)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleSet {
    #[serde(rename = "id")]
    pub id: RuleSetId,
    #[doc = "Identifies a document which the rule set is associated with."]
    #[serde(rename = "loaderId")]
    pub loader_id: crate::browser_protocol::network::types::LoaderId,
    #[doc = "Source text of JSON representing the rule set. If it comes from\n`<script>` tag, it is the textContent of the node. Note that it is\na JSON for valid case.\n\nSee also:\n- https://wicg.github.io/nav-speculation/speculation-rules.html\n- https://github.com/WICG/nav-speculation/blob/main/triggers.md"]
    #[serde(rename = "sourceText")]
    pub source_text: String,
    #[doc = "A speculation rule set is either added through an inline\n`<script>` tag or through an external resource via the\n'Speculation-Rules' HTTP header. For the first case, we include\nthe BackendNodeId of the relevant `<script>` tag. For the second\ncase, we include the external URL where the rule set was loaded\nfrom, and also RequestId if Network domain is enabled.\n\nSee also:\n- https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script\n- https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header"]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_id: Option<crate::browser_protocol::network::types::RequestId>,
    #[doc = "Error information\n`errorMessage` is null iff `errorType` is null."]
    #[serde(rename = "errorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error_type: Option<RuleSetErrorType>,
    #[doc = "For more details, see:\nhttps://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md"]
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tag: Option<String>,
}
impl RuleSet {
    pub fn new(
        id: impl Into<RuleSetId>,
        loader_id: impl Into<crate::browser_protocol::network::types::LoaderId>,
        source_text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            loader_id: loader_id.into(),
            source_text: source_text.into(),
            backend_node_id: None,
            url: None,
            request_id: None,
            error_type: None,
            tag: None,
        }
    }
}
impl RuleSet {
    pub const IDENTIFIER: &'static str = "Preload.RuleSet";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleSetErrorType {
    #[serde(rename = "SourceIsNotJsonObject")]
    SourceIsNotJsonObject,
    #[serde(rename = "InvalidRulesSkipped")]
    InvalidRulesSkipped,
    #[serde(rename = "InvalidRulesetLevelTag")]
    InvalidRulesetLevelTag,
}
#[doc = "The type of preloading attempted. It corresponds to\nmojom::SpeculationAction (although PrefetchWithSubresources is omitted as it\nisn't being used by clients)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeculationAction {
    #[serde(rename = "Prefetch")]
    Prefetch,
    #[serde(rename = "Prerender")]
    Prerender,
    #[serde(rename = "PrerenderUntilScript")]
    PrerenderUntilScript,
}
#[doc = "Corresponds to mojom::SpeculationTargetHint.\nSee https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeculationTargetHint {
    #[serde(rename = "Blank")]
    Blank,
    #[serde(rename = "Self")]
    KSelf,
}
#[doc = "A key that identifies a preloading attempt.\n\nThe url used is the url specified by the trigger (i.e. the initial URL), and\nnot the final url that is navigated to. For example, prerendering allows\nsame-origin main frame navigations during the attempt, but the attempt is\nstill keyed with the initial URL.\n[PreloadingAttemptKey](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadingAttemptKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreloadingAttemptKey {
    #[serde(rename = "loaderId")]
    pub loader_id: crate::browser_protocol::network::types::LoaderId,
    #[serde(rename = "action")]
    pub action: SpeculationAction,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "targetHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_hint: Option<SpeculationTargetHint>,
}
impl PreloadingAttemptKey {
    pub fn new(
        loader_id: impl Into<crate::browser_protocol::network::types::LoaderId>,
        action: impl Into<SpeculationAction>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            loader_id: loader_id.into(),
            action: action.into(),
            url: url.into(),
            target_hint: None,
        }
    }
}
impl PreloadingAttemptKey {
    pub const IDENTIFIER: &'static str = "Preload.PreloadingAttemptKey";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Lists sources for a preloading attempt, specifically the ids of rule sets\nthat had a speculation rule that triggered the attempt, and the\nBackendNodeIds of <a href> or <area href> elements that triggered the\nattempt (in the case of attempts triggered by a document rule). It is\npossible for multiple rule sets and links to trigger a single attempt.\n[PreloadingAttemptSource](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadingAttemptSource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreloadingAttemptSource {
    #[serde(rename = "key")]
    pub key: PreloadingAttemptKey,
    #[serde(rename = "ruleSetIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_set_ids: Vec<RuleSetId>,
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<crate::browser_protocol::dom::types::BackendNodeId>,
}
impl PreloadingAttemptSource {
    pub fn new(
        key: impl Into<PreloadingAttemptKey>,
        rule_set_ids: Vec<RuleSetId>,
        node_ids: Vec<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        Self {
            key: key.into(),
            rule_set_ids,
            node_ids,
        }
    }
}
impl PreloadingAttemptSource {
    pub const IDENTIFIER: &'static str = "Preload.PreloadingAttemptSource";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Chrome manages different types of preloads together using a\nconcept of preloading pipeline. For example, if a site uses a\nSpeculationRules for prerender, Chrome first starts a prefetch and\nthen upgrades it to prerender.\n\nCDP events for them are emitted separately but they share\n`PreloadPipelineId`.\n[PreloadPipelineId](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PreloadPipelineId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct PreloadPipelineId(String);
impl PreloadPipelineId {
    pub fn new(val: impl Into<String>) -> Self {
        PreloadPipelineId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for PreloadPipelineId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<PreloadPipelineId> for String {
    fn from(el: PreloadPipelineId) -> String {
        el.0
    }
}
impl From<String> for PreloadPipelineId {
    fn from(expr: String) -> Self {
        PreloadPipelineId(expr)
    }
}
impl std::borrow::Borrow<str> for PreloadPipelineId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl PreloadPipelineId {
    pub const IDENTIFIER: &'static str = "Preload.PreloadPipelineId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "List of FinalStatus reasons for Prerender2."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrerenderFinalStatus {
    #[serde(rename = "Activated")]
    Activated,
    #[serde(rename = "Destroyed")]
    Destroyed,
    #[serde(rename = "LowEndDevice")]
    LowEndDevice,
    #[serde(rename = "InvalidSchemeRedirect")]
    InvalidSchemeRedirect,
    #[serde(rename = "InvalidSchemeNavigation")]
    InvalidSchemeNavigation,
    #[serde(rename = "NavigationRequestBlockedByCsp")]
    NavigationRequestBlockedByCsp,
    #[serde(rename = "MojoBinderPolicy")]
    MojoBinderPolicy,
    #[serde(rename = "RendererProcessCrashed")]
    RendererProcessCrashed,
    #[serde(rename = "RendererProcessKilled")]
    RendererProcessKilled,
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "TriggerDestroyed")]
    TriggerDestroyed,
    #[serde(rename = "NavigationNotCommitted")]
    NavigationNotCommitted,
    #[serde(rename = "NavigationBadHttpStatus")]
    NavigationBadHttpStatus,
    #[serde(rename = "ClientCertRequested")]
    ClientCertRequested,
    #[serde(rename = "NavigationRequestNetworkError")]
    NavigationRequestNetworkError,
    #[serde(rename = "CancelAllHostsForTesting")]
    CancelAllHostsForTesting,
    #[serde(rename = "DidFailLoad")]
    DidFailLoad,
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "SslCertificateError")]
    SslCertificateError,
    #[serde(rename = "LoginAuthRequested")]
    LoginAuthRequested,
    #[serde(rename = "UaChangeRequiresReload")]
    UaChangeRequiresReload,
    #[serde(rename = "BlockedByClient")]
    BlockedByClient,
    #[serde(rename = "AudioOutputDeviceRequested")]
    AudioOutputDeviceRequested,
    #[serde(rename = "MixedContent")]
    MixedContent,
    #[serde(rename = "TriggerBackgrounded")]
    TriggerBackgrounded,
    #[serde(rename = "MemoryLimitExceeded")]
    MemoryLimitExceeded,
    #[serde(rename = "DataSaverEnabled")]
    DataSaverEnabled,
    #[serde(rename = "TriggerUrlHasEffectiveUrl")]
    TriggerUrlHasEffectiveUrl,
    #[serde(rename = "ActivatedBeforeStarted")]
    ActivatedBeforeStarted,
    #[serde(rename = "InactivePageRestriction")]
    InactivePageRestriction,
    #[serde(rename = "StartFailed")]
    StartFailed,
    #[serde(rename = "TimeoutBackgrounded")]
    TimeoutBackgrounded,
    #[serde(rename = "CrossSiteRedirectInInitialNavigation")]
    CrossSiteRedirectInInitialNavigation,
    #[serde(rename = "CrossSiteNavigationInInitialNavigation")]
    CrossSiteNavigationInInitialNavigation,
    #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInInitialNavigation")]
    SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
    #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInInitialNavigation")]
    SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
    #[serde(rename = "ActivationNavigationParameterMismatch")]
    ActivationNavigationParameterMismatch,
    #[serde(rename = "ActivatedInBackground")]
    ActivatedInBackground,
    #[serde(rename = "EmbedderHostDisallowed")]
    EmbedderHostDisallowed,
    #[serde(rename = "ActivationNavigationDestroyedBeforeSuccess")]
    ActivationNavigationDestroyedBeforeSuccess,
    #[serde(rename = "TabClosedByUserGesture")]
    TabClosedByUserGesture,
    #[serde(rename = "TabClosedWithoutUserGesture")]
    TabClosedWithoutUserGesture,
    #[serde(rename = "PrimaryMainFrameRendererProcessCrashed")]
    PrimaryMainFrameRendererProcessCrashed,
    #[serde(rename = "PrimaryMainFrameRendererProcessKilled")]
    PrimaryMainFrameRendererProcessKilled,
    #[serde(rename = "ActivationFramePolicyNotCompatible")]
    ActivationFramePolicyNotCompatible,
    #[serde(rename = "PreloadingDisabled")]
    PreloadingDisabled,
    #[serde(rename = "BatterySaverEnabled")]
    BatterySaverEnabled,
    #[serde(rename = "ActivatedDuringMainFrameNavigation")]
    ActivatedDuringMainFrameNavigation,
    #[serde(rename = "PreloadingUnsupportedByWebContents")]
    PreloadingUnsupportedByWebContents,
    #[serde(rename = "CrossSiteRedirectInMainFrameNavigation")]
    CrossSiteRedirectInMainFrameNavigation,
    #[serde(rename = "CrossSiteNavigationInMainFrameNavigation")]
    CrossSiteNavigationInMainFrameNavigation,
    #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation")]
    SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
    #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation")]
    SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
    #[serde(rename = "MemoryPressureOnTrigger")]
    MemoryPressureOnTrigger,
    #[serde(rename = "MemoryPressureAfterTriggered")]
    MemoryPressureAfterTriggered,
    #[serde(rename = "PrerenderingDisabledByDevTools")]
    PrerenderingDisabledByDevTools,
    #[serde(rename = "SpeculationRuleRemoved")]
    SpeculationRuleRemoved,
    #[serde(rename = "ActivatedWithAuxiliaryBrowsingContexts")]
    ActivatedWithAuxiliaryBrowsingContexts,
    #[serde(rename = "MaxNumOfRunningEagerPrerendersExceeded")]
    MaxNumOfRunningEagerPrerendersExceeded,
    #[serde(rename = "MaxNumOfRunningNonEagerPrerendersExceeded")]
    MaxNumOfRunningNonEagerPrerendersExceeded,
    #[serde(rename = "MaxNumOfRunningEmbedderPrerendersExceeded")]
    MaxNumOfRunningEmbedderPrerendersExceeded,
    #[serde(rename = "PrerenderingUrlHasEffectiveUrl")]
    PrerenderingUrlHasEffectiveUrl,
    #[serde(rename = "RedirectedPrerenderingUrlHasEffectiveUrl")]
    RedirectedPrerenderingUrlHasEffectiveUrl,
    #[serde(rename = "ActivationUrlHasEffectiveUrl")]
    ActivationUrlHasEffectiveUrl,
    #[serde(rename = "JavaScriptInterfaceAdded")]
    JavaScriptInterfaceAdded,
    #[serde(rename = "JavaScriptInterfaceRemoved")]
    JavaScriptInterfaceRemoved,
    #[serde(rename = "AllPrerenderingCanceled")]
    AllPrerenderingCanceled,
    #[serde(rename = "WindowClosed")]
    WindowClosed,
    #[serde(rename = "SlowNetwork")]
    SlowNetwork,
    #[serde(rename = "OtherPrerenderedPageActivated")]
    OtherPrerenderedPageActivated,
    #[serde(rename = "V8OptimizerDisabled")]
    V8OptimizerDisabled,
    #[serde(rename = "PrerenderFailedDuringPrefetch")]
    PrerenderFailedDuringPrefetch,
    #[serde(rename = "BrowsingDataRemoved")]
    BrowsingDataRemoved,
    #[serde(rename = "PrerenderHostReused")]
    PrerenderHostReused,
}
#[doc = "Preloading status values, see also PreloadingTriggeringOutcome. This\nstatus is shared by prefetchStatusUpdated and prerenderStatusUpdated."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreloadingStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Ready")]
    Ready,
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Failure")]
    Failure,
    #[doc = "PreloadingTriggeringOutcome which not used by prefetch nor prerender."]
    #[serde(rename = "NotSupported")]
    NotSupported,
}
#[doc = "TODO(https://crbug.com/1384419): revisit the list of PrefetchStatus and\nfilter out the ones that aren't necessary to the developers."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrefetchStatus {
    #[doc = "Prefetch is not disabled by PrefetchHeldback."]
    #[serde(rename = "PrefetchAllowed")]
    PrefetchAllowed,
    #[serde(rename = "PrefetchFailedIneligibleRedirect")]
    PrefetchFailedIneligibleRedirect,
    #[serde(rename = "PrefetchFailedInvalidRedirect")]
    PrefetchFailedInvalidRedirect,
    #[serde(rename = "PrefetchFailedMIMENotSupported")]
    PrefetchFailedMimeNotSupported,
    #[serde(rename = "PrefetchFailedNetError")]
    PrefetchFailedNetError,
    #[serde(rename = "PrefetchFailedNon2XX")]
    PrefetchFailedNon2Xx,
    #[serde(rename = "PrefetchEvictedAfterBrowsingDataRemoved")]
    PrefetchEvictedAfterBrowsingDataRemoved,
    #[serde(rename = "PrefetchEvictedAfterCandidateRemoved")]
    PrefetchEvictedAfterCandidateRemoved,
    #[serde(rename = "PrefetchEvictedForNewerPrefetch")]
    PrefetchEvictedForNewerPrefetch,
    #[serde(rename = "PrefetchHeldback")]
    PrefetchHeldback,
    #[doc = "A previous prefetch to the origin got a HTTP 503 response with an\nRetry-After header that has no elapsed yet."]
    #[serde(rename = "PrefetchIneligibleRetryAfter")]
    PrefetchIneligibleRetryAfter,
    #[serde(rename = "PrefetchIsPrivacyDecoy")]
    PrefetchIsPrivacyDecoy,
    #[serde(rename = "PrefetchIsStale")]
    PrefetchIsStale,
    #[serde(rename = "PrefetchNotEligibleBrowserContextOffTheRecord")]
    PrefetchNotEligibleBrowserContextOffTheRecord,
    #[serde(rename = "PrefetchNotEligibleDataSaverEnabled")]
    PrefetchNotEligibleDataSaverEnabled,
    #[serde(rename = "PrefetchNotEligibleExistingProxy")]
    PrefetchNotEligibleExistingProxy,
    #[serde(rename = "PrefetchNotEligibleHostIsNonUnique")]
    PrefetchNotEligibleHostIsNonUnique,
    #[serde(rename = "PrefetchNotEligibleNonDefaultStoragePartition")]
    PrefetchNotEligibleNonDefaultStoragePartition,
    #[serde(rename = "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy")]
    PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
    #[serde(rename = "PrefetchNotEligibleSchemeIsNotHttps")]
    PrefetchNotEligibleSchemeIsNotHttps,
    #[serde(rename = "PrefetchNotEligibleUserHasCookies")]
    PrefetchNotEligibleUserHasCookies,
    #[serde(rename = "PrefetchNotEligibleUserHasServiceWorker")]
    PrefetchNotEligibleUserHasServiceWorker,
    #[serde(rename = "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler")]
    PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler,
    #[serde(rename = "PrefetchNotEligibleRedirectFromServiceWorker")]
    PrefetchNotEligibleRedirectFromServiceWorker,
    #[serde(rename = "PrefetchNotEligibleRedirectToServiceWorker")]
    PrefetchNotEligibleRedirectToServiceWorker,
    #[serde(rename = "PrefetchNotEligibleBatterySaverEnabled")]
    PrefetchNotEligibleBatterySaverEnabled,
    #[serde(rename = "PrefetchNotEligiblePreloadingDisabled")]
    PrefetchNotEligiblePreloadingDisabled,
    #[serde(rename = "PrefetchNotFinishedInTime")]
    PrefetchNotFinishedInTime,
    #[serde(rename = "PrefetchNotStarted")]
    PrefetchNotStarted,
    #[serde(rename = "PrefetchNotUsedCookiesChanged")]
    PrefetchNotUsedCookiesChanged,
    #[serde(rename = "PrefetchProxyNotAvailable")]
    PrefetchProxyNotAvailable,
    #[doc = "The response of the prefetch is used for the next navigation. This is\nthe final successful state."]
    #[serde(rename = "PrefetchResponseUsed")]
    PrefetchResponseUsed,
    #[doc = "The prefetch finished successfully but was never used."]
    #[serde(rename = "PrefetchSuccessfulButNotUsed")]
    PrefetchSuccessfulButNotUsed,
    #[serde(rename = "PrefetchNotUsedProbeFailed")]
    PrefetchNotUsedProbeFailed,
}
#[doc = "Information of headers to be displayed when the header mismatch occurred.\n[PrerenderMismatchedHeaders](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#type-PrerenderMismatchedHeaders)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrerenderMismatchedHeaders {
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[serde(rename = "initialValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initial_value: Option<String>,
    #[serde(rename = "activationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub activation_value: Option<String>,
}
impl PrerenderMismatchedHeaders {
    pub fn new(header_name: impl Into<String>) -> Self {
        Self {
            header_name: header_name.into(),
            initial_value: None,
            activation_value: None,
        }
    }
}
impl<T: Into<String>> From<T> for PrerenderMismatchedHeaders {
    fn from(url: T) -> Self {
        PrerenderMismatchedHeaders::new(url)
    }
}
impl PrerenderMismatchedHeaders {
    pub const IDENTIFIER: &'static str = "Preload.PrerenderMismatchedHeaders";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (PreloadTypes { RuleSetId (RuleSetId) , RuleSet (RuleSet) , RuleSetErrorType (RuleSetErrorType) , SpeculationAction (SpeculationAction) , SpeculationTargetHint (SpeculationTargetHint) , PreloadingAttemptKey (PreloadingAttemptKey) , PreloadingAttemptSource (PreloadingAttemptSource) , PreloadPipelineId (PreloadPipelineId) , PrerenderFinalStatus (PrerenderFinalStatus) , PreloadingStatus (PreloadingStatus) , PrefetchStatus (PrefetchStatus) , PrerenderMismatchedHeaders (PrerenderMismatchedHeaders) });
