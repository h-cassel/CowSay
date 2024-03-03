use serde_json::Value;
use serenity::builder::CreateCommand;
use serenity::json::json;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Resume {
    state_ref: StateHandle,
}

impl Resume {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        let resp = send_cmd(
            &self.state_ref,
            Request::new(
                "pause_resume/resume".to_string(),
                Value::Null
        )
    ).await;
    "Resumed".to_string()
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("resume").description("Resumes current print")
    }
}