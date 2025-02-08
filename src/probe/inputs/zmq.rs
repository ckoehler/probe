use crate::probe::config::ProbeConfig;
use zeromq::Socket;
use zeromq::SocketRecv;
use zeromq::SubSocket;

pub struct ZMQInput {
    pub name: String,
    socket: SubSocket,
}

impl ZMQInput {
    pub async fn from_probe(probe: &ProbeConfig) -> ZMQInput {
        let mut socket = zeromq::SubSocket::new();
        socket
            .connect(&probe.address)
            .await
            .expect("Failed to connect");

        socket.subscribe("").await.unwrap();
        ZMQInput {
            name: probe.name.clone(),
            socket,
        }
    }

    pub async fn get(&mut self) -> String {
        let data = self.socket.recv().await.unwrap();
        data.iter()
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
