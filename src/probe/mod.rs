mod app;
pub mod input;
pub mod ui;
pub use app::App;

use crate::config::Probe;

pub trait ProbeInput {
    /// blocking call that returns String data when available
    fn get(&self) -> String;

    /// return the name of this stream
    fn name(&self) -> String;

    /// Called before entering the event loop. Implement if something needs to be done.
    fn init(&self) {}
}

#[derive(Clone)]
pub struct ZMQInput<'a> {
    pub name: String,
    pub address: String,
    socket: Option<&'a zmq::Socket>,
}

impl<'a> ZMQInput<'a> {
    pub fn from_probes(probes: &Vec<Probe>) -> Vec<ZMQInput> {
        probes
            .iter()
            .map(|p| ZMQInput {
                name: p.name.clone(),
                address: p.address.clone(),
                socket: None,
            })
            .collect()
    }
}

impl<'a> ProbeInput for ZMQInput<'a> {
    fn init(&self) {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::SUB).unwrap();
        socket.connect(&self.address).unwrap();
        let subscription = format!("").into_bytes();
        socket.set_subscribe(&subscription).unwrap();
        self.socket = Some(&socket);
    }
    fn get(&self) -> String {
        self.socket
            .unwrap()
            .recv_msg(0)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
        // std::thread::sleep(std::time::Duration::from_millis(250));
        // return "Hello!".to_string();
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}
