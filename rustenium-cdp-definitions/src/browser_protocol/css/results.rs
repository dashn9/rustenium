use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddRuleResult {
    #[doc = "The newly created rule."]
    #[serde(rename = "rule")]
    pub rule: super::types::CssRule,
}
impl TryFrom<serde_json::Value> for AddRuleResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesResult {
    #[doc = "Class name list."]
    #[serde(rename = "classNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class_names: Vec<String>,
}
impl TryFrom<serde_json::Value> for CollectClassNamesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateStyleSheetResult {
    #[doc = "Identifier of the created \"via-inspector\" stylesheet."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
}
impl TryFrom<serde_json::Value> for CreateStyleSheetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForcePseudoStateResult {}
impl TryFrom<serde_json::Value> for ForcePseudoStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForceStartingStyleResult {}
impl TryFrom<serde_json::Value> for ForceStartingStyleResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBackgroundColorsResult {
    #[doc = "The range of background colors behind this element, if it contains any visible text. If no\nvisible text is present, this will be undefined. In the case of a flat background color,\nthis will consist of simply that color. In the case of a gradient, this will consist of each\nof the color stops. For anything more complicated, this will be an empty array. Images will\nbe ignored (as if the image had failed to load)."]
    #[serde(rename = "backgroundColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub background_colors: Option<Vec<String>>,
    #[doc = "The computed font size for this node, as a CSS computed value string (e.g. '12px')."]
    #[serde(rename = "computedFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub computed_font_size: Option<String>,
    #[doc = "The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or\n'100')."]
    #[serde(rename = "computedFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub computed_font_weight: Option<String>,
}
impl TryFrom<serde_json::Value> for GetBackgroundColorsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetComputedStyleForNodeResult {
    #[doc = "Computed style for the specified DOM node."]
    #[serde(rename = "computedStyle")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub computed_style: Vec<super::types::CssComputedStyleProperty>,
    #[doc = "A list of non-standard \"extra fields\" which blink stores alongside each\ncomputed style."]
    #[serde(rename = "extraFields")]
    pub extra_fields: super::types::ComputedStyleExtraFields,
}
impl TryFrom<serde_json::Value> for GetComputedStyleForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveValuesResult {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<String>,
}
impl TryFrom<serde_json::Value> for ResolveValuesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLonghandPropertiesResult {
    #[serde(rename = "longhandProperties")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub longhand_properties: Vec<super::types::CssProperty>,
}
impl TryFrom<serde_json::Value> for GetLonghandPropertiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetInlineStylesForNodeResult {
    #[doc = "Inline style for the specified DOM node."]
    #[serde(rename = "inlineStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inline_style: Option<super::types::CssStyle>,
    #[doc = "Attribute-defined element style (e.g. resulting from \"width=20 height=100%\")."]
    #[serde(rename = "attributesStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attributes_style: Option<super::types::CssStyle>,
}
impl TryFrom<serde_json::Value> for GetInlineStylesForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAnimatedStylesForNodeResult {
    #[doc = "Styles coming from animations."]
    #[serde(rename = "animationStyles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub animation_styles: Option<Vec<super::types::CssAnimationStyle>>,
    #[doc = "Style coming from transitions."]
    #[serde(rename = "transitionsStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transitions_style: Option<super::types::CssStyle>,
    #[doc = "Inherited style entries for animationsStyle and transitionsStyle from\nthe inheritance chain of the element."]
    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inherited: Option<Vec<super::types::InheritedAnimatedStyleEntry>>,
}
impl TryFrom<serde_json::Value> for GetAnimatedStylesForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMatchedStylesForNodeResult {
    #[doc = "Inline style for the specified DOM node."]
    #[serde(rename = "inlineStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inline_style: Option<super::types::CssStyle>,
    #[doc = "Attribute-defined element style (e.g. resulting from \"width=20 height=100%\")."]
    #[serde(rename = "attributesStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attributes_style: Option<super::types::CssStyle>,
    #[doc = "CSS rules matching this node, from all applicable stylesheets."]
    #[serde(rename = "matchedCSSRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub matched_css_rules: Option<Vec<super::types::RuleMatch>>,
    #[doc = "Pseudo style matches for this node."]
    #[serde(rename = "pseudoElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_elements: Option<Vec<super::types::PseudoElementMatches>>,
    #[doc = "A chain of inherited styles (from the immediate node parent up to the DOM tree root)."]
    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inherited: Option<Vec<super::types::InheritedStyleEntry>>,
    #[doc = "A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root)."]
    #[serde(rename = "inheritedPseudoElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inherited_pseudo_elements: Option<Vec<super::types::InheritedPseudoElementMatches>>,
    #[doc = "A list of CSS keyframed animations matching this node."]
    #[serde(rename = "cssKeyframesRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_keyframes_rules: Option<Vec<super::types::CssKeyframesRule>>,
    #[doc = "A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property."]
    #[serde(rename = "cssPositionTryRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_position_try_rules: Option<Vec<super::types::CssPositionTryRule>>,
    #[doc = "Index of the active fallback in the applied position-try-fallback property,\nwill not be set if there is no active position-try fallback."]
    #[serde(rename = "activePositionFallbackIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub active_position_fallback_index: Option<i64>,
    #[doc = "A list of CSS at-property rules matching this node."]
    #[serde(rename = "cssPropertyRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_property_rules: Option<Vec<super::types::CssPropertyRule>>,
    #[doc = "A list of CSS property registrations matching this node."]
    #[serde(rename = "cssPropertyRegistrations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_property_registrations: Option<Vec<super::types::CssPropertyRegistration>>,
    #[doc = "A list of simple @rules matching this node or its pseudo-elements."]
    #[serde(rename = "cssAtRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_at_rules: Option<Vec<super::types::CssAtRule>>,
    #[doc = "Id of the first parent element that does not have display: contents."]
    #[serde(rename = "parentLayoutNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_layout_node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    #[doc = "A list of CSS at-function rules referenced by styles of this node."]
    #[serde(rename = "cssFunctionRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_function_rules: Option<Vec<super::types::CssFunctionRule>>,
}
impl TryFrom<serde_json::Value> for GetMatchedStylesForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentVariablesResult {
    #[serde(rename = "environmentVariables")]
    pub environment_variables: serde_json::Value,
}
impl TryFrom<serde_json::Value> for GetEnvironmentVariablesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMediaQueriesResult {
    #[serde(rename = "medias")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub medias: Vec<super::types::CssMedia>,
}
impl TryFrom<serde_json::Value> for GetMediaQueriesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlatformFontsForNodeResult {
    #[doc = "Usage statistics for every employed platform font."]
    #[serde(rename = "fonts")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fonts: Vec<super::types::PlatformFontUsage>,
}
impl TryFrom<serde_json::Value> for GetPlatformFontsForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStyleSheetTextResult {
    #[doc = "The stylesheet text."]
    #[serde(rename = "text")]
    pub text: String,
}
impl TryFrom<serde_json::Value> for GetStyleSheetTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersForNodeResult {
    #[serde(rename = "rootLayer")]
    pub root_layer: super::types::CssLayerData,
}
impl TryFrom<serde_json::Value> for GetLayersForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLocationForSelectorResult {
    #[serde(rename = "ranges")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<super::types::SourceRange>,
}
impl TryFrom<serde_json::Value> for GetLocationForSelectorResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackComputedStyleUpdatesForNodeResult {}
impl TryFrom<serde_json::Value> for TrackComputedStyleUpdatesForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackComputedStyleUpdatesResult {}
impl TryFrom<serde_json::Value> for TrackComputedStyleUpdatesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeComputedStyleUpdatesResult {
    #[doc = "The list of node Ids that have their tracked computed styles updated."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<crate::browser_protocol::dom::types::NodeId>,
}
impl TryFrom<serde_json::Value> for TakeComputedStyleUpdatesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEffectivePropertyValueForNodeResult {}
impl TryFrom<serde_json::Value> for SetEffectivePropertyValueForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPropertyRulePropertyNameResult {
    #[doc = "The resulting key text after modification."]
    #[serde(rename = "propertyName")]
    pub property_name: super::types::Value,
}
impl TryFrom<serde_json::Value> for SetPropertyRulePropertyNameResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetKeyframeKeyResult {
    #[doc = "The resulting key text after modification."]
    #[serde(rename = "keyText")]
    pub key_text: super::types::Value,
}
impl TryFrom<serde_json::Value> for SetKeyframeKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMediaTextResult {
    #[doc = "The resulting CSS media rule after modification."]
    #[serde(rename = "media")]
    pub media: super::types::CssMedia,
}
impl TryFrom<serde_json::Value> for SetMediaTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetContainerQueryTextResult {
    #[doc = "The resulting CSS container query rule after modification."]
    #[serde(rename = "containerQuery")]
    pub container_query: super::types::CssContainerQuery,
}
impl TryFrom<serde_json::Value> for SetContainerQueryTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSupportsTextResult {
    #[doc = "The resulting CSS Supports rule after modification."]
    #[serde(rename = "supports")]
    pub supports: super::types::CssSupports,
}
impl TryFrom<serde_json::Value> for SetSupportsTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScopeTextResult {
    #[doc = "The resulting CSS Scope rule after modification."]
    #[serde(rename = "scope")]
    pub scope: super::types::CssScope,
}
impl TryFrom<serde_json::Value> for SetScopeTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRuleSelectorResult {
    #[doc = "The resulting selector list after modification."]
    #[serde(rename = "selectorList")]
    pub selector_list: super::types::SelectorList,
}
impl TryFrom<serde_json::Value> for SetRuleSelectorResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetStyleSheetTextResult {
    #[doc = "URL of source map associated with script (if any)."]
    #[serde(rename = "sourceMapURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_map_url: Option<String>,
}
impl TryFrom<serde_json::Value> for SetStyleSheetTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStyleTextsResult {
    #[doc = "The resulting styles after modification."]
    #[serde(rename = "styles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub styles: Vec<super::types::CssStyle>,
}
impl TryFrom<serde_json::Value> for SetStyleTextsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartRuleUsageTrackingResult {}
impl TryFrom<serde_json::Value> for StartRuleUsageTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRuleUsageTrackingResult {
    #[serde(rename = "ruleUsage")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_usage: Vec<super::types::RuleUsage>,
}
impl TryFrom<serde_json::Value> for StopRuleUsageTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeCoverageDeltaResult {
    #[serde(rename = "coverage")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<super::types::RuleUsage>,
    #[doc = "Monotonically increasing time, in seconds."]
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}
impl TryFrom<serde_json::Value> for TakeCoverageDeltaResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetLocalFontsEnabledResult {}
impl TryFrom<serde_json::Value> for SetLocalFontsEnabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
