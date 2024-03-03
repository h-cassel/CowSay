use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Estop {
    state_ref: StateHandle,
}

impl Estop {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        send_cmd(
            &self.state_ref,
            Request::new("emergency_stop".to_string(), None),
        )
        .await;
        "Emergency Stop".to_string()
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("estop").description("Stops current print")
    }
}
