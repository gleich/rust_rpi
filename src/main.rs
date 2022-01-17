use rust_gpiozero::LED;

fn main() {
    let mut led = LED::new(17);
    led.blink(1.0, 1.0);
    led.wait();
}
