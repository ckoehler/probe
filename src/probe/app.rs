use crate::probe::state::{AppState, Probe, TabsState};

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

    /// Takes the tab state (current tab) and selected probe on that tab and translates them to a
    /// Probe.
    // fn get_selected_probe_index(&self) -> Probe {
    //     todo!()
    // }

    pub fn probes_for_tab(&self) -> Vec<Probe> {
        self.state
            .probes_for_tab(self.tabs.selected_tab, self.tabs.probes_per_tab)
    }

    pub fn on_up(&mut self) {
        let sel = self.state.selected_probe as i32;
        // let probes_per_tab = self.tabs.probes_per_tab;
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
            .for_each(|p: &mut Probe| p.update_state());
    }

    pub fn process_message_for_stream(&mut self, stream: String, msg: String) {
        self.state
            .probes
            .iter_mut()
            .filter(|p| p.name == stream)
            .for_each(|p: &mut Probe| p.process_message(&msg));
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
                address: String::from(""),
            },
            ProbeConfig {
                name: String::from("1"),
                filter: None,
                address: String::from(""),
            },
            ProbeConfig {
                name: String::from("2"),
                filter: None,
                address: String::from(""),
            },
        ];
        let state = AppState::from_probes(config);
        let mut app = App::new("Probe", state);

        // set probes per tab manually
        app.tabs.probes_per_tab = 1;
        assert_eq!(app.state.selected_probe().name, String::from("0"));

        // there's only one probe, so this should do nothing
        app.on_down();
        assert_eq!(app.state.selected_probe().name, String::from("0"));
    }

    #[test]
    fn two_probes_per_tab() {
        let config = vec![
            ProbeConfig {
                name: String::from("0"),
                filter: None,
                address: String::from(""),
            },
            ProbeConfig {
                name: String::from("1"),
                filter: None,
                address: String::from(""),
            },
            ProbeConfig {
                name: String::from("2"),
                filter: None,
                address: String::from(""),
            },
        ];
        let state = AppState::from_probes(config);
        let mut app = App::new("Probe", state);

        // set probes per tab manually
        app.tabs.probes_per_tab = 2;
        assert_eq!(app.state.selected_probe().name, String::from("0"));

        app.on_down();
        assert_eq!(app.state.selected_probe().name, String::from("1"));

        // already on bottom probe, shouldn't do anything
        app.on_down();
        assert_eq!(app.state.selected_probe().name, String::from("1"));

        app.on_up();
        assert_eq!(app.state.selected_probe().name, String::from("0"));

        // already on top probe, shouldn't do anything
        app.on_up();
        assert_eq!(app.state.selected_probe().name, String::from("0"));
    }
}
