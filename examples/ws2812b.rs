#![no_std]
#![no_main]

extern crate panic_halt;
use attiny85::ws2812b::Color;
use attiny85::ws2812b::LedType_WS2812B_GRB;
use attiny85::ws2812b::WS2812B;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let portb = peripherals.PORTB;
    let mut controller = WS2812B::<LedType_WS2812B_GRB>::new(&portb);

    peripherals.CPU.osccal.write(|w| w.osccal().bits(0x7f));

    controller.init();
    let mut i = 0u8;
    let mut j = 0x20u8;
    loop {
        let mut w1 = 0u8;
        let mut w2 = 0u8;
        let mut w3 = 0u8;
        let mut w4 = 0u8;
        let mut w5 = 0u8;

        let acc1 = (i & 0x3f).wrapping_shl(1);
        let dcc1 = (0x40 - (i & 0x3f)).wrapping_shl(1);
        let acc2 = (j & 0x3f).wrapping_shl(1);
        let dcc2 = (0x40 - (j & 0x3f)).wrapping_shl(1);

        if i < 0x20 {
            w1 = acc1;
        } else if i < 0x40 {
            w1 = acc1;
            w2 = acc2;
        } else if i < 0x60 {
            w1 = dcc1;
            w2 = acc2;
            w3 = acc1;
        } else if i < 0x80 {
            w1 = dcc1;
            w2 = dcc2;
            w3 = acc1;
            w4 = acc2;
        } else if i < 0xa0 {
            w2 = dcc2;
            w3 = dcc1;
            w4 = acc2;
            w5 = acc1;
        } else if i < 0xc0 {
            w3 = dcc1;
            w4 = dcc2;
            w5 = acc1
        } else if i < 0xe0 {
            w4 = dcc2;
            w5 = dcc1;
        } else {
            w5 = dcc1;
        }

        controller.set_colors(&[
            Color::color(255, w1, w1),
            Color::color(w1, w2, 255),
            Color::color(0, 0, w3),
            Color::rgbwhite(255),
            Color::color(255, w4, w5),
            Color::color(255, w4, w5),
            Color::rgbwhite(255),
            Color::color(0, 0, w3),
            Color::color(w1, w2, 255),
            Color::color(255, w1, w1),
            Color::color(255, w1, w1),
            Color::color(w1, w2, 255),
            Color::color(0, 0, w3),
            Color::rgbwhite(255),
            Color::color(255, w4, w5),
            Color::color(255, w4, w5),
            Color::rgbwhite(255),
            Color::color(0, 0, w3),
            Color::color(w1, w2, 255),
            Color::color(255, w1, w1),
            Color::color(255, w1, w1),
            Color::color(w1, w2, 255),
            Color::color(0, 0, w3),
            Color::rgbwhite(255),
            Color::color(255, w4, w5),
        ]);

        i = i.wrapping_add(1);
        j = j.wrapping_add(1);

        // Sleep uses TC0. Reset prescaler for accurate sleep durations.
        peripherals.CPU.clkpr.write(|w| w.clkpce().set_bit());
        peripherals.CPU.clkpr.write(|w| w.clkps().prescaler_1());
        arduino_hal::delay_ms(30);
    }
}
