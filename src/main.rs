use std::{thread, time::Duration};

use rppal::{gpio::Gpio, system::DeviceInfo};

fn main() {
    println!(
        "Blinking an LED on a {}.",
        DeviceInfo::new()
            .expect("Failed to get device info")
            .model()
    );

    let mut pin = Gpio::new()
        .expect("Failed to create new pin")
        .get(17)
        .expect("Failed to get pin 17")
        .into_output();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}
