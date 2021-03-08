use crate::probe::state::{AppState, ProbeState, TabsState};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub state: AppState,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, state: AppState) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            state: state,
        }
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.state
            .probes
            .iter_mut()
            .for_each(|p: &mut ProbeState| p.update_state());
    }

    pub fn process_message_for_stream(&mut self, stream: String, msg: String) {
        self.state
            .probes
            .iter_mut()
            .filter(|p| p.name == stream)
            .for_each(|p: &mut ProbeState| p.process_message(&msg));
    }
}
