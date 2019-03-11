// Layout schema
//
// +----------------------------------+
// | tabs                             |
// +--------+-------------------------+
// | % bar  | voltage graph           |
// +--------+                         |
// | info   +-------------------------+
// | table  | amperage graph          |
// |        |                         |
// |        +-------------------------+
// |        | energy rate graph       |
// |        |                         |
// |        +-------------------------+
// |        | another graph?          |
// |        |                         |
// +--------+-------------------------+



use std::io;
use std::error::Error;
use std::time::Duration;

use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::{Frame, Terminal};
use tui::layout::*;
use tui::style::*;
use tui::widgets::*;

use battery::{Battery, State};
use battery::units::Unit;
use battery::units::power::watt;
use battery::units::time::second;
use battery::units::ratio::percent;
use battery::units::energy::watt_hour;
use battery::units::electric_potential::volt;
use battery::units::thermodynamic_temperature::degree_celsius;

use crate::ui::app::BatteryStats;
use crate::ui::util::event::{Event, Events};
use crate::ui::util::tabs::TabsState;
use crate::ui::util::graph::GraphData;

mod app;
mod util;

pub fn start() -> Result<(), Box<Error>> {
   // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    let manager = battery::Manager::new()?;

    let mut app = app::App::new(manager)?;

    loop {
        terminal.draw(|mut f| {
            let battery = &app.batteries[app.tabs.index];

            // Tabs + main window
            let main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                .split(f.size());

            // Left column with info and right column with graphs
            let main_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(40), Constraint::Min(20)].as_ref())
                .split(main[1]);

            // Percentage bar and information table
            let left_column = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // percentage bar
                    Constraint::Length(10), // common info
                    Constraint::Length(9), // energy stuff
                    Constraint::Length(5), // timings
                    Constraint::Min(4), // environment
                ].as_ref())
                .split(main_columns[0]);

            // Graphs
            let right_column = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ].as_ref())
                .split(main_columns[1]);

            draw_tabs(&mut f, main[0], &app.tabs);
            draw_percentage_bar(&mut f, left_column[0], &battery);
            draw_common_information(&mut f, left_column[1], &battery.battery);
            draw_energy_information(&mut f, left_column[2], &battery.battery);
            draw_time_information(&mut f, left_column[3], &battery.battery);
            draw_env_information(&mut f, left_column[4], &battery.battery);
            draw_chart(&mut f, right_column[0], &battery.temperature_graph);
            draw_chart(&mut f, right_column[1], &battery.voltage_graph);
            draw_chart(&mut f, right_column[2], &battery.energy_rate_graph);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            },
            Event::Tick => app.update(),
        }
    }

    Ok(())
}

fn draw_tabs<B>(f: &mut Frame<B>, area: Rect, tabs: &TabsState) where B: Backend {
    Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Batteries"))
        .titles(&tabs.titles)
        .select(tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow))
        .render(f, area);
}

fn draw_percentage_bar<B>(f: &mut Frame<B>, area: Rect, stat: &BatteryStats) where B: Backend {
    let value = f64::from(stat.battery.state_of_charge().get::<percent>());
    let block = Block::default()
        .title("Percentage")
        .borders(Borders::ALL);
    let color = match () {
        _ if value > 30.0 => Color::Green,
        _ if value > 15.0 => Color::Yellow,
        _ => Color::Red
    };
    Gauge::default()
        .block(block)
        .ratio(value / 100.0)
        .style(Style::default().fg(color))
        .label(&format!("{:.2} %", value))
        .render(f, area);
}


fn draw_chart<B>(f: &mut Frame<B>, area: Rect, graph: &GraphData) where B: Backend {
    let block = Block::default()
        .title(graph.title())
        .borders(Borders::ALL);
    let value = graph.current();
    // tui automatically hides chart legend if it's height is higher than `chart.height / 3`.
    // Since we have 4 charts already, legend will be invisible for most monitors,
    // so instead writing value as a X axis label
    let x_axis: Axis<String> = Axis::default()
        .title(&value)
        .style(Style::default().fg(Color::Yellow))
        .bounds(graph.x_bounds());
    let y_labels = graph.y_labels();
    let y_axis: Axis<String> = Axis::default()
        .title(graph.y_title())
        .labels(&y_labels)
        .bounds(graph.y_bounds());

    Chart::default()
        .block(block)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .datasets(&[
            Dataset::default()
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .data(graph.points())
        ])
        .render(f, area)
}

