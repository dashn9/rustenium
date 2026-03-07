use super::commands::*;
impl CaptureSnapshot {
    pub fn builder() -> CaptureSnapshotBuilder {
        <CaptureSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CaptureSnapshotBuilder {
    computed_styles: Option<Vec<String>>,
    include_paint_order: Option<bool>,
    include_dom_rects: Option<bool>,
    include_blended_background_colors: Option<bool>,
    include_text_color_opacities: Option<bool>,
}
impl CaptureSnapshotBuilder {
    pub fn computed_style(mut self, computed_style: impl Into<String>) -> Self {
        let v = self.computed_styles.get_or_insert(Vec::new());
        v.push(computed_style.into());
        self
    }
    pub fn computed_styles<I, S>(mut self, computed_styles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.computed_styles.get_or_insert(Vec::new());
        for val in computed_styles {
            v.push(val.into());
        }
        self
    }
    pub fn include_paint_order(mut self, include_paint_order: impl Into<bool>) -> Self {
        self.include_paint_order = Some(include_paint_order.into());
        self
    }
    pub fn include_dom_rects(mut self, include_dom_rects: impl Into<bool>) -> Self {
        self.include_dom_rects = Some(include_dom_rects.into());
        self
    }
    pub fn include_blended_background_colors(
        mut self,
        include_blended_background_colors: impl Into<bool>,
    ) -> Self {
        self.include_blended_background_colors = Some(include_blended_background_colors.into());
        self
    }
    pub fn include_text_color_opacities(
        mut self,
        include_text_color_opacities: impl Into<bool>,
    ) -> Self {
        self.include_text_color_opacities = Some(include_text_color_opacities.into());
        self
    }
    pub fn build(self) -> Result<CaptureSnapshot, String> {
        Ok(CaptureSnapshot {
            method: CaptureSnapshotMethod::CaptureSnapshot,
            params: CaptureSnapshotParams {
                computed_styles: self.computed_styles.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(computed_styles))
                })?,
                include_paint_order: self.include_paint_order,
                include_dom_rects: self.include_dom_rects,
                include_blended_background_colors: self.include_blended_background_colors,
                include_text_color_opacities: self.include_text_color_opacities,
            },
        })
    }
}
