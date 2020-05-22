/// Layout schema:
///
/// ```
///           +------+------+--------------------------------------------------------+
/// Tabs   →  | BAT0 | BAT1 | BAT2                                                   |
///           +------+------+----------+---------------------------------------------+  <------\
/// SoC    →  |:::::::::: 65%          | Voltage graph                               |         |
///           +------------------------|                                             |         |
/// Common    |                        | 33 % of the right column                    |         |
/// info   →  | Vendor: …              |                                             |         |
///           | Model: …               |                                             |
///           | S/N: …                 |                                             |
///           +------------------------+---------------------------------------------+         m
/// Energy    | Voltage: …             | Consumption graph                           |         a
/// info   →  | Consumption: …         |                                             |         i
///           +------------------------+ 33 % of the right column                    |         n
/// Timings   | Time to full: …        |                                             |
///        →  | Time to empty: …       |                                             |         w
///           +------------------------+                                             |         i
/// Environ   | Temperature: …         |                                             |         n
///        →  |                        +---------------------------------------------+         d
///           |                        | Temperature graph                           |         o
///           |                        |                                             |         w
///           |                        | 33+1 % of the right column                  |
///           |                        |                                             |         |
///           |                        |                                             |         |
///           |                        |                                             |         |
///           +------------------------+---------------------------------------------+         /
///                                                                                           /
///           ^            ↑                       ↑                                         /
///           |       left column            right column                                   /
///           \                                                                            /
///            \------------------ main window -------------------------------------------/
/// ```
use std::borrow::Cow;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Marker, Paragraph, Row, Table, Tabs, Text, Widget};
use tui::Frame;

use battery::units::electric_potential::volt;
use battery::units::energy::{joule, watt_hour};
use battery::units::power::watt;
use battery::units::ratio::{percent, ratio};
use battery::units::thermodynamic_temperature::{degree_celsius, kelvin};
use battery::units::time::second;
use battery::units::Unit;
use battery::State;

use super::{ChartData, TabBar, Units, View};

#[derive(Debug)]
pub struct Context<'i> {
    pub tabs: &'i TabBar,
    pub view: &'i View,
}

#[derive(Debug)]
pub struct Painter<'i>(Rc<Context<'i>>);

impl<'i> Painter<'i> {
    pub fn from_context(context: Rc<Context<'i>>) -> Painter<'i> {
        Painter(context)
    }

    pub fn draw<B: Backend>(&self, mut frame: Frame<B>) {
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // Tabs
                    Constraint::Min(10),   // Main window
                ]
                .as_ref(),
            )
            .split(frame.size());

