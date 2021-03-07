use argh::FromArgs;
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
    pub probes: Vec<Probe>,
}
#[derive(Debug, Deserialize)]
pub struct Probe {
    pub name: String,
    pub filters: Option<Vec<Filter>>,
    pub address: String,
    pub mode: String,
    pub count: u32,
}
#[derive(Debug, Deserialize)]
pub struct Filter {
    pub name: String,
    pub pattern: String,
    pub count: u32,
}
