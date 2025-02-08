use rand::Rng;
use std::error::Error;
use std::time::Duration;
use zeromq::{Socket, SocketSend, ZmqMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = zeromq::PubSocket::new();
    socket.bind("tcp://127.0.0.1:5556").await?;

    let mut rng = rand::rng();
    println!("Start server");
    println!("Start sending loop");
    loop {
        let zipcode = rng.random_range(10000..=10010);
        let temperature = rng.random_range(-80..=135);
        let relhumidity = rng.random_range(10..=60);
        let topic = "UNIT".to_string();
        let msg = format!("{zipcode} {temperature} {relhumidity}");
        let mut m: ZmqMessage = ZmqMessage::from(topic);
        m.push_back(msg.into());

        socket.send(m).await?;
        let delay = rng.random_range(25..=2200);
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }
}
