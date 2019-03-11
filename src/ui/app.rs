#![allow(dead_code)]

use super::util::tabs;
use super::util::graph;

use battery::{Battery, Result};
use battery::units::power::watt;
use battery::units::electric_potential::volt;
use battery::units::thermodynamic_temperature::degree_celsius;

#[derive(Debug)]
pub struct BatteryStats<'b> {
    pub battery: Battery,
    pub voltage_graph: graph::GraphData<'b>,
    pub energy_rate_graph: graph::GraphData<'b>,
    pub temperature_graph: graph::GraphData<'b>,
}

#[derive(Debug)]
pub struct App<'a> {
    pub manager: battery::Manager,
    pub batteries: Vec<BatteryStats<'a>>,
    pub tabs: tabs::TabsState,
}

impl<'a> App<'a> {
    pub fn new(manager: battery::Manager) -> Result<App<'a>> {
        let stats: Vec<BatteryStats> = manager.batteries()?
            .flatten()
            .map(|b| {
                BatteryStats {
                    battery: b,
                    voltage_graph: graph::GraphData::new("Voltage", graph::GraphUnits::Volt),
                    energy_rate_graph: graph::GraphData::new("Consumption", graph::GraphUnits::Watt),
                    temperature_graph: graph::GraphData::new("Temperature",graph::GraphUnits::Celsius),
                }
            })
            .collect();
        let names = stats.iter()
        .filter_map(|b| {
            if let Some(model) = b.battery.model() {
                return Some(model.to_string());
            }

            None
        })
        .collect();

        Ok(App {
            manager,
            batteries: stats,
            tabs: tabs::TabsState::new(names),
        })
    }

    pub fn update(&mut self) {
        for stat in self.batteries.iter_mut() {
            let _ = self.manager.refresh(&mut stat.battery);
            stat.voltage_graph.push(f64::from(stat.battery.voltage().get::<volt>()));
            stat.energy_rate_graph.push(f64::from(stat.battery.energy_rate().get::<watt>()));
            if let Some(temp) = stat.battery.temperature() {
                stat.temperature_graph.push(f64::from(temp.get::<degree_celsius>()));
            }
        }
    }
}
