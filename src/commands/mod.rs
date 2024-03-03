use crate::{
    klipper::{Request, Response},
    state::StateHandle,
};

pub mod brightness;
pub mod cancel;
pub mod estop;
pub mod ferris_says;
pub mod info;
pub mod pause;
pub mod ping;
pub mod queue;
pub mod resume;

pub async fn send_cmd(state: &StateHandle, cmd: Request) -> Response {
    let state = state.lock().await;
    let rx = state.resp_channel.0.subscribe();
    let tx = state.req_channel.0.clone();
    cmd.send(tx, rx).await
}
