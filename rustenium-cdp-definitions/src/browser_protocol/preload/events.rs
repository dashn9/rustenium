use serde::{Deserialize, Serialize};
#[doc = "Upsert. Currently, it is only emitted when a rule set added.\n[ruleSetUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#event-ruleSetUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleSetUpdated {
    #[serde(rename = "ruleSet")]
    pub rule_set: super::types::RuleSet,
}
impl RuleSetUpdated {
    pub const IDENTIFIER: &'static str = "Preload.ruleSetUpdated";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleSetRemoved {
    #[serde(rename = "id")]
    pub id: super::types::RuleSetId,
}
impl RuleSetRemoved {
    pub const IDENTIFIER: &'static str = "Preload.ruleSetRemoved";
}
#[doc = "Fired when a preload enabled state is updated.\n[preloadEnabledStateUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#event-preloadEnabledStateUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreloadEnabledStateUpdated {
    #[serde(rename = "disabledByPreference")]
    pub disabled_by_preference: bool,
    #[serde(rename = "disabledByDataSaver")]
    pub disabled_by_data_saver: bool,
    #[serde(rename = "disabledByBatterySaver")]
    pub disabled_by_battery_saver: bool,
    #[serde(rename = "disabledByHoldbackPrefetchSpeculationRules")]
    pub disabled_by_holdback_prefetch_speculation_rules: bool,
    #[serde(rename = "disabledByHoldbackPrerenderSpeculationRules")]
    pub disabled_by_holdback_prerender_speculation_rules: bool,
}
impl PreloadEnabledStateUpdated {
    pub const IDENTIFIER: &'static str = "Preload.preloadEnabledStateUpdated";
}
#[doc = "Fired when a prefetch attempt is updated.\n[prefetchStatusUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#event-prefetchStatusUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefetchStatusUpdated {
    #[serde(rename = "key")]
    pub key: super::types::PreloadingAttemptKey,
    #[serde(rename = "pipelineId")]
    pub pipeline_id: super::types::PreloadPipelineId,
    #[doc = "The frame id of the frame initiating prefetch."]
    #[serde(rename = "initiatingFrameId")]
    pub initiating_frame_id: super::super::page::types::FrameId,
    #[serde(rename = "prefetchUrl")]
    pub prefetch_url: String,
    #[serde(rename = "status")]
    pub status: super::types::PreloadingStatus,
    #[serde(rename = "prefetchStatus")]
    pub prefetch_status: super::types::PrefetchStatus,
    #[serde(rename = "requestId")]
    pub request_id: super::super::network::types::RequestId,
}
impl PrefetchStatusUpdated {
    pub const IDENTIFIER: &'static str = "Preload.prefetchStatusUpdated";
}
#[doc = "Fired when a prerender attempt is updated.\n[prerenderStatusUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#event-prerenderStatusUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrerenderStatusUpdated {
    #[serde(rename = "key")]
    pub key: super::types::PreloadingAttemptKey,
    #[serde(rename = "pipelineId")]
    pub pipeline_id: super::types::PreloadPipelineId,
    #[serde(rename = "status")]
    pub status: super::types::PreloadingStatus,
    #[serde(rename = "prerenderStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prerender_status: Option<super::types::PrerenderFinalStatus>,
    #[doc = "This is used to give users more information about the name of Mojo interface\nthat is incompatible with prerender and has caused the cancellation of the attempt."]
    #[serde(rename = "disallowedMojoInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub disallowed_mojo_interface: Option<String>,
    #[serde(rename = "mismatchedHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mismatched_headers: Option<Vec<super::types::PrerenderMismatchedHeaders>>,
}
impl PrerenderStatusUpdated {
    pub const IDENTIFIER: &'static str = "Preload.prerenderStatusUpdated";
}
#[doc = "Send a list of sources for all preloading attempts in a document.\n[preloadingAttemptSourcesUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Preload/#event-preloadingAttemptSourcesUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreloadingAttemptSourcesUpdated {
    #[serde(rename = "loaderId")]
    pub loader_id: super::super::network::types::LoaderId,
    #[serde(rename = "preloadingAttemptSources")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub preloading_attempt_sources: Vec<super::types::PreloadingAttemptSource>,
}
impl PreloadingAttemptSourcesUpdated {
    pub const IDENTIFIER: &'static str = "Preload.preloadingAttemptSourcesUpdated";
}
group_enum ! (PreloadEvents { RuleSetUpdated (RuleSetUpdated) , RuleSetRemoved (RuleSetRemoved) , PreloadEnabledStateUpdated (PreloadEnabledStateUpdated) , PrefetchStatusUpdated (PrefetchStatusUpdated) , PrerenderStatusUpdated (PrerenderStatusUpdated) , PreloadingAttemptSourcesUpdated (PreloadingAttemptSourcesUpdated) });
