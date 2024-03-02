use std::sync::Arc;

use tokio::sync::Mutex;

pub struct PrintJob {
    pub color: Option<String>,
    pub speed: u8,
}

pub struct BotState {
    pub queue: Vec<PrintJob>,
    pub current_job: Option<PrintJob>,
}

impl BotState {
    pub fn new() -> BotState {
        BotState {
            queue: Vec::new(),
            current_job: None,
        }
    }
}

pub type StateHandle = Arc<Mutex<BotState>>;
