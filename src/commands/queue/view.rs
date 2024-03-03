use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::state::StateHandle;

pub struct View {
    state_ref: StateHandle,
}

impl View {
    pub fn new(state_ref: StateHandle) -> Self {
        Self { state_ref }
    }

    pub async fn run<'a>(&self, _options: &[ResolvedOption<'a>]) -> String {
        let queue = {
            let state = self.state_ref.lock().await;
            state.queue.clone()
        };
        let f = queue
            .iter()
            .enumerate()
            .map(|(i, job)| {
                format!(
                    "{}: {} (Requested by {})",
                    i + 1,
                    job.name,
                    job.requesting_user
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        if f.is_empty() {
            "The queue is empty!".to_string()
        } else {
            f
        }
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("queue").description("View the queue")
    }
}
