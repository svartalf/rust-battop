use std::sync::Arc;

use battery::units::electric_potential::volt;
use battery::units::power::watt;
use battery::units::thermodynamic_temperature::{degree_celsius, kelvin};
use battery::units::Unit;
use battery::State;
use itertools::{Itertools, MinMaxResult};

use super::Units;
use crate::app::Config;

const RESOLUTION: usize = 512;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChartType {
    Voltage,
    EnergyRate,
    Temperature,
}

#[derive(Debug)]
pub struct ChartData {
    config: Arc<Config>,
    chart_type: ChartType,
    enabled: bool,

    battery_state: State,

    points: Vec<(f64, f64)>,
    value_latest: f64,
    value_min: f64,
    value_max: f64,
}

impl ChartData {
    pub fn new(config: Arc<Config>, chart_type: ChartType) -> Self {
        ChartData {
            config,
            chart_type,
            enabled: true,

            battery_state: State::Unknown,

            points: Vec::with_capacity(256),
            value_latest: 0.0,
            value_min: 100.0,
            value_max: 0.0,
        }
    }

    pub fn enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn battery_state(&mut self) -> &mut State {
        &mut self.battery_state
    }

    #[allow(clippy::cast_lossless)]
    pub fn push<T>(&mut self, value: T)
    where
        T: Into<f64>,
    {
        let value = value.into();

        if self.points.len() == RESOLUTION {
            self.points.remove(0);
        }
        for (x, _) in self.points.iter_mut() {
            *x -= 0.5;
        }

        self.value_latest = value;

        self.points.push((RESOLUTION as f64 / 2.0, value));
        match self.points.iter().minmax_by_key(|(_, y)| y) {
            MinMaxResult::MinMax((_, min), (_, max)) => {
                self.value_min = *min;
                self.value_max = *max;
            }
            MinMaxResult::OneElement((_, el)) => {
                self.value_min = *el;
                self.value_max = *el;
            }
            _ => {}
        }
    }

    // Texts and titles

    pub fn title(&self) -> &str {
        match self.chart_type {
            ChartType::Voltage => "Voltage",
            ChartType::EnergyRate => match self.battery_state {
                State::Charging => "Charging with",
                State::Discharging => "Discharging with",
                _ => "Consumption",
            },
            ChartType::Temperature => "Temperature",
        }
    }

    /// Current value formatted with proper units
    pub fn current(&self) -> String {
        if self.enabled {
            match self.chart_type {
                ChartType::Voltage => format!("{:.2} {}", self.value_latest, volt::abbreviation()),
                ChartType::EnergyRate => format!("{:.2} {}", self.value_latest, watt::abbreviation()),
                ChartType::Temperature => match self.config.units() {
                    Units::Human => format!("{:.2} {}", self.value_latest, degree_celsius::abbreviation()),
                    Units::Si => format!("{:.2} {}", self.value_latest, kelvin::abbreviation()),
                },
            }
        } else {
            "NOT AVAILABLE".to_string()
        }
    }

    // Data

    pub fn points(&self) -> &[(f64, f64)] {
        self.points.as_ref()
    }

    // X scale

    pub fn x_bounds(&self) -> [f64; 2] {
        [0.0, 256.0]
    }

    // Y scale

    pub fn y_title(&self) -> &str {
        match self.chart_type {
            ChartType::Voltage => volt::abbreviation(),
            ChartType::EnergyRate => watt::abbreviation(),
            ChartType::Temperature => match self.config.units() {
                Units::Human => degree_celsius::abbreviation(),
                Units::Si => kelvin::abbreviation(),
            },
        }
    }

    fn y_lower(&self) -> f64 {
        if self.enabled {
            let mut value = (self.value_min - 1.0).floor();
            if value < 0.0 {
                value = -1.0;
            }
            value
        } else {
            0.0
        }
    }

    fn y_upper(&self) -> f64 {
        if self.enabled {
            (self.value_max + 1.0).ceil()
        } else {
            0.0
        }
    }

    pub fn y_labels(&self) -> Vec<String> {
        vec![format!("{:2.0}", self.y_lower()), format!("{:2.0}", self.y_upper())]
    }

    pub fn y_bounds(&self) -> [f64; 2] {
        [self.y_lower(), self.y_upper()]
    }
}
