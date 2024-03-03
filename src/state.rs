use std::{collections::VecDeque, sync::Arc};

use tokio::sync::{broadcast, Mutex};

use crate::klipper::{Request, Response};

#[derive(Debug, Clone)]
pub struct PrintJob {
    pub cdn_link: String,
    pub name: String,
    pub requesting_user: String,
    pub color: Option<String>,
    pub speed: u8,
}

impl PrintJob {
    pub fn new(name: String, cdn_link: String, requesting_user: String) -> PrintJob {
        PrintJob {
            name,
            cdn_link,
            requesting_user,
            color: None,
            speed: 100,
        }
    }
}

pub struct BotState {
    pub queue: VecDeque<PrintJob>,
    pub current_job: Option<PrintJob>,
    pub req_channel: (broadcast::Sender<Request>, broadcast::Receiver<Request>),
    pub resp_channel: (broadcast::Sender<Response>, broadcast::Receiver<Response>),
}

impl BotState {
    pub fn new() -> BotState {
        let (tx, rx) = broadcast::channel(10);
        let (tx2, rx2) = broadcast::channel(10);
        BotState {
            queue: VecDeque::new(),
            current_job: None,
            resp_channel: (tx, rx),
            req_channel: (tx2, rx2),
        }
    }
}

pub type StateHandle = Arc<Mutex<BotState>>;
