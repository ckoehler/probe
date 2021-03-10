use crate::probe::state::{AppState, ProbeState, TabsState};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState,
    pub state: AppState,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, state: AppState) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(),
            state: state,
        }
    }

    pub fn probes_for_tab(&self) -> Vec<ProbeState> {
        self.state
            .probes_for_tab(self.tabs.index, self.tabs.probe_num)
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
            'h' => {
                self.tabs.previous();
            }
            'l' => {
                self.tabs.next();
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
