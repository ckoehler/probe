mod zmq;
use crate::probe::config::Probe;
use crate::probe::inputs::zmq::ZMQInput;
use std::sync::mpsc;
use std::thread;

pub type Message = (String, String);

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Inputs {
    rx: mpsc::Receiver<Message>,
}

impl Inputs {
    pub fn with_probes(probes: Vec<Probe>) -> Inputs {
        let (tx, rx) = mpsc::channel();

        probes.iter().for_each(|p| {
            let p = p.clone();
            let tx = tx.clone();
            thread::spawn(move || {
                let z = ZMQInput::from_probe(&p);
                loop {
                    let msg = z.get();
                    let name = z.name();
                    if tx.send((name, msg)).is_err() {
                        break;
                    }
                }
            });
        });
        Inputs { rx }
    }

    pub fn next(&self) -> Result<Message, mpsc::RecvError> {
        self.rx.recv()
    }
}
