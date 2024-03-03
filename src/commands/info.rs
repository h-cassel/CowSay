use std::str::FromStr;

use serde_json::Value;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
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
        let resp = send_cmd(
            &self.state_ref,
            Request::new(
                "objects/query".to_string(),
                Value::from_str(QUERY_PARAMS).unwrap(),
            ),
        )
        .await;
        format!("Info: {:?}", resp)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("info").description("Get printer info")
    }
}
