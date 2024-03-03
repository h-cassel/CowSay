extern crate ferris_says;

use ferris_says::say;
use serenity::all::ResolvedValue;

use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::StateHandle;
use serenity::all::CommandOptionType;
use serenity::all::CreateCommandOption;

pub struct FerrisSays;

impl FerrisSays {
    pub fn new(_state_ref: StateHandle) -> Self {
        Self
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {

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

    pub fn register() -> CreateCommand {
        CreateCommand::new("ferris-says").description("A visit from Ferris! What does he have to say?").add_option(CreateCommandOption::new(CommandOptionType::String, "dialogue", "What should Ferris say?"))
    }
}