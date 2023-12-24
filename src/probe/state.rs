// use crate::probe::config;
use regex::Regex;
use std::cmp;
use std::collections::VecDeque;

use super::config::ProbeConfig;

#[derive(Debug)]
pub struct TabsState {
    pub num_tabs: usize,
    pub num_probes: usize,
    pub probes_per_tab: usize,
    pub selected_tab: usize,
    pub selected_probe: usize,
}

impl TabsState {
    pub fn new(num_probes: usize) -> Self {
        Self {
            num_tabs: 1,
            num_probes,
            probes_per_tab: num_probes,
            selected_tab: Default::default(),
            selected_probe: Default::default(),
        }
    }
    pub fn recalculate_layout(&mut self, num_probes: usize, probes_per_tab: usize) {
        let new_num_tabs = ((num_probes as f64 / probes_per_tab as f64).ceil()) as usize;

        // if we're changing the layout, likely because of a resize, also reset the currently
        // selected probe and tab.
        if new_num_tabs != self.num_tabs || probes_per_tab != self.probes_per_tab {
            self.selected_probe = 0;
            self.selected_tab = 0;
        }

        self.num_tabs = new_num_tabs;
        self.probes_per_tab = probes_per_tab;
    }

    pub fn next(&mut self) {
        self.selected_probe = 0;
        self.selected_tab = (self.selected_tab + 1) % self.num_tabs;
    }

    pub fn previous(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = self.num_tabs - 1;
        }
        self.selected_probe = 0;
    }

    pub fn next_probe(&mut self) {
        if self.selected_probe < self.probes_on_selected_page() - 1 {
            self.selected_probe += 1;
        }
    }

    pub fn prev_probe(&mut self) {
        if self.selected_probe > 0 {
            self.selected_probe -= 1;
        }
    }

    fn probes_on_selected_page(&self) -> usize {
        // all pages but the last
        if self.selected_tab < self.num_tabs - 1 {
            self.probes_per_tab

        // last page
        } else {
            self.num_probes % self.probes_per_tab
        }
    }

    pub fn selected_probe_index(&self) -> usize {
        self.selected_probe + self.selected_tab * self.probes_per_tab
    }
}

#[derive(Debug)]
pub struct AppState {
    pub probes: Vec<Probe>,
    pub detail_view: bool,
}

#[derive(Clone, Debug)]
pub struct Probe {
    pub name: String,
    pub filter: String,
    pub count: u32,
    ring: VecDeque<u64>,
    ring_buffer: u64,
    messages: VecDeque<String>,
}

impl AppState {
    pub fn from_probes(p: Vec<ProbeConfig>) -> AppState {
        AppState {
            probes: p.iter().map(|i| Probe::from(i.clone())).collect(),
            detail_view: false,
        }
    }

    pub fn probes_for_tab(&self, index: usize, num: usize) -> Vec<Probe> {
        let upper = cmp::min(index * num + num, self.probes.len());
        self.probes[index * num..upper].to_vec()
    }
}

impl Probe {
    pub fn process_message(&mut self, msg: &String) {
        if !self.filter.is_empty() {
            let re = Regex::new(&self.filter).unwrap();
            if re.is_match(msg) {
                self.update_message_buffer(msg);
                self.count += 1;
                self.ring_buffer += 1;
            }
        } else {
            self.update_message_buffer(msg);
        }
    }

    pub fn messages(self) -> String {
        self.messages.clone().make_contiguous().to_vec().join("\n")
    }

    pub fn update_message_buffer(&mut self, msg: &String) {
        self.messages.push_front(msg.to_string());
        if self.messages.len() >= 60 {
            self.messages.pop_back();
        }
    }

    // this is called once per tick, so do display related stuff here.
    pub fn update_state(&mut self) {
        self.ring.push_front(self.ring_buffer);
        if self.ring.len() >= 180 {
            self.ring.pop_back();
        }
        self.ring_buffer = 0;
    }

    pub fn histogram(&self) -> Vec<u64> {
        self.ring.clone().make_contiguous().to_vec()
    }
}

impl From<ProbeConfig> for Probe {
    fn from(item: ProbeConfig) -> Self {
        Probe {
            name: item.name,
            filter: item.filter.unwrap_or(".*".to_string()),
            count: 0,
            ring_buffer: 0,
            messages: VecDeque::with_capacity(60),
            ring: VecDeque::with_capacity(60),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tabstate() {
        let num_probes = 3;
        let probes_per_tab = 1;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.selected_probe, 0);
    }

    #[test]
    fn select_next_with_only_one() {
        let num_probes = 3;
        let probes_per_tab = 1;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.selected_probe, 0);
        state.next_probe();
        assert_eq!(state.selected_probe, 0);
    }

    #[test]
    fn select_next_with_multiple() {
        let num_probes = 3;
        let probes_per_tab = 2;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.selected_probe, 0);
        state.next_probe();
        assert_eq!(state.selected_probe, 1);
        state.next_probe();
        assert_eq!(state.selected_probe, 1);
    }

    #[test]
    fn select_prev_with_only_one() {
        let num_probes = 3;
        let probes_per_tab = 1;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.selected_probe, 0);
        state.prev_probe();
        assert_eq!(state.selected_probe, 0);
    }

    #[test]
    fn select_prev_with_multiple() {
        let num_probes = 3;
        let probes_per_tab = 2;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.selected_probe, 0);
        state.prev_probe();
        assert_eq!(state.selected_probe, 0);
        state.next_probe();
        assert_eq!(state.selected_probe, 1);
        state.prev_probe();
        assert_eq!(state.selected_probe, 0);
    }

    #[test]
    fn select_next_on_last() {
        let num_probes = 3;
        let probes_per_tab = 2;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        state.next_probe();
        assert_eq!(state.selected_probe, 1);

        // next page, where there's only one probe
        state.next();
        assert_eq!(state.selected_probe, 0);

        // should stay at 0, since this page only has one probe
        state.next_probe();
        assert_eq!(state.selected_probe, 0);
    }

    #[test]
    fn probes_on_selected_page() {
        let num_probes = 3;
        let probes_per_tab = 2;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.probes_on_selected_page(), 2);

        // next page, where there's only one probe
        state.next();
        assert_eq!(state.probes_on_selected_page(), 1);
    }

    #[test]
    fn test_selected_probe_index() {
        let num_probes = 3;
        let probes_per_tab = 2;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        state.next();
        assert_eq!(state.selected_probe_index(), 2);
    }

    #[test]
    fn test_bug1() {
        let num_probes = 10;
        let probes_per_tab = 8;
        let mut state = TabsState::new(num_probes);
        state.recalculate_layout(num_probes, probes_per_tab);
        assert_eq!(state.probes_on_selected_page(), 8);
        state.next();
        assert_eq!(state.probes_on_selected_page(), 2);
    }
}
