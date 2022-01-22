use std::{thread, time::Duration};

use log::info;
use log::LevelFilter;
use rppal::gpio::Gpio;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .expect("Failed to configure logger");

    let mut pin = Gpio::new()
        .expect("Failed to create new pin")
        .get(17)
        .expect("Failed to get pin 17")
        .into_output();

    loop {
        pin.toggle();
        info!("Toggled LED");
        thread::sleep(Duration::from_millis(200));
    }
}
