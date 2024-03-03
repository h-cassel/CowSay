use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::StateHandle;

pub struct Clear {
    state_ref: StateHandle,
}

impl Clear {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        {
            let mut state = self.state_ref.lock().await;
            state.queue.clear();
        }
        "Cleared the queue!".to_string()
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("clear-queue").description("Clear the queue")
    }
}
