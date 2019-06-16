#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;

use std::sync::Arc;

use structopt::StructOpt;

mod app;
mod errors;

pub use self::errors::{Error, Result};

fn main() -> Result<()> {
    let config = Arc::new(app::config::Config::from_args());
    stderrlog::new()
        .module(module_path!())
        .verbosity(config.verbosity())
        .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    trace!("Starting with {:?}", &config);
    let mut app = app::init(config)?;

    match app.run() {
        Err(Error::UserExit) => {
            app.stop()?;
            trace!("Exit was requested by user, terminating");
            Ok(())
        }
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error occurred: {:?}", e);
            Err(e)
        }
    }
}
