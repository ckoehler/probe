use crate::probe::config::ProbeConfig;
use async_zmq::StreamExt;
use std::str;

pub struct ZMQInput {
    pub name: String,
    socket: async_zmq::Subscribe,
}

impl ZMQInput {
    pub fn from_probe(probe: &ProbeConfig) -> ZMQInput {
        let socket = async_zmq::subscribe(&probe.address)
            .unwrap()
            .connect()
            .unwrap();
        let subscription = String::new();
        socket.set_subscribe(&subscription).unwrap();
        ZMQInput {
            name: probe.name.clone(),
            socket,
        }
    }

    pub async fn get(&mut self) -> String {
        self.socket
            .next()
            .await
            .unwrap()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
