extern crate ferris_says;

use ferris_says::say;
use serenity::all::ResolvedValue;
use std::io::{ Write };

use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::StateHandle;
use serenity::all::CommandOptionType;
use serenity::all::CreateCommandOption;
use super::slash::SlashCommand;

pub struct FerrisSays;

impl SlashCommand for FerrisSays {
    fn new(_state_ref: StateHandle) -> Self {
        Self
    }

    fn run(&self, _options: &[ResolvedOption]) -> String {

        let out = if let Some(ResolvedOption { value: ResolvedValue::String(output), .. }) = _options.first() {
            *output
        } else {
            "Invalid Arg!"
        };

        let width = 24;
    
        let mut output: Vec<u8> = vec![];
        say(&out, width, &mut output).unwrap();
        let output = String::from_utf8(output).unwrap();
        let output2 = format!("```  \n  {output}  \n  ```");

        output2
    }

    fn register() -> CreateCommand {
        CreateCommand::new("ferris-says").description("A visit from Ferris! What does he have to say?").add_option(CreateCommandOption::new(CommandOptionType::String, "dialogue", "What should Ferris say?"))
    }
}