use std::sync::Arc;

use battery::units;

use super::{ChartData, ChartType, Units};
use crate::app::Config;
use crate::Result;

/// View is a content of one separate tab - information about one specific battery
#[derive(Debug)]
pub struct View {
    config: Arc<Config>,
    battery: battery::Battery,
    voltage: ChartData,
    energy_rate: ChartData,
    temperature: ChartData,
}

impl View {
    pub fn new(config: Arc<Config>, battery: battery::Battery) -> View {
        View {
            config: config.clone(),
            battery,
            voltage: ChartData::new(config.clone(), ChartType::Voltage),
            energy_rate: ChartData::new(config.clone(), ChartType::EnergyRate),
            temperature: ChartData::new(config, ChartType::Temperature),
        }
    }

    /// Update internal state, but do not re-draw it
    pub fn update(&mut self, manager: &mut battery::Manager) -> Result<()> {
        manager.refresh(&mut self.battery)?;

        self.voltage
            .push(self.battery.voltage().get::<units::electric_potential::volt>());
        *self.voltage.battery_state() = self.battery.state();

        self.energy_rate
            .push(self.battery.energy_rate().get::<units::power::watt>());
        *self.energy_rate.battery_state() = self.battery.state();

        if let Some(temp) = self.battery.temperature() {
            let value = match self.config.units() {
                Units::Human => temp.get::<units::thermodynamic_temperature::degree_celsius>(),
                Units::Si => temp.get::<units::thermodynamic_temperature::kelvin>(),
            };
            self.temperature.push(value);
            *self.temperature.battery_state() = self.battery.state();
            self.temperature.enabled(true);
        } else {
            self.temperature.enabled(false);
        }

        Ok(())
    }

    /// Return view title used in a tab header
    pub fn title(&self) -> String {
        if let Some(model) = self.battery.model() {
            trace!("View is going to use battery model as a tab title: {}", model);
            return model.to_string();
        }

        if let Some(vendor) = self.battery.vendor() {
            trace!("View is going to use battery vendor as a tab title: {}", vendor);
            return vendor.to_string();
        }

        if let Some(sn) = self.battery.serial_number() {
            trace!("View is going to use battery S/N as a tab title: {}", sn);
            return sn.to_string();
        }

        warn!("View is unable to determine proper tab title, falling back to unknown");
        "Unknown battery".to_string()
    }

    pub fn battery(&self) -> &battery::Battery {
        &self.battery
    }

    pub fn voltage(&self) -> &ChartData {
        &self.voltage
    }

    pub fn energy_rate(&self) -> &ChartData {
        &self.energy_rate
    }

    pub fn temperature(&self) -> &ChartData {
        &self.temperature
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
