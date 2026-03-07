use super::commands::*;
impl AddScriptToEvaluateOnNewDocument {
    pub fn builder() -> AddScriptToEvaluateOnNewDocumentBuilder {
        <AddScriptToEvaluateOnNewDocumentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddScriptToEvaluateOnNewDocumentBuilder {
    source: Option<String>,
    world_name: Option<String>,
    include_command_line_api: Option<bool>,
    run_immediately: Option<bool>,
}
impl AddScriptToEvaluateOnNewDocumentBuilder {
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn world_name(mut self, world_name: impl Into<String>) -> Self {
        self.world_name = Some(world_name.into());
        self
    }
    pub fn include_command_line_api(mut self, include_command_line_api: impl Into<bool>) -> Self {
        self.include_command_line_api = Some(include_command_line_api.into());
        self
    }
    pub fn run_immediately(mut self, run_immediately: impl Into<bool>) -> Self {
        self.run_immediately = Some(run_immediately.into());
        self
    }
    pub fn build(self) -> Result<AddScriptToEvaluateOnNewDocument, String> {
        Ok(AddScriptToEvaluateOnNewDocument {
            method: AddScriptToEvaluateOnNewDocumentMethod::AddScriptToEvaluateOnNewDocument,
            params: AddScriptToEvaluateOnNewDocumentParams {
                source: self
                    .source
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
                world_name: self.world_name,
                include_command_line_api: self.include_command_line_api,
                run_immediately: self.run_immediately,
            },
        })
    }
}
impl CaptureScreenshot {
    pub fn builder() -> CaptureScreenshotBuilder {
        <CaptureScreenshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CaptureScreenshotBuilder {
    format: Option<CaptureScreenshotFormat>,
    quality: Option<i64>,
    clip: Option<super::types::Viewport>,
    from_surface: Option<bool>,
    capture_beyond_viewport: Option<bool>,
    optimize_for_speed: Option<bool>,
}
impl CaptureScreenshotBuilder {
    pub fn format(mut self, format: impl Into<CaptureScreenshotFormat>) -> Self {
        self.format = Some(format.into());
        self
    }
    pub fn quality(mut self, quality: impl Into<i64>) -> Self {
        self.quality = Some(quality.into());
        self
    }
    pub fn clip(mut self, clip: impl Into<super::types::Viewport>) -> Self {
        self.clip = Some(clip.into());
        self
    }
    pub fn from_surface(mut self, from_surface: impl Into<bool>) -> Self {
        self.from_surface = Some(from_surface.into());
        self
    }
    pub fn capture_beyond_viewport(mut self, capture_beyond_viewport: impl Into<bool>) -> Self {
        self.capture_beyond_viewport = Some(capture_beyond_viewport.into());
        self
    }
    pub fn optimize_for_speed(mut self, optimize_for_speed: impl Into<bool>) -> Self {
        self.optimize_for_speed = Some(optimize_for_speed.into());
        self
    }
    pub fn build(self) -> CaptureScreenshot {
        CaptureScreenshot {
            method: CaptureScreenshotMethod::CaptureScreenshot,
            params: CaptureScreenshotParams {
                format: self.format,
                quality: self.quality,
                clip: self.clip,
                from_surface: self.from_surface,
                capture_beyond_viewport: self.capture_beyond_viewport,
                optimize_for_speed: self.optimize_for_speed,
            },
        }
    }
}
impl CaptureSnapshot {
    pub fn builder() -> CaptureSnapshotBuilder {
        <CaptureSnapshotBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CaptureSnapshotBuilder {
    format: Option<CaptureSnapshotFormat>,
}
impl CaptureSnapshotBuilder {
    pub fn format(mut self, format: impl Into<CaptureSnapshotFormat>) -> Self {
        self.format = Some(format.into());
        self
    }
    pub fn build(self) -> CaptureSnapshot {
        CaptureSnapshot {
            method: CaptureSnapshotMethod::CaptureSnapshot,
            params: CaptureSnapshotParams {
                format: self.format,
            },
        }
    }
}
impl CreateIsolatedWorld {
    pub fn builder() -> CreateIsolatedWorldBuilder {
        <CreateIsolatedWorldBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CreateIsolatedWorldBuilder {
    frame_id: Option<super::types::FrameId>,
    world_name: Option<String>,
    grant_univeral_access: Option<bool>,
}
impl CreateIsolatedWorldBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn world_name(mut self, world_name: impl Into<String>) -> Self {
        self.world_name = Some(world_name.into());
        self
    }
    pub fn grant_univeral_access(mut self, grant_univeral_access: impl Into<bool>) -> Self {
        self.grant_univeral_access = Some(grant_univeral_access.into());
        self
    }
    pub fn build(self) -> Result<CreateIsolatedWorld, String> {
        Ok(CreateIsolatedWorld {
            method: CreateIsolatedWorldMethod::CreateIsolatedWorld,
            params: CreateIsolatedWorldParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
                world_name: self.world_name,
                grant_univeral_access: self.grant_univeral_access,
            },
        })
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        <EnableBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    enable_file_chooser_opened_event: Option<bool>,
}
impl EnableBuilder {
    pub fn enable_file_chooser_opened_event(
        mut self,
        enable_file_chooser_opened_event: impl Into<bool>,
    ) -> Self {
        self.enable_file_chooser_opened_event = Some(enable_file_chooser_opened_event.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                enable_file_chooser_opened_event: self.enable_file_chooser_opened_event,
            },
        }
    }
}
impl GetAppManifest {
    pub fn builder() -> GetAppManifestBuilder {
        <GetAppManifestBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAppManifestBuilder {
    manifest_id: Option<String>,
}
impl GetAppManifestBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn build(self) -> GetAppManifest {
        GetAppManifest {
            method: GetAppManifestMethod::GetAppManifest,
            params: GetAppManifestParams {
                manifest_id: self.manifest_id,
            },
        }
    }
}
impl GetAdScriptAncestry {
    pub fn builder() -> GetAdScriptAncestryBuilder {
        <GetAdScriptAncestryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAdScriptAncestryBuilder {
    frame_id: Option<super::types::FrameId>,
}
impl GetAdScriptAncestryBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<GetAdScriptAncestry, String> {
        Ok(GetAdScriptAncestry {
            method: GetAdScriptAncestryMethod::GetAdScriptAncestry,
            params: GetAdScriptAncestryParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
            },
        })
    }
}
impl GetResourceContent {
    pub fn builder() -> GetResourceContentBuilder {
        <GetResourceContentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetResourceContentBuilder {
    frame_id: Option<super::types::FrameId>,
    url: Option<String>,
}
impl GetResourceContentBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<GetResourceContent, String> {
        Ok(GetResourceContent {
            method: GetResourceContentMethod::GetResourceContent,
            params: GetResourceContentParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            },
        })
    }
}
impl HandleJavaScriptDialog {
    pub fn builder() -> HandleJavaScriptDialogBuilder {
        <HandleJavaScriptDialogBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct HandleJavaScriptDialogBuilder {
    accept: Option<bool>,
    prompt_text: Option<String>,
}
impl HandleJavaScriptDialogBuilder {
    pub fn accept(mut self, accept: impl Into<bool>) -> Self {
        self.accept = Some(accept.into());
        self
    }
    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }
    pub fn build(self) -> Result<HandleJavaScriptDialog, String> {
        Ok(HandleJavaScriptDialog {
            method: HandleJavaScriptDialogMethod::HandleJavaScriptDialog,
            params: HandleJavaScriptDialogParams {
                accept: self
                    .accept
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(accept)))?,
                prompt_text: self.prompt_text,
            },
        })
    }
}
impl Navigate {
    pub fn builder() -> NavigateBuilder {
        <NavigateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NavigateBuilder {
    url: Option<String>,
    referrer: Option<String>,
    transition_type: Option<super::types::TransitionType>,
    frame_id: Option<super::types::FrameId>,
    referrer_policy: Option<super::types::ReferrerPolicy>,
}
impl NavigateBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn referrer(mut self, referrer: impl Into<String>) -> Self {
        self.referrer = Some(referrer.into());
        self
    }
    pub fn transition_type(
        mut self,
        transition_type: impl Into<super::types::TransitionType>,
    ) -> Self {
        self.transition_type = Some(transition_type.into());
        self
    }
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn referrer_policy(
        mut self,
        referrer_policy: impl Into<super::types::ReferrerPolicy>,
    ) -> Self {
        self.referrer_policy = Some(referrer_policy.into());
        self
    }
    pub fn build(self) -> Result<Navigate, String> {
        Ok(Navigate {
            method: NavigateMethod::Navigate,
            params: NavigateParams {
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
                referrer: self.referrer,
                transition_type: self.transition_type,
                frame_id: self.frame_id.map(Box::new),
                referrer_policy: self.referrer_policy,
            },
        })
    }
}
impl NavigateToHistoryEntry {
    pub fn builder() -> NavigateToHistoryEntryBuilder {
        <NavigateToHistoryEntryBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NavigateToHistoryEntryBuilder {
    entry_id: Option<i64>,
}
impl NavigateToHistoryEntryBuilder {
    pub fn entry_id(mut self, entry_id: impl Into<i64>) -> Self {
        self.entry_id = Some(entry_id.into());
        self
    }
    pub fn build(self) -> Result<NavigateToHistoryEntry, String> {
        Ok(NavigateToHistoryEntry {
            method: NavigateToHistoryEntryMethod::NavigateToHistoryEntry,
            params: NavigateToHistoryEntryParams {
                entry_id: self.entry_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(entry_id))
                })?,
            },
        })
    }
}
impl PrintToPdf {
    pub fn builder() -> PrintToPdfBuilder {
        <PrintToPdfBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PrintToPdfBuilder {
    landscape: Option<bool>,
    display_header_footer: Option<bool>,
    print_background: Option<bool>,
    scale: Option<f64>,
    paper_width: Option<f64>,
    paper_height: Option<f64>,
    margin_top: Option<f64>,
    margin_bottom: Option<f64>,
    margin_left: Option<f64>,
    margin_right: Option<f64>,
    page_ranges: Option<String>,
    header_template: Option<String>,
    footer_template: Option<String>,
    prefer_css_page_size: Option<bool>,
    transfer_mode: Option<PrintToPdfTransferMode>,
    generate_tagged_pdf: Option<bool>,
    generate_document_outline: Option<bool>,
}
impl PrintToPdfBuilder {
    pub fn landscape(mut self, landscape: impl Into<bool>) -> Self {
        self.landscape = Some(landscape.into());
        self
    }
    pub fn display_header_footer(mut self, display_header_footer: impl Into<bool>) -> Self {
        self.display_header_footer = Some(display_header_footer.into());
        self
    }
    pub fn print_background(mut self, print_background: impl Into<bool>) -> Self {
        self.print_background = Some(print_background.into());
        self
    }
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn paper_width(mut self, paper_width: impl Into<f64>) -> Self {
        self.paper_width = Some(paper_width.into());
        self
    }
    pub fn paper_height(mut self, paper_height: impl Into<f64>) -> Self {
        self.paper_height = Some(paper_height.into());
        self
    }
    pub fn margin_top(mut self, margin_top: impl Into<f64>) -> Self {
        self.margin_top = Some(margin_top.into());
        self
    }
    pub fn margin_bottom(mut self, margin_bottom: impl Into<f64>) -> Self {
        self.margin_bottom = Some(margin_bottom.into());
        self
    }
    pub fn margin_left(mut self, margin_left: impl Into<f64>) -> Self {
        self.margin_left = Some(margin_left.into());
        self
    }
    pub fn margin_right(mut self, margin_right: impl Into<f64>) -> Self {
        self.margin_right = Some(margin_right.into());
        self
    }
    pub fn page_ranges(mut self, page_ranges: impl Into<String>) -> Self {
        self.page_ranges = Some(page_ranges.into());
        self
    }
    pub fn header_template(mut self, header_template: impl Into<String>) -> Self {
        self.header_template = Some(header_template.into());
        self
    }
    pub fn footer_template(mut self, footer_template: impl Into<String>) -> Self {
        self.footer_template = Some(footer_template.into());
        self
    }
    pub fn prefer_css_page_size(mut self, prefer_css_page_size: impl Into<bool>) -> Self {
        self.prefer_css_page_size = Some(prefer_css_page_size.into());
        self
    }
    pub fn transfer_mode(mut self, transfer_mode: impl Into<PrintToPdfTransferMode>) -> Self {
        self.transfer_mode = Some(transfer_mode.into());
        self
    }
    pub fn generate_tagged_pdf(mut self, generate_tagged_pdf: impl Into<bool>) -> Self {
        self.generate_tagged_pdf = Some(generate_tagged_pdf.into());
        self
    }
    pub fn generate_document_outline(mut self, generate_document_outline: impl Into<bool>) -> Self {
        self.generate_document_outline = Some(generate_document_outline.into());
        self
    }
    pub fn build(self) -> PrintToPdf {
        PrintToPdf {
            method: PrintToPdfMethod::PrintToPdf,
            params: PrintToPdfParams {
                landscape: self.landscape,
                display_header_footer: self.display_header_footer,
                print_background: self.print_background,
                scale: self.scale,
                paper_width: self.paper_width,
                paper_height: self.paper_height,
                margin_top: self.margin_top,
                margin_bottom: self.margin_bottom,
                margin_left: self.margin_left,
                margin_right: self.margin_right,
                page_ranges: self.page_ranges,
                header_template: self.header_template,
                footer_template: self.footer_template,
                prefer_css_page_size: self.prefer_css_page_size,
                transfer_mode: self.transfer_mode,
                generate_tagged_pdf: self.generate_tagged_pdf,
                generate_document_outline: self.generate_document_outline,
            },
        }
    }
}
impl Reload {
    pub fn builder() -> ReloadBuilder {
        <ReloadBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ReloadBuilder {
    ignore_cache: Option<bool>,
    script_to_evaluate_on_load: Option<String>,
    loader_id: Option<crate::browser_protocol::network::types::LoaderId>,
}
impl ReloadBuilder {
    pub fn ignore_cache(mut self, ignore_cache: impl Into<bool>) -> Self {
        self.ignore_cache = Some(ignore_cache.into());
        self
    }
    pub fn script_to_evaluate_on_load(
        mut self,
        script_to_evaluate_on_load: impl Into<String>,
    ) -> Self {
        self.script_to_evaluate_on_load = Some(script_to_evaluate_on_load.into());
        self
    }
    pub fn loader_id(
        mut self,
        loader_id: impl Into<crate::browser_protocol::network::types::LoaderId>,
    ) -> Self {
        self.loader_id = Some(loader_id.into());
        self
    }
    pub fn build(self) -> Reload {
        Reload {
            method: ReloadMethod::Reload,
            params: ReloadParams {
                ignore_cache: self.ignore_cache,
                script_to_evaluate_on_load: self.script_to_evaluate_on_load,
                loader_id: self.loader_id,
            },
        }
    }
}
impl RemoveScriptToEvaluateOnNewDocument {
    pub fn builder() -> RemoveScriptToEvaluateOnNewDocumentBuilder {
        <RemoveScriptToEvaluateOnNewDocumentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveScriptToEvaluateOnNewDocumentBuilder {
    identifier: Option<super::types::ScriptIdentifier>,
}
impl RemoveScriptToEvaluateOnNewDocumentBuilder {
    pub fn identifier(mut self, identifier: impl Into<super::types::ScriptIdentifier>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }
    pub fn build(self) -> Result<RemoveScriptToEvaluateOnNewDocument, String> {
        Ok(RemoveScriptToEvaluateOnNewDocument {
            method: RemoveScriptToEvaluateOnNewDocumentMethod::RemoveScriptToEvaluateOnNewDocument,
            params: RemoveScriptToEvaluateOnNewDocumentParams {
                identifier: self.identifier.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(identifier))
                })?,
            },
        })
    }
}
impl ScreencastFrameAck {
    pub fn builder() -> ScreencastFrameAckBuilder {
        <ScreencastFrameAckBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreencastFrameAckBuilder {
    session_id: Option<i64>,
}
impl ScreencastFrameAckBuilder {
    pub fn session_id(mut self, session_id: impl Into<i64>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }
    pub fn build(self) -> Result<ScreencastFrameAck, String> {
        Ok(ScreencastFrameAck {
            method: ScreencastFrameAckMethod::ScreencastFrameAck,
            params: ScreencastFrameAckParams {
                session_id: self.session_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(session_id))
                })?,
            },
        })
    }
}
impl SearchInResource {
    pub fn builder() -> SearchInResourceBuilder {
        <SearchInResourceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SearchInResourceBuilder {
    frame_id: Option<super::types::FrameId>,
    url: Option<String>,
    query: Option<String>,
    case_sensitive: Option<bool>,
    is_regex: Option<bool>,
}
impl SearchInResourceBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn case_sensitive(mut self, case_sensitive: impl Into<bool>) -> Self {
        self.case_sensitive = Some(case_sensitive.into());
        self
    }
    pub fn is_regex(mut self, is_regex: impl Into<bool>) -> Self {
        self.is_regex = Some(is_regex.into());
        self
    }
    pub fn build(self) -> Result<SearchInResource, String> {
        Ok(SearchInResource {
            method: SearchInResourceMethod::SearchInResource,
            params: SearchInResourceParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
                query: self
                    .query
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(query)))?,
                case_sensitive: self.case_sensitive,
                is_regex: self.is_regex,
            },
        })
    }
}
impl SetAdBlockingEnabled {
    pub fn builder() -> SetAdBlockingEnabledBuilder {
        <SetAdBlockingEnabledBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAdBlockingEnabledBuilder {
    enabled: Option<bool>,
}
impl SetAdBlockingEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetAdBlockingEnabled, String> {
        Ok(SetAdBlockingEnabled {
            method: SetAdBlockingEnabledMethod::SetAdBlockingEnabled,
            params: SetAdBlockingEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SetBypassCsp {
    pub fn builder() -> SetBypassCspBuilder {
        <SetBypassCspBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBypassCspBuilder {
    enabled: Option<bool>,
}
impl SetBypassCspBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetBypassCsp, String> {
        Ok(SetBypassCsp {
            method: SetBypassCspMethod::SetBypassCsp,
            params: SetBypassCspParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl GetPermissionsPolicyState {
    pub fn builder() -> GetPermissionsPolicyStateBuilder {
        <GetPermissionsPolicyStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetPermissionsPolicyStateBuilder {
    frame_id: Option<super::types::FrameId>,
}
impl GetPermissionsPolicyStateBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<GetPermissionsPolicyState, String> {
        Ok(GetPermissionsPolicyState {
            method: GetPermissionsPolicyStateMethod::GetPermissionsPolicyState,
            params: GetPermissionsPolicyStateParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
            },
        })
    }
}
impl GetOriginTrials {
    pub fn builder() -> GetOriginTrialsBuilder {
        <GetOriginTrialsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetOriginTrialsBuilder {
    frame_id: Option<super::types::FrameId>,
}
impl GetOriginTrialsBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn build(self) -> Result<GetOriginTrials, String> {
        Ok(GetOriginTrials {
            method: GetOriginTrialsMethod::GetOriginTrials,
            params: GetOriginTrialsParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
            },
        })
    }
}
impl SetFontFamilies {
    pub fn builder() -> SetFontFamiliesBuilder {
        <SetFontFamiliesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetFontFamiliesBuilder {
    font_families: Option<super::types::FontFamilies>,
    for_scripts: Option<Vec<super::types::ScriptFontFamilies>>,
}
impl SetFontFamiliesBuilder {
    pub fn font_families(mut self, font_families: impl Into<super::types::FontFamilies>) -> Self {
        self.font_families = Some(font_families.into());
        self
    }
    pub fn for_script(mut self, for_script: impl Into<super::types::ScriptFontFamilies>) -> Self {
        let v = self.for_scripts.get_or_insert(Vec::new());
        v.push(for_script.into());
        self
    }
    pub fn for_scripts<I, S>(mut self, for_scripts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ScriptFontFamilies>,
    {
        let v = self.for_scripts.get_or_insert(Vec::new());
        for val in for_scripts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetFontFamilies, String> {
        Ok(SetFontFamilies {
            method: SetFontFamiliesMethod::SetFontFamilies,
            params: SetFontFamiliesParams {
                font_families: self.font_families.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(font_families))
                })?,
                for_scripts: self.for_scripts,
            },
        })
    }
}
impl SetFontSizes {
    pub fn builder() -> SetFontSizesBuilder {
        <SetFontSizesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetFontSizesBuilder {
    font_sizes: Option<super::types::FontSizes>,
}
impl SetFontSizesBuilder {
    pub fn font_sizes(mut self, font_sizes: impl Into<super::types::FontSizes>) -> Self {
        self.font_sizes = Some(font_sizes.into());
        self
    }
    pub fn build(self) -> Result<SetFontSizes, String> {
        Ok(SetFontSizes {
            method: SetFontSizesMethod::SetFontSizes,
            params: SetFontSizesParams {
                font_sizes: self.font_sizes.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(font_sizes))
                })?,
            },
        })
    }
}
impl SetDocumentContent {
    pub fn builder() -> SetDocumentContentBuilder {
        <SetDocumentContentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDocumentContentBuilder {
    frame_id: Option<super::types::FrameId>,
    html: Option<String>,
}
impl SetDocumentContentBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }
    pub fn build(self) -> Result<SetDocumentContent, String> {
        Ok(SetDocumentContent {
            method: SetDocumentContentMethod::SetDocumentContent,
            params: SetDocumentContentParams {
                frame_id: Box::new(self.frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(frame_id))
                })?),
                html: self
                    .html
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(html)))?,
            },
        })
    }
}
impl SetLifecycleEventsEnabled {
    pub fn builder() -> SetLifecycleEventsEnabledBuilder {
        <SetLifecycleEventsEnabledBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetLifecycleEventsEnabledBuilder {
    enabled: Option<bool>,
}
impl SetLifecycleEventsEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetLifecycleEventsEnabled, String> {
        Ok(SetLifecycleEventsEnabled {
            method: SetLifecycleEventsEnabledMethod::SetLifecycleEventsEnabled,
            params: SetLifecycleEventsEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl StartScreencast {
    pub fn builder() -> StartScreencastBuilder {
        <StartScreencastBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartScreencastBuilder {
    format: Option<StartScreencastFormat>,
    quality: Option<i64>,
    max_width: Option<i64>,
    max_height: Option<i64>,
    every_nth_frame: Option<i64>,
}
impl StartScreencastBuilder {
    pub fn format(mut self, format: impl Into<StartScreencastFormat>) -> Self {
        self.format = Some(format.into());
        self
    }
    pub fn quality(mut self, quality: impl Into<i64>) -> Self {
        self.quality = Some(quality.into());
        self
    }
    pub fn max_width(mut self, max_width: impl Into<i64>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }
    pub fn max_height(mut self, max_height: impl Into<i64>) -> Self {
        self.max_height = Some(max_height.into());
        self
    }
    pub fn every_nth_frame(mut self, every_nth_frame: impl Into<i64>) -> Self {
        self.every_nth_frame = Some(every_nth_frame.into());
        self
    }
    pub fn build(self) -> StartScreencast {
        StartScreencast {
            method: StartScreencastMethod::StartScreencast,
            params: StartScreencastParams {
                format: self.format,
                quality: self.quality,
                max_width: self.max_width,
                max_height: self.max_height,
                every_nth_frame: self.every_nth_frame,
            },
        }
    }
}
impl SetWebLifecycleState {
    pub fn builder() -> SetWebLifecycleStateBuilder {
        <SetWebLifecycleStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetWebLifecycleStateBuilder {
    state: Option<SetWebLifecycleStateState>,
}
impl SetWebLifecycleStateBuilder {
    pub fn state(mut self, state: impl Into<SetWebLifecycleStateState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn build(self) -> Result<SetWebLifecycleState, String> {
        Ok(SetWebLifecycleState {
            method: SetWebLifecycleStateMethod::SetWebLifecycleState,
            params: SetWebLifecycleStateParams {
                state: self
                    .state
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
            },
        })
    }
}
impl ProduceCompilationCache {
    pub fn builder() -> ProduceCompilationCacheBuilder {
        <ProduceCompilationCacheBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProduceCompilationCacheBuilder {
    scripts: Option<Vec<super::types::CompilationCacheParams>>,
}
impl ProduceCompilationCacheBuilder {
    pub fn script(mut self, script: impl Into<super::types::CompilationCacheParams>) -> Self {
        let v = self.scripts.get_or_insert(Vec::new());
        v.push(script.into());
        self
    }
    pub fn scripts<I, S>(mut self, scripts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::CompilationCacheParams>,
    {
        let v = self.scripts.get_or_insert(Vec::new());
        for val in scripts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ProduceCompilationCache, String> {
        Ok(ProduceCompilationCache {
            method: ProduceCompilationCacheMethod::ProduceCompilationCache,
            params: ProduceCompilationCacheParams {
                scripts: self
                    .scripts
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scripts)))?,
            },
        })
    }
}
impl AddCompilationCache {
    pub fn builder() -> AddCompilationCacheBuilder {
        <AddCompilationCacheBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddCompilationCacheBuilder {
    url: Option<String>,
    data: Option<crate::Binary>,
}
impl AddCompilationCacheBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn data(mut self, data: impl Into<crate::Binary>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn build(self) -> Result<AddCompilationCache, String> {
        Ok(AddCompilationCache {
            method: AddCompilationCacheMethod::AddCompilationCache,
            params: AddCompilationCacheParams {
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
                data: self
                    .data
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
            },
        })
    }
}
impl SetSpcTransactionMode {
    pub fn builder() -> SetSpcTransactionModeBuilder {
        <SetSpcTransactionModeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSpcTransactionModeBuilder {
    mode: Option<SetSpcTransactionModeMode>,
}
impl SetSpcTransactionModeBuilder {
    pub fn mode(mut self, mode: impl Into<SetSpcTransactionModeMode>) -> Self {
        self.mode = Some(mode.into());
        self
    }
    pub fn build(self) -> Result<SetSpcTransactionMode, String> {
        Ok(SetSpcTransactionMode {
            method: SetSpcTransactionModeMethod::SetSpcTransactionMode,
            params: SetSpcTransactionModeParams {
                mode: self
                    .mode
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mode)))?,
            },
        })
    }
}
impl SetRphRegistrationMode {
    pub fn builder() -> SetRphRegistrationModeBuilder {
        <SetRphRegistrationModeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetRphRegistrationModeBuilder {
    mode: Option<SetRphRegistrationModeMode>,
}
impl SetRphRegistrationModeBuilder {
    pub fn mode(mut self, mode: impl Into<SetRphRegistrationModeMode>) -> Self {
        self.mode = Some(mode.into());
        self
    }
    pub fn build(self) -> Result<SetRphRegistrationMode, String> {
        Ok(SetRphRegistrationMode {
            method: SetRphRegistrationModeMethod::SetRphRegistrationMode,
            params: SetRphRegistrationModeParams {
                mode: self
                    .mode
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mode)))?,
            },
        })
    }
}
impl GenerateTestReport {
    pub fn builder() -> GenerateTestReportBuilder {
        <GenerateTestReportBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GenerateTestReportBuilder {
    message: Option<String>,
    group: Option<String>,
}
impl GenerateTestReportBuilder {
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }
    pub fn build(self) -> Result<GenerateTestReport, String> {
        Ok(GenerateTestReport {
            method: GenerateTestReportMethod::GenerateTestReport,
            params: GenerateTestReportParams {
                message: self
                    .message
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(message)))?,
                group: self.group,
            },
        })
    }
}
impl SetInterceptFileChooserDialog {
    pub fn builder() -> SetInterceptFileChooserDialogBuilder {
        <SetInterceptFileChooserDialogBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInterceptFileChooserDialogBuilder {
    enabled: Option<bool>,
    cancel: Option<bool>,
}
impl SetInterceptFileChooserDialogBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn cancel(mut self, cancel: impl Into<bool>) -> Self {
        self.cancel = Some(cancel.into());
        self
    }
    pub fn build(self) -> Result<SetInterceptFileChooserDialog, String> {
        Ok(SetInterceptFileChooserDialog {
            method: SetInterceptFileChooserDialogMethod::SetInterceptFileChooserDialog,
            params: SetInterceptFileChooserDialogParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
                cancel: self.cancel,
            },
        })
    }
}
impl SetPrerenderingAllowed {
    pub fn builder() -> SetPrerenderingAllowedBuilder {
        <SetPrerenderingAllowedBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPrerenderingAllowedBuilder {
    is_allowed: Option<bool>,
}
impl SetPrerenderingAllowedBuilder {
    pub fn is_allowed(mut self, is_allowed: impl Into<bool>) -> Self {
        self.is_allowed = Some(is_allowed.into());
        self
    }
    pub fn build(self) -> Result<SetPrerenderingAllowed, String> {
        Ok(SetPrerenderingAllowed {
            method: SetPrerenderingAllowedMethod::SetPrerenderingAllowed,
            params: SetPrerenderingAllowedParams {
                is_allowed: self.is_allowed.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(is_allowed))
                })?,
            },
        })
    }
}
impl GetAnnotatedPageContent {
    pub fn builder() -> GetAnnotatedPageContentBuilder {
        <GetAnnotatedPageContentBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetAnnotatedPageContentBuilder {
    include_actionable_information: Option<bool>,
}
impl GetAnnotatedPageContentBuilder {
    pub fn include_actionable_information(
        mut self,
        include_actionable_information: impl Into<bool>,
    ) -> Self {
        self.include_actionable_information = Some(include_actionable_information.into());
        self
    }
    pub fn build(self) -> GetAnnotatedPageContent {
        GetAnnotatedPageContent {
            method: GetAnnotatedPageContentMethod::GetAnnotatedPageContent,
            params: GetAnnotatedPageContentParams {
                include_actionable_information: self.include_actionable_information,
            },
        }
    }
}
