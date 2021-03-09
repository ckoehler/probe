use crate::probe::config::Probe;
use std::str;

pub struct ZMQInput {
    pub name: String,
    socket: zmq::Socket,
}

impl ZMQInput {
    pub fn from_probe(probe: &Probe) -> ZMQInput {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::SUB).unwrap();
        socket.connect(&probe.address).unwrap();
        let subscription = format!("").into_bytes();
        socket.set_subscribe(&subscription).unwrap();
        ZMQInput {
            name: probe.name.clone(),
            socket: socket,
        }
    }

    pub fn get(&self) -> String {
        self.socket
            .recv_multipart(0)
            .unwrap()
            .iter()
            .map(|v| str::from_utf8(v).unwrap_or(""))
            .collect()
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}
