use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Cancel {
    state_ref: StateHandle,
}

impl Cancel {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        send_cmd(
            &self.state_ref,
            Request::new("pause_resume/cancel".to_string(), None),
        )
        .await;
        "Cancelled".to_string()
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("cancel").description("Cancels current print")
    }
}
