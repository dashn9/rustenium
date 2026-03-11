use rustenium_bidi_definitions::browsing_context::command_builders::CreateBuilder;
use rustenium_bidi_definitions::browsing_context::results::CreateResult;
use rustenium_bidi_definitions::browsing_context::types::{
    BrowsingContext as BrowsingContextDefinition, CreateType,
};

use crate::error::CommandResultError;
use crate::session::BidiSession;
use crate::transport::ConnectionTransport;

#[derive(Clone)]
pub struct CreateBrowsingContext<'a, T: ConnectionTransport> {
    session: &'a BidiSession<T>,
    r#type: CreateType,
    reference_context: Option<&'a BrowsingContext>,
    background: bool,
}

impl<'a, T: ConnectionTransport> CreateBrowsingContext<'a, T> {
    fn new(session: &'a mut BidiSession<T>) -> Self {
        Self {
            session,
            r#type: CreateType::Tab,
            reference_context: None,
            background: false,
        }
    }

    pub fn r#type(mut self, create_type: CreateType) -> Self {
        self.r#type = create_type;
        self
    }

    pub fn reference_context(mut self, reference_context: &'a BrowsingContext) -> Self {
        self.reference_context = Option::from(reference_context);
        self
    }

    pub fn background(mut self, background: bool) -> Self {
        self.background = background;
        self
    }

    pub async fn create(self) -> Result<BrowsingContext, CommandResultError> {
        let create_browsing_context_command = CreateBuilder::default()
            .r#type(self.r#type.clone())
            .background(self.background);
        if let Some(reference_context) = self.reference_context {
            create_browsing_context_command.reference_context(reference_context.into());
        }
        let context = self
            .session
            .send(create_browsing_context_command.build().unwrap())
            .await;
        match context {
            Ok(response) => {
                let result: CreateResult = response
                    .result
                    .try_into()
                    .map_err(|_| CommandResultError::InvalidResultTypeError(response.result))?;
                Ok(BrowsingContext {
                    r#type: self.r#type,
                    id: result.context,
                })
            }
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
    }
}
pub struct BrowsingContext {
    pub r#type: CreateType,
    id: BrowsingContextDefinition,
}

impl BrowsingContext {
    /// Create a Context from existing ID and type
    pub fn from_id(
        id: impl Into<BrowsingContextDefinition>,
        context_type: CreateType,
    ) -> Self {
        Self {
            r#type: context_type,
            id: id.into(),
        }
    }

    /// Get the context ID as a string reference
    pub fn id(&self) -> &BrowsingContextDefinition {
        &self.id
    }
}

impl From<BrowsingContext> for String {
    fn from(browsing_context: BrowsingContext) -> String {
        browsing_context.id.as_ref().to_string()
    }
}
