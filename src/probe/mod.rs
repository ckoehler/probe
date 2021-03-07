mod app;
pub mod input;
pub mod ui;
pub use app::App;

pub trait ProbeInput {
    /// blocking call that returns String data when available
    fn get(&self) -> String;

    /// return the name of this stream
    fn name(&self) -> String;

    /// Called before entering the event loop. Implement if something needs to be done.
    fn init(&self) {}
}

#[derive(Clone)]
pub struct ZMQInput {
    pub name: String,
}

impl ProbeInput for ZMQInput {
    fn get(&self) -> String {
        std::thread::sleep(std::time::Duration::from_millis(250));
        return "Hello!".to_string();
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}
