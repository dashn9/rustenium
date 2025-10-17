use rustenium_bidi_commands::browsing_context::commands::{BrowsingContextCreateMethod, Create as BrowsingContextCreate, CreateParameters as BrowsingContextCreateParameters, BrowsingContextResult};
use rustenium_bidi_commands::browsing_context::types::CreateType as BrowsingContextCreateType;
use rustenium_bidi_commands::{BrowsingContextCommand, CommandData, ResultData};
use crate::error::BrowsingContextCreationInvalidResultError;
use crate::Session;
use crate::session::SessionSendError;
use crate::transport::ConnectionTransport;

pub struct BrowsingContext {
    r#type: BrowsingContextCreateType,
    context: String,
}

impl BrowsingContext {
    async fn new<'oa, OT: ConnectionTransport<'oa>>(session: &mut Session<'oa, OT>, context_creation_type: Option<BrowsingContextCreateType>, reference_context: Option<&BrowsingContext>, background: bool) -> Result<Self, BrowsingContextCreationError>  {
        let context_creation_type = context_creation_type.unwrap_or(BrowsingContextCreateType::Tab);
        let create_browsing_context_command = BrowsingContextCreate {
            method: BrowsingContextCreateMethod::BrowsingContextCreate,
            params: BrowsingContextCreateParameters {
                r#type: context_creation_type.clone(),
                reference_context: match reference_context {
                    Some(reference_context) =>  Some(reference_context.context.clone()),
                    None => None
                },
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
                        context: context.context
                    })
                },
                _ => Err(BrowsingContextCreationError::BrowsingContextCreationInvalidResultError(BrowsingContextCreationInvalidResultError))
            },
            Err(e) => Err(BrowsingContextCreationError::SessionSendError(e)),
            _ => Err(BrowsingContextCreationError::BrowsingContextCreationInvalidResultError(BrowsingContextCreationInvalidResultError))
        }
    }
}

enum BrowsingContextCreationError {
    SessionSendError(SessionSendError),
    BrowsingContextCreationInvalidResultError(BrowsingContextCreationInvalidResultError)
}

impl std::fmt::Display for BrowsingContextCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BrowsingContextCreationError::SessionSendError(err) => write!(f, "{}", err),
            BrowsingContextCreationError::BrowsingContextCreationInvalidResultError(err) => write!(f, "{}", err),
        }
    }
}