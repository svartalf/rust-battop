#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        fn main() {
            println!("battop is not intended to work with Windows yet!");
        }
    } else {
        use std::error::Error;

        mod ui;

        fn main() -> Result<(), Box<Error>> {
            ui::start()
        }
    }
}
