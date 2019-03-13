use std::str::FromStr;
use std::time::Duration;
use std::u64;

use crate::app::ui::Units;

fn parse_duration(raw: &str) -> Result<Duration, String> {
    match u64::from_str(raw) {
        Ok(seconds) if seconds > 0 => Ok(Duration::from_secs(seconds)),
        _ => Err(format!("{} isn't a positive number", raw)),
    }
}

/// Interactive batteries viewer.
///
/// The following commands are supported while in battop:
///
/// * Right: move to next tab
///
/// * Left: move to previous tab
///
/// * Q, Ctrl+C, Esc: close viewer
#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(short = "v", long = "verbose", max_values = 5, parse(from_occurrences))]
    /// Verbosity level, might be repeated up to 5 times (-vvvvv).
    /// Log is accessible from the stderr.
    verbose: usize,

    #[structopt(
        short = "d",
        long = "delay",
        default_value = "1",
        parse(try_from_str = "parse_duration")
    )]
    /// Delay between updates, in seconds
    delay: Duration,

    #[structopt(
        short = "u",
        long = "units",
        default_value = "human",
        raw(possible_values = "&Units::arg_variants()", case_insensitive = "true")
    )]
    /// Measurement units displayed
    units: Units,
}

impl Config {
    pub fn verbosity(&self) -> usize {
        self.verbose
    }

    pub fn delay(&self) -> &Duration {
        &self.delay
    }

    pub fn units(&self) -> Units {
        self.units
    }
}
