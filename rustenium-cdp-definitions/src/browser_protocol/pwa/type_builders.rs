use super::types::*;
impl FileHandlerAccept {
    pub fn builder() -> FileHandlerAcceptBuilder {
        <FileHandlerAcceptBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FileHandlerAcceptBuilder {
    media_type: Option<String>,
    file_extensions: Option<Vec<String>>,
}
impl FileHandlerAcceptBuilder {
    pub fn media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }
    pub fn file_extension(mut self, file_extension: impl Into<String>) -> Self {
        let v = self.file_extensions.get_or_insert(Vec::new());
        v.push(file_extension.into());
        self
    }
    pub fn file_extensions<I, S>(mut self, file_extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.file_extensions.get_or_insert(Vec::new());
        for val in file_extensions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<FileHandlerAccept, String> {
        Ok(FileHandlerAccept {
            media_type: self
                .media_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(media_type)))?,
            file_extensions: self.file_extensions.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(file_extensions))
            })?,
        })
    }
}
impl FileHandler {
    pub fn builder() -> FileHandlerBuilder {
        <FileHandlerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FileHandlerBuilder {
    action: Option<String>,
    accepts: Option<Vec<FileHandlerAccept>>,
    display_name: Option<String>,
}
impl FileHandlerBuilder {
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn accept(mut self, accept: impl Into<FileHandlerAccept>) -> Self {
        let v = self.accepts.get_or_insert(Vec::new());
        v.push(accept.into());
        self
    }
    pub fn accepts<I, S>(mut self, accepts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FileHandlerAccept>,
    {
        let v = self.accepts.get_or_insert(Vec::new());
        for val in accepts {
            v.push(val.into());
        }
        self
    }
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }
    pub fn build(self) -> Result<FileHandler, String> {
        Ok(FileHandler {
            action: self
                .action
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(action)))?,
            accepts: self
                .accepts
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(accepts)))?,
            display_name: self.display_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(display_name))
            })?,
        })
    }
}
