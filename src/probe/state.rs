use crate::probe::config;
use regex::Regex;
use std::cmp;
use std::collections::VecDeque;

pub struct TabsState {
    pub num: usize,
    pub probe_num: usize,
    pub index: usize,
}

impl TabsState {
    pub fn new() -> TabsState {
        TabsState {
            num: 1,
            probe_num: 0,
            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.num;
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.num - 1;
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub probes: Vec<ProbeState>,
    pub selected_probe: usize,
    pub detail_view: bool,
}
#[derive(Clone, Debug)]
pub struct ProbeState {
    pub name: String,
    pub filter: String,
    pub count: u32,
    ring: VecDeque<u64>,
    ring_buffer: u64,
    messages: VecDeque<String>,
}

impl AppState {
    pub fn from_probes(p: Vec<config::Probe>) -> AppState {
        AppState {
            probes: p.iter().map(|i| ProbeState::from(i.clone())).collect(),
            selected_probe: 0,
            detail_view: false,
        }
    }
    pub fn selected_probe(&self) -> ProbeState {
        self.probes[self.selected_probe].clone()
    }

    pub fn probes_for_tab(&self, index: usize, num: usize) -> Vec<ProbeState> {
        let upper = cmp::min(index * num + num, self.probes.len());
        self.probes[index * num..upper].to_vec()
    }
}

impl ProbeState {
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

impl From<config::Probe> for ProbeState {
    fn from(item: config::Probe) -> Self {
        ProbeState {
            name: item.name,
            filter: item.filter.unwrap_or(".*".to_string()),
            count: 0,
            ring_buffer: 0,
            messages: VecDeque::with_capacity(60),
            ring: VecDeque::with_capacity(60),
        }
    }
}
