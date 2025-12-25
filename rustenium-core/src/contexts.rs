use rustenium_bidi_commands::browsing_context::commands::{BrowsingContextCreateMethod, Create as BrowsingContextCreate, CreateParameters as BrowsingContextCreateParameters, BrowsingContextResult};
use rustenium_bidi_commands::browsing_context::types::{BrowsingContext, CreateType as BrowsingContextCreateType};
use rustenium_bidi_commands::{BrowsingContextCommand, CommandData, ResultData};
use crate::error::{CommandResultError};
use crate::Session;
use crate::transport::ConnectionTransport;

#[derive(Debug, Clone)]
pub struct Context {
    pub r#type: BrowsingContextCreateType,
    id: BrowsingContext,
}

impl Context {
    /// Create a Context from existing ID and type
    pub fn from_id(id: String, context_type: BrowsingContextCreateType) -> Self {
        Self {
            r#type: context_type,
            id,
        }
    }

    pub async fn new<OT: ConnectionTransport>(session: &mut Session<OT>, context_creation_type: Option<BrowsingContextCreateType>, reference_context: Option<&Context>, background: bool) -> Result<Self, CommandResultError>  {
        let context_creation_type = context_creation_type.unwrap_or(BrowsingContextCreateType::Tab);
        let create_browsing_context_command = BrowsingContextCreate {
            method: BrowsingContextCreateMethod::BrowsingContextCreate,
            params: BrowsingContextCreateParameters {
                r#type: context_creation_type.clone(),
                reference_context: reference_context.map(|c| c.id.clone()),
                background: Option::from(background),
                // I don't know how to deal with UserContext yet.
                user_context: None,
            },
        };
        let context = session.send(CommandData::BrowsingContextCommand(BrowsingContextCommand::Create(create_browsing_context_command))).await;
        match context {
            Ok(ResultData::BrowsingContextResult(context)) => match context {
                BrowsingContextResult::CreateResult(context) => {
                    Ok(Self {
                        r#type: context_creation_type,
                        id: context.context
                    })
                },
                _ => Err(CommandResultError::InvalidResultTypeError(ResultData::BrowsingContextResult(context)))
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
    }

    /// Get the context ID as a string reference
    pub fn id(&self) -> &BrowsingContext {
        &self.id
    }

    /// Get a cloned context ID (deprecated - use id() instead)
    pub fn get_context_id(&self) -> String {
        self.id.clone()
    }
}
