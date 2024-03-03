use serde_json::Value;
use serenity::builder::CreateCommand;
use serenity::json::json;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Progress {
    state_ref: StateHandle,
}

impl Progress {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        let resp = send_cmd(
            &self.state_ref,
            Request::new(
                "objects/query".to_string(),
                Some(json!({"objects": { "display_status": null }})),
            ),
        )
        .await;
        // Ugly but wtv
        let progress = if let Value::Object(obj) = resp.result {
            if let Some(Value::Object(status)) = obj.get("status") {
                if let Some(Value::Object(display_status)) = status.get("display_status") {
                    if let Some(Value::Number(progress)) = display_status.get("progress") {
                        progress.as_f64().map(|dec| format!("Progress: {:.2}%", dec * 100.0))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        if let Some(progress) = progress {
            progress
        } else {
            "No active job (or current job hasn't run long enough)".to_string()
        }
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("info").description("Get printer info")
    }
}
