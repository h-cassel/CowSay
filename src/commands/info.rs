use serenity::builder::CreateCommand;
use serenity::json::json;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Info {
    state_ref: StateHandle,
}

impl Info {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        let resp = send_cmd(
            &self.state_ref,
            Request::new(
                "info".to_string(),
                Some(json!({"client_info": { "name": "CowSay Bot", "version": env!("CARGO_PKG_VERSION") }}))
            ),
        )
        .await;
        format!("Info: {:?}", resp)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("info").description("Get printer info")
    }
}
