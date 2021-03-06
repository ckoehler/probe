use argh::FromArgs;
use serde::Deserialize;

/// Probe Config
#[derive(Debug, FromArgs)]
pub struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    pub tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    pub enhanced_graphics: bool,
    /// config file.
    #[argh(option, default = "String::from(\"prober.toml\")")]
    pub config: String,
}

#[derive(Debug, Deserialize)]
pub struct Probes {
    pub probes: Vec<Probe>,
}
#[derive(Debug, Deserialize)]
pub struct Probe {
    pub name: String,
    pub filters: Vec<Filter>,
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
