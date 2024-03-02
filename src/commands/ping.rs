use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::StateHandle;

pub struct Ping;

impl Ping {
    pub fn new(_state_ref: StateHandle) -> Self {
        Self
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        "Pong!".to_string()
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("ping").description("Replies with Pong!")
    }
}
