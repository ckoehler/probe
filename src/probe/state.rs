// use crate::probe::config;
use regex::Regex;
use std::cmp;
use std::collections::VecDeque;

use super::config::ProbeConfig;

pub struct TabsState {
    pub num_tabs: usize,
    pub probes_per_tab: usize,
    pub selected_tab: usize,
}

impl TabsState {
    pub fn new(num_probes: usize, probes_per_tab: usize) -> TabsState {
        let num_tabs = ((num_probes as f64 / probes_per_tab as f64).ceil()) as usize;
        TabsState {
            num_tabs,
            probes_per_tab,
            selected_tab: 0,
        }
    }
    pub fn next(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.num_tabs;
    }

    pub fn previous(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = self.num_tabs - 1;
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub probes: Vec<Probe>,
    pub selected_probe: usize,
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
            selected_probe: 0,
            detail_view: false,
        }
    }
    pub fn selected_probe(&self) -> Probe {
        self.probes[self.selected_probe].clone()
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
        let _ = TabState::new(num_probes, probes_per_tab);
    }
}
