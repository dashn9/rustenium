use serde::{Deserialize, Serialize};
#[doc = "Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the\nposition specified by `location`.\n[addRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-addRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddRuleParams {
    #[doc = "The css style sheet identifier where a new rule should be inserted."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[doc = "The text of a new rule."]
    #[serde(rename = "ruleText")]
    pub rule_text: String,
    #[doc = "Text position of a new rule in the target style sheet."]
    #[serde(rename = "location")]
    pub location: super::types::SourceRange,
    #[doc = "NodeId for the DOM node in whose context custom property declarations for registered properties should be\nvalidated. If omitted, declarations in the new rule text can only be validated statically, which may produce\nincorrect results if the declaration contains a var() for example."]
    #[serde(rename = "nodeForPropertySyntaxValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_for_property_syntax_validation: Option<crate::browser_protocol::dom::types::NodeId>,
}
impl AddRuleParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        rule_text: impl Into<String>,
        location: impl Into<super::types::SourceRange>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            rule_text: rule_text.into(),
            location: location.into(),
            node_for_property_syntax_validation: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddRuleMethod {
    #[serde(rename = "CSS.addRule")]
    AddRule,
}
#[doc = "Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the\nposition specified by `location`.\n[addRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-addRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddRule {
    pub method: AddRuleMethod,
    pub params: AddRuleParams,
}
impl AddRule {
    pub const IDENTIFIER: &'static str = "CSS.addRule";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AddRule {
    type Result = super::results::AddRuleResult;
}
#[doc = "Returns all class names from specified stylesheet.\n[collectClassNames](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-collectClassNames)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
}
impl CollectClassNamesParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectClassNamesMethod {
    #[serde(rename = "CSS.collectClassNames")]
    CollectClassNames,
}
#[doc = "Returns all class names from specified stylesheet.\n[collectClassNames](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-collectClassNames)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNames {
    pub method: CollectClassNamesMethod,
    pub params: CollectClassNamesParams,
}
impl CollectClassNames {
    pub const IDENTIFIER: &'static str = "CSS.collectClassNames";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CollectClassNames {
    type Result = super::results::CollectClassNamesResult;
}
#[doc = "Creates a new special \"via-inspector\" stylesheet in the frame with given `frameId`.\n[createStyleSheet](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-createStyleSheet)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateStyleSheetParams {
    #[doc = "Identifier of the frame where \"via-inspector\" stylesheet should be created."]
    #[serde(rename = "frameId")]
    pub frame_id: crate::browser_protocol::page::types::FrameId,
    #[doc = "If true, creates a new stylesheet for every call. If false,\nreturns a stylesheet previously created by a call with force=false\nfor the frame's document if it exists or creates a new stylesheet\n(default: false)."]
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub force: Option<bool>,
}
impl CreateStyleSheetParams {
    pub fn new(frame_id: impl Into<crate::browser_protocol::page::types::FrameId>) -> Self {
        Self {
            frame_id: frame_id.into(),
            force: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateStyleSheetMethod {
    #[serde(rename = "CSS.createStyleSheet")]
    CreateStyleSheet,
}
#[doc = "Creates a new special \"via-inspector\" stylesheet in the frame with given `frameId`.\n[createStyleSheet](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-createStyleSheet)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateStyleSheet {
    pub method: CreateStyleSheetMethod,
    pub params: CreateStyleSheetParams,
}
impl CreateStyleSheet {
    pub const IDENTIFIER: &'static str = "CSS.createStyleSheet";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CreateStyleSheet {
    type Result = super::results::CreateStyleSheetResult;
}
#[doc = "Disables the CSS agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "CSS.disable")]
    Disable,
}
#[doc = "Disables the CSS agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "CSS.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been\nenabled until the result of this command is received.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "CSS.enable")]
    Enable,
}
#[doc = "Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been\nenabled until the result of this command is received.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "CSS.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Ensures that the given node will have specified pseudo-classes whenever its style is computed by\nthe browser.\n[forcePseudoState](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-forcePseudoState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcePseudoStateParams {
    #[doc = "The element id for which to force the pseudo state."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
    #[doc = "Element pseudo classes to force when computing the element's style."]
    #[serde(rename = "forcedPseudoClasses")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forced_pseudo_classes: Vec<String>,
}
impl ForcePseudoStateParams {
    pub fn new(
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
        forced_pseudo_classes: Vec<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            forced_pseudo_classes,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForcePseudoStateMethod {
    #[serde(rename = "CSS.forcePseudoState")]
    ForcePseudoState,
}
#[doc = "Ensures that the given node will have specified pseudo-classes whenever its style is computed by\nthe browser.\n[forcePseudoState](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-forcePseudoState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcePseudoState {
    pub method: ForcePseudoStateMethod,
    pub params: ForcePseudoStateParams,
}
impl ForcePseudoState {
    pub const IDENTIFIER: &'static str = "CSS.forcePseudoState";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ForcePseudoState {
    type Result = super::results::ForcePseudoStateResult;
}
#[doc = "Ensures that the given node is in its starting-style state.\n[forceStartingStyle](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-forceStartingStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceStartingStyleParams {
    #[doc = "The element id for which to force the starting-style state."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
    #[doc = "Boolean indicating if this is on or off."]
    #[serde(rename = "forced")]
    pub forced: bool,
}
impl ForceStartingStyleParams {
    pub fn new(
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
        forced: impl Into<bool>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            forced: forced.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForceStartingStyleMethod {
    #[serde(rename = "CSS.forceStartingStyle")]
    ForceStartingStyle,
}
#[doc = "Ensures that the given node is in its starting-style state.\n[forceStartingStyle](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-forceStartingStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceStartingStyle {
    pub method: ForceStartingStyleMethod,
    pub params: ForceStartingStyleParams,
}
impl ForceStartingStyle {
    pub const IDENTIFIER: &'static str = "CSS.forceStartingStyle";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ForceStartingStyle {
    type Result = super::results::ForceStartingStyleResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBackgroundColorsParams {
    #[doc = "Id of the node to get background colors for."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetBackgroundColorsParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBackgroundColorsMethod {
    #[serde(rename = "CSS.getBackgroundColors")]
    GetBackgroundColors,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBackgroundColors {
    pub method: GetBackgroundColorsMethod,
    pub params: GetBackgroundColorsParams,
}
impl GetBackgroundColors {
    pub const IDENTIFIER: &'static str = "CSS.getBackgroundColors";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetBackgroundColors {
    type Result = super::results::GetBackgroundColorsResult;
}
#[doc = "Returns the computed style for a DOM node identified by `nodeId`.\n[getComputedStyleForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getComputedStyleForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetComputedStyleForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetComputedStyleForNodeParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetComputedStyleForNodeMethod {
    #[serde(rename = "CSS.getComputedStyleForNode")]
    GetComputedStyleForNode,
}
#[doc = "Returns the computed style for a DOM node identified by `nodeId`.\n[getComputedStyleForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getComputedStyleForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetComputedStyleForNode {
    pub method: GetComputedStyleForNodeMethod,
    pub params: GetComputedStyleForNodeParams,
}
impl GetComputedStyleForNode {
    pub const IDENTIFIER: &'static str = "CSS.getComputedStyleForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetComputedStyleForNode {
    type Result = super::results::GetComputedStyleForNodeResult;
}
#[doc = "Resolve the specified values in the context of the provided element.\nFor example, a value of '1em' is evaluated according to the computed\n'font-size' of the element and a value 'calc(1px + 2px)' will be\nresolved to '3px'.\nIf the `propertyName` was specified the `values` are resolved as if\nthey were property's declaration. If a value cannot be parsed according\nto the provided property syntax, the value is parsed using combined\nsyntax as if null `propertyName` was provided. If the value cannot be\nresolved even then, return the provided value without any changes.\nNote: this function currently does not resolve CSS random() function,\nit returns unmodified random() function parts.`\n[resolveValues](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-resolveValues)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveValuesParams {
    #[doc = "Cascade-dependent keywords (revert/revert-layer) do not work."]
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "Id of the node in whose context the expression is evaluated"]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
    #[doc = "Only longhands and custom property names are accepted."]
    #[serde(rename = "propertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub property_name: Option<String>,
    #[doc = "Pseudo element type, only works for pseudo elements that generate\nelements in the tree, such as ::before and ::after."]
    #[serde(rename = "pseudoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_type: Option<crate::browser_protocol::dom::types::PseudoType>,
    #[doc = "Pseudo element custom ident."]
    #[serde(rename = "pseudoIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_identifier: Option<String>,
}
impl ResolveValuesParams {
    pub fn new(
        values: Vec<String>,
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
    ) -> Self {
        Self {
            values,
            node_id: node_id.into(),
            property_name: None,
            pseudo_type: None,
            pseudo_identifier: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolveValuesMethod {
    #[serde(rename = "CSS.resolveValues")]
    ResolveValues,
}
#[doc = "Resolve the specified values in the context of the provided element.\nFor example, a value of '1em' is evaluated according to the computed\n'font-size' of the element and a value 'calc(1px + 2px)' will be\nresolved to '3px'.\nIf the `propertyName` was specified the `values` are resolved as if\nthey were property's declaration. If a value cannot be parsed according\nto the provided property syntax, the value is parsed using combined\nsyntax as if null `propertyName` was provided. If the value cannot be\nresolved even then, return the provided value without any changes.\nNote: this function currently does not resolve CSS random() function,\nit returns unmodified random() function parts.`\n[resolveValues](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-resolveValues)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveValues {
    pub method: ResolveValuesMethod,
    pub params: ResolveValuesParams,
}
impl ResolveValues {
    pub const IDENTIFIER: &'static str = "CSS.resolveValues";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ResolveValues {
    type Result = super::results::ResolveValuesResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLonghandPropertiesParams {
    #[serde(rename = "shorthandName")]
    pub shorthand_name: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl GetLonghandPropertiesParams {
    pub fn new(shorthand_name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            shorthand_name: shorthand_name.into(),
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetLonghandPropertiesMethod {
    #[serde(rename = "CSS.getLonghandProperties")]
    GetLonghandProperties,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLonghandProperties {
    pub method: GetLonghandPropertiesMethod,
    pub params: GetLonghandPropertiesParams,
}
impl GetLonghandProperties {
    pub const IDENTIFIER: &'static str = "CSS.getLonghandProperties";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetLonghandProperties {
    type Result = super::results::GetLonghandPropertiesResult;
}
#[doc = "Returns the styles defined inline (explicitly in the \"style\" attribute and implicitly, using DOM\nattributes) for a DOM node identified by `nodeId`.\n[getInlineStylesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getInlineStylesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInlineStylesForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetInlineStylesForNodeParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetInlineStylesForNodeMethod {
    #[serde(rename = "CSS.getInlineStylesForNode")]
    GetInlineStylesForNode,
}
#[doc = "Returns the styles defined inline (explicitly in the \"style\" attribute and implicitly, using DOM\nattributes) for a DOM node identified by `nodeId`.\n[getInlineStylesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getInlineStylesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInlineStylesForNode {
    pub method: GetInlineStylesForNodeMethod,
    pub params: GetInlineStylesForNodeParams,
}
impl GetInlineStylesForNode {
    pub const IDENTIFIER: &'static str = "CSS.getInlineStylesForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetInlineStylesForNode {
    type Result = super::results::GetInlineStylesForNodeResult;
}
#[doc = "Returns the styles coming from animations & transitions\nincluding the animation & transition styles coming from inheritance chain.\n[getAnimatedStylesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getAnimatedStylesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnimatedStylesForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetAnimatedStylesForNodeParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAnimatedStylesForNodeMethod {
    #[serde(rename = "CSS.getAnimatedStylesForNode")]
    GetAnimatedStylesForNode,
}
#[doc = "Returns the styles coming from animations & transitions\nincluding the animation & transition styles coming from inheritance chain.\n[getAnimatedStylesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getAnimatedStylesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnimatedStylesForNode {
    pub method: GetAnimatedStylesForNodeMethod,
    pub params: GetAnimatedStylesForNodeParams,
}
impl GetAnimatedStylesForNode {
    pub const IDENTIFIER: &'static str = "CSS.getAnimatedStylesForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetAnimatedStylesForNode {
    type Result = super::results::GetAnimatedStylesForNodeResult;
}
#[doc = "Returns requested styles for a DOM node identified by `nodeId`.\n[getMatchedStylesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getMatchedStylesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMatchedStylesForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetMatchedStylesForNodeParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetMatchedStylesForNodeMethod {
    #[serde(rename = "CSS.getMatchedStylesForNode")]
    GetMatchedStylesForNode,
}
#[doc = "Returns requested styles for a DOM node identified by `nodeId`.\n[getMatchedStylesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getMatchedStylesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMatchedStylesForNode {
    pub method: GetMatchedStylesForNodeMethod,
    pub params: GetMatchedStylesForNodeParams,
}
impl GetMatchedStylesForNode {
    pub const IDENTIFIER: &'static str = "CSS.getMatchedStylesForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetMatchedStylesForNode {
    type Result = super::results::GetMatchedStylesForNodeResult;
}
#[doc = "Returns the values of the default UA-defined environment variables used in env()\n[getEnvironmentVariables](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getEnvironmentVariables)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentVariablesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetEnvironmentVariablesMethod {
    #[serde(rename = "CSS.getEnvironmentVariables")]
    GetEnvironmentVariables,
}
#[doc = "Returns the values of the default UA-defined environment variables used in env()\n[getEnvironmentVariables](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getEnvironmentVariables)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentVariables {
    pub method: GetEnvironmentVariablesMethod,
    pub params: GetEnvironmentVariablesParams,
}
impl GetEnvironmentVariables {
    pub const IDENTIFIER: &'static str = "CSS.getEnvironmentVariables";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetEnvironmentVariables {
    type Result = super::results::GetEnvironmentVariablesResult;
}
#[doc = "Returns all media queries parsed by the rendering engine.\n[getMediaQueries](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getMediaQueries)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMediaQueriesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetMediaQueriesMethod {
    #[serde(rename = "CSS.getMediaQueries")]
    GetMediaQueries,
}
#[doc = "Returns all media queries parsed by the rendering engine.\n[getMediaQueries](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getMediaQueries)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMediaQueries {
    pub method: GetMediaQueriesMethod,
    pub params: GetMediaQueriesParams,
}
impl GetMediaQueries {
    pub const IDENTIFIER: &'static str = "CSS.getMediaQueries";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetMediaQueries {
    type Result = super::results::GetMediaQueriesResult;
}
#[doc = "Requests information about platform fonts which we used to render child TextNodes in the given\nnode.\n[getPlatformFontsForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getPlatformFontsForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlatformFontsForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetPlatformFontsForNodeParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetPlatformFontsForNodeMethod {
    #[serde(rename = "CSS.getPlatformFontsForNode")]
    GetPlatformFontsForNode,
}
#[doc = "Requests information about platform fonts which we used to render child TextNodes in the given\nnode.\n[getPlatformFontsForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getPlatformFontsForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlatformFontsForNode {
    pub method: GetPlatformFontsForNodeMethod,
    pub params: GetPlatformFontsForNodeParams,
}
impl GetPlatformFontsForNode {
    pub const IDENTIFIER: &'static str = "CSS.getPlatformFontsForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetPlatformFontsForNode {
    type Result = super::results::GetPlatformFontsForNodeResult;
}
#[doc = "Returns the current textual content for a stylesheet.\n[getStyleSheetText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getStyleSheetText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStyleSheetTextParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
}
impl GetStyleSheetTextParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetStyleSheetTextMethod {
    #[serde(rename = "CSS.getStyleSheetText")]
    GetStyleSheetText,
}
#[doc = "Returns the current textual content for a stylesheet.\n[getStyleSheetText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getStyleSheetText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStyleSheetText {
    pub method: GetStyleSheetTextMethod,
    pub params: GetStyleSheetTextParams,
}
impl GetStyleSheetText {
    pub const IDENTIFIER: &'static str = "CSS.getStyleSheetText";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetStyleSheetText {
    type Result = super::results::GetStyleSheetTextResult;
}
#[doc = "Returns all layers parsed by the rendering engine for the tree scope of a node.\nGiven a DOM element identified by nodeId, getLayersForNode returns the root\nlayer for the nearest ancestor document or shadow root. The layer root contains\nthe full layer tree for the tree scope and their ordering.\n[getLayersForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getLayersForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
impl GetLayersForNodeParams {
    pub fn new(node_id: impl Into<crate::browser_protocol::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetLayersForNodeMethod {
    #[serde(rename = "CSS.getLayersForNode")]
    GetLayersForNode,
}
#[doc = "Returns all layers parsed by the rendering engine for the tree scope of a node.\nGiven a DOM element identified by nodeId, getLayersForNode returns the root\nlayer for the nearest ancestor document or shadow root. The layer root contains\nthe full layer tree for the tree scope and their ordering.\n[getLayersForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getLayersForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersForNode {
    pub method: GetLayersForNodeMethod,
    pub params: GetLayersForNodeParams,
}
impl GetLayersForNode {
    pub const IDENTIFIER: &'static str = "CSS.getLayersForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetLayersForNode {
    type Result = super::results::GetLayersForNodeResult;
}
#[doc = "Given a CSS selector text and a style sheet ID, getLocationForSelector\nreturns an array of locations of the CSS selector in the style sheet.\n[getLocationForSelector](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getLocationForSelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLocationForSelectorParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "selectorText")]
    pub selector_text: String,
}
impl GetLocationForSelectorParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        selector_text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            selector_text: selector_text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetLocationForSelectorMethod {
    #[serde(rename = "CSS.getLocationForSelector")]
    GetLocationForSelector,
}
#[doc = "Given a CSS selector text and a style sheet ID, getLocationForSelector\nreturns an array of locations of the CSS selector in the style sheet.\n[getLocationForSelector](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-getLocationForSelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLocationForSelector {
    pub method: GetLocationForSelectorMethod,
    pub params: GetLocationForSelectorParams,
}
impl GetLocationForSelector {
    pub const IDENTIFIER: &'static str = "CSS.getLocationForSelector";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetLocationForSelector {
    type Result = super::results::GetLocationForSelectorResult;
}
#[doc = "Starts tracking the given node for the computed style updates\nand whenever the computed style is updated for node, it queues\na `computedStyleUpdated` event with throttling.\nThere can only be 1 node tracked for computed style updates\nso passing a new node id removes tracking from the previous node.\nPass `undefined` to disable tracking.\n[trackComputedStyleUpdatesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-trackComputedStyleUpdatesForNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackComputedStyleUpdatesForNodeParams {
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<crate::browser_protocol::dom::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackComputedStyleUpdatesForNodeMethod {
    #[serde(rename = "CSS.trackComputedStyleUpdatesForNode")]
    TrackComputedStyleUpdatesForNode,
}
#[doc = "Starts tracking the given node for the computed style updates\nand whenever the computed style is updated for node, it queues\na `computedStyleUpdated` event with throttling.\nThere can only be 1 node tracked for computed style updates\nso passing a new node id removes tracking from the previous node.\nPass `undefined` to disable tracking.\n[trackComputedStyleUpdatesForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-trackComputedStyleUpdatesForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackComputedStyleUpdatesForNode {
    pub method: TrackComputedStyleUpdatesForNodeMethod,
    pub params: TrackComputedStyleUpdatesForNodeParams,
}
impl TrackComputedStyleUpdatesForNode {
    pub const IDENTIFIER: &'static str = "CSS.trackComputedStyleUpdatesForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TrackComputedStyleUpdatesForNode {
    type Result = super::results::TrackComputedStyleUpdatesForNodeResult;
}
#[doc = "Starts tracking the given computed styles for updates. The specified array of properties\nreplaces the one previously specified. Pass empty array to disable tracking.\nUse takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.\nThe changes to computed style properties are only tracked for nodes pushed to the front-end\nby the DOM agent. If no changes to the tracked properties occur after the node has been pushed\nto the front-end, no updates will be issued for the node.\n[trackComputedStyleUpdates](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-trackComputedStyleUpdates)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackComputedStyleUpdatesParams {
    #[serde(rename = "propertiesToTrack")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties_to_track: Vec<super::types::CssComputedStyleProperty>,
}
impl TrackComputedStyleUpdatesParams {
    pub fn new(properties_to_track: Vec<super::types::CssComputedStyleProperty>) -> Self {
        Self {
            properties_to_track,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackComputedStyleUpdatesMethod {
    #[serde(rename = "CSS.trackComputedStyleUpdates")]
    TrackComputedStyleUpdates,
}
#[doc = "Starts tracking the given computed styles for updates. The specified array of properties\nreplaces the one previously specified. Pass empty array to disable tracking.\nUse takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.\nThe changes to computed style properties are only tracked for nodes pushed to the front-end\nby the DOM agent. If no changes to the tracked properties occur after the node has been pushed\nto the front-end, no updates will be issued for the node.\n[trackComputedStyleUpdates](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-trackComputedStyleUpdates)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackComputedStyleUpdates {
    pub method: TrackComputedStyleUpdatesMethod,
    pub params: TrackComputedStyleUpdatesParams,
}
impl TrackComputedStyleUpdates {
    pub const IDENTIFIER: &'static str = "CSS.trackComputedStyleUpdates";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TrackComputedStyleUpdates {
    type Result = super::results::TrackComputedStyleUpdatesResult;
}
#[doc = "Polls the next batch of computed style updates.\n[takeComputedStyleUpdates](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-takeComputedStyleUpdates)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeComputedStyleUpdatesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakeComputedStyleUpdatesMethod {
    #[serde(rename = "CSS.takeComputedStyleUpdates")]
    TakeComputedStyleUpdates,
}
#[doc = "Polls the next batch of computed style updates.\n[takeComputedStyleUpdates](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-takeComputedStyleUpdates)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeComputedStyleUpdates {
    pub method: TakeComputedStyleUpdatesMethod,
    pub params: TakeComputedStyleUpdatesParams,
}
impl TakeComputedStyleUpdates {
    pub const IDENTIFIER: &'static str = "CSS.takeComputedStyleUpdates";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TakeComputedStyleUpdates {
    type Result = super::results::TakeComputedStyleUpdatesResult;
}
#[doc = "Find a rule with the given active property for the given node and set the new value for this\nproperty\n[setEffectivePropertyValueForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setEffectivePropertyValueForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEffectivePropertyValueForNodeParams {
    #[doc = "The element id for which to set property."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl SetEffectivePropertyValueForNodeParams {
    pub fn new(
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
        property_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            property_name: property_name.into(),
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetEffectivePropertyValueForNodeMethod {
    #[serde(rename = "CSS.setEffectivePropertyValueForNode")]
    SetEffectivePropertyValueForNode,
}
#[doc = "Find a rule with the given active property for the given node and set the new value for this\nproperty\n[setEffectivePropertyValueForNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setEffectivePropertyValueForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEffectivePropertyValueForNode {
    pub method: SetEffectivePropertyValueForNodeMethod,
    pub params: SetEffectivePropertyValueForNodeParams,
}
impl SetEffectivePropertyValueForNode {
    pub const IDENTIFIER: &'static str = "CSS.setEffectivePropertyValueForNode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetEffectivePropertyValueForNode {
    type Result = super::results::SetEffectivePropertyValueForNodeResult;
}
#[doc = "Modifies the property rule property name.\n[setPropertyRulePropertyName](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setPropertyRulePropertyName)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPropertyRulePropertyNameParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "propertyName")]
    pub property_name: String,
}
impl SetPropertyRulePropertyNameParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        property_name: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            property_name: property_name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPropertyRulePropertyNameMethod {
    #[serde(rename = "CSS.setPropertyRulePropertyName")]
    SetPropertyRulePropertyName,
}
#[doc = "Modifies the property rule property name.\n[setPropertyRulePropertyName](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setPropertyRulePropertyName)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPropertyRulePropertyName {
    pub method: SetPropertyRulePropertyNameMethod,
    pub params: SetPropertyRulePropertyNameParams,
}
impl SetPropertyRulePropertyName {
    pub const IDENTIFIER: &'static str = "CSS.setPropertyRulePropertyName";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetPropertyRulePropertyName {
    type Result = super::results::SetPropertyRulePropertyNameResult;
}
#[doc = "Modifies the keyframe rule key text.\n[setKeyframeKey](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setKeyframeKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetKeyframeKeyParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "keyText")]
    pub key_text: String,
}
impl SetKeyframeKeyParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        key_text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            key_text: key_text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetKeyframeKeyMethod {
    #[serde(rename = "CSS.setKeyframeKey")]
    SetKeyframeKey,
}
#[doc = "Modifies the keyframe rule key text.\n[setKeyframeKey](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setKeyframeKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetKeyframeKey {
    pub method: SetKeyframeKeyMethod,
    pub params: SetKeyframeKeyParams,
}
impl SetKeyframeKey {
    pub const IDENTIFIER: &'static str = "CSS.setKeyframeKey";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetKeyframeKey {
    type Result = super::results::SetKeyframeKeyResult;
}
#[doc = "Modifies the rule selector.\n[setMediaText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setMediaText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMediaTextParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "text")]
    pub text: String,
}
impl SetMediaTextParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetMediaTextMethod {
    #[serde(rename = "CSS.setMediaText")]
    SetMediaText,
}
#[doc = "Modifies the rule selector.\n[setMediaText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setMediaText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMediaText {
    pub method: SetMediaTextMethod,
    pub params: SetMediaTextParams,
}
impl SetMediaText {
    pub const IDENTIFIER: &'static str = "CSS.setMediaText";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetMediaText {
    type Result = super::results::SetMediaTextResult;
}
#[doc = "Modifies the expression of a container query.\n[setContainerQueryText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setContainerQueryText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetContainerQueryTextParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "text")]
    pub text: String,
}
impl SetContainerQueryTextParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetContainerQueryTextMethod {
    #[serde(rename = "CSS.setContainerQueryText")]
    SetContainerQueryText,
}
#[doc = "Modifies the expression of a container query.\n[setContainerQueryText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setContainerQueryText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetContainerQueryText {
    pub method: SetContainerQueryTextMethod,
    pub params: SetContainerQueryTextParams,
}
impl SetContainerQueryText {
    pub const IDENTIFIER: &'static str = "CSS.setContainerQueryText";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetContainerQueryText {
    type Result = super::results::SetContainerQueryTextResult;
}
#[doc = "Modifies the expression of a supports at-rule.\n[setSupportsText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setSupportsText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSupportsTextParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "text")]
    pub text: String,
}
impl SetSupportsTextParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSupportsTextMethod {
    #[serde(rename = "CSS.setSupportsText")]
    SetSupportsText,
}
#[doc = "Modifies the expression of a supports at-rule.\n[setSupportsText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setSupportsText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSupportsText {
    pub method: SetSupportsTextMethod,
    pub params: SetSupportsTextParams,
}
impl SetSupportsText {
    pub const IDENTIFIER: &'static str = "CSS.setSupportsText";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSupportsText {
    type Result = super::results::SetSupportsTextResult;
}
#[doc = "Modifies the expression of a scope at-rule.\n[setScopeText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setScopeText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScopeTextParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "text")]
    pub text: String,
}
impl SetScopeTextParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetScopeTextMethod {
    #[serde(rename = "CSS.setScopeText")]
    SetScopeText,
}
#[doc = "Modifies the expression of a scope at-rule.\n[setScopeText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setScopeText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScopeText {
    pub method: SetScopeTextMethod,
    pub params: SetScopeTextParams,
}
impl SetScopeText {
    pub const IDENTIFIER: &'static str = "CSS.setScopeText";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetScopeText {
    type Result = super::results::SetScopeTextResult;
}
#[doc = "Modifies the rule selector.\n[setRuleSelector](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setRuleSelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRuleSelectorParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "range")]
    pub range: super::types::SourceRange,
    #[serde(rename = "selector")]
    pub selector: String,
}
impl SetRuleSelectorParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        range: impl Into<super::types::SourceRange>,
        selector: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            selector: selector.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetRuleSelectorMethod {
    #[serde(rename = "CSS.setRuleSelector")]
    SetRuleSelector,
}
#[doc = "Modifies the rule selector.\n[setRuleSelector](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setRuleSelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRuleSelector {
    pub method: SetRuleSelectorMethod,
    pub params: SetRuleSelectorParams,
}
impl SetRuleSelector {
    pub const IDENTIFIER: &'static str = "CSS.setRuleSelector";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetRuleSelector {
    type Result = super::results::SetRuleSelectorResult;
}
#[doc = "Sets the new stylesheet text.\n[setStyleSheetText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setStyleSheetText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStyleSheetTextParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
    #[serde(rename = "text")]
    pub text: String,
}
impl SetStyleSheetTextParams {
    pub fn new(
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetStyleSheetTextMethod {
    #[serde(rename = "CSS.setStyleSheetText")]
    SetStyleSheetText,
}
#[doc = "Sets the new stylesheet text.\n[setStyleSheetText](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setStyleSheetText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStyleSheetText {
    pub method: SetStyleSheetTextMethod,
    pub params: SetStyleSheetTextParams,
}
impl SetStyleSheetText {
    pub const IDENTIFIER: &'static str = "CSS.setStyleSheetText";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetStyleSheetText {
    type Result = super::results::SetStyleSheetTextResult;
}
#[doc = "Applies specified style edits one after another in the given order.\n[setStyleTexts](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setStyleTexts)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStyleTextsParams {
    #[serde(rename = "edits")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub edits: Vec<super::types::StyleDeclarationEdit>,
    #[doc = "NodeId for the DOM node in whose context custom property declarations for registered properties should be\nvalidated. If omitted, declarations in the new rule text can only be validated statically, which may produce\nincorrect results if the declaration contains a var() for example."]
    #[serde(rename = "nodeForPropertySyntaxValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_for_property_syntax_validation: Option<crate::browser_protocol::dom::types::NodeId>,
}
impl SetStyleTextsParams {
    pub fn new(edits: Vec<super::types::StyleDeclarationEdit>) -> Self {
        Self {
            edits,
            node_for_property_syntax_validation: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetStyleTextsMethod {
    #[serde(rename = "CSS.setStyleTexts")]
    SetStyleTexts,
}
#[doc = "Applies specified style edits one after another in the given order.\n[setStyleTexts](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setStyleTexts)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStyleTexts {
    pub method: SetStyleTextsMethod,
    pub params: SetStyleTextsParams,
}
impl SetStyleTexts {
    pub const IDENTIFIER: &'static str = "CSS.setStyleTexts";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetStyleTexts {
    type Result = super::results::SetStyleTextsResult;
}
#[doc = "Enables the selector recording.\n[startRuleUsageTracking](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-startRuleUsageTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartRuleUsageTrackingParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartRuleUsageTrackingMethod {
    #[serde(rename = "CSS.startRuleUsageTracking")]
    StartRuleUsageTracking,
}
#[doc = "Enables the selector recording.\n[startRuleUsageTracking](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-startRuleUsageTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartRuleUsageTracking {
    pub method: StartRuleUsageTrackingMethod,
    pub params: StartRuleUsageTrackingParams,
}
impl StartRuleUsageTracking {
    pub const IDENTIFIER: &'static str = "CSS.startRuleUsageTracking";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartRuleUsageTracking {
    type Result = super::results::StartRuleUsageTrackingResult;
}
#[doc = "Stop tracking rule usage and return the list of rules that were used since last call to\n`takeCoverageDelta` (or since start of coverage instrumentation).\n[stopRuleUsageTracking](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-stopRuleUsageTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRuleUsageTrackingParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopRuleUsageTrackingMethod {
    #[serde(rename = "CSS.stopRuleUsageTracking")]
    StopRuleUsageTracking,
}
#[doc = "Stop tracking rule usage and return the list of rules that were used since last call to\n`takeCoverageDelta` (or since start of coverage instrumentation).\n[stopRuleUsageTracking](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-stopRuleUsageTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRuleUsageTracking {
    pub method: StopRuleUsageTrackingMethod,
    pub params: StopRuleUsageTrackingParams,
}
impl StopRuleUsageTracking {
    pub const IDENTIFIER: &'static str = "CSS.stopRuleUsageTracking";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopRuleUsageTracking {
    type Result = super::results::StopRuleUsageTrackingResult;
}
#[doc = "Obtain list of rules that became used since last call to this method (or since start of coverage\ninstrumentation).\n[takeCoverageDelta](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-takeCoverageDelta)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeCoverageDeltaParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakeCoverageDeltaMethod {
    #[serde(rename = "CSS.takeCoverageDelta")]
    TakeCoverageDelta,
}
#[doc = "Obtain list of rules that became used since last call to this method (or since start of coverage\ninstrumentation).\n[takeCoverageDelta](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-takeCoverageDelta)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeCoverageDelta {
    pub method: TakeCoverageDeltaMethod,
    pub params: TakeCoverageDeltaParams,
}
impl TakeCoverageDelta {
    pub const IDENTIFIER: &'static str = "CSS.takeCoverageDelta";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TakeCoverageDelta {
    type Result = super::results::TakeCoverageDeltaResult;
}
#[doc = "Enables/disables rendering of local CSS fonts (enabled by default).\n[setLocalFontsEnabled](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setLocalFontsEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLocalFontsEnabledParams {
    #[doc = "Whether rendering of local fonts is enabled."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetLocalFontsEnabledParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetLocalFontsEnabledMethod {
    #[serde(rename = "CSS.setLocalFontsEnabled")]
    SetLocalFontsEnabled,
}
#[doc = "Enables/disables rendering of local CSS fonts (enabled by default).\n[setLocalFontsEnabled](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#method-setLocalFontsEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLocalFontsEnabled {
    pub method: SetLocalFontsEnabledMethod,
    pub params: SetLocalFontsEnabledParams,
}
impl SetLocalFontsEnabled {
    pub const IDENTIFIER: &'static str = "CSS.setLocalFontsEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetLocalFontsEnabled {
    type Result = super::results::SetLocalFontsEnabledResult;
}
group_enum ! (CssCommands { AddRule (AddRule) , CollectClassNames (CollectClassNames) , CreateStyleSheet (CreateStyleSheet) , Disable (Disable) , Enable (Enable) , ForcePseudoState (ForcePseudoState) , ForceStartingStyle (ForceStartingStyle) , GetBackgroundColors (GetBackgroundColors) , GetComputedStyleForNode (GetComputedStyleForNode) , ResolveValues (ResolveValues) , GetLonghandProperties (GetLonghandProperties) , GetInlineStylesForNode (GetInlineStylesForNode) , GetAnimatedStylesForNode (GetAnimatedStylesForNode) , GetMatchedStylesForNode (GetMatchedStylesForNode) , GetEnvironmentVariables (GetEnvironmentVariables) , GetMediaQueries (GetMediaQueries) , GetPlatformFontsForNode (GetPlatformFontsForNode) , GetStyleSheetText (GetStyleSheetText) , GetLayersForNode (GetLayersForNode) , GetLocationForSelector (GetLocationForSelector) , TrackComputedStyleUpdatesForNode (TrackComputedStyleUpdatesForNode) , TrackComputedStyleUpdates (TrackComputedStyleUpdates) , TakeComputedStyleUpdates (TakeComputedStyleUpdates) , SetEffectivePropertyValueForNode (SetEffectivePropertyValueForNode) , SetPropertyRulePropertyName (SetPropertyRulePropertyName) , SetKeyframeKey (SetKeyframeKey) , SetMediaText (SetMediaText) , SetContainerQueryText (SetContainerQueryText) , SetSupportsText (SetSupportsText) , SetScopeText (SetScopeText) , SetRuleSelector (SetRuleSelector) , SetStyleSheetText (SetStyleSheetText) , SetStyleTexts (SetStyleTexts) , StartRuleUsageTracking (StartRuleUsageTracking) , StopRuleUsageTracking (StopRuleUsageTracking) , TakeCoverageDelta (TakeCoverageDelta) , SetLocalFontsEnabled (SetLocalFontsEnabled) } + identifiable);
