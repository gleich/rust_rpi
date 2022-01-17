use std::{thread, time};

use rust_gpiozero::LED;

fn main() {
    println!("Started program");
    let led = LED::new(17);
    led.on();
    println!("Turned LED on");
    thread::sleep(time::Duration::from_secs(1));
    led.off();
    println!("Turned LED off");
    led.close();
}