        // Left column with info and right column with graphs
        let main_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(40), // Information
                    Constraint::Min(20),    // Graphs
                ]
                .as_ref(),
            )
            .split(main[1]);

        // Percentage bar and information table
        let left_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),  // percentage bar
                    Constraint::Length(10), // common info
                    Constraint::Length(9),  // energy stuff
                    Constraint::Length(5),  // timings
                    Constraint::Min(4),     // environment
                ]
                .as_ref(),
            )
            .split(main_columns[0]);

        // Graphs
        let right_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(33), // Voltage
                    Constraint::Percentage(33), // Consumption
                    Constraint::Percentage(34), // Temperature
                ]
                .as_ref(),
            )
            .split(main_columns[1]);

        // Drawing all the things now!
        self.draw_tabs(&mut frame, main[0]);
        self.draw_state_of_charge_bar(&mut frame, left_column[0]);
        self.draw_common_info(&mut frame, left_column[1]);
        self.draw_energy_info(&mut frame, left_column[2]);
        self.draw_timing_info(&mut frame, left_column[3]);
        self.draw_environment_info(&mut frame, left_column[4]);
        self.draw_chart(&self.view.voltage(), &mut frame, right_column[0]);
        self.draw_chart(&self.view.energy_rate(), &mut frame, right_column[1]);
        self.draw_chart(&self.view.temperature(), &mut frame, right_column[2]);
    }

    pub fn draw_tabs<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let mut title = String::from("Batteries");

        Tabs::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.format_section_title(&mut title))
                    .title_style(self.get_section_title_style()),
            )
            .titles(self.tabs.titles())
            .select(self.tabs.index())
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::White))
            .render(frame, area);
    }

    pub fn draw_state_of_charge_bar<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let value = f64::from(self.view.battery().state_of_charge().get::<ratio>());
        let value_label = f64::from(self.view.battery().state_of_charge().get::<percent>());
        let mut title = "Charge Percentage".to_string();

        let gauge_block = Block::default()
            .title(self.format_section_title(&mut title))
            .title_style(self.get_section_title_style())
            .borders(Borders::ALL);
        let text_block = Block::default().borders(Borders::ALL);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(("|100.00 %|".len()) as u16)].as_ref())
            .split(area);

        let (gauge_area, text_area) = (chunks[0], chunks[1]);

        let color = match () {
            _ if value > 0.3 => Color::Green,
            _ if value > 0.15 => Color::Yellow,
            _ => Color::Red,
        };
        Gauge::default()
            .block(gauge_block)
            .ratio(value)
            .style(Style::default().bg(Color::Black).fg(color))
            .label(&"")
            .render(frame, gauge_area);

        let text = Text::Raw(Cow::from(format!("{:.2} %\n", value_label)));
        Paragraph::new([text].iter())
            .block(text_block)
            .alignment(Alignment::Right)
            .render(frame, text_area);
    }

    pub fn draw_chart<B: Backend>(&self, data: &ChartData, frame: &mut Frame<B>, area: Rect) {
        let mut title = data.title().to_string();
        let block = Block::default()
            .title(self.format_section_title(&mut title))
            .title_style(self.get_section_title_style())
            .borders(Borders::ALL);
        let value = data.current();
        // tui automatically hides chart legend if it's height is higher than `chart.height / 3`.
        // Since we have 3 charts already, legend will be invisible for most monitors,
        // so instead writing value as a X axis label
        let x_axis: Axis<String> = Axis::default()
            .title(&value)
            .style(Style::default().fg(Color::Reset))
            .bounds(data.x_bounds());
        let y_labels = data.y_labels();
        let y_axis: Axis<String> = Axis::default()
            .title(data.y_title())
            .labels(&y_labels)
            .bounds(data.y_bounds());

        Chart::default()
            .block(block)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .datasets(&[Dataset::default()
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Green))
                .data(data.points())])
            .render(frame, area)
    }

    fn draw_common_info<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let mut title = "Information".to_string();
        let block = Block::default()
            .title(self.format_section_title(&mut title))
            .title_style(self.get_section_title_style())
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT);

        let tech = &format!("{}", self.view.battery().technology());
        let state = &format!("{}", self.view.battery().state());
        let cycles = &match self.view.battery().cycle_count() {
            Some(cycles) => format!("{}", cycles),
            None => "N/A".to_string(),
        };

        let items = vec![
            ["Vendor", self.view.battery().vendor().unwrap_or("N/A")],
            ["Model", self.view.battery().model().unwrap_or("N/A")],
            ["S/N", self.view.battery().serial_number().unwrap_or("N/A")],
            ["Technology", tech],
            ["Charge state", state],
            ["Cycles count", cycles],
        ];
        let header = ["Device", ""];

        self.draw_info_table(header, &items, block, frame, area);
    }

    fn draw_energy_info<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default().borders(Borders::LEFT | Borders::RIGHT);
        let battery = self.view.battery();
        let config = self.view.config();

        let consumption = &format!("{:.2} {}", battery.energy_rate().get::<watt>(), watt::abbreviation());
        let voltage = &format!("{:.2} {}", battery.voltage().get::<volt>(), volt::abbreviation());
        let capacity = &format!(
            "{:.2} {}",
            battery.state_of_health().get::<percent>(),
            percent::abbreviation()
        );
        let current = &match config.units() {
            Units::Human => format!(
                "{:.2} {}",
                battery.energy().get::<watt_hour>(),
                watt_hour::abbreviation()
            ),
            Units::Si => format!("{:.2} {}", battery.energy().get::<joule>(), joule::abbreviation()),
        };
        let last_full = &match config.units() {
            Units::Human => format!(
                "{:.2} {}",
                battery.energy_full().get::<watt_hour>(),
                watt_hour::abbreviation()
            ),
            Units::Si => format!("{:.2} {}", battery.energy_full().get::<joule>(), joule::abbreviation()),
        };
        let full_design = &match config.units() {
            Units::Human => format!(
                "{:.2} {}",
                battery.energy_full_design().get::<watt_hour>(),
                watt_hour::abbreviation()
            ),
            Units::Si => format!(
                "{:.2} {}",
                battery.energy_full_design().get::<joule>(),
                joule::abbreviation()
            ),
        };
        let consumption_label = match battery.state() {
            State::Charging => "Charging with",
            State::Discharging => "Discharging with",
            _ => "Consumption",
        };

        let items = vec![
            [consumption_label, consumption],
            ["Voltage", voltage],
            ["Capacity", capacity],
            ["Current", current],
            ["Last full", last_full],
            ["Full design", full_design],
        ];
        let header = ["Energy", ""];

        self.draw_info_table(header, &items, block, frame, area);
    }

    fn draw_timing_info<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default().borders(Borders::LEFT | Borders::RIGHT);
        let battery = self.view.battery();

        let time_to_full = &match battery.time_to_full() {
            Some(time) => humantime::format_duration(Duration::from_secs(time.get::<second>() as u64)).to_string(),
            None => "N/A".to_string(),
        };

        let time_to_empty = &match battery.time_to_empty() {
            Some(time) => humantime::format_duration(Duration::from_secs(time.get::<second>() as u64)).to_string(),
            None => "N/A".to_string(),
        };

        let items = vec![["Time to full", time_to_full], ["Time to empty", time_to_empty]];
        let header = ["Time", ""];

        self.draw_info_table(header, &items, block, frame, area);
    }

    fn draw_environment_info<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);
        let battery = self.view.battery();
        let config = self.view.config();

        let temperature = &match battery.temperature() {
            Some(temp) => match config.units() {
                Units::Human => format!("{:.2} {}", temp.get::<degree_celsius>(), degree_celsius::abbreviation()),
                Units::Si => format!("{:.2} {}", temp.get::<kelvin>(), kelvin::abbreviation()),
            },
            None => "N/A".to_string(),
        };

        let items = vec![["Temperature", temperature]];
        let header = ["Environment", ""];

        self.draw_info_table(header, &items, block, frame, area);
    }

    fn draw_info_table<B: Backend>(
        &self,
        header: [&str; 2],
        items: &[[&str; 2]],
        block: Block,
        frame: &mut Frame<B>,
        area: Rect,
    ) {
        // convert header and items to strings
        let header: Vec<String> = header.iter().cloned().map(|elem| elem.to_string()).collect();
        let items: Vec<[String; 2]> = items
            .iter()
            .cloned()
            .map(|item| [item[0].to_string() + ":", item[1].to_string()])
            .collect();

        // convert items to rows
        let rows = items.iter().map(|item| Row::Data(item.iter()));

        // create table
        Table::new(header.iter(), rows)
            .header_style(Style::default().modifier(Modifier::UNDERLINED))
            .block(block)
            .widths(&[17, 17])
            .render(frame, area);
    }

    fn format_section_title<'a>(&self, title: &'a mut String) -> &'a String {
        // put formatted contents into title buffer
        mem::swap(title, &mut format!(" {} ", &title));
        title
    }

    fn get_section_title_style(&self) -> Style {
        Style::default()
    }
}

impl<'i> Deref for Painter<'i> {
    type Target = Context<'i>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
