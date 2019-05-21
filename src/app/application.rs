use std::fmt;
use std::sync::Arc;

use tui::backend::Backend;

use super::config::Config;
use super::events::{Event, EventHandler};
use super::ui;
use crate::{Error, Result};

pub fn init(config: Arc<Config>) -> Result<Application<impl Backend>> {
    let manager = battery::Manager::new()?;

    // This vec will be used for UI data pre-population before the first tick
    let batteries = manager
        .batteries()?
        .flatten()
        .map(|battery| ui::View::new(config.clone(), battery))
        .collect::<Vec<_>>();

    // Probing if any batteries are installed at all
    if batteries.is_empty() {
        error!("Unable to find any batteries in system, exiting");
        return Err(Error::NoBatteries);
    } else {
        trace!("Found {} batteries during initialization", batteries.len());
    }

    // Interface should be initialized before the events handler,
    // since it switches terminal into a proper mode
    let interface = ui::init(config.clone(), batteries)?;
    let events = EventHandler::from_config(&config);

    Ok(Application {
        manager,
        config,
        events,
        interface,
    })
}

pub struct Application<B: Backend> {
    manager: battery::Manager,
    config: Arc<Config>,
    events: EventHandler,
    interface: ui::Interface<B>,
}

impl<B: Backend> Application<B> {
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.interface.draw()?;
            self.handle_event()?;
        }
    }

    fn handle_event(&mut self) -> Result<()> {
        match self.events.next()? {
            Event::Exit => Err(Error::UserExit),
            Event::PreviousTab => {
                self.interface.tabs_mut().previous();
                Ok(())
            }
            Event::NextTab => {
                self.interface.tabs_mut().next();
                Ok(())
            }
            Event::Tick => {
                for view in self.interface.views_mut() {
                    view.update(&mut self.manager)?;
                }
                Ok(())
            }
        }
    }
}

impl<B: Backend> fmt::Debug for Application<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Application")
            .field("config", &self.config)
            .field("manager", &self.manager)
            .finish()
    }
}
