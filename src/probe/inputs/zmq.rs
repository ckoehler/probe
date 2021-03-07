use crate::probe::config::Probe;

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
            .recv_msg(0)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}