fn draw_common_information<B>(f: &mut Frame<B>, area: Rect, battery: &Battery) where B: Backend {
    let block = Block::default()
        .title("Information")
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT);

    let tech = &format!("{}", battery.technology());
    let state = &format!("{}", battery.state());
    let cycles = match battery.cycle_count() {
        Some(cycles) => format!("{}", cycles),
        None => "N/A".to_string(),
    };
    let items = vec![
        vec!["Vendor", battery.vendor().unwrap_or("N/A")],
        vec!["Model", battery.model().unwrap_or("N/A")],
        vec!["S/N", battery.serial_number().unwrap_or("N/A")],
        vec!["Technology", tech],
        vec!["Charge state", state],
        vec!["Cycles count", &cycles],
    ];
    let header = ["Device", ""];

    let rows = items.iter().map(|item| {
        Row::Data(item.iter())
    });

    Table::new(header.iter(), rows)
        .header_style(Style::default().fg(Color::DarkGray))
        .block(block)
        .widths(&[17, 17])
        .render(f, area);
}

fn draw_energy_information<B>(f: &mut Frame<B>, area: Rect, battery: &Battery) where B: Backend {
    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT);

    let consumption = &format!("{:.2} {}", battery.energy_rate().get::<watt>(), watt::abbreviation());
    let voltage = &format!("{:.2} {}", battery.voltage().get::<volt>(), volt::abbreviation());
    let capacity = &format!("{:.2} {}", battery.state_of_health().get::<percent>(), percent::abbreviation());
    let current = &format!("{:.2} {}", battery.energy().get::<watt_hour>(), watt_hour::abbreviation());
    let last_full = &format!("{:.2} {}", battery.energy_full().get::<watt_hour>(), watt_hour::abbreviation());
    let full_design = &format!("{:.2} {}", battery.energy_full_design().get::<watt_hour>(), watt_hour::abbreviation());
    let consumption_label = match battery.state() {
        State::Charging => "Charging with",
        State::Discharging => "Discharging with",
        _ => "Consumption",
    };
    let items = vec![
        vec![consumption_label, consumption],
        vec!["Voltage", voltage],
        vec!["Capacity", capacity],
        vec!["Current", current],
        vec!["Last full", last_full],
        vec!["Full design", full_design],
    ];
    let header = ["Energy", ""];

    let rows = items.iter().map(|item| {
        Row::Data(item.iter())
    });

    Table::new(header.iter(), rows)
        .header_style(Style::default().fg(Color::DarkGray))
        .block(block)
        .widths(&[17, 17])
        .render(f, area);
}

fn draw_time_information<B>(f: &mut Frame<B>, area: Rect, battery: &Battery) where B: Backend {
    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT);

    let time_to_full = match battery.time_to_full() {
        Some(time) => humantime::format_duration(Duration::from_secs(time.get::<second>() as u64)).to_string(),
        None => "N/A".to_string(),
    };

    let time_to_empty = match battery.time_to_empty() {
        Some(time) => humantime::format_duration(Duration::from_secs(time.get::<second>() as u64)).to_string(),
        None => "N/A".to_string(),
    };
    let items = vec![
        vec!["Time to full", &time_to_full],
        vec!["Time to empty", &time_to_empty],
    ];
    let header = ["Time", ""];

    let rows = items.iter().map(|item| {
        Row::Data(item.iter())
    });

    Table::new(header.iter(), rows)
        .header_style(Style::default().fg(Color::DarkGray))
        .block(block)
        .widths(&[17, 17])
        .render(f, area);
}

fn draw_env_information<B>(f: &mut Frame<B>, area: Rect, battery: &Battery) where B: Backend {
    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

    let temperature = match battery.temperature() {
        Some(temp) => format!("{:.2} {}", temp.get::<degree_celsius>(), degree_celsius::abbreviation()),
        None => "N/A".to_string(),
    };
    let items = vec![
        vec!["Temperature", &temperature],
    ];
    let header = ["Environment", ""];

    let rows = items.iter().map(|item| {
        Row::Data(item.iter())
    });

    Table::new(header.iter(), rows)
        .header_style(Style::default().fg(Color::DarkGray))
        .block(block)
        .widths(&[17, 17])
        .render(f, area);
}
