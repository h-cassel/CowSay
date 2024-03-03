use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixSocket, UnixStream},
    sync::broadcast,
};

use crate::klipper::{Request, Response, SEP_CHAR};

pub struct KlippyConnection {
    sock: UnixStream,
}

impl KlippyConnection {
    pub async fn new(path: String) -> KlippyConnection {
        let sock = UnixSocket::new_stream().unwrap();
        let sock = sock.connect(path).await.unwrap();
        KlippyConnection { sock }
    }

    pub async fn req_resp_loop(
        &mut self,
        tx: broadcast::Sender<Response>,
        mut rx: broadcast::Receiver<Request>,
    ) {
        println!("Starting req_resp_loop");

        loop {
            tokio::select! {
                req = rx.recv() => {
                    if let Ok(req) = req {
                        let req = serde_json::to_string(&req).unwrap();
                        let msg = format!("{req}{SEP_CHAR}");
                        println!("Sending data: {}", msg);
                        self.sock
                            .write_all(msg.as_bytes())
                            .await
                            .unwrap();
                        self.sock.flush().await.unwrap();
                        println!("Sent data");
                    }
                }
                _ = self.sock.readable() => {
                    println!("Socket is readable");
                    let mut data = vec![0; 1024];
                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    self.sock.read_exact(&mut data).await.unwrap();
                    let data = String::from_utf8(data).unwrap();
                    println!("Raw data: {}", data);
                    let parts = data.split(SEP_CHAR);
                    for msg in parts {
                        if !msg.is_empty() {
                            println!("Received data: {}", msg);
                            if let Ok(resp) = serde_json::from_str::<Response>(msg) {
                                tx.send(resp).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
