use super::commands::*;
impl AddRule {
    pub fn builder() -> AddRuleBuilder {
        AddRuleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AddRuleBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    rule_text: Option<String>,
    location: Option<super::types::SourceRange>,
    node_for_property_syntax_validation: Option<super::super::dom::types::NodeId>,
}
impl AddRuleBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn rule_text(mut self, rule_text: impl Into<String>) -> Self {
        self.rule_text = Some(rule_text.into());
        self
    }
    pub fn location(mut self, location: impl Into<super::types::SourceRange>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn node_for_property_syntax_validation(
        mut self,
        node_for_property_syntax_validation: impl Into<super::super::dom::types::NodeId>,
    ) -> Self {
        self.node_for_property_syntax_validation = Some(node_for_property_syntax_validation.into());
        self
    }
    pub fn build(self) -> Result<AddRule, String> {
        Ok(AddRule {
            method: AddRuleMethod::AddRule,
            params: AddRuleParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                rule_text: self.rule_text.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(rule_text))
                })?,
                location: self.location.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(location))
                })?,
                node_for_property_syntax_validation: self.node_for_property_syntax_validation,
            },
        })
    }
}
impl CollectClassNames {
    pub fn builder() -> CollectClassNamesBuilder {
        CollectClassNamesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CollectClassNamesBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
}
impl CollectClassNamesBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn build(self) -> Result<CollectClassNames, String> {
        Ok(CollectClassNames {
            method: CollectClassNamesMethod::CollectClassNames,
            params: CollectClassNamesParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
            },
        })
    }
}
impl CreateStyleSheet {
    pub fn builder() -> CreateStyleSheetBuilder {
        CreateStyleSheetBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CreateStyleSheetBuilder {
    frame_id: Option<super::super::page::types::FrameId>,
    force: Option<bool>,
}
impl CreateStyleSheetBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn force(mut self, force: impl Into<bool>) -> Self {
        self.force = Some(force.into());
        self
    }
    pub fn build(self) -> Result<CreateStyleSheet, String> {
        Ok(CreateStyleSheet {
            method: CreateStyleSheetMethod::CreateStyleSheet,
            params: CreateStyleSheetParams {
                frame_id: self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?,
                force: self.force,
            },
        })
    }
}
impl ForcePseudoState {
    pub fn builder() -> ForcePseudoStateBuilder {
        ForcePseudoStateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ForcePseudoStateBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
    forced_pseudo_classes: Option<Vec<String>>,
}
impl ForcePseudoStateBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn forced_pseudo_classe(mut self, forced_pseudo_classe: impl Into<String>) -> Self {
        let v = self.forced_pseudo_classes.get_or_insert(Vec::new());
        v.push(forced_pseudo_classe.into());
        self
    }
    pub fn forced_pseudo_classes<I, S>(mut self, forced_pseudo_classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.forced_pseudo_classes.get_or_insert(Vec::new());
        for val in forced_pseudo_classes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ForcePseudoState, String> {
        Ok(ForcePseudoState {
            method: ForcePseudoStateMethod::ForcePseudoState,
            params: ForcePseudoStateParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                forced_pseudo_classes: self.forced_pseudo_classes.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(forced_pseudo_classes)
                    )
                })?,
            },
        })
    }
}
impl ForceStartingStyle {
    pub fn builder() -> ForceStartingStyleBuilder {
        ForceStartingStyleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ForceStartingStyleBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
    forced: Option<bool>,
}
impl ForceStartingStyleBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn forced(mut self, forced: impl Into<bool>) -> Self {
        self.forced = Some(forced.into());
        self
    }
    pub fn build(self) -> Result<ForceStartingStyle, String> {
        Ok(ForceStartingStyle {
            method: ForceStartingStyleMethod::ForceStartingStyle,
            params: ForceStartingStyleParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                forced: self
                    .forced
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(forced)))?,
            },
        })
    }
}
impl GetBackgroundColors {
    pub fn builder() -> GetBackgroundColorsBuilder {
        GetBackgroundColorsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetBackgroundColorsBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetBackgroundColorsBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetBackgroundColors, String> {
        Ok(GetBackgroundColors {
            method: GetBackgroundColorsMethod::GetBackgroundColors,
            params: GetBackgroundColorsParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetComputedStyleForNode {
    pub fn builder() -> GetComputedStyleForNodeBuilder {
        GetComputedStyleForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetComputedStyleForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetComputedStyleForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetComputedStyleForNode, String> {
        Ok(GetComputedStyleForNode {
            method: GetComputedStyleForNodeMethod::GetComputedStyleForNode,
            params: GetComputedStyleForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl ResolveValues {
    pub fn builder() -> ResolveValuesBuilder {
        ResolveValuesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ResolveValuesBuilder {
    values: Option<Vec<String>>,
    node_id: Option<super::super::dom::types::NodeId>,
    property_name: Option<String>,
    pseudo_type: Option<super::super::dom::types::PseudoType>,
    pseudo_identifier: Option<String>,
}
impl ResolveValuesBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let v = self.values.get_or_insert(Vec::new());
        v.push(value.into());
        self
    }
    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.values.get_or_insert(Vec::new());
        for val in values {
            v.push(val.into());
        }
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = Some(property_name.into());
        self
    }
    pub fn pseudo_type(
        mut self,
        pseudo_type: impl Into<super::super::dom::types::PseudoType>,
    ) -> Self {
        self.pseudo_type = Some(pseudo_type.into());
        self
    }
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<String>) -> Self {
        self.pseudo_identifier = Some(pseudo_identifier.into());
        self
    }
    pub fn build(self) -> Result<ResolveValues, String> {
        Ok(ResolveValues {
            method: ResolveValuesMethod::ResolveValues,
            params: ResolveValuesParams {
                values: self
                    .values
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(values)))?,
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                property_name: self.property_name,
                pseudo_type: self.pseudo_type,
                pseudo_identifier: self.pseudo_identifier,
            },
        })
    }
}
impl GetLonghandProperties {
    pub fn builder() -> GetLonghandPropertiesBuilder {
        GetLonghandPropertiesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetLonghandPropertiesBuilder {
    shorthand_name: Option<String>,
    value: Option<String>,
}
impl GetLonghandPropertiesBuilder {
    pub fn shorthand_name(mut self, shorthand_name: impl Into<String>) -> Self {
        self.shorthand_name = Some(shorthand_name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<GetLonghandProperties, String> {
        Ok(GetLonghandProperties {
            method: GetLonghandPropertiesMethod::GetLonghandProperties,
            params: GetLonghandPropertiesParams {
                shorthand_name: self.shorthand_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(shorthand_name))
                })?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            },
        })
    }
}
impl GetInlineStylesForNode {
    pub fn builder() -> GetInlineStylesForNodeBuilder {
        GetInlineStylesForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetInlineStylesForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetInlineStylesForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetInlineStylesForNode, String> {
        Ok(GetInlineStylesForNode {
            method: GetInlineStylesForNodeMethod::GetInlineStylesForNode,
            params: GetInlineStylesForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetAnimatedStylesForNode {
    pub fn builder() -> GetAnimatedStylesForNodeBuilder {
        GetAnimatedStylesForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAnimatedStylesForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetAnimatedStylesForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetAnimatedStylesForNode, String> {
        Ok(GetAnimatedStylesForNode {
            method: GetAnimatedStylesForNodeMethod::GetAnimatedStylesForNode,
            params: GetAnimatedStylesForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetMatchedStylesForNode {
    pub fn builder() -> GetMatchedStylesForNodeBuilder {
        GetMatchedStylesForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetMatchedStylesForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetMatchedStylesForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetMatchedStylesForNode, String> {
        Ok(GetMatchedStylesForNode {
            method: GetMatchedStylesForNodeMethod::GetMatchedStylesForNode,
            params: GetMatchedStylesForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetPlatformFontsForNode {
    pub fn builder() -> GetPlatformFontsForNodeBuilder {
        GetPlatformFontsForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetPlatformFontsForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetPlatformFontsForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetPlatformFontsForNode, String> {
        Ok(GetPlatformFontsForNode {
            method: GetPlatformFontsForNodeMethod::GetPlatformFontsForNode,
            params: GetPlatformFontsForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetStyleSheetText {
    pub fn builder() -> GetStyleSheetTextBuilder {
        GetStyleSheetTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetStyleSheetTextBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
}
impl GetStyleSheetTextBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn build(self) -> Result<GetStyleSheetText, String> {
        Ok(GetStyleSheetText {
            method: GetStyleSheetTextMethod::GetStyleSheetText,
            params: GetStyleSheetTextParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
            },
        })
    }
}
impl GetLayersForNode {
    pub fn builder() -> GetLayersForNodeBuilder {
        GetLayersForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetLayersForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl GetLayersForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<GetLayersForNode, String> {
        Ok(GetLayersForNode {
            method: GetLayersForNodeMethod::GetLayersForNode,
            params: GetLayersForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            },
        })
    }
}
impl GetLocationForSelector {
    pub fn builder() -> GetLocationForSelectorBuilder {
        GetLocationForSelectorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetLocationForSelectorBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    selector_text: Option<String>,
}
impl GetLocationForSelectorBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn selector_text(mut self, selector_text: impl Into<String>) -> Self {
        self.selector_text = Some(selector_text.into());
        self
    }
    pub fn build(self) -> Result<GetLocationForSelector, String> {
        Ok(GetLocationForSelector {
            method: GetLocationForSelectorMethod::GetLocationForSelector,
            params: GetLocationForSelectorParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                selector_text: self.selector_text.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(selector_text))
                })?,
            },
        })
    }
}
impl TrackComputedStyleUpdatesForNode {
    pub fn builder() -> TrackComputedStyleUpdatesForNodeBuilder {
        TrackComputedStyleUpdatesForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TrackComputedStyleUpdatesForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
}
impl TrackComputedStyleUpdatesForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> TrackComputedStyleUpdatesForNode {
        TrackComputedStyleUpdatesForNode {
            method: TrackComputedStyleUpdatesForNodeMethod::TrackComputedStyleUpdatesForNode,
            params: TrackComputedStyleUpdatesForNodeParams {
                node_id: self.node_id,
            },
        }
    }
}
impl TrackComputedStyleUpdates {
    pub fn builder() -> TrackComputedStyleUpdatesBuilder {
        TrackComputedStyleUpdatesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TrackComputedStyleUpdatesBuilder {
    properties_to_track: Option<Vec<super::types::CssComputedStyleProperty>>,
}
impl TrackComputedStyleUpdatesBuilder {
    pub fn properties_to_track(
        mut self,
        properties_to_track: impl Into<super::types::CssComputedStyleProperty>,
    ) -> Self {
        let v = self.properties_to_track.get_or_insert(Vec::new());
        v.push(properties_to_track.into());
        self
    }
    pub fn properties_to_tracks<I, S>(mut self, properties_to_tracks: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CssComputedStyleProperty>,
    {
        let v = self.properties_to_track.get_or_insert(Vec::new());
        for val in properties_to_tracks {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<TrackComputedStyleUpdates, String> {
        Ok(TrackComputedStyleUpdates {
            method: TrackComputedStyleUpdatesMethod::TrackComputedStyleUpdates,
            params: TrackComputedStyleUpdatesParams {
                properties_to_track: self.properties_to_track.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(properties_to_track)
                    )
                })?,
            },
        })
    }
}
impl SetEffectivePropertyValueForNode {
    pub fn builder() -> SetEffectivePropertyValueForNodeBuilder {
        SetEffectivePropertyValueForNodeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetEffectivePropertyValueForNodeBuilder {
    node_id: Option<super::super::dom::types::NodeId>,
    property_name: Option<String>,
    value: Option<String>,
}
impl SetEffectivePropertyValueForNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = Some(property_name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SetEffectivePropertyValueForNode, String> {
        Ok(SetEffectivePropertyValueForNode {
            method: SetEffectivePropertyValueForNodeMethod::SetEffectivePropertyValueForNode,
            params: SetEffectivePropertyValueForNodeParams {
                node_id: self
                    .node_id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
                property_name: self.property_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(property_name))
                })?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            },
        })
    }
}
impl SetPropertyRulePropertyName {
    pub fn builder() -> SetPropertyRulePropertyNameBuilder {
        SetPropertyRulePropertyNameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPropertyRulePropertyNameBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    property_name: Option<String>,
}
impl SetPropertyRulePropertyNameBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = Some(property_name.into());
        self
    }
    pub fn build(self) -> Result<SetPropertyRulePropertyName, String> {
        Ok(SetPropertyRulePropertyName {
            method: SetPropertyRulePropertyNameMethod::SetPropertyRulePropertyName,
            params: SetPropertyRulePropertyNameParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                property_name: self.property_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(property_name))
                })?,
            },
        })
    }
}
impl SetKeyframeKey {
    pub fn builder() -> SetKeyframeKeyBuilder {
        SetKeyframeKeyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetKeyframeKeyBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    key_text: Option<String>,
}
impl SetKeyframeKeyBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn key_text(mut self, key_text: impl Into<String>) -> Self {
        self.key_text = Some(key_text.into());
        self
    }
    pub fn build(self) -> Result<SetKeyframeKey, String> {
        Ok(SetKeyframeKey {
            method: SetKeyframeKeyMethod::SetKeyframeKey,
            params: SetKeyframeKeyParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                key_text: self.key_text.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(key_text))
                })?,
            },
        })
    }
}
impl SetMediaText {
    pub fn builder() -> SetMediaTextBuilder {
        SetMediaTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetMediaTextBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    text: Option<String>,
}
impl SetMediaTextBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<SetMediaText, String> {
        Ok(SetMediaText {
            method: SetMediaTextMethod::SetMediaText,
            params: SetMediaTextParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            },
        })
    }
}
impl SetContainerQueryText {
    pub fn builder() -> SetContainerQueryTextBuilder {
        SetContainerQueryTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetContainerQueryTextBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    text: Option<String>,
}
impl SetContainerQueryTextBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<SetContainerQueryText, String> {
        Ok(SetContainerQueryText {
            method: SetContainerQueryTextMethod::SetContainerQueryText,
            params: SetContainerQueryTextParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            },
        })
    }
}
impl SetSupportsText {
    pub fn builder() -> SetSupportsTextBuilder {
        SetSupportsTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSupportsTextBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    text: Option<String>,
}
impl SetSupportsTextBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<SetSupportsText, String> {
        Ok(SetSupportsText {
            method: SetSupportsTextMethod::SetSupportsText,
            params: SetSupportsTextParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            },
        })
    }
}
impl SetScopeText {
    pub fn builder() -> SetScopeTextBuilder {
        SetScopeTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetScopeTextBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    text: Option<String>,
}
impl SetScopeTextBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<SetScopeText, String> {
        Ok(SetScopeText {
            method: SetScopeTextMethod::SetScopeText,
            params: SetScopeTextParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            },
        })
    }
}
impl SetRuleSelector {
    pub fn builder() -> SetRuleSelectorBuilder {
        SetRuleSelectorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetRuleSelectorBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    range: Option<super::types::SourceRange>,
    selector: Option<String>,
}
impl SetRuleSelectorBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<super::types::SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }
    pub fn build(self) -> Result<SetRuleSelector, String> {
        Ok(SetRuleSelector {
            method: SetRuleSelectorMethod::SetRuleSelector,
            params: SetRuleSelectorParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                range: self
                    .range
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
                selector: self.selector.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(selector))
                })?,
            },
        })
    }
}
impl SetStyleSheetText {
    pub fn builder() -> SetStyleSheetTextBuilder {
        SetStyleSheetTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetStyleSheetTextBuilder {
    style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    text: Option<String>,
}
impl SetStyleSheetTextBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<SetStyleSheetText, String> {
        Ok(SetStyleSheetText {
            method: SetStyleSheetTextMethod::SetStyleSheetText,
            params: SetStyleSheetTextParams {
                style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
                })?,
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            },
        })
    }
}
impl SetStyleTexts {
    pub fn builder() -> SetStyleTextsBuilder {
        SetStyleTextsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetStyleTextsBuilder {
    edits: Option<Vec<super::types::StyleDeclarationEdit>>,
    node_for_property_syntax_validation: Option<super::super::dom::types::NodeId>,
}
impl SetStyleTextsBuilder {
    pub fn edit(mut self, edit: impl Into<super::types::StyleDeclarationEdit>) -> Self {
        let v = self.edits.get_or_insert(Vec::new());
        v.push(edit.into());
        self
    }
    pub fn edits<I, S>(mut self, edits: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::StyleDeclarationEdit>,
    {
        let v = self.edits.get_or_insert(Vec::new());
        for val in edits {
            v.push(val.into());
        }
        self
    }
    pub fn node_for_property_syntax_validation(
        mut self,
        node_for_property_syntax_validation: impl Into<super::super::dom::types::NodeId>,
    ) -> Self {
        self.node_for_property_syntax_validation = Some(node_for_property_syntax_validation.into());
        self
    }
    pub fn build(self) -> Result<SetStyleTexts, String> {
        Ok(SetStyleTexts {
            method: SetStyleTextsMethod::SetStyleTexts,
            params: SetStyleTextsParams {
                edits: self
                    .edits
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(edits)))?,
                node_for_property_syntax_validation: self.node_for_property_syntax_validation,
            },
        })
    }
}
impl SetLocalFontsEnabled {
    pub fn builder() -> SetLocalFontsEnabledBuilder {
        SetLocalFontsEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetLocalFontsEnabledBuilder {
    enabled: Option<bool>,
}
impl SetLocalFontsEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetLocalFontsEnabled, String> {
        Ok(SetLocalFontsEnabled {
            method: SetLocalFontsEnabledMethod::SetLocalFontsEnabled,
            params: SetLocalFontsEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
