use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue, User};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::{PrintJob, StateHandle};

pub struct Add {
    state_ref: StateHandle,
}

impl Add {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, user: &User, options: &[ResolvedOption<'a>]) -> String {
        if let Some(ResolvedOption {
            value: ResolvedValue::Attachment(attachment),
            ..
        }) = options.first()
        {
            let cdn_link = attachment.url.clone();
            let name = attachment.filename.clone();
            let item = PrintJob::new(name, cdn_link, user.name.clone());

            {
                let mut state = self.state_ref.lock().await;
                state.queue.push_back(item);
            }

            "You print has been added to the queue! Use `/queue` to view it!".to_string()
        } else {
            "Please provide a valid attachment".to_string()
        }
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("print")
            .description("Add a GCODE file to the queue")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Attachment,
                    "file",
                    "The gcode file to print",
                )
                .required(true),
            )
    }
}
