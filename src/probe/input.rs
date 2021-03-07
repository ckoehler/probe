use crate::probe::ProbeInput;
use std::sync::mpsc;
use std::thread;

pub type Message = (String, String);

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Inputs {
    rx: mpsc::Receiver<Message>,
    probe_handles: Vec<thread::JoinHandle<()>>,
}

impl Inputs {
    pub fn with_probes<T: 'static + ProbeInput + Sync + Clone + Send>(pis: Vec<T>) -> Inputs {
        let (tx, rx) = mpsc::channel();

        let mut probe_handles = Vec::new();
        pis.iter().for_each(|p| {
            let p = p.clone();
            let tx = tx.clone();
            let h = thread::spawn(move || {
                p.init();
                loop {
                    let msg = p.get();
                    let name = p.name();
                    if tx.send((name, msg)).is_err() {
                        break;
                    }
                }
            });
            probe_handles.push(h);
        });
        Inputs { rx, probe_handles }
    }

    pub fn next(&self) -> Result<Message, mpsc::RecvError> {
        self.rx.recv()
    }
}
