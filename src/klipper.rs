use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub id: u32,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub id: u32,
    pub result: Value,
}

pub const SEP_CHAR: char = '\x03';

impl Request {
    pub fn new(method: String, params: Option<Value>) -> Request {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>();
        Request { id, method, params }
    }

    pub async fn send(
        &self,
        tx: broadcast::Sender<Request>,
        mut rx: broadcast::Receiver<Response>,
    ) -> Response {
        tx.send(self.clone()).unwrap();
        loop {
            let resp = rx.recv().await.unwrap();
            if resp.id == self.id {
                return resp;
            }
        }
    }
}
