#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_hal::hal::port::mode::Output;
use arduino_hal::hal::port::Pin;
use arduino_hal::hal::port::PB1;

type PIN = PB1;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let mut led: Pin<Output, PIN> = pins.d1.into_output();
    led.toggle();
    loop {
        stutter_blink(&mut led, 25);
        arduino_hal::delay_ms(1_000);
    }
}

fn stutter_blink(led: &mut Pin<Output, PIN>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle();
        arduino_hal::delay_ms(i as u16);
    });
}
