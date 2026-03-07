use super::types::*;
impl PseudoElementMatches {
    pub fn builder() -> PseudoElementMatchesBuilder {
        <PseudoElementMatchesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PseudoElementMatchesBuilder {
    pseudo_type: Option<crate::browser_protocol::dom::types::PseudoType>,
    pseudo_identifier: Option<String>,
    matches: Option<Vec<RuleMatch>>,
}
impl PseudoElementMatchesBuilder {
    pub fn pseudo_type(
        mut self,
        pseudo_type: impl Into<crate::browser_protocol::dom::types::PseudoType>,
    ) -> Self {
        self.pseudo_type = Some(pseudo_type.into());
        self
    }
    pub fn pseudo_identifier(mut self, pseudo_identifier: impl Into<String>) -> Self {
        self.pseudo_identifier = Some(pseudo_identifier.into());
        self
    }
    pub fn matche(mut self, matche: impl Into<RuleMatch>) -> Self {
        let v = self.matches.get_or_insert(Vec::new());
        v.push(matche.into());
        self
    }
    pub fn matches<I, S>(mut self, matches: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<RuleMatch>,
    {
        let v = self.matches.get_or_insert(Vec::new());
        for val in matches {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<PseudoElementMatches, String> {
        Ok(PseudoElementMatches {
            pseudo_type: self
                .pseudo_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(pseudo_type)))?,
            pseudo_identifier: self.pseudo_identifier,
            matches: self
                .matches
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(matches)))?,
        })
    }
}
impl CssAnimationStyle {
    pub fn builder() -> CssAnimationStyleBuilder {
        <CssAnimationStyleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssAnimationStyleBuilder {
    name: Option<String>,
    style: Option<CssStyle>,
}
impl CssAnimationStyleBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn build(self) -> Result<CssAnimationStyle, String> {
        Ok(CssAnimationStyle {
            name: self.name,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
        })
    }
}
impl InheritedStyleEntry {
    pub fn builder() -> InheritedStyleEntryBuilder {
        <InheritedStyleEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InheritedStyleEntryBuilder {
    inline_style: Option<CssStyle>,
    matched_css_rules: Option<Vec<RuleMatch>>,
}
impl InheritedStyleEntryBuilder {
    pub fn inline_style(mut self, inline_style: impl Into<CssStyle>) -> Self {
        self.inline_style = Some(inline_style.into());
        self
    }
    pub fn matched_css_rule(mut self, matched_css_rule: impl Into<RuleMatch>) -> Self {
        let v = self.matched_css_rules.get_or_insert(Vec::new());
        v.push(matched_css_rule.into());
        self
    }
    pub fn matched_css_rules<I, S>(mut self, matched_css_rules: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<RuleMatch>,
    {
        let v = self.matched_css_rules.get_or_insert(Vec::new());
        for val in matched_css_rules {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<InheritedStyleEntry, String> {
        Ok(InheritedStyleEntry {
            inline_style: self.inline_style,
            matched_css_rules: self.matched_css_rules.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(matched_css_rules)
                )
            })?,
        })
    }
}
impl InheritedAnimatedStyleEntry {
    pub fn builder() -> InheritedAnimatedStyleEntryBuilder {
        <InheritedAnimatedStyleEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InheritedAnimatedStyleEntryBuilder {
    animation_styles: Option<Vec<CssAnimationStyle>>,
    transitions_style: Option<CssStyle>,
}
impl InheritedAnimatedStyleEntryBuilder {
    pub fn animation_style(mut self, animation_style: impl Into<CssAnimationStyle>) -> Self {
        let v = self.animation_styles.get_or_insert(Vec::new());
        v.push(animation_style.into());
        self
    }
    pub fn animation_styles<I, S>(mut self, animation_styles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssAnimationStyle>,
    {
        let v = self.animation_styles.get_or_insert(Vec::new());
        for val in animation_styles {
            v.push(val.into());
        }
        self
    }
    pub fn transitions_style(mut self, transitions_style: impl Into<CssStyle>) -> Self {
        self.transitions_style = Some(transitions_style.into());
        self
    }
    pub fn build(self) -> InheritedAnimatedStyleEntry {
        InheritedAnimatedStyleEntry {
            animation_styles: self.animation_styles,
            transitions_style: self.transitions_style,
        }
    }
}
impl InheritedPseudoElementMatches {
    pub fn builder() -> InheritedPseudoElementMatchesBuilder {
        <InheritedPseudoElementMatchesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InheritedPseudoElementMatchesBuilder {
    pseudo_elements: Option<Vec<PseudoElementMatches>>,
}
impl InheritedPseudoElementMatchesBuilder {
    pub fn pseudo_element(mut self, pseudo_element: impl Into<PseudoElementMatches>) -> Self {
        let v = self.pseudo_elements.get_or_insert(Vec::new());
        v.push(pseudo_element.into());
        self
    }
    pub fn pseudo_elements<I, S>(mut self, pseudo_elements: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PseudoElementMatches>,
    {
        let v = self.pseudo_elements.get_or_insert(Vec::new());
        for val in pseudo_elements {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<InheritedPseudoElementMatches, String> {
        Ok(InheritedPseudoElementMatches {
            pseudo_elements: self.pseudo_elements.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(pseudo_elements))
            })?,
        })
    }
}
impl RuleMatch {
    pub fn builder() -> RuleMatchBuilder {
        <RuleMatchBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RuleMatchBuilder {
    rule: Option<CssRule>,
    matching_selectors: Option<Vec<i64>>,
}
impl RuleMatchBuilder {
    pub fn rule(mut self, rule: impl Into<CssRule>) -> Self {
        self.rule = Some(rule.into());
        self
    }
    pub fn matching_selector(mut self, matching_selector: impl Into<i64>) -> Self {
        let v = self.matching_selectors.get_or_insert(Vec::new());
        v.push(matching_selector.into());
        self
    }
    pub fn matching_selectors<I, S>(mut self, matching_selectors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.matching_selectors.get_or_insert(Vec::new());
        for val in matching_selectors {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RuleMatch, String> {
        Ok(RuleMatch {
            rule: self
                .rule
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rule)))?,
            matching_selectors: self.matching_selectors.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(matching_selectors)
                )
            })?,
        })
    }
}
impl Value {
    pub fn builder() -> ValueBuilder {
        <ValueBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ValueBuilder {
    text: Option<String>,
    range: Option<SourceRange>,
    specificity: Option<Specificity>,
}
impl ValueBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn specificity(mut self, specificity: impl Into<Specificity>) -> Self {
        self.specificity = Some(specificity.into());
        self
    }
    pub fn build(self) -> Result<Value, String> {
        Ok(Value {
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            range: self.range,
            specificity: self.specificity,
        })
    }
}
impl Specificity {
    pub fn builder() -> SpecificityBuilder {
        <SpecificityBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SpecificityBuilder {
    a: Option<i64>,
    b: Option<i64>,
    c: Option<i64>,
}
impl SpecificityBuilder {
    pub fn a(mut self, a: impl Into<i64>) -> Self {
        self.a = Some(a.into());
        self
    }
    pub fn b(mut self, b: impl Into<i64>) -> Self {
        self.b = Some(b.into());
        self
    }
    pub fn c(mut self, c: impl Into<i64>) -> Self {
        self.c = Some(c.into());
        self
    }
    pub fn build(self) -> Result<Specificity, String> {
        Ok(Specificity {
            a: self
                .a
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(a)))?,
            b: self
                .b
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(b)))?,
            c: self
                .c
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(c)))?,
        })
    }
}
impl SelectorList {
    pub fn builder() -> SelectorListBuilder {
        <SelectorListBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SelectorListBuilder {
    selectors: Option<Vec<Value>>,
    text: Option<String>,
}
impl SelectorListBuilder {
    pub fn selector(mut self, selector: impl Into<Value>) -> Self {
        let v = self.selectors.get_or_insert(Vec::new());
        v.push(selector.into());
        self
    }
    pub fn selectors<I, S>(mut self, selectors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Value>,
    {
        let v = self.selectors.get_or_insert(Vec::new());
        for val in selectors {
            v.push(val.into());
        }
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<SelectorList, String> {
        Ok(SelectorList {
            selectors: self
                .selectors
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(selectors)))?,
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
        })
    }
}
impl CssStyleSheetHeader {
    pub fn builder() -> CssStyleSheetHeaderBuilder {
        <CssStyleSheetHeaderBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssStyleSheetHeaderBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    source_url: Option<String>,
    source_map_url: Option<String>,
    origin: Option<StyleSheetOrigin>,
    title: Option<String>,
    owner_node: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    disabled: Option<bool>,
    has_source_url: Option<bool>,
    is_inline: Option<bool>,
    is_mutable: Option<bool>,
    is_constructed: Option<bool>,
    start_line: Option<f64>,
    start_column: Option<f64>,
    length: Option<f64>,
    end_line: Option<f64>,
    end_column: Option<f64>,
    loading_failed: Option<bool>,
}
impl CssStyleSheetHeaderBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn frame_id(
        mut self,
        frame_id: impl Into<crate::browser_protocol::page::types::FrameId>,
    ) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn source_url(mut self, source_url: impl Into<String>) -> Self {
        self.source_url = Some(source_url.into());
        self
    }
    pub fn source_map_url(mut self, source_map_url: impl Into<String>) -> Self {
        self.source_map_url = Some(source_map_url.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn owner_node(
        mut self,
        owner_node: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.owner_node = Some(owner_node.into());
        self
    }
    pub fn disabled(mut self, disabled: impl Into<bool>) -> Self {
        self.disabled = Some(disabled.into());
        self
    }
    pub fn has_source_url(mut self, has_source_url: impl Into<bool>) -> Self {
        self.has_source_url = Some(has_source_url.into());
        self
    }
    pub fn is_inline(mut self, is_inline: impl Into<bool>) -> Self {
        self.is_inline = Some(is_inline.into());
        self
    }
    pub fn is_mutable(mut self, is_mutable: impl Into<bool>) -> Self {
        self.is_mutable = Some(is_mutable.into());
        self
    }
    pub fn is_constructed(mut self, is_constructed: impl Into<bool>) -> Self {
        self.is_constructed = Some(is_constructed.into());
        self
    }
    pub fn start_line(mut self, start_line: impl Into<f64>) -> Self {
        self.start_line = Some(start_line.into());
        self
    }
    pub fn start_column(mut self, start_column: impl Into<f64>) -> Self {
        self.start_column = Some(start_column.into());
        self
    }
    pub fn length(mut self, length: impl Into<f64>) -> Self {
        self.length = Some(length.into());
        self
    }
    pub fn end_line(mut self, end_line: impl Into<f64>) -> Self {
        self.end_line = Some(end_line.into());
        self
    }
    pub fn end_column(mut self, end_column: impl Into<f64>) -> Self {
        self.end_column = Some(end_column.into());
        self
    }
    pub fn loading_failed(mut self, loading_failed: impl Into<bool>) -> Self {
        self.loading_failed = Some(loading_failed.into());
        self
    }
    pub fn build(self) -> Result<CssStyleSheetHeader, String> {
        Ok(CssStyleSheetHeader {
            style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
            })?,
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
            source_url: self
                .source_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source_url)))?,
            source_map_url: self.source_map_url,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            title: self
                .title
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(title)))?,
            owner_node: self.owner_node,
            disabled: self
                .disabled
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(disabled)))?,
            has_source_url: self.has_source_url,
            is_inline: self
                .is_inline
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_inline)))?,
            is_mutable: self
                .is_mutable
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_mutable)))?,
            is_constructed: self.is_constructed.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(is_constructed))
            })?,
            start_line: self
                .start_line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start_line)))?,
            start_column: self.start_column.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(start_column))
            })?,
            length: self
                .length
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(length)))?,
            end_line: self
                .end_line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_line)))?,
            end_column: self
                .end_column
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_column)))?,
            loading_failed: self.loading_failed,
        })
    }
}
impl CssRule {
    pub fn builder() -> CssRuleBuilder {
        <CssRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssRuleBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    selector_list: Option<SelectorList>,
    nesting_selectors: Option<Vec<String>>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CssStyle>,
    origin_tree_scope_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    media: Option<Vec<CssMedia>>,
    container_queries: Option<Vec<CssContainerQuery>>,
    supports: Option<Vec<CssSupports>>,
    layers: Option<Vec<CssLayer>>,
    scopes: Option<Vec<CssScope>>,
    rule_types: Option<Vec<CssRuleType>>,
    starting_styles: Option<Vec<CssStartingStyle>>,
}
impl CssRuleBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn selector_list(mut self, selector_list: impl Into<SelectorList>) -> Self {
        self.selector_list = Some(selector_list.into());
        self
    }
    pub fn nesting_selector(mut self, nesting_selector: impl Into<String>) -> Self {
        let v = self.nesting_selectors.get_or_insert(Vec::new());
        v.push(nesting_selector.into());
        self
    }
    pub fn nesting_selectors<I, S>(mut self, nesting_selectors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.nesting_selectors.get_or_insert(Vec::new());
        for val in nesting_selectors {
            v.push(val.into());
        }
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn origin_tree_scope_node_id(
        mut self,
        origin_tree_scope_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.origin_tree_scope_node_id = Some(origin_tree_scope_node_id.into());
        self
    }
    pub fn media(mut self, media: impl Into<CssMedia>) -> Self {
        let v = self.media.get_or_insert(Vec::new());
        v.push(media.into());
        self
    }
    pub fn medias<I, S>(mut self, medias: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssMedia>,
    {
        let v = self.media.get_or_insert(Vec::new());
        for val in medias {
            v.push(val.into());
        }
        self
    }
    pub fn container_querie(mut self, container_querie: impl Into<CssContainerQuery>) -> Self {
        let v = self.container_queries.get_or_insert(Vec::new());
        v.push(container_querie.into());
        self
    }
    pub fn container_queries<I, S>(mut self, container_queries: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssContainerQuery>,
    {
        let v = self.container_queries.get_or_insert(Vec::new());
        for val in container_queries {
            v.push(val.into());
        }
        self
    }
    pub fn support(mut self, support: impl Into<CssSupports>) -> Self {
        let v = self.supports.get_or_insert(Vec::new());
        v.push(support.into());
        self
    }
    pub fn supports<I, S>(mut self, supports: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssSupports>,
    {
        let v = self.supports.get_or_insert(Vec::new());
        for val in supports {
            v.push(val.into());
        }
        self
    }
    pub fn layer(mut self, layer: impl Into<CssLayer>) -> Self {
        let v = self.layers.get_or_insert(Vec::new());
        v.push(layer.into());
        self
    }
    pub fn layers<I, S>(mut self, layers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssLayer>,
    {
        let v = self.layers.get_or_insert(Vec::new());
        for val in layers {
            v.push(val.into());
        }
        self
    }
    pub fn scope(mut self, scope: impl Into<CssScope>) -> Self {
        let v = self.scopes.get_or_insert(Vec::new());
        v.push(scope.into());
        self
    }
    pub fn scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssScope>,
    {
        let v = self.scopes.get_or_insert(Vec::new());
        for val in scopes {
            v.push(val.into());
        }
        self
    }
    pub fn rule_type(mut self, rule_type: impl Into<CssRuleType>) -> Self {
        let v = self.rule_types.get_or_insert(Vec::new());
        v.push(rule_type.into());
        self
    }
    pub fn rule_types<I, S>(mut self, rule_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssRuleType>,
    {
        let v = self.rule_types.get_or_insert(Vec::new());
        for val in rule_types {
            v.push(val.into());
        }
        self
    }
    pub fn starting_style(mut self, starting_style: impl Into<CssStartingStyle>) -> Self {
        let v = self.starting_styles.get_or_insert(Vec::new());
        v.push(starting_style.into());
        self
    }
    pub fn starting_styles<I, S>(mut self, starting_styles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssStartingStyle>,
    {
        let v = self.starting_styles.get_or_insert(Vec::new());
        for val in starting_styles {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<CssRule, String> {
        Ok(CssRule {
            style_sheet_id: self.style_sheet_id,
            selector_list: self.selector_list.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(selector_list))
            })?,
            nesting_selectors: self.nesting_selectors,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
            origin_tree_scope_node_id: self.origin_tree_scope_node_id,
            media: self.media,
            container_queries: self.container_queries,
            supports: self.supports,
            layers: self.layers,
            scopes: self.scopes,
            rule_types: self.rule_types,
            starting_styles: self.starting_styles,
        })
    }
}
impl RuleUsage {
    pub fn builder() -> RuleUsageBuilder {
        <RuleUsageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RuleUsageBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    start_offset: Option<f64>,
    end_offset: Option<f64>,
    used: Option<bool>,
}
impl RuleUsageBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn start_offset(mut self, start_offset: impl Into<f64>) -> Self {
        self.start_offset = Some(start_offset.into());
        self
    }
    pub fn end_offset(mut self, end_offset: impl Into<f64>) -> Self {
        self.end_offset = Some(end_offset.into());
        self
    }
    pub fn used(mut self, used: impl Into<bool>) -> Self {
        self.used = Some(used.into());
        self
    }
    pub fn build(self) -> Result<RuleUsage, String> {
        Ok(RuleUsage {
            style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
            })?,
            start_offset: self.start_offset.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(start_offset))
            })?,
            end_offset: self
                .end_offset
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_offset)))?,
            used: self
                .used
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(used)))?,
        })
    }
}
impl SourceRange {
    pub fn builder() -> SourceRangeBuilder {
        <SourceRangeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SourceRangeBuilder {
    start_line: Option<i64>,
    start_column: Option<i64>,
    end_line: Option<i64>,
    end_column: Option<i64>,
}
impl SourceRangeBuilder {
    pub fn start_line(mut self, start_line: impl Into<i64>) -> Self {
        self.start_line = Some(start_line.into());
        self
    }
    pub fn start_column(mut self, start_column: impl Into<i64>) -> Self {
        self.start_column = Some(start_column.into());
        self
    }
    pub fn end_line(mut self, end_line: impl Into<i64>) -> Self {
        self.end_line = Some(end_line.into());
        self
    }
    pub fn end_column(mut self, end_column: impl Into<i64>) -> Self {
        self.end_column = Some(end_column.into());
        self
    }
    pub fn build(self) -> Result<SourceRange, String> {
        Ok(SourceRange {
            start_line: self
                .start_line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start_line)))?,
            start_column: self.start_column.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(start_column))
            })?,
            end_line: self
                .end_line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_line)))?,
            end_column: self
                .end_column
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_column)))?,
        })
    }
}
impl ShorthandEntry {
    pub fn builder() -> ShorthandEntryBuilder {
        <ShorthandEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ShorthandEntryBuilder {
    name: Option<String>,
    value: Option<String>,
    important: Option<bool>,
}
impl ShorthandEntryBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn important(mut self, important: impl Into<bool>) -> Self {
        self.important = Some(important.into());
        self
    }
    pub fn build(self) -> Result<ShorthandEntry, String> {
        Ok(ShorthandEntry {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            important: self.important,
        })
    }
}
impl CssComputedStyleProperty {
    pub fn builder() -> CssComputedStylePropertyBuilder {
        <CssComputedStylePropertyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssComputedStylePropertyBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl CssComputedStylePropertyBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<CssComputedStyleProperty, String> {
        Ok(CssComputedStyleProperty {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl ComputedStyleExtraFields {
    pub fn builder() -> ComputedStyleExtraFieldsBuilder {
        <ComputedStyleExtraFieldsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ComputedStyleExtraFieldsBuilder {
    is_appearance_base: Option<bool>,
}
impl ComputedStyleExtraFieldsBuilder {
    pub fn is_appearance_base(mut self, is_appearance_base: impl Into<bool>) -> Self {
        self.is_appearance_base = Some(is_appearance_base.into());
        self
    }
    pub fn build(self) -> Result<ComputedStyleExtraFields, String> {
        Ok(ComputedStyleExtraFields {
            is_appearance_base: self.is_appearance_base.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(is_appearance_base)
                )
            })?,
        })
    }
}
impl CssStyle {
    pub fn builder() -> CssStyleBuilder {
        <CssStyleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssStyleBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    css_properties: Option<Vec<CssProperty>>,
    shorthand_entries: Option<Vec<ShorthandEntry>>,
    css_text: Option<String>,
    range: Option<SourceRange>,
}
impl CssStyleBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn css_propertie(mut self, css_propertie: impl Into<CssProperty>) -> Self {
        let v = self.css_properties.get_or_insert(Vec::new());
        v.push(css_propertie.into());
        self
    }
    pub fn css_properties<I, S>(mut self, css_properties: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssProperty>,
    {
        let v = self.css_properties.get_or_insert(Vec::new());
        for val in css_properties {
            v.push(val.into());
        }
        self
    }
    pub fn shorthand_entrie(mut self, shorthand_entrie: impl Into<ShorthandEntry>) -> Self {
        let v = self.shorthand_entries.get_or_insert(Vec::new());
        v.push(shorthand_entrie.into());
        self
    }
    pub fn shorthand_entries<I, S>(mut self, shorthand_entries: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ShorthandEntry>,
    {
        let v = self.shorthand_entries.get_or_insert(Vec::new());
        for val in shorthand_entries {
            v.push(val.into());
        }
        self
    }
    pub fn css_text(mut self, css_text: impl Into<String>) -> Self {
        self.css_text = Some(css_text.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn build(self) -> Result<CssStyle, String> {
        Ok(CssStyle {
            style_sheet_id: self.style_sheet_id,
            css_properties: self.css_properties.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(css_properties))
            })?,
            shorthand_entries: self.shorthand_entries.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(shorthand_entries)
                )
            })?,
            css_text: self.css_text,
            range: self.range,
        })
    }
}
impl CssProperty {
    pub fn builder() -> CssPropertyBuilder {
        <CssPropertyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssPropertyBuilder {
    name: Option<String>,
    value: Option<String>,
    important: Option<bool>,
    implicit: Option<bool>,
    text: Option<String>,
    parsed_ok: Option<bool>,
    disabled: Option<bool>,
    range: Option<SourceRange>,
    longhand_properties: Option<Vec<CssProperty>>,
}
impl CssPropertyBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn important(mut self, important: impl Into<bool>) -> Self {
        self.important = Some(important.into());
        self
    }
    pub fn implicit(mut self, implicit: impl Into<bool>) -> Self {
        self.implicit = Some(implicit.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn parsed_ok(mut self, parsed_ok: impl Into<bool>) -> Self {
        self.parsed_ok = Some(parsed_ok.into());
        self
    }
    pub fn disabled(mut self, disabled: impl Into<bool>) -> Self {
        self.disabled = Some(disabled.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn longhand_propertie(mut self, longhand_propertie: impl Into<CssProperty>) -> Self {
        let v = self.longhand_properties.get_or_insert(Vec::new());
        v.push(longhand_propertie.into());
        self
    }
    pub fn longhand_properties<I, S>(mut self, longhand_properties: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssProperty>,
    {
        let v = self.longhand_properties.get_or_insert(Vec::new());
        for val in longhand_properties {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<CssProperty, String> {
        Ok(CssProperty {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            important: self.important,
            implicit: self.implicit,
            text: self.text,
            parsed_ok: self.parsed_ok,
            disabled: self.disabled,
            range: self.range,
            longhand_properties: self.longhand_properties,
        })
    }
}
impl CssMedia {
    pub fn builder() -> CssMediaBuilder {
        <CssMediaBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssMediaBuilder {
    text: Option<String>,
    source: Option<CssMediaSource>,
    source_url: Option<String>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    media_list: Option<Vec<MediaQuery>>,
}
impl CssMediaBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn source(mut self, source: impl Into<CssMediaSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn source_url(mut self, source_url: impl Into<String>) -> Self {
        self.source_url = Some(source_url.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn media_list(mut self, media_list: impl Into<MediaQuery>) -> Self {
        let v = self.media_list.get_or_insert(Vec::new());
        v.push(media_list.into());
        self
    }
    pub fn media_lists<I, S>(mut self, media_lists: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<MediaQuery>,
    {
        let v = self.media_list.get_or_insert(Vec::new());
        for val in media_lists {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<CssMedia, String> {
        Ok(CssMedia {
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            source: self
                .source
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
            source_url: self.source_url,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
            media_list: self.media_list,
        })
    }
}
impl MediaQuery {
    pub fn builder() -> MediaQueryBuilder {
        <MediaQueryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct MediaQueryBuilder {
    expressions: Option<Vec<MediaQueryExpression>>,
    active: Option<bool>,
}
impl MediaQueryBuilder {
    pub fn expression(mut self, expression: impl Into<MediaQueryExpression>) -> Self {
        let v = self.expressions.get_or_insert(Vec::new());
        v.push(expression.into());
        self
    }
    pub fn expressions<I, S>(mut self, expressions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<MediaQueryExpression>,
    {
        let v = self.expressions.get_or_insert(Vec::new());
        for val in expressions {
            v.push(val.into());
        }
        self
    }
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.active = Some(active.into());
        self
    }
    pub fn build(self) -> Result<MediaQuery, String> {
        Ok(MediaQuery {
            expressions: self
                .expressions
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(expressions)))?,
            active: self
                .active
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(active)))?,
        })
    }
}
impl MediaQueryExpression {
    pub fn builder() -> MediaQueryExpressionBuilder {
        <MediaQueryExpressionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct MediaQueryExpressionBuilder {
    value: Option<f64>,
    unit: Option<String>,
    feature: Option<String>,
    value_range: Option<SourceRange>,
    computed_length: Option<f64>,
}
impl MediaQueryExpressionBuilder {
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.feature = Some(feature.into());
        self
    }
    pub fn value_range(mut self, value_range: impl Into<SourceRange>) -> Self {
        self.value_range = Some(value_range.into());
        self
    }
    pub fn computed_length(mut self, computed_length: impl Into<f64>) -> Self {
        self.computed_length = Some(computed_length.into());
        self
    }
    pub fn build(self) -> Result<MediaQueryExpression, String> {
        Ok(MediaQueryExpression {
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            unit: self
                .unit
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(unit)))?,
            feature: self
                .feature
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(feature)))?,
            value_range: self.value_range,
            computed_length: self.computed_length,
        })
    }
}
impl CssContainerQuery {
    pub fn builder() -> CssContainerQueryBuilder {
        <CssContainerQueryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssContainerQueryBuilder {
    text: Option<String>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    name: Option<String>,
    physical_axes: Option<crate::browser_protocol::dom::types::PhysicalAxes>,
    logical_axes: Option<crate::browser_protocol::dom::types::LogicalAxes>,
    queries_scroll_state: Option<bool>,
    queries_anchored: Option<bool>,
}
impl CssContainerQueryBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn physical_axes(
        mut self,
        physical_axes: impl Into<crate::browser_protocol::dom::types::PhysicalAxes>,
    ) -> Self {
        self.physical_axes = Some(physical_axes.into());
        self
    }
    pub fn logical_axes(
        mut self,
        logical_axes: impl Into<crate::browser_protocol::dom::types::LogicalAxes>,
    ) -> Self {
        self.logical_axes = Some(logical_axes.into());
        self
    }
    pub fn queries_scroll_state(mut self, queries_scroll_state: impl Into<bool>) -> Self {
        self.queries_scroll_state = Some(queries_scroll_state.into());
        self
    }
    pub fn queries_anchored(mut self, queries_anchored: impl Into<bool>) -> Self {
        self.queries_anchored = Some(queries_anchored.into());
        self
    }
    pub fn build(self) -> Result<CssContainerQuery, String> {
        Ok(CssContainerQuery {
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
            name: self.name,
            physical_axes: self.physical_axes,
            logical_axes: self.logical_axes,
            queries_scroll_state: self.queries_scroll_state,
            queries_anchored: self.queries_anchored,
        })
    }
}
impl CssSupports {
    pub fn builder() -> CssSupportsBuilder {
        <CssSupportsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssSupportsBuilder {
    text: Option<String>,
    active: Option<bool>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
}
impl CssSupportsBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.active = Some(active.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn build(self) -> Result<CssSupports, String> {
        Ok(CssSupports {
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            active: self
                .active
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(active)))?,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
        })
    }
}
impl CssScope {
    pub fn builder() -> CssScopeBuilder {
        <CssScopeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssScopeBuilder {
    text: Option<String>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
}
impl CssScopeBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn build(self) -> Result<CssScope, String> {
        Ok(CssScope {
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
        })
    }
}
impl CssLayer {
    pub fn builder() -> CssLayerBuilder {
        <CssLayerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssLayerBuilder {
    text: Option<String>,
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
}
impl CssLayerBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn build(self) -> Result<CssLayer, String> {
        Ok(CssLayer {
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            range: self.range,
            style_sheet_id: self.style_sheet_id,
        })
    }
}
impl CssStartingStyle {
    pub fn builder() -> CssStartingStyleBuilder {
        <CssStartingStyleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssStartingStyleBuilder {
    range: Option<SourceRange>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
}
impl CssStartingStyleBuilder {
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn build(self) -> CssStartingStyle {
        CssStartingStyle {
            range: self.range,
            style_sheet_id: self.style_sheet_id,
        }
    }
}
impl CssLayerData {
    pub fn builder() -> CssLayerDataBuilder {
        <CssLayerDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssLayerDataBuilder {
    name: Option<String>,
    sub_layers: Option<Vec<CssLayerData>>,
    order: Option<f64>,
}
impl CssLayerDataBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn sub_layer(mut self, sub_layer: impl Into<CssLayerData>) -> Self {
        let v = self.sub_layers.get_or_insert(Vec::new());
        v.push(sub_layer.into());
        self
    }
    pub fn sub_layers<I, S>(mut self, sub_layers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssLayerData>,
    {
        let v = self.sub_layers.get_or_insert(Vec::new());
        for val in sub_layers {
            v.push(val.into());
        }
        self
    }
    pub fn order(mut self, order: impl Into<f64>) -> Self {
        self.order = Some(order.into());
        self
    }
    pub fn build(self) -> Result<CssLayerData, String> {
        Ok(CssLayerData {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            sub_layers: self.sub_layers,
            order: self
                .order
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(order)))?,
        })
    }
}
impl PlatformFontUsage {
    pub fn builder() -> PlatformFontUsageBuilder {
        <PlatformFontUsageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlatformFontUsageBuilder {
    family_name: Option<String>,
    post_script_name: Option<String>,
    is_custom_font: Option<bool>,
    glyph_count: Option<f64>,
}
impl PlatformFontUsageBuilder {
    pub fn family_name(mut self, family_name: impl Into<String>) -> Self {
        self.family_name = Some(family_name.into());
        self
    }
    pub fn post_script_name(mut self, post_script_name: impl Into<String>) -> Self {
        self.post_script_name = Some(post_script_name.into());
        self
    }
    pub fn is_custom_font(mut self, is_custom_font: impl Into<bool>) -> Self {
        self.is_custom_font = Some(is_custom_font.into());
        self
    }
    pub fn glyph_count(mut self, glyph_count: impl Into<f64>) -> Self {
        self.glyph_count = Some(glyph_count.into());
        self
    }
    pub fn build(self) -> Result<PlatformFontUsage, String> {
        Ok(PlatformFontUsage {
            family_name: self
                .family_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(family_name)))?,
            post_script_name: self.post_script_name.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(post_script_name)
                )
            })?,
            is_custom_font: self.is_custom_font.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(is_custom_font))
            })?,
            glyph_count: self
                .glyph_count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(glyph_count)))?,
        })
    }
}
impl FontVariationAxis {
    pub fn builder() -> FontVariationAxisBuilder {
        <FontVariationAxisBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FontVariationAxisBuilder {
    tag: Option<String>,
    name: Option<String>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    default_value: Option<f64>,
}
impl FontVariationAxisBuilder {
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn min_value(mut self, min_value: impl Into<f64>) -> Self {
        self.min_value = Some(min_value.into());
        self
    }
    pub fn max_value(mut self, max_value: impl Into<f64>) -> Self {
        self.max_value = Some(max_value.into());
        self
    }
    pub fn default_value(mut self, default_value: impl Into<f64>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
    pub fn build(self) -> Result<FontVariationAxis, String> {
        Ok(FontVariationAxis {
            tag: self
                .tag
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(tag)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            min_value: self
                .min_value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(min_value)))?,
            max_value: self
                .max_value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(max_value)))?,
            default_value: self.default_value.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(default_value))
            })?,
        })
    }
}
impl FontFace {
    pub fn builder() -> FontFaceBuilder {
        <FontFaceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FontFaceBuilder {
    font_family: Option<String>,
    font_style: Option<String>,
    font_variant: Option<String>,
    font_weight: Option<String>,
    font_stretch: Option<String>,
    font_display: Option<String>,
    unicode_range: Option<String>,
    src: Option<String>,
    platform_font_family: Option<String>,
    font_variation_axes: Option<Vec<FontVariationAxis>>,
}
impl FontFaceBuilder {
    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }
    pub fn font_style(mut self, font_style: impl Into<String>) -> Self {
        self.font_style = Some(font_style.into());
        self
    }
    pub fn font_variant(mut self, font_variant: impl Into<String>) -> Self {
        self.font_variant = Some(font_variant.into());
        self
    }
    pub fn font_weight(mut self, font_weight: impl Into<String>) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }
    pub fn font_stretch(mut self, font_stretch: impl Into<String>) -> Self {
        self.font_stretch = Some(font_stretch.into());
        self
    }
    pub fn font_display(mut self, font_display: impl Into<String>) -> Self {
        self.font_display = Some(font_display.into());
        self
    }
    pub fn unicode_range(mut self, unicode_range: impl Into<String>) -> Self {
        self.unicode_range = Some(unicode_range.into());
        self
    }
    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }
    pub fn platform_font_family(mut self, platform_font_family: impl Into<String>) -> Self {
        self.platform_font_family = Some(platform_font_family.into());
        self
    }
    pub fn font_variation_axe(mut self, font_variation_axe: impl Into<FontVariationAxis>) -> Self {
        let v = self.font_variation_axes.get_or_insert(Vec::new());
        v.push(font_variation_axe.into());
        self
    }
    pub fn font_variation_axes<I, S>(mut self, font_variation_axes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FontVariationAxis>,
    {
        let v = self.font_variation_axes.get_or_insert(Vec::new());
        for val in font_variation_axes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<FontFace, String> {
        Ok(FontFace {
            font_family: self
                .font_family
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(font_family)))?,
            font_style: self
                .font_style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(font_style)))?,
            font_variant: self.font_variant.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(font_variant))
            })?,
            font_weight: self
                .font_weight
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(font_weight)))?,
            font_stretch: self.font_stretch.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(font_stretch))
            })?,
            font_display: self.font_display.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(font_display))
            })?,
            unicode_range: self.unicode_range.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(unicode_range))
            })?,
            src: self
                .src
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(src)))?,
            platform_font_family: self.platform_font_family.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(platform_font_family)
                )
            })?,
            font_variation_axes: self.font_variation_axes,
        })
    }
}
impl CssTryRule {
    pub fn builder() -> CssTryRuleBuilder {
        <CssTryRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssTryRuleBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CssStyle>,
}
impl CssTryRuleBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn build(self) -> Result<CssTryRule, String> {
        Ok(CssTryRule {
            style_sheet_id: self.style_sheet_id,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
        })
    }
}
impl CssPositionTryRule {
    pub fn builder() -> CssPositionTryRuleBuilder {
        <CssPositionTryRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssPositionTryRuleBuilder {
    name: Option<Value>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CssStyle>,
    active: Option<bool>,
}
impl CssPositionTryRuleBuilder {
    pub fn name(mut self, name: impl Into<Value>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.active = Some(active.into());
        self
    }
    pub fn build(self) -> Result<CssPositionTryRule, String> {
        Ok(CssPositionTryRule {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            style_sheet_id: self.style_sheet_id,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
            active: self
                .active
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(active)))?,
        })
    }
}
impl CssKeyframesRule {
    pub fn builder() -> CssKeyframesRuleBuilder {
        <CssKeyframesRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssKeyframesRuleBuilder {
    animation_name: Option<Value>,
    keyframes: Option<Vec<CssKeyframeRule>>,
}
impl CssKeyframesRuleBuilder {
    pub fn animation_name(mut self, animation_name: impl Into<Value>) -> Self {
        self.animation_name = Some(animation_name.into());
        self
    }
    pub fn keyframe(mut self, keyframe: impl Into<CssKeyframeRule>) -> Self {
        let v = self.keyframes.get_or_insert(Vec::new());
        v.push(keyframe.into());
        self
    }
    pub fn keyframes<I, S>(mut self, keyframes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssKeyframeRule>,
    {
        let v = self.keyframes.get_or_insert(Vec::new());
        for val in keyframes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<CssKeyframesRule, String> {
        Ok(CssKeyframesRule {
            animation_name: self.animation_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(animation_name))
            })?,
            keyframes: self
                .keyframes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(keyframes)))?,
        })
    }
}
impl CssPropertyRegistration {
    pub fn builder() -> CssPropertyRegistrationBuilder {
        <CssPropertyRegistrationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssPropertyRegistrationBuilder {
    property_name: Option<String>,
    initial_value: Option<Value>,
    inherits: Option<bool>,
    syntax: Option<String>,
}
impl CssPropertyRegistrationBuilder {
    pub fn property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = Some(property_name.into());
        self
    }
    pub fn initial_value(mut self, initial_value: impl Into<Value>) -> Self {
        self.initial_value = Some(initial_value.into());
        self
    }
    pub fn inherits(mut self, inherits: impl Into<bool>) -> Self {
        self.inherits = Some(inherits.into());
        self
    }
    pub fn syntax(mut self, syntax: impl Into<String>) -> Self {
        self.syntax = Some(syntax.into());
        self
    }
    pub fn build(self) -> Result<CssPropertyRegistration, String> {
        Ok(CssPropertyRegistration {
            property_name: self.property_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(property_name))
            })?,
            initial_value: self.initial_value,
            inherits: self
                .inherits
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(inherits)))?,
            syntax: self
                .syntax
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(syntax)))?,
        })
    }
}
impl CssAtRule {
    pub fn builder() -> CssAtRuleBuilder {
        <CssAtRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssAtRuleBuilder {
    r#type: Option<CssAtRuleType>,
    subsection: Option<CssAtRuleSubsection>,
    name: Option<Value>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    origin: Option<StyleSheetOrigin>,
    style: Option<CssStyle>,
}
impl CssAtRuleBuilder {
    pub fn r#type(mut self, r#type: impl Into<CssAtRuleType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn subsection(mut self, subsection: impl Into<CssAtRuleSubsection>) -> Self {
        self.subsection = Some(subsection.into());
        self
    }
    pub fn name(mut self, name: impl Into<Value>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn build(self) -> Result<CssAtRule, String> {
        Ok(CssAtRule {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            subsection: self.subsection,
            name: self.name,
            style_sheet_id: self.style_sheet_id,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
        })
    }
}
impl CssPropertyRule {
    pub fn builder() -> CssPropertyRuleBuilder {
        <CssPropertyRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssPropertyRuleBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    origin: Option<StyleSheetOrigin>,
    property_name: Option<Value>,
    style: Option<CssStyle>,
}
impl CssPropertyRuleBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn property_name(mut self, property_name: impl Into<Value>) -> Self {
        self.property_name = Some(property_name.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn build(self) -> Result<CssPropertyRule, String> {
        Ok(CssPropertyRule {
            style_sheet_id: self.style_sheet_id,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            property_name: self.property_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(property_name))
            })?,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
        })
    }
}
impl CssFunctionParameter {
    pub fn builder() -> CssFunctionParameterBuilder {
        <CssFunctionParameterBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssFunctionParameterBuilder {
    name: Option<String>,
    r#type: Option<String>,
}
impl CssFunctionParameterBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<CssFunctionParameter, String> {
        Ok(CssFunctionParameter {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl CssFunctionConditionNode {
    pub fn builder() -> CssFunctionConditionNodeBuilder {
        <CssFunctionConditionNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssFunctionConditionNodeBuilder {
    media: Option<CssMedia>,
    container_queries: Option<CssContainerQuery>,
    supports: Option<CssSupports>,
    children: Option<Vec<CssFunctionNode>>,
    condition_text: Option<String>,
}
impl CssFunctionConditionNodeBuilder {
    pub fn media(mut self, media: impl Into<CssMedia>) -> Self {
        self.media = Some(media.into());
        self
    }
    pub fn container_queries(mut self, container_queries: impl Into<CssContainerQuery>) -> Self {
        self.container_queries = Some(container_queries.into());
        self
    }
    pub fn supports(mut self, supports: impl Into<CssSupports>) -> Self {
        self.supports = Some(supports.into());
        self
    }
    pub fn children(mut self, children: impl Into<CssFunctionNode>) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssFunctionNode>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn condition_text(mut self, condition_text: impl Into<String>) -> Self {
        self.condition_text = Some(condition_text.into());
        self
    }
    pub fn build(self) -> Result<CssFunctionConditionNode, String> {
        Ok(CssFunctionConditionNode {
            media: self.media,
            container_queries: self.container_queries,
            supports: self.supports,
            children: self
                .children
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(children)))?,
            condition_text: self.condition_text.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(condition_text))
            })?,
        })
    }
}
impl CssFunctionNode {
    pub fn builder() -> CssFunctionNodeBuilder {
        <CssFunctionNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssFunctionNodeBuilder {
    condition: Option<CssFunctionConditionNode>,
    style: Option<CssStyle>,
}
impl CssFunctionNodeBuilder {
    pub fn condition(mut self, condition: impl Into<CssFunctionConditionNode>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn build(self) -> CssFunctionNode {
        CssFunctionNode {
            condition: self.condition,
            style: self.style,
        }
    }
}
impl CssFunctionRule {
    pub fn builder() -> CssFunctionRuleBuilder {
        <CssFunctionRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssFunctionRuleBuilder {
    name: Option<Value>,
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    origin: Option<StyleSheetOrigin>,
    parameters: Option<Vec<CssFunctionParameter>>,
    children: Option<Vec<CssFunctionNode>>,
}
impl CssFunctionRuleBuilder {
    pub fn name(mut self, name: impl Into<Value>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn parameter(mut self, parameter: impl Into<CssFunctionParameter>) -> Self {
        let v = self.parameters.get_or_insert(Vec::new());
        v.push(parameter.into());
        self
    }
    pub fn parameters<I, S>(mut self, parameters: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssFunctionParameter>,
    {
        let v = self.parameters.get_or_insert(Vec::new());
        for val in parameters {
            v.push(val.into());
        }
        self
    }
    pub fn children(mut self, children: impl Into<CssFunctionNode>) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CssFunctionNode>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<CssFunctionRule, String> {
        Ok(CssFunctionRule {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            style_sheet_id: self.style_sheet_id,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            parameters: self
                .parameters
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(parameters)))?,
            children: self
                .children
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(children)))?,
        })
    }
}
impl CssKeyframeRule {
    pub fn builder() -> CssKeyframeRuleBuilder {
        <CssKeyframeRuleBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CssKeyframeRuleBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    origin: Option<StyleSheetOrigin>,
    key_text: Option<Value>,
    style: Option<CssStyle>,
}
impl CssKeyframeRuleBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<StyleSheetOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn key_text(mut self, key_text: impl Into<Value>) -> Self {
        self.key_text = Some(key_text.into());
        self
    }
    pub fn style(mut self, style: impl Into<CssStyle>) -> Self {
        self.style = Some(style.into());
        self
    }
    pub fn build(self) -> Result<CssKeyframeRule, String> {
        Ok(CssKeyframeRule {
            style_sheet_id: self.style_sheet_id,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            key_text: self
                .key_text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key_text)))?,
            style: self
                .style
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(style)))?,
        })
    }
}
impl StyleDeclarationEdit {
    pub fn builder() -> StyleDeclarationEditBuilder {
        <StyleDeclarationEditBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StyleDeclarationEditBuilder {
    style_sheet_id: Option<crate::browser_protocol::dom::types::StyleSheetId>,
    range: Option<SourceRange>,
    text: Option<String>,
}
impl StyleDeclarationEditBuilder {
    pub fn style_sheet_id(
        mut self,
        style_sheet_id: impl Into<crate::browser_protocol::dom::types::StyleSheetId>,
    ) -> Self {
        self.style_sheet_id = Some(style_sheet_id.into());
        self
    }
    pub fn range(mut self, range: impl Into<SourceRange>) -> Self {
        self.range = Some(range.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<StyleDeclarationEdit, String> {
        Ok(StyleDeclarationEdit {
            style_sheet_id: self.style_sheet_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(style_sheet_id))
            })?,
            range: self
                .range
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(range)))?,
            text: self
                .text
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
        })
    }
}
