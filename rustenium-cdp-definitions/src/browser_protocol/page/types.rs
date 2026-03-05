use serde::{Deserialize, Serialize};
#[doc = "Unique frame identifier.\n[FrameId](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct FrameId(String);
impl FrameId {
    pub fn new(val: impl Into<String>) -> Self {
        FrameId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for FrameId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<FrameId> for String {
    fn from(el: FrameId) -> String {
        el.0
    }
}
impl From<String> for FrameId {
    fn from(expr: String) -> Self {
        FrameId(expr)
    }
}
impl std::borrow::Borrow<str> for FrameId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl FrameId {
    pub const IDENTIFIER: &'static str = "Page.FrameId";
}
#[doc = "Indicates whether a frame has been identified as an ad."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdFrameType {
    #[serde(rename = "none")]
    None,
    #[doc = "This frame is a subframe of an ad frame."]
    #[serde(rename = "child")]
    Child,
    #[doc = "This frame is the root of an ad frame."]
    #[serde(rename = "root")]
    Root,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdFrameExplanation {
    #[serde(rename = "ParentIsAd")]
    ParentIsAd,
    #[serde(rename = "CreatedByAdScript")]
    CreatedByAdScript,
    #[serde(rename = "MatchedBlockingRule")]
    MatchedBlockingRule,
}
#[doc = "Indicates whether a frame has been identified as an ad and why.\n[AdFrameStatus](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdFrameStatus)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdFrameStatus {
    #[serde(rename = "adFrameType")]
    pub ad_frame_type: AdFrameType,
    #[serde(rename = "explanations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub explanations: Option<Vec<AdFrameExplanation>>,
}
impl AdFrameStatus {
    pub fn new(ad_frame_type: impl Into<AdFrameType>) -> Self {
        Self {
            ad_frame_type: ad_frame_type.into(),
            explanations: None,
        }
    }
}
impl AdFrameStatus {
    pub const IDENTIFIER: &'static str = "Page.AdFrameStatus";
}
#[doc = "Identifies the script which caused a script or frame to be labelled as an\nad.\n[AdScriptId](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdScriptId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdScriptId {
    #[doc = "Script Id of the script which caused a script or frame to be labelled as\nan ad."]
    #[serde(rename = "scriptId")]
    pub script_id: super::super::super::js_protocol::runtime::types::ScriptId,
    #[doc = "Id of scriptId's debugger."]
    #[serde(rename = "debuggerId")]
    pub debugger_id: super::super::super::js_protocol::runtime::types::UniqueDebuggerId,
}
impl AdScriptId {
    pub fn new(
        script_id: impl Into<super::super::super::js_protocol::runtime::types::ScriptId>,
        debugger_id: impl Into<super::super::super::js_protocol::runtime::types::UniqueDebuggerId>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            debugger_id: debugger_id.into(),
        }
    }
}
impl AdScriptId {
    pub const IDENTIFIER: &'static str = "Page.AdScriptId";
}
#[doc = "Encapsulates the script ancestry and the root script filterlist rule that\ncaused the frame to be labelled as an ad. Only created when `ancestryChain`\nis not empty.\n[AdScriptAncestry](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AdScriptAncestry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdScriptAncestry {
    #[doc = "A chain of `AdScriptId`s representing the ancestry of an ad script that\nled to the creation of a frame. The chain is ordered from the script\nitself (lower level) up to its root ancestor that was flagged by\nfilterlist."]
    #[serde(rename = "ancestryChain")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ancestry_chain: Vec<AdScriptId>,
    #[doc = "The filterlist rule that caused the root (last) script in\n`ancestryChain` to be ad-tagged. Only populated if the rule is\navailable."]
    #[serde(rename = "rootScriptFilterlistRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub root_script_filterlist_rule: Option<String>,
}
impl AdScriptAncestry {
    pub fn new(ancestry_chain: Vec<AdScriptId>) -> Self {
        Self {
            ancestry_chain,
            root_script_filterlist_rule: None,
        }
    }
}
impl AdScriptAncestry {
    pub const IDENTIFIER: &'static str = "Page.AdScriptAncestry";
}
#[doc = "Indicates whether the frame is a secure context and why it is the case."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureContextType {
    #[doc = "The origin is a secure context."]
    #[serde(rename = "Secure")]
    Secure,
    #[doc = "The host is localhost and hence is considered secure."]
    #[serde(rename = "SecureLocalhost")]
    SecureLocalhost,
    #[doc = "The origin has an insecure scheme and is not localhost."]
    #[serde(rename = "InsecureScheme")]
    InsecureScheme,
    #[doc = "One of the ancestor frames is not a secure context."]
    #[serde(rename = "InsecureAncestor")]
    InsecureAncestor,
}
#[doc = "Indicates whether the frame is cross-origin isolated and why it is the case."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrossOriginIsolatedContextType {
    #[doc = "The origin is cross-origin isolated."]
    #[serde(rename = "Isolated")]
    Isolated,
    #[doc = "The origin is not cross-origin isolated."]
    #[serde(rename = "NotIsolated")]
    NotIsolated,
    #[doc = "The cross-origin isolation feature is disabled."]
    #[serde(rename = "NotIsolatedFeatureDisabled")]
    NotIsolatedFeatureDisabled,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GatedApiFeatures {
    #[serde(rename = "SharedArrayBuffers")]
    SharedArrayBuffers,
    #[serde(rename = "SharedArrayBuffersTransferAllowed")]
    SharedArrayBuffersTransferAllowed,
    #[serde(rename = "PerformanceMeasureMemory")]
    PerformanceMeasureMemory,
    #[serde(rename = "PerformanceProfile")]
    PerformanceProfile,
}
#[doc = "All Permissions Policy features. This enum should match the one defined\nin services/network/public/cpp/permissions_policy/permissions_policy_features.json5.\nLINT.IfChange(PermissionsPolicyFeature)"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionsPolicyFeature {
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "all-screens-capture")]
    AllScreensCapture,
    #[serde(rename = "ambient-light-sensor")]
    AmbientLightSensor,
    #[serde(rename = "aria-notify")]
    AriaNotify,
    #[serde(rename = "attribution-reporting")]
    AttributionReporting,
    #[serde(rename = "autofill")]
    Autofill,
    #[serde(rename = "autoplay")]
    Autoplay,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "browsing-topics")]
    BrowsingTopics,
    #[serde(rename = "camera")]
    Camera,
    #[serde(rename = "captured-surface-control")]
    CapturedSurfaceControl,
    #[serde(rename = "ch-dpr")]
    ChDpr,
    #[serde(rename = "ch-device-memory")]
    ChDeviceMemory,
    #[serde(rename = "ch-downlink")]
    ChDownlink,
    #[serde(rename = "ch-ect")]
    ChEct,
    #[serde(rename = "ch-prefers-color-scheme")]
    ChPrefersColorScheme,
    #[serde(rename = "ch-prefers-reduced-motion")]
    ChPrefersReducedMotion,
    #[serde(rename = "ch-prefers-reduced-transparency")]
    ChPrefersReducedTransparency,
    #[serde(rename = "ch-rtt")]
    ChRtt,
    #[serde(rename = "ch-save-data")]
    ChSaveData,
    #[serde(rename = "ch-ua")]
    ChUa,
    #[serde(rename = "ch-ua-arch")]
    ChUaArch,
    #[serde(rename = "ch-ua-bitness")]
    ChUaBitness,
    #[serde(rename = "ch-ua-high-entropy-values")]
    ChUaHighEntropyValues,
    #[serde(rename = "ch-ua-platform")]
    ChUaPlatform,
    #[serde(rename = "ch-ua-model")]
    ChUaModel,
    #[serde(rename = "ch-ua-mobile")]
    ChUaMobile,
    #[serde(rename = "ch-ua-form-factors")]
    ChUaFormFactors,
    #[serde(rename = "ch-ua-full-version")]
    ChUaFullVersion,
    #[serde(rename = "ch-ua-full-version-list")]
    ChUaFullVersionList,
    #[serde(rename = "ch-ua-platform-version")]
    ChUaPlatformVersion,
    #[serde(rename = "ch-ua-wow64")]
    ChUaWow64,
    #[serde(rename = "ch-viewport-height")]
    ChViewportHeight,
    #[serde(rename = "ch-viewport-width")]
    ChViewportWidth,
    #[serde(rename = "ch-width")]
    ChWidth,
    #[serde(rename = "clipboard-read")]
    ClipboardRead,
    #[serde(rename = "clipboard-write")]
    ClipboardWrite,
    #[serde(rename = "compute-pressure")]
    ComputePressure,
    #[serde(rename = "controlled-frame")]
    ControlledFrame,
    #[serde(rename = "cross-origin-isolated")]
    CrossOriginIsolated,
    #[serde(rename = "deferred-fetch")]
    DeferredFetch,
    #[serde(rename = "deferred-fetch-minimal")]
    DeferredFetchMinimal,
    #[serde(rename = "device-attributes")]
    DeviceAttributes,
    #[serde(rename = "digital-credentials-create")]
    DigitalCredentialsCreate,
    #[serde(rename = "digital-credentials-get")]
    DigitalCredentialsGet,
    #[serde(rename = "direct-sockets")]
    DirectSockets,
    #[serde(rename = "direct-sockets-multicast")]
    DirectSocketsMulticast,
    #[serde(rename = "direct-sockets-private")]
    DirectSocketsPrivate,
    #[serde(rename = "display-capture")]
    DisplayCapture,
    #[serde(rename = "document-domain")]
    DocumentDomain,
    #[serde(rename = "encrypted-media")]
    EncryptedMedia,
    #[serde(rename = "execution-while-out-of-viewport")]
    ExecutionWhileOutOfViewport,
    #[serde(rename = "execution-while-not-rendered")]
    ExecutionWhileNotRendered,
    #[serde(rename = "fenced-unpartitioned-storage-read")]
    FencedUnpartitionedStorageRead,
    #[serde(rename = "focus-without-user-activation")]
    FocusWithoutUserActivation,
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "frobulate")]
    Frobulate,
    #[serde(rename = "gamepad")]
    Gamepad,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "hid")]
    Hid,
    #[serde(rename = "identity-credentials-get")]
    IdentityCredentialsGet,
    #[serde(rename = "idle-detection")]
    IdleDetection,
    #[serde(rename = "interest-cohort")]
    InterestCohort,
    #[serde(rename = "join-ad-interest-group")]
    JoinAdInterestGroup,
    #[serde(rename = "keyboard-map")]
    KeyboardMap,
    #[serde(rename = "language-detector")]
    LanguageDetector,
    #[serde(rename = "language-model")]
    LanguageModel,
    #[serde(rename = "local-fonts")]
    LocalFonts,
    #[serde(rename = "local-network")]
    LocalNetwork,
    #[serde(rename = "local-network-access")]
    LocalNetworkAccess,
    #[serde(rename = "loopback-network")]
    LoopbackNetwork,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "manual-text")]
    ManualText,
    #[serde(rename = "media-playback-while-not-visible")]
    MediaPlaybackWhileNotVisible,
    #[serde(rename = "microphone")]
    Microphone,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "on-device-speech-recognition")]
    OnDeviceSpeechRecognition,
    #[serde(rename = "otp-credentials")]
    OtpCredentials,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "picture-in-picture")]
    PictureInPicture,
    #[serde(rename = "private-aggregation")]
    PrivateAggregation,
    #[serde(rename = "private-state-token-issuance")]
    PrivateStateTokenIssuance,
    #[serde(rename = "private-state-token-redemption")]
    PrivateStateTokenRedemption,
    #[serde(rename = "publickey-credentials-create")]
    PublickeyCredentialsCreate,
    #[serde(rename = "publickey-credentials-get")]
    PublickeyCredentialsGet,
    #[serde(rename = "record-ad-auction-events")]
    RecordAdAuctionEvents,
    #[serde(rename = "rewriter")]
    Rewriter,
    #[serde(rename = "run-ad-auction")]
    RunAdAuction,
    #[serde(rename = "screen-wake-lock")]
    ScreenWakeLock,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "shared-storage")]
    SharedStorage,
    #[serde(rename = "shared-storage-select-url")]
    SharedStorageSelectUrl,
    #[serde(rename = "smart-card")]
    SmartCard,
    #[serde(rename = "speaker-selection")]
    SpeakerSelection,
    #[serde(rename = "storage-access")]
    StorageAccess,
    #[serde(rename = "sub-apps")]
    SubApps,
    #[serde(rename = "summarizer")]
    Summarizer,
    #[serde(rename = "sync-xhr")]
    SyncXhr,
    #[serde(rename = "translator")]
    Translator,
    #[serde(rename = "unload")]
    Unload,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "usb-unrestricted")]
    UsbUnrestricted,
    #[serde(rename = "vertical-scroll")]
    VerticalScroll,
    #[serde(rename = "web-app-installation")]
    WebAppInstallation,
    #[serde(rename = "web-printing")]
    WebPrinting,
    #[serde(rename = "web-share")]
    WebShare,
    #[serde(rename = "window-management")]
    WindowManagement,
    #[serde(rename = "writer")]
    Writer,
    #[serde(rename = "xr-spatial-tracking")]
    XrSpatialTracking,
}
#[doc = "LINT.ThenChange(//services/network/public/cpp/permissions_policy/permissions_policy_features.json5:PermissionsPolicy)\nReason for a permissions policy feature to be disabled."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionsPolicyBlockReason {
    #[doc = "Declaration in HTTP header."]
    #[serde(rename = "Header")]
    Header,
    #[doc = "Declaration in iframe attribute."]
    #[serde(rename = "IframeAttribute")]
    IframeAttribute,
    #[doc = "Inside fenced frame."]
    #[serde(rename = "InFencedFrameTree")]
    InFencedFrameTree,
    #[doc = "Inside an Isolated Application."]
    #[serde(rename = "InIsolatedApp")]
    InIsolatedApp,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPolicyBlockLocator {
    #[serde(rename = "frameId")]
    pub frame_id: FrameId,
    #[serde(rename = "blockReason")]
    pub block_reason: PermissionsPolicyBlockReason,
}
impl PermissionsPolicyBlockLocator {
    pub fn new(
        frame_id: impl Into<FrameId>,
        block_reason: impl Into<PermissionsPolicyBlockReason>,
    ) -> Self {
        Self {
            frame_id: frame_id.into(),
            block_reason: block_reason.into(),
        }
    }
}
impl PermissionsPolicyBlockLocator {
    pub const IDENTIFIER: &'static str = "Page.PermissionsPolicyBlockLocator";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionsPolicyFeatureState {
    #[serde(rename = "feature")]
    pub feature: PermissionsPolicyFeature,
    #[serde(rename = "allowed")]
    pub allowed: bool,
    #[serde(rename = "locator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub locator: Option<PermissionsPolicyBlockLocator>,
}
impl PermissionsPolicyFeatureState {
    pub fn new(feature: impl Into<PermissionsPolicyFeature>, allowed: impl Into<bool>) -> Self {
        Self {
            feature: feature.into(),
            allowed: allowed.into(),
            locator: None,
        }
    }
}
impl PermissionsPolicyFeatureState {
    pub const IDENTIFIER: &'static str = "Page.PermissionsPolicyFeatureState";
}
#[doc = "Origin Trial(https://www.chromium.org/blink/origin-trials) support.\nStatus for an Origin Trial token."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OriginTrialTokenStatus {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "Insecure")]
    Insecure,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "WrongOrigin")]
    WrongOrigin,
    #[serde(rename = "InvalidSignature")]
    InvalidSignature,
    #[serde(rename = "Malformed")]
    Malformed,
    #[serde(rename = "WrongVersion")]
    WrongVersion,
    #[serde(rename = "FeatureDisabled")]
    FeatureDisabled,
    #[serde(rename = "TokenDisabled")]
    TokenDisabled,
    #[serde(rename = "FeatureDisabledForUser")]
    FeatureDisabledForUser,
    #[serde(rename = "UnknownTrial")]
    UnknownTrial,
}
#[doc = "Status for an Origin Trial."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OriginTrialStatus {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "ValidTokenNotProvided")]
    ValidTokenNotProvided,
    #[serde(rename = "OSNotSupported")]
    OsNotSupported,
    #[serde(rename = "TrialNotAllowed")]
    TrialNotAllowed,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OriginTrialUsageRestriction {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Subset")]
    Subset,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginTrialToken {
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "matchSubDomains")]
    pub match_sub_domains: bool,
    #[serde(rename = "trialName")]
    pub trial_name: String,
    #[serde(rename = "expiryTime")]
    pub expiry_time: super::super::network::types::TimeSinceEpoch,
    #[serde(rename = "isThirdParty")]
    pub is_third_party: bool,
    #[serde(rename = "usageRestriction")]
    pub usage_restriction: OriginTrialUsageRestriction,
}
impl OriginTrialToken {
    pub const IDENTIFIER: &'static str = "Page.OriginTrialToken";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginTrialTokenWithStatus {
    #[serde(rename = "rawTokenText")]
    pub raw_token_text: String,
    #[doc = "`parsedToken` is present only when the token is extractable and\nparsable."]
    #[serde(rename = "parsedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parsed_token: Option<OriginTrialToken>,
    #[serde(rename = "status")]
    pub status: OriginTrialTokenStatus,
}
impl OriginTrialTokenWithStatus {
    pub fn new(
        raw_token_text: impl Into<String>,
        status: impl Into<OriginTrialTokenStatus>,
    ) -> Self {
        Self {
            raw_token_text: raw_token_text.into(),
            status: status.into(),
            parsed_token: None,
        }
    }
}
impl OriginTrialTokenWithStatus {
    pub const IDENTIFIER: &'static str = "Page.OriginTrialTokenWithStatus";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginTrial {
    #[serde(rename = "trialName")]
    pub trial_name: String,
    #[serde(rename = "status")]
    pub status: OriginTrialStatus,
    #[serde(rename = "tokensWithStatus")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tokens_with_status: Vec<OriginTrialTokenWithStatus>,
}
impl OriginTrial {
    pub fn new(
        trial_name: impl Into<String>,
        status: impl Into<OriginTrialStatus>,
        tokens_with_status: Vec<OriginTrialTokenWithStatus>,
    ) -> Self {
        Self {
            trial_name: trial_name.into(),
            status: status.into(),
            tokens_with_status,
        }
    }
}
impl OriginTrial {
    pub const IDENTIFIER: &'static str = "Page.OriginTrial";
}
#[doc = "Additional information about the frame document's security origin.\n[SecurityOriginDetails](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-SecurityOriginDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityOriginDetails {
    #[doc = "Indicates whether the frame document's security origin is one\nof the local hostnames (e.g. \"localhost\") or IP addresses (IPv4\n127.0.0.0/8 or IPv6 ::1)."]
    #[serde(rename = "isLocalhost")]
    pub is_localhost: bool,
}
impl SecurityOriginDetails {
    pub fn new(is_localhost: impl Into<bool>) -> Self {
        Self {
            is_localhost: is_localhost.into(),
        }
    }
}
impl SecurityOriginDetails {
    pub const IDENTIFIER: &'static str = "Page.SecurityOriginDetails";
}
#[doc = "Information about the Frame on the page.\n[Frame](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-Frame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    #[doc = "Frame unique identifier."]
    #[serde(rename = "id")]
    pub id: FrameId,
    #[doc = "Parent frame identifier."]
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<FrameId>,
    #[doc = "Identifier of the loader associated with this frame."]
    #[serde(rename = "loaderId")]
    pub loader_id: super::super::network::types::LoaderId,
    #[doc = "Frame's name as specified in the tag."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[doc = "Frame document's URL without fragment."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Frame document's URL fragment including the '#'."]
    #[serde(rename = "urlFragment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_fragment: Option<String>,
    #[doc = "Frame document's registered domain, taking the public suffixes list into account.\nExtracted from the Frame's url.\nExample URLs: http://www.google.com/file.html -> \"google.com\"\nhttp://a.b.co.uk/file.html      -> \"b.co.uk\""]
    #[serde(rename = "domainAndRegistry")]
    pub domain_and_registry: String,
    #[doc = "Frame document's security origin."]
    #[serde(rename = "securityOrigin")]
    pub security_origin: String,
    #[doc = "Additional details about the frame document's security origin."]
    #[serde(rename = "securityOriginDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin_details: Option<SecurityOriginDetails>,
    #[doc = "Frame document's mimeType as determined by the browser."]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[doc = "If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment."]
    #[serde(rename = "unreachableUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unreachable_url: Option<String>,
    #[doc = "Indicates whether this frame was tagged as an ad and why."]
    #[serde(rename = "adFrameStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ad_frame_status: Option<AdFrameStatus>,
    #[doc = "Indicates whether the main document is a secure context and explains why that is the case."]
    #[serde(rename = "secureContextType")]
    pub secure_context_type: SecureContextType,
    #[doc = "Indicates whether this is a cross origin isolated context."]
    #[serde(rename = "crossOriginIsolatedContextType")]
    pub cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    #[doc = "Indicated which gated APIs / features are available."]
    #[serde(rename = "gatedAPIFeatures")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gated_api_features: Vec<GatedApiFeatures>,
}
impl Frame {
    pub const IDENTIFIER: &'static str = "Page.Frame";
}
#[doc = "Information about the Resource on the page.\n[FrameResource](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameResource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameResource {
    #[doc = "Resource URL."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Type of this resource."]
    #[serde(rename = "type")]
    pub r#type: super::super::network::types::ResourceType,
    #[doc = "Resource mimeType as determined by the browser."]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[doc = "last-modified timestamp as reported by server."]
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub last_modified: Option<super::super::network::types::TimeSinceEpoch>,
    #[doc = "Resource content size."]
    #[serde(rename = "contentSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_size: Option<f64>,
    #[doc = "True if the resource failed to load."]
    #[serde(rename = "failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub failed: Option<bool>,
    #[doc = "True if the resource was canceled during loading."]
    #[serde(rename = "canceled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub canceled: Option<bool>,
}
impl FrameResource {
    pub fn new(
        url: impl Into<String>,
        r#type: impl Into<super::super::network::types::ResourceType>,
        mime_type: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            r#type: r#type.into(),
            mime_type: mime_type.into(),
            last_modified: None,
            content_size: None,
            failed: None,
            canceled: None,
        }
    }
}
impl FrameResource {
    pub const IDENTIFIER: &'static str = "Page.FrameResource";
}
#[doc = "Information about the Frame hierarchy along with their cached resources.\n[FrameResourceTree](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameResourceTree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameResourceTree {
    #[doc = "Frame information for this tree item."]
    #[serde(rename = "frame")]
    pub frame: Frame,
    #[doc = "Child frames."]
    #[serde(rename = "childFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub child_frames: Option<Vec<FrameResourceTree>>,
    #[doc = "Information about frame resources."]
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<FrameResource>,
}
impl FrameResourceTree {
    pub fn new(frame: impl Into<Frame>, resources: Vec<FrameResource>) -> Self {
        Self {
            frame: frame.into(),
            resources,
            child_frames: None,
        }
    }
}
impl FrameResourceTree {
    pub const IDENTIFIER: &'static str = "Page.FrameResourceTree";
}
#[doc = "Information about the Frame hierarchy.\n[FrameTree](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FrameTree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameTree {
    #[doc = "Frame information for this tree item."]
    #[serde(rename = "frame")]
    pub frame: Frame,
    #[doc = "Child frames."]
    #[serde(rename = "childFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub child_frames: Option<Vec<FrameTree>>,
}
impl FrameTree {
    pub fn new(frame: impl Into<Frame>) -> Self {
        Self {
            frame: frame.into(),
            child_frames: None,
        }
    }
}
impl FrameTree {
    pub const IDENTIFIER: &'static str = "Page.FrameTree";
}
#[doc = "Unique script identifier.\n[ScriptIdentifier](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScriptIdentifier)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct ScriptIdentifier(String);
impl ScriptIdentifier {
    pub fn new(val: impl Into<String>) -> Self {
        ScriptIdentifier(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for ScriptIdentifier {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<ScriptIdentifier> for String {
    fn from(el: ScriptIdentifier) -> String {
        el.0
    }
}
impl From<String> for ScriptIdentifier {
    fn from(expr: String) -> Self {
        ScriptIdentifier(expr)
    }
}
impl ScriptIdentifier {
    pub const IDENTIFIER: &'static str = "Page.ScriptIdentifier";
}
#[doc = "Transition type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionType {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "typed")]
    Typed,
    #[serde(rename = "address_bar")]
    AddressBar,
    #[serde(rename = "auto_bookmark")]
    AutoBookmark,
    #[serde(rename = "auto_subframe")]
    AutoSubframe,
    #[serde(rename = "manual_subframe")]
    ManualSubframe,
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "auto_toplevel")]
    AutoToplevel,
    #[serde(rename = "form_submit")]
    FormSubmit,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "keyword_generated")]
    KeywordGenerated,
    #[serde(rename = "other")]
    Other,
}
#[doc = "Navigation history entry.\n[NavigationEntry](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-NavigationEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationEntry {
    #[doc = "Unique id of the navigation history entry."]
    #[serde(rename = "id")]
    pub id: i64,
    #[doc = "URL of the navigation history entry."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "URL that the user typed in the url bar."]
    #[serde(rename = "userTypedURL")]
    pub user_typed_url: String,
    #[doc = "Title of the navigation history entry."]
    #[serde(rename = "title")]
    pub title: String,
    #[doc = "Transition type."]
    #[serde(rename = "transitionType")]
    pub transition_type: TransitionType,
}
impl NavigationEntry {
    pub const IDENTIFIER: &'static str = "Page.NavigationEntry";
}
#[doc = "Screencast frame metadata.\n[ScreencastFrameMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScreencastFrameMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastFrameMetadata {
    #[doc = "Top offset in DIP."]
    #[serde(rename = "offsetTop")]
    pub offset_top: f64,
    #[doc = "Page scale factor."]
    #[serde(rename = "pageScaleFactor")]
    pub page_scale_factor: f64,
    #[doc = "Device screen width in DIP."]
    #[serde(rename = "deviceWidth")]
    pub device_width: f64,
    #[doc = "Device screen height in DIP."]
    #[serde(rename = "deviceHeight")]
    pub device_height: f64,
    #[doc = "Position of horizontal scroll in CSS pixels."]
    #[serde(rename = "scrollOffsetX")]
    pub scroll_offset_x: f64,
    #[doc = "Position of vertical scroll in CSS pixels."]
    #[serde(rename = "scrollOffsetY")]
    pub scroll_offset_y: f64,
    #[doc = "Frame swap timestamp."]
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timestamp: Option<super::super::network::types::TimeSinceEpoch>,
}
impl ScreencastFrameMetadata {
    pub const IDENTIFIER: &'static str = "Page.ScreencastFrameMetadata";
}
#[doc = "Javascript dialog type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogType {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "confirm")]
    Confirm,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "beforeunload")]
    Beforeunload,
}
#[doc = "Error while paring app manifest.\n[AppManifestError](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AppManifestError)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppManifestError {
    #[doc = "Error message."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "If critical, this is a non-recoverable parse error."]
    #[serde(rename = "critical")]
    pub critical: i64,
    #[doc = "Error line."]
    #[serde(rename = "line")]
    pub line: i64,
    #[doc = "Error column."]
    #[serde(rename = "column")]
    pub column: i64,
}
impl AppManifestError {
    pub fn new(
        message: impl Into<String>,
        critical: impl Into<i64>,
        line: impl Into<i64>,
        column: impl Into<i64>,
    ) -> Self {
        Self {
            message: message.into(),
            critical: critical.into(),
            line: line.into(),
            column: column.into(),
        }
    }
}
impl AppManifestError {
    pub const IDENTIFIER: &'static str = "Page.AppManifestError";
}
#[doc = "Parsed app manifest properties.\n[AppManifestParsedProperties](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-AppManifestParsedProperties)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppManifestParsedProperties {
    #[doc = "Computed scope value"]
    #[serde(rename = "scope")]
    pub scope: String,
}
impl AppManifestParsedProperties {
    pub fn new(scope: impl Into<String>) -> Self {
        Self {
            scope: scope.into(),
        }
    }
}
impl<T: Into<String>> From<T> for AppManifestParsedProperties {
    fn from(url: T) -> Self {
        AppManifestParsedProperties::new(url)
    }
}
impl AppManifestParsedProperties {
    pub const IDENTIFIER: &'static str = "Page.AppManifestParsedProperties";
}
#[doc = "Layout viewport position and dimensions.\n[LayoutViewport](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-LayoutViewport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutViewport {
    #[doc = "Horizontal offset relative to the document (CSS pixels)."]
    #[serde(rename = "pageX")]
    pub page_x: i64,
    #[doc = "Vertical offset relative to the document (CSS pixels)."]
    #[serde(rename = "pageY")]
    pub page_y: i64,
    #[doc = "Width (CSS pixels), excludes scrollbar if present."]
    #[serde(rename = "clientWidth")]
    pub client_width: i64,
    #[doc = "Height (CSS pixels), excludes scrollbar if present."]
    #[serde(rename = "clientHeight")]
    pub client_height: i64,
}
impl LayoutViewport {
    pub fn new(
        page_x: impl Into<i64>,
        page_y: impl Into<i64>,
        client_width: impl Into<i64>,
        client_height: impl Into<i64>,
    ) -> Self {
        Self {
            page_x: page_x.into(),
            page_y: page_y.into(),
            client_width: client_width.into(),
            client_height: client_height.into(),
        }
    }
}
impl LayoutViewport {
    pub const IDENTIFIER: &'static str = "Page.LayoutViewport";
}
#[doc = "Visual viewport position, dimensions, and scale.\n[VisualViewport](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-VisualViewport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualViewport {
    #[doc = "Horizontal offset relative to the layout viewport (CSS pixels)."]
    #[serde(rename = "offsetX")]
    pub offset_x: f64,
    #[doc = "Vertical offset relative to the layout viewport (CSS pixels)."]
    #[serde(rename = "offsetY")]
    pub offset_y: f64,
    #[doc = "Horizontal offset relative to the document (CSS pixels)."]
    #[serde(rename = "pageX")]
    pub page_x: f64,
    #[doc = "Vertical offset relative to the document (CSS pixels)."]
    #[serde(rename = "pageY")]
    pub page_y: f64,
    #[doc = "Width (CSS pixels), excludes scrollbar if present."]
    #[serde(rename = "clientWidth")]
    pub client_width: f64,
    #[doc = "Height (CSS pixels), excludes scrollbar if present."]
    #[serde(rename = "clientHeight")]
    pub client_height: f64,
    #[doc = "Scale relative to the ideal viewport (size at width=device-width)."]
    #[serde(rename = "scale")]
    pub scale: f64,
    #[doc = "Page zoom factor (CSS to device independent pixels ratio)."]
    #[serde(rename = "zoom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub zoom: Option<f64>,
}
impl VisualViewport {
    pub const IDENTIFIER: &'static str = "Page.VisualViewport";
}
#[doc = "Viewport for capturing screenshot.\n[Viewport](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-Viewport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    #[doc = "X offset in device independent pixels (dip)."]
    #[serde(rename = "x")]
    pub x: f64,
    #[doc = "Y offset in device independent pixels (dip)."]
    #[serde(rename = "y")]
    pub y: f64,
    #[doc = "Rectangle width in device independent pixels (dip)."]
    #[serde(rename = "width")]
    pub width: f64,
    #[doc = "Rectangle height in device independent pixels (dip)."]
    #[serde(rename = "height")]
    pub height: f64,
    #[doc = "Page scale factor."]
    #[serde(rename = "scale")]
    pub scale: f64,
}
impl Viewport {
    pub const IDENTIFIER: &'static str = "Page.Viewport";
}
#[doc = "Generic font families collection.\n[FontFamilies](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FontFamilies)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FontFamilies {
    #[doc = "The standard font-family."]
    #[serde(rename = "standard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub standard: Option<String>,
    #[doc = "The fixed font-family."]
    #[serde(rename = "fixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed: Option<String>,
    #[doc = "The serif font-family."]
    #[serde(rename = "serif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serif: Option<String>,
    #[doc = "The sansSerif font-family."]
    #[serde(rename = "sansSerif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sans_serif: Option<String>,
    #[doc = "The cursive font-family."]
    #[serde(rename = "cursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cursive: Option<String>,
    #[doc = "The fantasy font-family."]
    #[serde(rename = "fantasy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fantasy: Option<String>,
    #[doc = "The math font-family."]
    #[serde(rename = "math")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub math: Option<String>,
}
impl FontFamilies {
    pub const IDENTIFIER: &'static str = "Page.FontFamilies";
}
#[doc = "Font families collection for a script.\n[ScriptFontFamilies](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ScriptFontFamilies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptFontFamilies {
    #[doc = "Name of the script which these font families are defined for."]
    #[serde(rename = "script")]
    pub script: String,
    #[doc = "Generic font families collection for the script."]
    #[serde(rename = "fontFamilies")]
    pub font_families: FontFamilies,
}
impl ScriptFontFamilies {
    pub fn new(script: impl Into<String>, font_families: impl Into<FontFamilies>) -> Self {
        Self {
            script: script.into(),
            font_families: font_families.into(),
        }
    }
}
impl ScriptFontFamilies {
    pub const IDENTIFIER: &'static str = "Page.ScriptFontFamilies";
}
#[doc = "Default font sizes.\n[FontSizes](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FontSizes)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FontSizes {
    #[doc = "Default standard font size."]
    #[serde(rename = "standard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub standard: Option<i64>,
    #[doc = "Default fixed font size."]
    #[serde(rename = "fixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed: Option<i64>,
}
impl FontSizes {
    pub const IDENTIFIER: &'static str = "Page.FontSizes";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientNavigationReason {
    #[serde(rename = "anchorClick")]
    AnchorClick,
    #[serde(rename = "formSubmissionGet")]
    FormSubmissionGet,
    #[serde(rename = "formSubmissionPost")]
    FormSubmissionPost,
    #[serde(rename = "httpHeaderRefresh")]
    HttpHeaderRefresh,
    #[serde(rename = "initialFrameNavigation")]
    InitialFrameNavigation,
    #[serde(rename = "metaTagRefresh")]
    MetaTagRefresh,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "pageBlockInterstitial")]
    PageBlockInterstitial,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "scriptInitiated")]
    ScriptInitiated,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientNavigationDisposition {
    #[serde(rename = "currentTab")]
    CurrentTab,
    #[serde(rename = "newTab")]
    NewTab,
    #[serde(rename = "newWindow")]
    NewWindow,
    #[serde(rename = "download")]
    Download,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallabilityErrorArgument {
    #[doc = "Argument name (e.g. name:'minimum-icon-size-in-pixels')."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Argument value (e.g. value:'64')."]
    #[serde(rename = "value")]
    pub value: String,
}
impl InstallabilityErrorArgument {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl InstallabilityErrorArgument {
    pub const IDENTIFIER: &'static str = "Page.InstallabilityErrorArgument";
}
#[doc = "The installability error\n[InstallabilityError](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-InstallabilityError)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallabilityError {
    #[doc = "The error id (e.g. 'manifest-missing-suitable-icon')."]
    #[serde(rename = "errorId")]
    pub error_id: String,
    #[doc = "The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'})."]
    #[serde(rename = "errorArguments")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub error_arguments: Vec<InstallabilityErrorArgument>,
}
impl InstallabilityError {
    pub fn new(
        error_id: impl Into<String>,
        error_arguments: Vec<InstallabilityErrorArgument>,
    ) -> Self {
        Self {
            error_id: error_id.into(),
            error_arguments,
        }
    }
}
impl InstallabilityError {
    pub const IDENTIFIER: &'static str = "Page.InstallabilityError";
}
#[doc = "The referring-policy used for the navigation."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReferrerPolicy {
    #[serde(rename = "noReferrer")]
    NoReferrer,
    #[serde(rename = "noReferrerWhenDowngrade")]
    NoReferrerWhenDowngrade,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "originWhenCrossOrigin")]
    OriginWhenCrossOrigin,
    #[serde(rename = "sameOrigin")]
    SameOrigin,
    #[serde(rename = "strictOrigin")]
    StrictOrigin,
    #[serde(rename = "strictOriginWhenCrossOrigin")]
    StrictOriginWhenCrossOrigin,
    #[serde(rename = "unsafeUrl")]
    UnsafeUrl,
}
#[doc = "Per-script compilation cache parameters for `Page.produceCompilationCache`\n[CompilationCacheParams](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-CompilationCacheParams)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilationCacheParams {
    #[doc = "The URL of the script to produce a compilation cache entry for."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "A hint to the backend whether eager compilation is recommended.\n(the actual compilation mode used is upon backend discretion)."]
    #[serde(rename = "eager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub eager: Option<bool>,
}
impl CompilationCacheParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            eager: None,
        }
    }
}
impl<T: Into<String>> From<T> for CompilationCacheParams {
    fn from(url: T) -> Self {
        CompilationCacheParams::new(url)
    }
}
impl CompilationCacheParams {
    pub const IDENTIFIER: &'static str = "Page.CompilationCacheParams";
}
#[doc = "The manifest of a webapp, see\nhttps://www.w3.org/TR/appmanifest/#dfn-manifest.\nSome fields do not appear in the standard since the API is designed to\nexpose more browser internal states.\n[FileFilter](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-FileFilter)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileFilter {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "accepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accepts: Option<Vec<String>>,
}
impl FileFilter {
    pub const IDENTIFIER: &'static str = "Page.FileFilter";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileHandler {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub icons: Option<Vec<ImageResource>>,
    #[doc = "Mimic a map, name is the key, accepts is the value."]
    #[serde(rename = "accepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accepts: Option<Vec<FileFilter>>,
    #[doc = "Won't repeat the enums, using string for easy comparison. Same as the\nother enums below."]
    #[serde(rename = "launchType")]
    pub launch_type: String,
}
impl FileHandler {
    pub fn new(
        action: impl Into<String>,
        name: impl Into<String>,
        launch_type: impl Into<String>,
    ) -> Self {
        Self {
            action: action.into(),
            name: name.into(),
            launch_type: launch_type.into(),
            icons: None,
            accepts: None,
        }
    }
}
impl FileHandler {
    pub const IDENTIFIER: &'static str = "Page.FileHandler";
}
#[doc = "The image definition used in both icon and screenshot.\n[ImageResource](https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-ImageResource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageResource {
    #[doc = "The src field in the definition, but changing to url in favor of\nconsistency."]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "sizes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sizes: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<String>,
}
impl ImageResource {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            sizes: None,
            r#type: None,
        }
    }
}
impl<T: Into<String>> From<T> for ImageResource {
    fn from(url: T) -> Self {
        ImageResource::new(url)
    }
}
impl ImageResource {
    pub const IDENTIFIER: &'static str = "Page.ImageResource";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchHandler {
    #[serde(rename = "clientMode")]
    pub client_mode: String,
}
impl LaunchHandler {
    pub fn new(client_mode: impl Into<String>) -> Self {
        Self {
            client_mode: client_mode.into(),
        }
    }
}
impl<T: Into<String>> From<T> for LaunchHandler {
    fn from(url: T) -> Self {
        LaunchHandler::new(url)
    }
}
impl LaunchHandler {
    pub const IDENTIFIER: &'static str = "Page.LaunchHandler";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolHandler {
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "url")]
    pub url: String,
}
impl ProtocolHandler {
    pub fn new(protocol: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            protocol: protocol.into(),
            url: url.into(),
        }
    }
}
impl ProtocolHandler {
    pub const IDENTIFIER: &'static str = "Page.ProtocolHandler";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedApplication {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
}
impl RelatedApplication {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            id: None,
        }
    }
}
impl<T: Into<String>> From<T> for RelatedApplication {
    fn from(url: T) -> Self {
        RelatedApplication::new(url)
    }
}
impl RelatedApplication {
    pub const IDENTIFIER: &'static str = "Page.RelatedApplication";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScopeExtension {
    #[doc = "Instead of using tuple, this field always returns the serialized string\nfor easy understanding and comparison."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "hasOriginWildcard")]
    pub has_origin_wildcard: bool,
}
impl ScopeExtension {
    pub fn new(origin: impl Into<String>, has_origin_wildcard: impl Into<bool>) -> Self {
        Self {
            origin: origin.into(),
            has_origin_wildcard: has_origin_wildcard.into(),
        }
    }
}
impl ScopeExtension {
    pub const IDENTIFIER: &'static str = "Page.ScopeExtension";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Screenshot {
    #[serde(rename = "image")]
    pub image: ImageResource,
    #[serde(rename = "formFactor")]
    pub form_factor: String,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub label: Option<String>,
}
impl Screenshot {
    pub fn new(image: impl Into<ImageResource>, form_factor: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            form_factor: form_factor.into(),
            label: None,
        }
    }
}
impl Screenshot {
    pub const IDENTIFIER: &'static str = "Page.Screenshot";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareTarget {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "enctype")]
    pub enctype: String,
    #[doc = "Embed the ShareTargetParams"]
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub files: Option<Vec<FileFilter>>,
}
impl ShareTarget {
    pub fn new(
        action: impl Into<String>,
        method: impl Into<String>,
        enctype: impl Into<String>,
    ) -> Self {
        Self {
            action: action.into(),
            method: method.into(),
            enctype: enctype.into(),
            title: None,
            text: None,
            url: None,
            files: None,
        }
    }
}
impl ShareTarget {
    pub const IDENTIFIER: &'static str = "Page.ShareTarget";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shortcut {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}
impl Shortcut {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }
}
impl Shortcut {
    pub const IDENTIFIER: &'static str = "Page.Shortcut";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WebAppManifest {
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub background_color: Option<String>,
    #[doc = "The extra description provided by the manifest."]
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dir: Option<String>,
    #[serde(rename = "display")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display: Option<String>,
    #[doc = "The overrided display mode controlled by the user."]
    #[serde(rename = "displayOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_overrides: Option<Vec<String>>,
    #[doc = "The handlers to open files."]
    #[serde(rename = "fileHandlers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub file_handlers: Option<Vec<FileHandler>>,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub icons: Option<Vec<ImageResource>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang: Option<String>,
    #[doc = "TODO(crbug.com/1231886): This field is non-standard and part of a Chrome\nexperiment. See:\nhttps://github.com/WICG/web-app-launch/blob/main/launch_handler.md"]
    #[serde(rename = "launchHandler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub launch_handler: Option<LaunchHandler>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub orientation: Option<String>,
    #[serde(rename = "preferRelatedApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prefer_related_applications: Option<bool>,
    #[doc = "The handlers to open protocols."]
    #[serde(rename = "protocolHandlers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub protocol_handlers: Option<Vec<ProtocolHandler>>,
    #[serde(rename = "relatedApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub related_applications: Option<Vec<RelatedApplication>>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scope: Option<String>,
    #[doc = "Non-standard, see\nhttps://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md"]
    #[serde(rename = "scopeExtensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scope_extensions: Option<Vec<ScopeExtension>>,
    #[doc = "The screenshots used by chromium."]
    #[serde(rename = "screenshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screenshots: Option<Vec<Screenshot>>,
    #[serde(rename = "shareTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub share_target: Option<ShareTarget>,
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub short_name: Option<String>,
    #[serde(rename = "shortcuts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shortcuts: Option<Vec<Shortcut>>,
    #[serde(rename = "startUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_url: Option<String>,
    #[serde(rename = "themeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub theme_color: Option<String>,
}
impl WebAppManifest {
    pub const IDENTIFIER: &'static str = "Page.WebAppManifest";
}
#[doc = "The type of a frameNavigated event."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigationType {
    #[serde(rename = "Navigation")]
    Navigation,
    #[serde(rename = "BackForwardCacheRestore")]
    BackForwardCacheRestore,
}
#[doc = "List of not restored reasons for back-forward cache."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackForwardCacheNotRestoredReason {
    #[serde(rename = "NotPrimaryMainFrame")]
    NotPrimaryMainFrame,
    #[serde(rename = "BackForwardCacheDisabled")]
    BackForwardCacheDisabled,
    #[serde(rename = "RelatedActiveContentsExist")]
    RelatedActiveContentsExist,
    #[serde(rename = "HTTPStatusNotOK")]
    HttpStatusNotOk,
    #[serde(rename = "SchemeNotHTTPOrHTTPS")]
    SchemeNotHttpOrHttps,
    #[serde(rename = "Loading")]
    Loading,
    #[serde(rename = "WasGrantedMediaAccess")]
    WasGrantedMediaAccess,
    #[serde(rename = "DisableForRenderFrameHostCalled")]
    DisableForRenderFrameHostCalled,
    #[serde(rename = "DomainNotAllowed")]
    DomainNotAllowed,
    #[serde(rename = "HTTPMethodNotGET")]
    HttpMethodNotGet,
    #[serde(rename = "SubframeIsNavigating")]
    SubframeIsNavigating,
    #[serde(rename = "Timeout")]
    Timeout,
    #[serde(rename = "CacheLimit")]
    CacheLimit,
    #[serde(rename = "JavaScriptExecution")]
    JavaScriptExecution,
    #[serde(rename = "RendererProcessKilled")]
    RendererProcessKilled,
    #[serde(rename = "RendererProcessCrashed")]
    RendererProcessCrashed,
    #[serde(rename = "SchedulerTrackedFeatureUsed")]
    SchedulerTrackedFeatureUsed,
    #[serde(rename = "ConflictingBrowsingInstance")]
    ConflictingBrowsingInstance,
    #[serde(rename = "CacheFlushed")]
    CacheFlushed,
    #[serde(rename = "ServiceWorkerVersionActivation")]
    ServiceWorkerVersionActivation,
    #[serde(rename = "SessionRestored")]
    SessionRestored,
    #[serde(rename = "ServiceWorkerPostMessage")]
    ServiceWorkerPostMessage,
    #[serde(rename = "EnteredBackForwardCacheBeforeServiceWorkerHostAdded")]
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    #[serde(rename = "RenderFrameHostReused_SameSite")]
    RenderFrameHostReusedSameSite,
    #[serde(rename = "RenderFrameHostReused_CrossSite")]
    RenderFrameHostReusedCrossSite,
    #[serde(rename = "ServiceWorkerClaim")]
    ServiceWorkerClaim,
    #[serde(rename = "IgnoreEventAndEvict")]
    IgnoreEventAndEvict,
    #[serde(rename = "HaveInnerContents")]
    HaveInnerContents,
    #[serde(rename = "TimeoutPuttingInCache")]
    TimeoutPuttingInCache,
    #[serde(rename = "BackForwardCacheDisabledByLowMemory")]
    BackForwardCacheDisabledByLowMemory,
    #[serde(rename = "BackForwardCacheDisabledByCommandLine")]
    BackForwardCacheDisabledByCommandLine,
    #[serde(rename = "NetworkRequestDatapipeDrainedAsBytesConsumer")]
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    #[serde(rename = "NetworkRequestRedirected")]
    NetworkRequestRedirected,
    #[serde(rename = "NetworkRequestTimeout")]
    NetworkRequestTimeout,
    #[serde(rename = "NetworkExceedsBufferLimit")]
    NetworkExceedsBufferLimit,
    #[serde(rename = "NavigationCancelledWhileRestoring")]
    NavigationCancelledWhileRestoring,
    #[serde(rename = "NotMostRecentNavigationEntry")]
    NotMostRecentNavigationEntry,
    #[serde(rename = "BackForwardCacheDisabledForPrerender")]
    BackForwardCacheDisabledForPrerender,
    #[serde(rename = "UserAgentOverrideDiffers")]
    UserAgentOverrideDiffers,
    #[serde(rename = "ForegroundCacheLimit")]
    ForegroundCacheLimit,
    #[serde(rename = "BrowsingInstanceNotSwapped")]
    BrowsingInstanceNotSwapped,
    #[serde(rename = "BackForwardCacheDisabledForDelegate")]
    BackForwardCacheDisabledForDelegate,
    #[serde(rename = "UnloadHandlerExistsInMainFrame")]
    UnloadHandlerExistsInMainFrame,
    #[serde(rename = "UnloadHandlerExistsInSubFrame")]
    UnloadHandlerExistsInSubFrame,
    #[serde(rename = "ServiceWorkerUnregistration")]
    ServiceWorkerUnregistration,
    #[serde(rename = "CacheControlNoStore")]
    CacheControlNoStore,
    #[serde(rename = "CacheControlNoStoreCookieModified")]
    CacheControlNoStoreCookieModified,
    #[serde(rename = "CacheControlNoStoreHTTPOnlyCookieModified")]
    CacheControlNoStoreHttpOnlyCookieModified,
    #[serde(rename = "NoResponseHead")]
    NoResponseHead,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "ActivationNavigationsDisallowedForBug1234857")]
    ActivationNavigationsDisallowedForBug1234857,
    #[serde(rename = "ErrorDocument")]
    ErrorDocument,
    #[serde(rename = "FencedFramesEmbedder")]
    FencedFramesEmbedder,
    #[serde(rename = "CookieDisabled")]
    CookieDisabled,
    #[serde(rename = "HTTPAuthRequired")]
    HttpAuthRequired,
    #[serde(rename = "CookieFlushed")]
    CookieFlushed,
    #[serde(rename = "BroadcastChannelOnMessage")]
    BroadcastChannelOnMessage,
    #[serde(rename = "WebViewSettingsChanged")]
    WebViewSettingsChanged,
    #[serde(rename = "WebViewJavaScriptObjectChanged")]
    WebViewJavaScriptObjectChanged,
    #[serde(rename = "WebViewMessageListenerInjected")]
    WebViewMessageListenerInjected,
    #[serde(rename = "WebViewSafeBrowsingAllowlistChanged")]
    WebViewSafeBrowsingAllowlistChanged,
    #[serde(rename = "WebViewDocumentStartJavascriptChanged")]
    WebViewDocumentStartJavascriptChanged,
    #[doc = "Blocklisted features"]
    #[serde(rename = "WebSocket")]
    WebSocket,
    #[serde(rename = "WebTransport")]
    WebTransport,
    #[serde(rename = "WebRTC")]
    WebRtc,
    #[serde(rename = "MainResourceHasCacheControlNoStore")]
    MainResourceHasCacheControlNoStore,
    #[serde(rename = "MainResourceHasCacheControlNoCache")]
    MainResourceHasCacheControlNoCache,
    #[serde(rename = "SubresourceHasCacheControlNoStore")]
    SubresourceHasCacheControlNoStore,
    #[serde(rename = "SubresourceHasCacheControlNoCache")]
    SubresourceHasCacheControlNoCache,
    #[serde(rename = "ContainsPlugins")]
    ContainsPlugins,
    #[serde(rename = "DocumentLoaded")]
    DocumentLoaded,
    #[serde(rename = "OutstandingNetworkRequestOthers")]
    OutstandingNetworkRequestOthers,
    #[serde(rename = "RequestedMIDIPermission")]
    RequestedMidiPermission,
    #[serde(rename = "RequestedAudioCapturePermission")]
    RequestedAudioCapturePermission,
    #[serde(rename = "RequestedVideoCapturePermission")]
    RequestedVideoCapturePermission,
    #[serde(rename = "RequestedBackForwardCacheBlockedSensors")]
    RequestedBackForwardCacheBlockedSensors,
    #[serde(rename = "RequestedBackgroundWorkPermission")]
    RequestedBackgroundWorkPermission,
    #[serde(rename = "BroadcastChannel")]
    BroadcastChannel,
    #[serde(rename = "WebXR")]
    WebXr,
    #[serde(rename = "SharedWorker")]
    SharedWorker,
    #[serde(rename = "SharedWorkerMessage")]
    SharedWorkerMessage,
    #[serde(rename = "SharedWorkerWithNoActiveClient")]
    SharedWorkerWithNoActiveClient,
    #[serde(rename = "WebLocks")]
    WebLocks,
    #[serde(rename = "WebLocksContention")]
    WebLocksContention,
    #[serde(rename = "WebHID")]
    WebHid,
    #[serde(rename = "WebBluetooth")]
    WebBluetooth,
    #[serde(rename = "WebShare")]
    WebShare,
    #[serde(rename = "RequestedStorageAccessGrant")]
    RequestedStorageAccessGrant,
    #[serde(rename = "WebNfc")]
    WebNfc,
    #[serde(rename = "OutstandingNetworkRequestFetch")]
    OutstandingNetworkRequestFetch,
    #[serde(rename = "OutstandingNetworkRequestXHR")]
    OutstandingNetworkRequestXhr,
    #[serde(rename = "AppBanner")]
    AppBanner,
    #[serde(rename = "Printing")]
    Printing,
    #[serde(rename = "WebDatabase")]
    WebDatabase,
    #[serde(rename = "PictureInPicture")]
    PictureInPicture,
    #[serde(rename = "SpeechRecognizer")]
    SpeechRecognizer,
    #[serde(rename = "IdleManager")]
    IdleManager,
    #[serde(rename = "PaymentManager")]
    PaymentManager,
    #[serde(rename = "SpeechSynthesis")]
    SpeechSynthesis,
    #[serde(rename = "KeyboardLock")]
    KeyboardLock,
    #[serde(rename = "WebOTPService")]
    WebOtpService,
    #[serde(rename = "OutstandingNetworkRequestDirectSocket")]
    OutstandingNetworkRequestDirectSocket,
    #[serde(rename = "InjectedJavascript")]
    InjectedJavascript,
    #[serde(rename = "InjectedStyleSheet")]
    InjectedStyleSheet,
    #[serde(rename = "KeepaliveRequest")]
    KeepaliveRequest,
    #[serde(rename = "IndexedDBEvent")]
    IndexedDbEvent,
    #[serde(rename = "Dummy")]
    Dummy,
    #[serde(rename = "JsNetworkRequestReceivedCacheControlNoStoreResource")]
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    #[serde(rename = "WebRTCUsedWithCCNS")]
    WebRtcUsedWithCcns,
    #[serde(rename = "WebTransportUsedWithCCNS")]
    WebTransportUsedWithCcns,
    #[serde(rename = "WebSocketUsedWithCCNS")]
    WebSocketUsedWithCcns,
    #[serde(rename = "SmartCard")]
    SmartCard,
    #[serde(rename = "LiveMediaStreamTrack")]
    LiveMediaStreamTrack,
    #[serde(rename = "UnloadHandler")]
    UnloadHandler,
    #[serde(rename = "ParserAborted")]
    ParserAborted,
    #[doc = "Disabled for RenderFrameHost reasons\nSee content/browser/renderer_host/back_forward_cache_disable.h for explanations."]
    #[serde(rename = "ContentSecurityHandler")]
    ContentSecurityHandler,
    #[serde(rename = "ContentWebAuthenticationAPI")]
    ContentWebAuthenticationApi,
    #[serde(rename = "ContentFileChooser")]
    ContentFileChooser,
    #[serde(rename = "ContentSerial")]
    ContentSerial,
    #[serde(rename = "ContentFileSystemAccess")]
    ContentFileSystemAccess,
    #[serde(rename = "ContentMediaDevicesDispatcherHost")]
    ContentMediaDevicesDispatcherHost,
    #[serde(rename = "ContentWebBluetooth")]
    ContentWebBluetooth,
    #[serde(rename = "ContentWebUSB")]
    ContentWebUsb,
    #[serde(rename = "ContentMediaSessionService")]
    ContentMediaSessionService,
    #[serde(rename = "ContentScreenReader")]
    ContentScreenReader,
    #[serde(rename = "ContentDiscarded")]
    ContentDiscarded,
    #[doc = "See components/back_forward_cache/back_forward_cache_disable.h for explanations."]
    #[serde(rename = "EmbedderPopupBlockerTabHelper")]
    EmbedderPopupBlockerTabHelper,
    #[serde(rename = "EmbedderSafeBrowsingTriggeredPopupBlocker")]
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    #[serde(rename = "EmbedderSafeBrowsingThreatDetails")]
    EmbedderSafeBrowsingThreatDetails,
    #[serde(rename = "EmbedderAppBannerManager")]
    EmbedderAppBannerManager,
    #[serde(rename = "EmbedderDomDistillerViewerSource")]
    EmbedderDomDistillerViewerSource,
    #[serde(rename = "EmbedderDomDistillerSelfDeletingRequestDelegate")]
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    #[serde(rename = "EmbedderOomInterventionTabHelper")]
    EmbedderOomInterventionTabHelper,
    #[serde(rename = "EmbedderOfflinePage")]
    EmbedderOfflinePage,
    #[serde(rename = "EmbedderChromePasswordManagerClientBindCredentialManager")]
    EmbedderChromePasswordManagerClientBindCredentialManager,
    #[serde(rename = "EmbedderPermissionRequestManager")]
    EmbedderPermissionRequestManager,
    #[serde(rename = "EmbedderModalDialog")]
    EmbedderModalDialog,
    #[serde(rename = "EmbedderExtensions")]
    EmbedderExtensions,
    #[serde(rename = "EmbedderExtensionMessaging")]
    EmbedderExtensionMessaging,
    #[serde(rename = "EmbedderExtensionMessagingForOpenPort")]
    EmbedderExtensionMessagingForOpenPort,
    #[serde(rename = "EmbedderExtensionSentMessageToCachedFrame")]
    EmbedderExtensionSentMessageToCachedFrame,
    #[serde(rename = "RequestedByWebViewClient")]
    RequestedByWebViewClient,
    #[serde(rename = "PostMessageByWebViewClient")]
    PostMessageByWebViewClient,
    #[serde(rename = "CacheControlNoStoreDeviceBoundSessionTerminated")]
    CacheControlNoStoreDeviceBoundSessionTerminated,
    #[serde(rename = "CacheLimitPrunedOnModerateMemoryPressure")]
    CacheLimitPrunedOnModerateMemoryPressure,
    #[serde(rename = "CacheLimitPrunedOnCriticalMemoryPressure")]
    CacheLimitPrunedOnCriticalMemoryPressure,
}
#[doc = "Types of not restored reasons for back-forward cache."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackForwardCacheNotRestoredReasonType {
    #[serde(rename = "SupportPending")]
    SupportPending,
    #[serde(rename = "PageSupportNeeded")]
    PageSupportNeeded,
    #[serde(rename = "Circumstantial")]
    Circumstantial,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackForwardCacheBlockingDetails {
    #[doc = "Url of the file where blockage happened. Optional because of tests."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "Function name where blockage happened. Optional because of anonymous functions and tests."]
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub function: Option<String>,
    #[doc = "Line number in the script (0-based)."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "Column number in the script (0-based)."]
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
impl BackForwardCacheBlockingDetails {
    pub fn new(line_number: impl Into<i64>, column_number: impl Into<i64>) -> Self {
        Self {
            line_number: line_number.into(),
            column_number: column_number.into(),
            url: None,
            function: None,
        }
    }
}
impl BackForwardCacheBlockingDetails {
    pub const IDENTIFIER: &'static str = "Page.BackForwardCacheBlockingDetails";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackForwardCacheNotRestoredExplanation {
    #[doc = "Type of the reason"]
    #[serde(rename = "type")]
    pub r#type: BackForwardCacheNotRestoredReasonType,
    #[doc = "Not restored reason"]
    #[serde(rename = "reason")]
    pub reason: BackForwardCacheNotRestoredReason,
    #[doc = "Context associated with the reason. The meaning of this context is\ndependent on the reason:\n- EmbedderExtensionSentMessageToCachedFrame: the extension ID.\n"]
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<String>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub details: Option<Vec<BackForwardCacheBlockingDetails>>,
}
impl BackForwardCacheNotRestoredExplanation {
    pub fn new(
        r#type: impl Into<BackForwardCacheNotRestoredReasonType>,
        reason: impl Into<BackForwardCacheNotRestoredReason>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            reason: reason.into(),
            context: None,
            details: None,
        }
    }
}
impl BackForwardCacheNotRestoredExplanation {
    pub const IDENTIFIER: &'static str = "Page.BackForwardCacheNotRestoredExplanation";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackForwardCacheNotRestoredExplanationTree {
    #[doc = "URL of each frame"]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Not restored reasons of each frame"]
    #[serde(rename = "explanations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub explanations: Vec<BackForwardCacheNotRestoredExplanation>,
    #[doc = "Array of children frame"]
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<BackForwardCacheNotRestoredExplanationTree>,
}
impl BackForwardCacheNotRestoredExplanationTree {
    pub fn new(
        url: impl Into<String>,
        explanations: Vec<BackForwardCacheNotRestoredExplanation>,
        children: Vec<BackForwardCacheNotRestoredExplanationTree>,
    ) -> Self {
        Self {
            url: url.into(),
            explanations,
            children,
        }
    }
}
impl BackForwardCacheNotRestoredExplanationTree {
    pub const IDENTIFIER: &'static str = "Page.BackForwardCacheNotRestoredExplanationTree";
}
group_enum ! (PageTypes { FrameId (FrameId) , AdFrameType (AdFrameType) , AdFrameExplanation (AdFrameExplanation) , AdFrameStatus (AdFrameStatus) , AdScriptId (AdScriptId) , AdScriptAncestry (AdScriptAncestry) , SecureContextType (SecureContextType) , CrossOriginIsolatedContextType (CrossOriginIsolatedContextType) , GatedApiFeatures (GatedApiFeatures) , PermissionsPolicyFeature (PermissionsPolicyFeature) , PermissionsPolicyBlockReason (PermissionsPolicyBlockReason) , PermissionsPolicyBlockLocator (PermissionsPolicyBlockLocator) , PermissionsPolicyFeatureState (PermissionsPolicyFeatureState) , OriginTrialTokenStatus (OriginTrialTokenStatus) , OriginTrialStatus (OriginTrialStatus) , OriginTrialUsageRestriction (OriginTrialUsageRestriction) , OriginTrialToken (OriginTrialToken) , OriginTrialTokenWithStatus (OriginTrialTokenWithStatus) , OriginTrial (OriginTrial) , SecurityOriginDetails (SecurityOriginDetails) , Frame (Frame) , FrameResource (FrameResource) , FrameResourceTree (FrameResourceTree) , FrameTree (FrameTree) , ScriptIdentifier (ScriptIdentifier) , TransitionType (TransitionType) , NavigationEntry (NavigationEntry) , ScreencastFrameMetadata (ScreencastFrameMetadata) , DialogType (DialogType) , AppManifestError (AppManifestError) , AppManifestParsedProperties (AppManifestParsedProperties) , LayoutViewport (LayoutViewport) , VisualViewport (VisualViewport) , Viewport (Viewport) , FontFamilies (FontFamilies) , ScriptFontFamilies (ScriptFontFamilies) , FontSizes (FontSizes) , ClientNavigationReason (ClientNavigationReason) , ClientNavigationDisposition (ClientNavigationDisposition) , InstallabilityErrorArgument (InstallabilityErrorArgument) , InstallabilityError (InstallabilityError) , ReferrerPolicy (ReferrerPolicy) , CompilationCacheParams (CompilationCacheParams) , FileFilter (FileFilter) , FileHandler (FileHandler) , ImageResource (ImageResource) , LaunchHandler (LaunchHandler) , ProtocolHandler (ProtocolHandler) , RelatedApplication (RelatedApplication) , ScopeExtension (ScopeExtension) , Screenshot (Screenshot) , ShareTarget (ShareTarget) , Shortcut (Shortcut) , WebAppManifest (WebAppManifest) , NavigationType (NavigationType) , BackForwardCacheNotRestoredReason (BackForwardCacheNotRestoredReason) , BackForwardCacheNotRestoredReasonType (BackForwardCacheNotRestoredReasonType) , BackForwardCacheBlockingDetails (BackForwardCacheBlockingDetails) , BackForwardCacheNotRestoredExplanation (BackForwardCacheNotRestoredExplanation) , BackForwardCacheNotRestoredExplanationTree (BackForwardCacheNotRestoredExplanationTree) });
