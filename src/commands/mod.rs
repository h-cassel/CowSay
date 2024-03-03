use crate::{
    klipper::{Request, Response},
    state::StateHandle,
};

pub mod info;
pub mod ping;
pub mod brightness;
pub mod ferris_says;

pub async fn send_cmd(state: &StateHandle, cmd: Request) -> Response {
    let state = state.lock().await;
    let rx = state.resp_channel.0.subscribe();
    let tx = state.req_channel.0.clone();
    cmd.send(tx, rx).await
}
