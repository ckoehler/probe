mod app;
pub mod ui;
pub use app::App;

pub trait ProbeInput {
    /// blocking call that returns String data when available
    fn get(&self) -> String;

    /// Called before entering the event loop. Implement if something needs to be done.
    fn init(&self) {}
}

#[derive(Clone, Copy)]
pub struct ZMQInput {}

impl ProbeInput for ZMQInput {
    fn get(&self) -> String {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        return "Hello!".to_string();
    }
}
