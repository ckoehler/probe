use argh::FromArgs;
use itertools::Itertools;
use regex::Regex;
use serde::Deserialize;

/// Probe Config
#[derive(Debug, FromArgs)]
pub struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "1000")]
    pub tick_rate: u64,
    /// config file.
    #[argh(option, default = "String::from(\"probe.toml\")")]
    pub config: String,
}

#[derive(Debug, Deserialize)]
pub struct Probes {
    pub probes: Vec<ProbeConfig>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct ProbeConfig {
    pub name: String,
    pub filter: Option<String>,
    pub address: String,
}

impl Probes {
    pub fn validate(&self) {
        self.probes.iter().for_each(|p| p.validate());

        // make sure all probe names are unique
        let p: Vec<&String> = self.probes.iter().map(|p| &p.name).unique().collect();
        assert!(
            p.len() == self.probes.len(),
            "Make sure Probe names are unique."
        );
    }
}
impl ProbeConfig {
    fn validate(&self) {
        // make sure Filter is a valid regex
        Regex::new(self.filter.as_ref().unwrap_or(&".*".to_string())).unwrap();
    }
}
