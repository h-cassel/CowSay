use std::str::FromStr;

use serde_json::Value;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Info {
    state_ref: StateHandle,
}

const QUERY_PARAMS: &str = include_str!("obj_query.json");

impl Info {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        let state = self.state_ref.lock().await;
        let cmd = Request::new(
            "objects/query".to_string(),
            Value::from_str(QUERY_PARAMS).unwrap(),
        );
        let rx = state.resp_channel.0.subscribe();
        let tx = state.req_channel.0.clone();
        let resp = cmd.send(tx, rx).await;
        format!("Info: {:?}", resp)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("info").description("Get printer info")
    }
}
