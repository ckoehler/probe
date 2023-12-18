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
            state,
        }
    }

    pub fn probes_for_tab(&self) -> Vec<ProbeState> {
        self.state
            .probes_for_tab(self.tabs.index, self.tabs.probe_num)
    }

    pub fn on_up(&mut self) {
        let sel = self.state.selected_probe as i32;
        let num_probes = self.state.probes.len() as i32;
        self.state.selected_probe = (sel - 1).rem_euclid(num_probes) as usize;
    }

    pub fn on_down(&mut self) {
        let sel = self.state.selected_probe as i32;
        let num_probes = self.state.probes.len() as i32;
        self.state.selected_probe = (sel + 1).rem_euclid(num_probes) as usize;
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
        self.state.selected_probe = 0;
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
        self.state.selected_probe = 0;
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'h' => {
                self.on_left();
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'l' => {
                self.on_right();
            }
            '\n' => {
                self.state.detail_view = !self.state.detail_view;
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
