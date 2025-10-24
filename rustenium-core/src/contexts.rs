use rustenium_bidi_commands::browsing_context::commands::{BrowsingContextCreateMethod, Create as BrowsingContextCreate, CreateParameters as BrowsingContextCreateParameters, BrowsingContextResult};
use rustenium_bidi_commands::browsing_context::types::CreateType as BrowsingContextCreateType;
use rustenium_bidi_commands::{BrowsingContextCommand, CommandData, ResultData};
use crate::error::{CommandResultError, SessionSendError};
use crate::Session;
use crate::transport::ConnectionTransport;

#[derive(Debug)]
pub struct BrowsingContext {
    pub r#type: BrowsingContextCreateType,
    pub context: String,
}

impl BrowsingContext {
    pub async fn new<OT: ConnectionTransport>(session: &mut Session<OT>, context_creation_type: Option<BrowsingContextCreateType>, reference_context: Option<&BrowsingContext>, background: bool) -> Result<Self, CommandResultError>  {
        let context_creation_type = context_creation_type.unwrap_or(BrowsingContextCreateType::Tab);
        let create_browsing_context_command = BrowsingContextCreate {
            method: BrowsingContextCreateMethod::BrowsingContextCreate,
            params: BrowsingContextCreateParameters {
                r#type: BrowsingContextCreateType::Tab,
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
                _ => Err(CommandResultError::InvalidResultTypeError(ResultData::BrowsingContextResult(context)))
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
    }
    
    pub fn get_context_id(&self) -> String {
        self.context.clone()
    }
}
