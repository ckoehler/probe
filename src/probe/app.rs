use crate::probe::state::{AppState, Probe, TabsState};

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState,
    pub state: AppState,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, state: AppState) -> App<'a> {
        let num_probes = state.probes.len();
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(num_probes),
            state,
        }
    }

    pub fn probes_for_tab(&self) -> Vec<Probe> {
        self.state
            .probes_for_tab(self.tabs.selected_tab, self.tabs.probes_per_tab)
    }

    pub fn on_up(&mut self) {
        self.tabs.prev_probe();
    }

    pub fn on_down(&mut self) {
        self.tabs.next_probe();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn selected_probe(&self) -> Probe {
        self.state.probes[self.tabs.selected_probe_index()].clone()
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
            .for_each(|p: &mut Probe| p.update_state());
    }

    pub fn process_message_for_stream(&mut self, stream: &str, msg: &str) {
        self.state
            .probes
            .iter_mut()
            .filter(|p| p.name == stream)
            .for_each(|p: &mut Probe| p.process_message(msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::config::ProbeConfig;

    #[test]
    fn single_probe_per_tab_should_stay_selected() {
        let config = vec![
            ProbeConfig {
                name: String::from("0"),
                filter: None,
                address: String::new(),
            },
            ProbeConfig {
                name: String::from("1"),
                filter: None,
                address: String::new(),
            },
            ProbeConfig {
                name: String::from("2"),
                filter: None,
                address: String::new(),
            },
        ];
        let state = AppState::from_probes(&config);
        let mut app = App::new("Probe", state);

        // set layout to only have one tab
        app.tabs.recalculate_layout(config.len(), 1);

        // check it's the first
        assert_eq!(app.selected_probe().name, String::from("0"));

        // there's only one probe, so this should do nothing
        app.on_down();
        assert_eq!(app.selected_probe().name, String::from("0"));
    }

    #[test]
    fn two_probes_per_tab() {
        let config = vec![
            ProbeConfig {
                name: String::from("0"),
                filter: None,
                address: String::new(),
            },
            ProbeConfig {
                name: String::from("1"),
                filter: None,
                address: String::new(),
            },
            ProbeConfig {
                name: String::from("2"),
                filter: None,
                address: String::new(),
            },
        ];
        let state = AppState::from_probes(&config);
        let mut app = App::new("Probe", state);

        // set probes per tab manually
        app.tabs.recalculate_layout(config.len(), 2);
        assert_eq!(app.selected_probe().name, String::from("0"));

        app.on_down();
        assert_eq!(app.selected_probe().name, String::from("1"));

        // already on bottom probe, shouldn't do anything
        app.on_down();
        assert_eq!(app.selected_probe().name, String::from("1"));

        app.on_up();
        assert_eq!(app.selected_probe().name, String::from("0"));

        // already on top probe, shouldn't do anything
        app.on_up();
        assert_eq!(app.selected_probe().name, String::from("0"));
    }
}
