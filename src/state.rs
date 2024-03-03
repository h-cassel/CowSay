use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use crate::klipper::{Request, Response};

pub struct PrintJob {
    pub color: Option<String>,
    pub speed: u8,
}

pub struct BotState {
    pub queue: Vec<PrintJob>,
    pub current_job: Option<PrintJob>,
    pub req_channel: (broadcast::Sender<Request>, broadcast::Receiver<Request>),
    pub resp_channel: (broadcast::Sender<Response>, broadcast::Receiver<Response>),
}

impl BotState {
    pub fn new() -> BotState {
        let (tx, rx) = broadcast::channel(10);
        let (tx2, rx2) = broadcast::channel(10);
        BotState {
            queue: Vec::new(),
            current_job: None,
            resp_channel: (tx, rx),
            req_channel: (tx2, rx2),
        }
    }
}

pub type StateHandle = Arc<Mutex<BotState>>;
