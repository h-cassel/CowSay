use serde_json::json;
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::commands::send_cmd;
use crate::klipper::Request;
use crate::state::StateHandle;

pub struct Brightness {
    state_ref: StateHandle,
}

impl Brightness {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        if let Some(ResolvedOption {
            value: ResolvedValue::Integer(brightness),
            ..
        }) = _options.first()
        {
            if 0 <= *brightness && *brightness <= 100 {
                let cmd = Request::new(
                    "gcode/script".to_string(),
                    json!({
                        "script": format!("FLOOD_LIGHTS BRIGHTNESS={brightness}")
                    }),
                );
                send_cmd(&self.state_ref, cmd).await;
                format!("Brightness set to {}", brightness)
            } else {
                "Invalid brightness value".to_string()
            }
        } else {
            "Invalid brightness value".to_string()
        }
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("brightness")
            .description("Set the brightness of the printer LEDs")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "brightness",
                    "The brightness to set the LEDs to",
                )
                .required(true),
            )
    }
}
