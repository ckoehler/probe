use rand::Rng;
use std::time::Duration;

fn main() {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::PUB).unwrap();
    socket.bind("tcp://127.0.0.1:5556").unwrap();
    socket.send("hello world!", 0).unwrap();

    let mut rng = rand::thread_rng();
    println!("Start server");
    println!("Start sending loop");
    loop {
        let zipcode = rng.gen_range(10000, 10010);
        let temperature = rng.gen_range(-80, 135);
        let relhumidity = rng.gen_range(10, 60);
        let topic = "UNIT".to_string();
        let msg = format!("{} {} {}", zipcode, temperature, relhumidity);
        let msg = vec![topic, msg];
        socket.send_multipart(msg.iter(), 0).unwrap();
        let delay = rng.gen_range(25, 2200);
        std::thread::sleep(Duration::from_millis(delay));
    }
}
