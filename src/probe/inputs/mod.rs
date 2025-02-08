mod zmq;
use crate::probe::config::ProbeConfig;
use crate::probe::inputs::zmq::ZMQInput;
use tokio::sync::mpsc;

pub type Message = (String, String);

/// A small event handler that wraps input and tick events. Each event
/// type is handled in its own task and returned to a common `Receiver`
pub struct Inputs {
    rx: mpsc::Receiver<Message>,
}

impl Inputs {
    pub fn with_probes(probes: &[ProbeConfig]) -> Inputs {
        let (tx, rx) = mpsc::channel(10);

        for p in probes {
            let p = p.clone();
            let tx = tx.clone();
            tokio::spawn(async move {
                let mut z = ZMQInput::from_probe(&p).await;
                loop {
                    // this is blocking!!
                    let msg = z.get().await;
                    let name = z.name();
                    if tx.send((name, msg)).await.is_err() {
                        break;
                    }
                }
            });
        }
        Inputs { rx }
    }

    pub async fn next(&mut self) -> Option<Message> {
        self.rx.recv().await
    }
}
