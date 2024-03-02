use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::StateHandle;

use super::slash::SlashCommand;

pub struct Ping;

impl SlashCommand for Ping {
    fn new(_state_ref: StateHandle) -> Self {
        Self
    }

    fn run(&self, _options: &[ResolvedOption]) -> String {
        "Pong!".to_string()
    }

    fn register() -> CreateCommand {
        CreateCommand::new("ping").description("Replies with Pong!")
    }
}
