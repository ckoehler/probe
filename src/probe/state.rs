use crate::probe::config;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub probes: Vec<ProbeState>,
}
#[derive(Clone, Debug)]
pub struct ProbeState {
    pub name: String,
    pub filters: Vec<config::Filter>,
    pub count: u32,
}

impl AppState {
    pub fn from_probes(p: Vec<config::Probe>) -> AppState {
        AppState {
            probes: p.iter().map(|i| ProbeState::from(i.clone())).collect(),
        }
    }
}

impl From<config::Probe> for ProbeState {
    fn from(item: config::Probe) -> Self {
        ProbeState {
            name: item.name,
            filters: item.filters.unwrap_or(vec![]),
            count: 0,
        }
    }
}
