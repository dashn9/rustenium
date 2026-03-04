use serde::{Deserialize, Serialize};
#[doc = "Stylesheet type: \"injected\" for stylesheets injected via extension, \"user-agent\" for user-agent\nstylesheets, \"inspector\" for stylesheets created by the inspector (i.e. those holding the \"via\ninspector\" rules), \"regular\" for regular stylesheets."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StyleSheetOrigin {
    #[serde(rename = "injected")]
    Injected,
    #[serde(rename = "user-agent")]
    UserAgent,
    #[serde(rename = "inspector")]
    Inspector,
    #[serde(rename = "regular")]
    Regular,
}
#[doc = "CSS rule collection for a single pseudo style.\n[PseudoElementMatches](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-PseudoElementMatches)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementMatches {
    #[doc = "Pseudo element type."]
    #[serde(rename = "pseudoType")]
    pub pseudo_type: super::super::dom::types::PseudoType,
    #[doc = "Pseudo element custom ident."]
    #[serde(rename = "pseudoIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_identifier: Option<String>,
    #[doc = "Matches of CSS rules applicable to the pseudo style."]
    #[serde(rename = "matches")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matches: Vec<RuleMatch>,
}
impl PseudoElementMatches {
    pub fn new(
        pseudo_type: impl Into<super::super::dom::types::PseudoType>,
        matches: Vec<RuleMatch>,
    ) -> Self {
        Self {
            pseudo_type: pseudo_type.into(),
            matches,
            pseudo_identifier: None,
        }
    }
}
impl PseudoElementMatches {
    pub const IDENTIFIER: &'static str = "CSS.PseudoElementMatches";
}
#[doc = "CSS style coming from animations with the name of the animation.\n[CSSAnimationStyle](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSAnimationStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssAnimationStyle {
    #[doc = "The name of the animation."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[doc = "The style coming from the animation."]
    #[serde(rename = "style")]
    pub style: CssStyle,
}
impl CssAnimationStyle {
    pub fn new(style: impl Into<CssStyle>) -> Self {
        Self {
            style: style.into(),
            name: None,
        }
    }
}
impl CssAnimationStyle {
    pub const IDENTIFIER: &'static str = "CSS.CSSAnimationStyle";
}
#[doc = "Inherited CSS rule collection from ancestor node.\n[InheritedStyleEntry](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-InheritedStyleEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InheritedStyleEntry {
    #[doc = "The ancestor node's inline style, if any, in the style inheritance chain."]
    #[serde(rename = "inlineStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inline_style: Option<CssStyle>,
    #[doc = "Matches of CSS rules matching the ancestor node in the style inheritance chain."]
    #[serde(rename = "matchedCSSRules")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matched_css_rules: Vec<RuleMatch>,
}
impl InheritedStyleEntry {
    pub fn new(matched_css_rules: Vec<RuleMatch>) -> Self {
        Self {
            matched_css_rules,
            inline_style: None,
        }
    }
}
impl InheritedStyleEntry {
    pub const IDENTIFIER: &'static str = "CSS.InheritedStyleEntry";
}
#[doc = "Inherited CSS style collection for animated styles from ancestor node.\n[InheritedAnimatedStyleEntry](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-InheritedAnimatedStyleEntry)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InheritedAnimatedStyleEntry {
    #[doc = "Styles coming from the animations of the ancestor, if any, in the style inheritance chain."]
    #[serde(rename = "animationStyles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub animation_styles: Option<Vec<CssAnimationStyle>>,
    #[doc = "The style coming from the transitions of the ancestor, if any, in the style inheritance chain."]
    #[serde(rename = "transitionsStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transitions_style: Option<CssStyle>,
}
impl InheritedAnimatedStyleEntry {
    pub const IDENTIFIER: &'static str = "CSS.InheritedAnimatedStyleEntry";
}
#[doc = "Inherited pseudo element matches from pseudos of an ancestor node.\n[InheritedPseudoElementMatches](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-InheritedPseudoElementMatches)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InheritedPseudoElementMatches {
    #[doc = "Matches of pseudo styles from the pseudos of an ancestor node."]
    #[serde(rename = "pseudoElements")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pseudo_elements: Vec<PseudoElementMatches>,
}
impl InheritedPseudoElementMatches {
    pub fn new(pseudo_elements: Vec<PseudoElementMatches>) -> Self {
        Self { pseudo_elements }
    }
}
impl InheritedPseudoElementMatches {
    pub const IDENTIFIER: &'static str = "CSS.InheritedPseudoElementMatches";
}
#[doc = "Match data for a CSS rule.\n[RuleMatch](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-RuleMatch)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleMatch {
    #[doc = "CSS rule in the match."]
    #[serde(rename = "rule")]
    pub rule: CssRule,
    #[doc = "Matching selector indices in the rule's selectorList selectors (0-based)."]
    #[serde(rename = "matchingSelectors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matching_selectors: Vec<i64>,
}
impl RuleMatch {
    pub fn new(rule: impl Into<CssRule>, matching_selectors: Vec<i64>) -> Self {
        Self {
            rule: rule.into(),
            matching_selectors,
        }
    }
}
impl RuleMatch {
    pub const IDENTIFIER: &'static str = "CSS.RuleMatch";
}
#[doc = "Data for a simple selector (these are delimited by commas in a selector list).\n[Value](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-Value)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    #[doc = "Value text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Value range in the underlying resource (if available)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Specificity of the selector."]
    #[serde(rename = "specificity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub specificity: Option<Specificity>,
}
impl Value {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            range: None,
            specificity: None,
        }
    }
}
impl<T: Into<String>> From<T> for Value {
    fn from(url: T) -> Self {
        Value::new(url)
    }
}
impl Value {
    pub const IDENTIFIER: &'static str = "CSS.Value";
}
#[doc = "Specificity:\nhttps://drafts.csswg.org/selectors/#specificity-rules\n[Specificity](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-Specificity)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specificity {
    #[doc = "The a component, which represents the number of ID selectors."]
    #[serde(rename = "a")]
    pub a: i64,
    #[doc = "The b component, which represents the number of class selectors, attributes selectors, and\npseudo-classes."]
    #[serde(rename = "b")]
    pub b: i64,
    #[doc = "The c component, which represents the number of type selectors and pseudo-elements."]
    #[serde(rename = "c")]
    pub c: i64,
}
impl Specificity {
    pub fn new(a: impl Into<i64>, b: impl Into<i64>, c: impl Into<i64>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}
impl Specificity {
    pub const IDENTIFIER: &'static str = "CSS.Specificity";
}
#[doc = "Selector list data.\n[SelectorList](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-SelectorList)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectorList {
    #[doc = "Selectors in the list."]
    #[serde(rename = "selectors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<Value>,
    #[doc = "Rule selector text."]
    #[serde(rename = "text")]
    pub text: String,
}
impl SelectorList {
    pub fn new(selectors: Vec<Value>, text: impl Into<String>) -> Self {
        Self {
            selectors,
            text: text.into(),
        }
    }
}
impl SelectorList {
    pub const IDENTIFIER: &'static str = "CSS.SelectorList";
}
#[doc = "CSS stylesheet metainformation.\n[CSSStyleSheetHeader](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStyleSheetHeader)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssStyleSheetHeader {
    #[doc = "The stylesheet identifier."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: super::super::dom::types::StyleSheetId,
    #[doc = "Owner frame identifier."]
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
    #[doc = "Stylesheet resource URL. Empty if this is a constructed stylesheet created using\nnew CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported\nas a CSS module script)."]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[doc = "URL of source map associated with the stylesheet (if any)."]
    #[serde(rename = "sourceMapURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_map_url: Option<String>,
    #[doc = "Stylesheet origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Stylesheet title."]
    #[serde(rename = "title")]
    pub title: String,
    #[doc = "The backend id for the owner node of the stylesheet."]
    #[serde(rename = "ownerNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub owner_node: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "Denotes whether the stylesheet is disabled."]
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[doc = "Whether the sourceURL field value comes from the sourceURL comment."]
    #[serde(rename = "hasSourceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_source_url: Option<bool>,
    #[doc = "Whether this stylesheet is created for STYLE tag by parser. This flag is not set for\ndocument.written STYLE tags."]
    #[serde(rename = "isInline")]
    pub is_inline: bool,
    #[doc = "Whether this stylesheet is mutable. Inline stylesheets become mutable\nafter they have been modified via CSSOM API.\n`<link>` element's stylesheets become mutable only if DevTools modifies them.\nConstructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation."]
    #[serde(rename = "isMutable")]
    pub is_mutable: bool,
    #[doc = "True if this stylesheet is created through new CSSStyleSheet() or imported as a\nCSS module script."]
    #[serde(rename = "isConstructed")]
    pub is_constructed: bool,
    #[doc = "Line offset of the stylesheet within the resource (zero based)."]
    #[serde(rename = "startLine")]
    pub start_line: f64,
    #[doc = "Column offset of the stylesheet within the resource (zero based)."]
    #[serde(rename = "startColumn")]
    pub start_column: f64,
    #[doc = "Size of the content (in characters)."]
    #[serde(rename = "length")]
    pub length: f64,
    #[doc = "Line offset of the end of the stylesheet within the resource (zero based)."]
    #[serde(rename = "endLine")]
    pub end_line: f64,
    #[doc = "Column offset of the end of the stylesheet within the resource (zero based)."]
    #[serde(rename = "endColumn")]
    pub end_column: f64,
    #[doc = "If the style sheet was loaded from a network resource, this indicates when the resource failed to load"]
    #[serde(rename = "loadingFailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub loading_failed: Option<bool>,
}
impl CssStyleSheetHeader {
    pub const IDENTIFIER: &'static str = "CSS.CSSStyleSheetHeader";
}
#[doc = "CSS rule representation.\n[CSSRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssRule {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Rule selector data."]
    #[serde(rename = "selectorList")]
    pub selector_list: SelectorList,
    #[doc = "Array of selectors from ancestor style rules, sorted by distance from the current rule."]
    #[serde(rename = "nestingSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nesting_selectors: Option<Vec<String>>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    #[serde(rename = "style")]
    pub style: CssStyle,
    #[doc = "The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule."]
    #[serde(rename = "originTreeScopeNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin_tree_scope_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "Media list array (for rules involving media queries). The array enumerates media queries\nstarting with the innermost one, going outwards."]
    #[serde(rename = "media")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub media: Option<Vec<CssMedia>>,
    #[doc = "Container query list array (for rules involving container queries).\nThe array enumerates container queries starting with the innermost one, going outwards."]
    #[serde(rename = "containerQueries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_queries: Option<Vec<CssContainerQuery>>,
    #[doc = "@supports CSS at-rule array.\nThe array enumerates @supports at-rules starting with the innermost one, going outwards."]
    #[serde(rename = "supports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub supports: Option<Vec<CssSupports>>,
    #[doc = "Cascade layer array. Contains the layer hierarchy that this rule belongs to starting\nwith the innermost layer and going outwards."]
    #[serde(rename = "layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layers: Option<Vec<CssLayer>>,
    #[doc = "@scope CSS at-rule array.\nThe array enumerates @scope at-rules starting with the innermost one, going outwards."]
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scopes: Option<Vec<CssScope>>,
    #[doc = "The array keeps the types of ancestor CSSRules from the innermost going outwards."]
    #[serde(rename = "ruleTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rule_types: Option<Vec<CssRuleType>>,
    #[doc = "@starting-style CSS at-rule array.\nThe array enumerates @starting-style at-rules starting with the innermost one, going outwards."]
    #[serde(rename = "startingStyles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub starting_styles: Option<Vec<CssStartingStyle>>,
}
impl CssRule {
    pub fn new(
        selector_list: impl Into<SelectorList>,
        origin: impl Into<StyleSheetOrigin>,
        style: impl Into<CssStyle>,
    ) -> Self {
        Self {
            selector_list: selector_list.into(),
            origin: origin.into(),
            style: style.into(),
            style_sheet_id: None,
            nesting_selectors: None,
            origin_tree_scope_node_id: None,
            media: None,
            container_queries: None,
            supports: None,
            layers: None,
            scopes: None,
            rule_types: None,
            starting_styles: None,
        }
    }
}
impl CssRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSRule";
}
#[doc = "Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.\nThis list only contains rule types that are collected during the ancestor rule collection."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CssRuleType {
    #[serde(rename = "MediaRule")]
    MediaRule,
    #[serde(rename = "SupportsRule")]
    SupportsRule,
    #[serde(rename = "ContainerRule")]
    ContainerRule,
    #[serde(rename = "LayerRule")]
    LayerRule,
    #[serde(rename = "ScopeRule")]
    ScopeRule,
    #[serde(rename = "StyleRule")]
    StyleRule,
    #[serde(rename = "StartingStyleRule")]
    StartingStyleRule,
}
#[doc = "CSS coverage information.\n[RuleUsage](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-RuleUsage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleUsage {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: super::super::dom::types::StyleSheetId,
    #[doc = "Offset of the start of the rule (including selector) from the beginning of the stylesheet."]
    #[serde(rename = "startOffset")]
    pub start_offset: f64,
    #[doc = "Offset of the end of the rule body from the beginning of the stylesheet."]
    #[serde(rename = "endOffset")]
    pub end_offset: f64,
    #[doc = "Indicates whether the rule was actually used by some element in the page."]
    #[serde(rename = "used")]
    pub used: bool,
}
impl RuleUsage {
    pub fn new(
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
        start_offset: impl Into<f64>,
        end_offset: impl Into<f64>,
        used: impl Into<bool>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            start_offset: start_offset.into(),
            end_offset: end_offset.into(),
            used: used.into(),
        }
    }
}
impl RuleUsage {
    pub const IDENTIFIER: &'static str = "CSS.RuleUsage";
}
#[doc = "Text range within a resource. All numbers are zero-based.\n[SourceRange](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-SourceRange)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRange {
    #[doc = "Start line of range."]
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[doc = "Start column of range (inclusive)."]
    #[serde(rename = "startColumn")]
    pub start_column: i64,
    #[doc = "End line of range"]
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[doc = "End column of range (exclusive)."]
    #[serde(rename = "endColumn")]
    pub end_column: i64,
}
impl SourceRange {
    pub fn new(
        start_line: impl Into<i64>,
        start_column: impl Into<i64>,
        end_line: impl Into<i64>,
        end_column: impl Into<i64>,
    ) -> Self {
        Self {
            start_line: start_line.into(),
            start_column: start_column.into(),
            end_line: end_line.into(),
            end_column: end_column.into(),
        }
    }
}
impl SourceRange {
    pub const IDENTIFIER: &'static str = "CSS.SourceRange";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShorthandEntry {
    #[doc = "Shorthand name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Shorthand value."]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "Whether the property has \"!important\" annotation (implies `false` if absent)."]
    #[serde(rename = "important")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub important: Option<bool>,
}
impl ShorthandEntry {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            important: None,
        }
    }
}
impl ShorthandEntry {
    pub const IDENTIFIER: &'static str = "CSS.ShorthandEntry";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssComputedStyleProperty {
    #[doc = "Computed style property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Computed style property value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl CssComputedStyleProperty {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl CssComputedStyleProperty {
    pub const IDENTIFIER: &'static str = "CSS.CSSComputedStyleProperty";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputedStyleExtraFields {
    #[doc = "Returns whether or not this node is being rendered with base appearance,\nwhich happens when it has its appearance property set to base/base-select\nor it is in the subtree of an element being rendered with base appearance."]
    #[serde(rename = "isAppearanceBase")]
    pub is_appearance_base: bool,
}
impl ComputedStyleExtraFields {
    pub fn new(is_appearance_base: impl Into<bool>) -> Self {
        Self {
            is_appearance_base: is_appearance_base.into(),
        }
    }
}
impl ComputedStyleExtraFields {
    pub const IDENTIFIER: &'static str = "CSS.ComputedStyleExtraFields";
}
#[doc = "CSS style representation.\n[CSSStyle](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssStyle {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "CSS properties in the style."]
    #[serde(rename = "cssProperties")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub css_properties: Vec<CssProperty>,
    #[doc = "Computed values for all shorthands found in the style."]
    #[serde(rename = "shorthandEntries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shorthand_entries: Vec<ShorthandEntry>,
    #[doc = "Style declaration text (if available)."]
    #[serde(rename = "cssText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_text: Option<String>,
    #[doc = "Style declaration range in the enclosing stylesheet (if available)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
}
impl CssStyle {
    pub fn new(css_properties: Vec<CssProperty>, shorthand_entries: Vec<ShorthandEntry>) -> Self {
        Self {
            css_properties,
            shorthand_entries,
            style_sheet_id: None,
            css_text: None,
            range: None,
        }
    }
}
impl CssStyle {
    pub const IDENTIFIER: &'static str = "CSS.CSSStyle";
}
#[doc = "CSS property declaration data.\n[CSSProperty](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSProperty)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssProperty {
    #[doc = "The property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The property value."]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "Whether the property has \"!important\" annotation (implies `false` if absent)."]
    #[serde(rename = "important")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub important: Option<bool>,
    #[doc = "Whether the property is implicit (implies `false` if absent)."]
    #[serde(rename = "implicit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub implicit: Option<bool>,
    #[doc = "The full property text as specified in the style."]
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,
    #[doc = "Whether the property is understood by the browser (implies `true` if absent)."]
    #[serde(rename = "parsedOk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parsed_ok: Option<bool>,
    #[doc = "Whether the property is disabled by the user (present for source-based properties only)."]
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub disabled: Option<bool>,
    #[doc = "The entire property range in the enclosing style declaration (if available)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Parsed longhand components of this property if it is a shorthand.\nThis field will be empty if the given property is not a shorthand."]
    #[serde(rename = "longhandProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub longhand_properties: Option<Vec<CssProperty>>,
}
impl CssProperty {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            important: None,
            implicit: None,
            text: None,
            parsed_ok: None,
            disabled: None,
            range: None,
            longhand_properties: None,
        }
    }
}
impl CssProperty {
    pub const IDENTIFIER: &'static str = "CSS.CSSProperty";
}
#[doc = "CSS media rule descriptor.\n[CSSMedia](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSMedia)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssMedia {
    #[doc = "Media query text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Source of the media query: \"mediaRule\" if specified by a @media rule, \"importRule\" if\nspecified by an @import rule, \"linkedSheet\" if specified by a \"media\" attribute in a linked\nstylesheet's LINK tag, \"inlineSheet\" if specified by a \"media\" attribute in an inline\nstylesheet's STYLE tag."]
    #[serde(rename = "source")]
    pub source: CssMediaSource,
    #[doc = "URL of the document containing the media query description."]
    #[serde(rename = "sourceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_url: Option<String>,
    #[doc = "The associated rule (@media or @import) header range in the enclosing stylesheet (if\navailable)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Array of media queries."]
    #[serde(rename = "mediaList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub media_list: Option<Vec<MediaQuery>>,
}
#[doc = "Source of the media query: \"mediaRule\" if specified by a @media rule, \"importRule\" if\nspecified by an @import rule, \"linkedSheet\" if specified by a \"media\" attribute in a linked\nstylesheet's LINK tag, \"inlineSheet\" if specified by a \"media\" attribute in an inline\nstylesheet's STYLE tag."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CssMediaSource {
    #[serde(rename = "mediaRule")]
    MediaRule,
    #[serde(rename = "importRule")]
    ImportRule,
    #[serde(rename = "linkedSheet")]
    LinkedSheet,
    #[serde(rename = "inlineSheet")]
    InlineSheet,
}
impl CssMedia {
    pub fn new(text: impl Into<String>, source: impl Into<CssMediaSource>) -> Self {
        Self {
            text: text.into(),
            source: source.into(),
            source_url: None,
            range: None,
            style_sheet_id: None,
            media_list: None,
        }
    }
}
impl CssMedia {
    pub const IDENTIFIER: &'static str = "CSS.CSSMedia";
}
#[doc = "Media query descriptor.\n[MediaQuery](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-MediaQuery)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaQuery {
    #[doc = "Array of media query expressions."]
    #[serde(rename = "expressions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub expressions: Vec<MediaQueryExpression>,
    #[doc = "Whether the media query condition is satisfied."]
    #[serde(rename = "active")]
    pub active: bool,
}
impl MediaQuery {
    pub fn new(expressions: Vec<MediaQueryExpression>, active: impl Into<bool>) -> Self {
        Self {
            expressions,
            active: active.into(),
        }
    }
}
impl MediaQuery {
    pub const IDENTIFIER: &'static str = "CSS.MediaQuery";
}
#[doc = "Media query expression descriptor.\n[MediaQueryExpression](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-MediaQueryExpression)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaQueryExpression {
    #[doc = "Media query expression value."]
    #[serde(rename = "value")]
    pub value: f64,
    #[doc = "Media query expression units."]
    #[serde(rename = "unit")]
    pub unit: String,
    #[doc = "Media query expression feature."]
    #[serde(rename = "feature")]
    pub feature: String,
    #[doc = "The associated range of the value text in the enclosing stylesheet (if available)."]
    #[serde(rename = "valueRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value_range: Option<SourceRange>,
    #[doc = "Computed length of media query expression (if applicable)."]
    #[serde(rename = "computedLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub computed_length: Option<f64>,
}
impl MediaQueryExpression {
    pub fn new(value: impl Into<f64>, unit: impl Into<String>, feature: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            unit: unit.into(),
            feature: feature.into(),
            value_range: None,
            computed_length: None,
        }
    }
}
impl MediaQueryExpression {
    pub const IDENTIFIER: &'static str = "CSS.MediaQueryExpression";
}
#[doc = "CSS container query rule descriptor.\n[CSSContainerQuery](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSContainerQuery)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssContainerQuery {
    #[doc = "Container query text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "The associated rule header range in the enclosing stylesheet (if\navailable)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Optional name for the container."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[doc = "Optional physical axes queried for the container."]
    #[serde(rename = "physicalAxes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub physical_axes: Option<super::super::dom::types::PhysicalAxes>,
    #[doc = "Optional logical axes queried for the container."]
    #[serde(rename = "logicalAxes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logical_axes: Option<super::super::dom::types::LogicalAxes>,
    #[doc = "true if the query contains scroll-state() queries."]
    #[serde(rename = "queriesScrollState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub queries_scroll_state: Option<bool>,
    #[doc = "true if the query contains anchored() queries."]
    #[serde(rename = "queriesAnchored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub queries_anchored: Option<bool>,
}
impl CssContainerQuery {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            range: None,
            style_sheet_id: None,
            name: None,
            physical_axes: None,
            logical_axes: None,
            queries_scroll_state: None,
            queries_anchored: None,
        }
    }
}
impl<T: Into<String>> From<T> for CssContainerQuery {
    fn from(url: T) -> Self {
        CssContainerQuery::new(url)
    }
}
impl CssContainerQuery {
    pub const IDENTIFIER: &'static str = "CSS.CSSContainerQuery";
}
#[doc = "CSS Supports at-rule descriptor.\n[CSSSupports](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSSupports)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssSupports {
    #[doc = "Supports rule text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Whether the supports condition is satisfied."]
    #[serde(rename = "active")]
    pub active: bool,
    #[doc = "The associated rule header range in the enclosing stylesheet (if\navailable)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
}
impl CssSupports {
    pub fn new(text: impl Into<String>, active: impl Into<bool>) -> Self {
        Self {
            text: text.into(),
            active: active.into(),
            range: None,
            style_sheet_id: None,
        }
    }
}
impl CssSupports {
    pub const IDENTIFIER: &'static str = "CSS.CSSSupports";
}
#[doc = "CSS Scope at-rule descriptor.\n[CSSScope](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSScope)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssScope {
    #[doc = "Scope rule text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "The associated rule header range in the enclosing stylesheet (if\navailable)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
}
impl CssScope {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            range: None,
            style_sheet_id: None,
        }
    }
}
impl<T: Into<String>> From<T> for CssScope {
    fn from(url: T) -> Self {
        CssScope::new(url)
    }
}
impl CssScope {
    pub const IDENTIFIER: &'static str = "CSS.CSSScope";
}
#[doc = "CSS Layer at-rule descriptor.\n[CSSLayer](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSLayer)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssLayer {
    #[doc = "Layer name."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "The associated rule header range in the enclosing stylesheet (if\navailable)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
}
impl CssLayer {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            range: None,
            style_sheet_id: None,
        }
    }
}
impl<T: Into<String>> From<T> for CssLayer {
    fn from(url: T) -> Self {
        CssLayer::new(url)
    }
}
impl CssLayer {
    pub const IDENTIFIER: &'static str = "CSS.CSSLayer";
}
#[doc = "CSS Starting Style at-rule descriptor.\n[CSSStartingStyle](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSStartingStyle)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CssStartingStyle {
    #[doc = "The associated rule header range in the enclosing stylesheet (if\navailable)."]
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub range: Option<SourceRange>,
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
}
impl CssStartingStyle {
    pub const IDENTIFIER: &'static str = "CSS.CSSStartingStyle";
}
#[doc = "CSS Layer data.\n[CSSLayerData](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSLayerData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssLayerData {
    #[doc = "Layer name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Direct sub-layers"]
    #[serde(rename = "subLayers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sub_layers: Option<Vec<CssLayerData>>,
    #[doc = "Layer order. The order determines the order of the layer in the cascade order.\nA higher number has higher priority in the cascade order."]
    #[serde(rename = "order")]
    pub order: f64,
}
impl CssLayerData {
    pub fn new(name: impl Into<String>, order: impl Into<f64>) -> Self {
        Self {
            name: name.into(),
            order: order.into(),
            sub_layers: None,
        }
    }
}
impl CssLayerData {
    pub const IDENTIFIER: &'static str = "CSS.CSSLayerData";
}
#[doc = "Information about amount of glyphs that were rendered with given font.\n[PlatformFontUsage](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-PlatformFontUsage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformFontUsage {
    #[doc = "Font's family name reported by platform."]
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[doc = "Font's PostScript name reported by platform."]
    #[serde(rename = "postScriptName")]
    pub post_script_name: String,
    #[doc = "Indicates if the font was downloaded or resolved locally."]
    #[serde(rename = "isCustomFont")]
    pub is_custom_font: bool,
    #[doc = "Amount of glyphs that were rendered with this font."]
    #[serde(rename = "glyphCount")]
    pub glyph_count: f64,
}
impl PlatformFontUsage {
    pub fn new(
        family_name: impl Into<String>,
        post_script_name: impl Into<String>,
        is_custom_font: impl Into<bool>,
        glyph_count: impl Into<f64>,
    ) -> Self {
        Self {
            family_name: family_name.into(),
            post_script_name: post_script_name.into(),
            is_custom_font: is_custom_font.into(),
            glyph_count: glyph_count.into(),
        }
    }
}
impl PlatformFontUsage {
    pub const IDENTIFIER: &'static str = "CSS.PlatformFontUsage";
}
#[doc = "Information about font variation axes for variable fonts\n[FontVariationAxis](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-FontVariationAxis)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontVariationAxis {
    #[doc = "The font-variation-setting tag (a.k.a. \"axis tag\")."]
    #[serde(rename = "tag")]
    pub tag: String,
    #[doc = "Human-readable variation name in the default language (normally, \"en\")."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The minimum value (inclusive) the font supports for this tag."]
    #[serde(rename = "minValue")]
    pub min_value: f64,
    #[doc = "The maximum value (inclusive) the font supports for this tag."]
    #[serde(rename = "maxValue")]
    pub max_value: f64,
    #[doc = "The default value."]
    #[serde(rename = "defaultValue")]
    pub default_value: f64,
}
impl FontVariationAxis {
    pub const IDENTIFIER: &'static str = "CSS.FontVariationAxis";
}
#[doc = "Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions\nand additional information such as platformFontFamily and fontVariationAxes.\n[FontFace](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-FontFace)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontFace {
    #[doc = "The font-family."]
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[doc = "The font-style."]
    #[serde(rename = "fontStyle")]
    pub font_style: String,
    #[doc = "The font-variant."]
    #[serde(rename = "fontVariant")]
    pub font_variant: String,
    #[doc = "The font-weight."]
    #[serde(rename = "fontWeight")]
    pub font_weight: String,
    #[doc = "The font-stretch."]
    #[serde(rename = "fontStretch")]
    pub font_stretch: String,
    #[doc = "The font-display."]
    #[serde(rename = "fontDisplay")]
    pub font_display: String,
    #[doc = "The unicode-range."]
    #[serde(rename = "unicodeRange")]
    pub unicode_range: String,
    #[doc = "The src."]
    #[serde(rename = "src")]
    pub src: String,
    #[doc = "The resolved platform font family"]
    #[serde(rename = "platformFontFamily")]
    pub platform_font_family: String,
    #[doc = "Available variation settings (a.k.a. \"axes\")."]
    #[serde(rename = "fontVariationAxes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub font_variation_axes: Option<Vec<FontVariationAxis>>,
}
impl FontFace {
    pub const IDENTIFIER: &'static str = "CSS.FontFace";
}
#[doc = "CSS try rule representation.\n[CSSTryRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSTryRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssTryRule {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    #[serde(rename = "style")]
    pub style: CssStyle,
}
impl CssTryRule {
    pub fn new(origin: impl Into<StyleSheetOrigin>, style: impl Into<CssStyle>) -> Self {
        Self {
            origin: origin.into(),
            style: style.into(),
            style_sheet_id: None,
        }
    }
}
impl CssTryRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSTryRule";
}
#[doc = "CSS @position-try rule representation.\n[CSSPositionTryRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSPositionTryRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssPositionTryRule {
    #[doc = "The prelude dashed-ident name"]
    #[serde(rename = "name")]
    pub name: Value,
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    #[serde(rename = "style")]
    pub style: CssStyle,
    #[serde(rename = "active")]
    pub active: bool,
}
impl CssPositionTryRule {
    pub fn new(
        name: impl Into<Value>,
        origin: impl Into<StyleSheetOrigin>,
        style: impl Into<CssStyle>,
        active: impl Into<bool>,
    ) -> Self {
        Self {
            name: name.into(),
            origin: origin.into(),
            style: style.into(),
            active: active.into(),
            style_sheet_id: None,
        }
    }
}
impl CssPositionTryRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSPositionTryRule";
}
#[doc = "CSS keyframes rule representation.\n[CSSKeyframesRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSKeyframesRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssKeyframesRule {
    #[doc = "Animation name."]
    #[serde(rename = "animationName")]
    pub animation_name: Value,
    #[doc = "List of keyframes."]
    #[serde(rename = "keyframes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keyframes: Vec<CssKeyframeRule>,
}
impl CssKeyframesRule {
    pub fn new(animation_name: impl Into<Value>, keyframes: Vec<CssKeyframeRule>) -> Self {
        Self {
            animation_name: animation_name.into(),
            keyframes,
        }
    }
}
impl CssKeyframesRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSKeyframesRule";
}
#[doc = "Representation of a custom property registration through CSS.registerProperty\n[CSSPropertyRegistration](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSPropertyRegistration)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssPropertyRegistration {
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[serde(rename = "initialValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initial_value: Option<Value>,
    #[serde(rename = "inherits")]
    pub inherits: bool,
    #[serde(rename = "syntax")]
    pub syntax: String,
}
impl CssPropertyRegistration {
    pub fn new(
        property_name: impl Into<String>,
        inherits: impl Into<bool>,
        syntax: impl Into<String>,
    ) -> Self {
        Self {
            property_name: property_name.into(),
            inherits: inherits.into(),
            syntax: syntax.into(),
            initial_value: None,
        }
    }
}
impl CssPropertyRegistration {
    pub const IDENTIFIER: &'static str = "CSS.CSSPropertyRegistration";
}
#[doc = "CSS generic @rule representation.\n[CSSAtRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSAtRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssAtRule {
    #[doc = "Type of at-rule."]
    #[serde(rename = "type")]
    pub r#type: CssAtRuleType,
    #[doc = "Subsection of font-feature-values, if this is a subsection."]
    #[serde(rename = "subsection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subsection: Option<CssAtRuleSubsection>,
    #[doc = "LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)\nAssociated name, if applicable."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<Value>,
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    #[serde(rename = "style")]
    pub style: CssStyle,
}
#[doc = "Type of at-rule."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CssAtRuleType {
    #[serde(rename = "font-face")]
    FontFace,
    #[serde(rename = "font-feature-values")]
    FontFeatureValues,
    #[serde(rename = "font-palette-values")]
    FontPaletteValues,
}
#[doc = "Subsection of font-feature-values, if this is a subsection."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CssAtRuleSubsection {
    #[doc = "LINT.IfChange(FontVariantAlternatesFeatureType)"]
    #[serde(rename = "swash")]
    Swash,
    #[serde(rename = "annotation")]
    Annotation,
    #[serde(rename = "ornaments")]
    Ornaments,
    #[serde(rename = "stylistic")]
    Stylistic,
    #[serde(rename = "styleset")]
    Styleset,
    #[serde(rename = "character-variant")]
    CharacterVariant,
}
impl CssAtRule {
    pub fn new(
        r#type: impl Into<CssAtRuleType>,
        origin: impl Into<StyleSheetOrigin>,
        style: impl Into<CssStyle>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            origin: origin.into(),
            style: style.into(),
            subsection: None,
            name: None,
            style_sheet_id: None,
        }
    }
}
impl CssAtRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSAtRule";
}
#[doc = "CSS property at-rule representation.\n[CSSPropertyRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSPropertyRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssPropertyRule {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated property name."]
    #[serde(rename = "propertyName")]
    pub property_name: Value,
    #[doc = "Associated style declaration."]
    #[serde(rename = "style")]
    pub style: CssStyle,
}
impl CssPropertyRule {
    pub fn new(
        origin: impl Into<StyleSheetOrigin>,
        property_name: impl Into<Value>,
        style: impl Into<CssStyle>,
    ) -> Self {
        Self {
            origin: origin.into(),
            property_name: property_name.into(),
            style: style.into(),
            style_sheet_id: None,
        }
    }
}
impl CssPropertyRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSPropertyRule";
}
#[doc = "CSS function argument representation.\n[CSSFunctionParameter](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionParameter)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssFunctionParameter {
    #[doc = "The parameter name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The parameter type."]
    #[serde(rename = "type")]
    pub r#type: String,
}
impl CssFunctionParameter {
    pub fn new(name: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            r#type: r#type.into(),
        }
    }
}
impl CssFunctionParameter {
    pub const IDENTIFIER: &'static str = "CSS.CSSFunctionParameter";
}
#[doc = "CSS function conditional block representation.\n[CSSFunctionConditionNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionConditionNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssFunctionConditionNode {
    #[doc = "Media query for this conditional block. Only one type of condition should be set."]
    #[serde(rename = "media")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub media: Option<CssMedia>,
    #[doc = "Container query for this conditional block. Only one type of condition should be set."]
    #[serde(rename = "containerQueries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_queries: Option<CssContainerQuery>,
    #[doc = "@supports CSS at-rule condition. Only one type of condition should be set."]
    #[serde(rename = "supports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub supports: Option<CssSupports>,
    #[doc = "Block body."]
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CssFunctionNode>,
    #[doc = "The condition text."]
    #[serde(rename = "conditionText")]
    pub condition_text: String,
}
impl CssFunctionConditionNode {
    pub fn new(children: Vec<CssFunctionNode>, condition_text: impl Into<String>) -> Self {
        Self {
            children,
            condition_text: condition_text.into(),
            media: None,
            container_queries: None,
            supports: None,
        }
    }
}
impl CssFunctionConditionNode {
    pub const IDENTIFIER: &'static str = "CSS.CSSFunctionConditionNode";
}
#[doc = "Section of the body of a CSS function rule.\n[CSSFunctionNode](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CssFunctionNode {
    #[doc = "A conditional block. If set, style should not be set."]
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub condition: Option<CssFunctionConditionNode>,
    #[doc = "Values set by this node. If set, condition should not be set."]
    #[serde(rename = "style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style: Option<CssStyle>,
}
impl CssFunctionNode {
    pub const IDENTIFIER: &'static str = "CSS.CSSFunctionNode";
}
#[doc = "CSS function at-rule representation.\n[CSSFunctionRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSFunctionRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssFunctionRule {
    #[doc = "Name of the function."]
    #[serde(rename = "name")]
    pub name: Value,
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "List of parameters."]
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<CssFunctionParameter>,
    #[doc = "Function body."]
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CssFunctionNode>,
}
impl CssFunctionRule {
    pub fn new(
        name: impl Into<Value>,
        origin: impl Into<StyleSheetOrigin>,
        parameters: Vec<CssFunctionParameter>,
        children: Vec<CssFunctionNode>,
    ) -> Self {
        Self {
            name: name.into(),
            origin: origin.into(),
            parameters,
            children,
            style_sheet_id: None,
        }
    }
}
impl CssFunctionRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSFunctionRule";
}
#[doc = "CSS keyframe rule representation.\n[CSSKeyframeRule](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-CSSKeyframeRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssKeyframeRule {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\nstylesheet rules) this rule came from."]
    #[serde(rename = "styleSheetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_sheet_id: Option<super::super::dom::types::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated key text."]
    #[serde(rename = "keyText")]
    pub key_text: Value,
    #[doc = "Associated style declaration."]
    #[serde(rename = "style")]
    pub style: CssStyle,
}
impl CssKeyframeRule {
    pub fn new(
        origin: impl Into<StyleSheetOrigin>,
        key_text: impl Into<Value>,
        style: impl Into<CssStyle>,
    ) -> Self {
        Self {
            origin: origin.into(),
            key_text: key_text.into(),
            style: style.into(),
            style_sheet_id: None,
        }
    }
}
impl CssKeyframeRule {
    pub const IDENTIFIER: &'static str = "CSS.CSSKeyframeRule";
}
#[doc = "A descriptor of operation to mutate style declaration text.\n[StyleDeclarationEdit](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#type-StyleDeclarationEdit)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleDeclarationEdit {
    #[doc = "The css style sheet identifier."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: super::super::dom::types::StyleSheetId,
    #[doc = "The range of the style text in the enclosing stylesheet."]
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[doc = "New style text."]
    #[serde(rename = "text")]
    pub text: String,
}
impl StyleDeclarationEdit {
    pub fn new(
        style_sheet_id: impl Into<super::super::dom::types::StyleSheetId>,
        range: impl Into<SourceRange>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            style_sheet_id: style_sheet_id.into(),
            range: range.into(),
            text: text.into(),
        }
    }
}
impl StyleDeclarationEdit {
    pub const IDENTIFIER: &'static str = "CSS.StyleDeclarationEdit";
}
group_enum ! (Type { StyleSheetOrigin (StyleSheetOrigin) , PseudoElementMatches (PseudoElementMatches) , CssAnimationStyle (CssAnimationStyle) , InheritedStyleEntry (InheritedStyleEntry) , InheritedAnimatedStyleEntry (InheritedAnimatedStyleEntry) , InheritedPseudoElementMatches (InheritedPseudoElementMatches) , RuleMatch (RuleMatch) , Value (Value) , Specificity (Specificity) , SelectorList (SelectorList) , CssStyleSheetHeader (CssStyleSheetHeader) , CssRule (CssRule) , CssRuleType (CssRuleType) , RuleUsage (RuleUsage) , SourceRange (SourceRange) , ShorthandEntry (ShorthandEntry) , CssComputedStyleProperty (CssComputedStyleProperty) , ComputedStyleExtraFields (ComputedStyleExtraFields) , CssStyle (CssStyle) , CssProperty (CssProperty) , CssMedia (CssMedia) , MediaQuery (MediaQuery) , MediaQueryExpression (MediaQueryExpression) , CssContainerQuery (CssContainerQuery) , CssSupports (CssSupports) , CssScope (CssScope) , CssLayer (CssLayer) , CssStartingStyle (CssStartingStyle) , CssLayerData (CssLayerData) , PlatformFontUsage (PlatformFontUsage) , FontVariationAxis (FontVariationAxis) , FontFace (FontFace) , CssTryRule (CssTryRule) , CssPositionTryRule (CssPositionTryRule) , CssKeyframesRule (CssKeyframesRule) , CssPropertyRegistration (CssPropertyRegistration) , CssAtRule (CssAtRule) , CssPropertyRule (CssPropertyRule) , CssFunctionParameter (CssFunctionParameter) , CssFunctionConditionNode (CssFunctionConditionNode) , CssFunctionNode (CssFunctionNode) , CssFunctionRule (CssFunctionRule) , CssKeyframeRule (CssKeyframeRule) , StyleDeclarationEdit (StyleDeclarationEdit) });
