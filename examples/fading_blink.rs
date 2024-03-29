#![no_std]
#![no_main]

extern crate panic_halt;

#[arduino_hal::entry]
fn main() -> ! {
    // 64 evenly sampled sine values in range 0..200.
    let points: [u8; 64] = [
        100, 110, 120, 129, 138, 147, 156, 163, 171, 177, 183, 188, 192, 196, 198, 200, 200, 200,
        198, 196, 192, 188, 183, 177, 171, 163, 156, 147, 138, 129, 120, 110, 100, 90, 80, 71, 62,
        53, 44, 37, 29, 23, 17, 12, 8, 4, 2, 0, 0, 0, 2, 4, 8, 12, 17, 23, 29, 37, 44, 53, 62, 71,
        80, 90,
    ];
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let mut led = pins.d1.into_output();

    // Disable components to reduce power usage by 3mA.
    peripherals.CPU.prr.modify(|_, w| {
        w.pradc()
            .set_bit()
            .prusi()
            .set_bit()
            .prtim0()
            .set_bit()
            .prtim1()
            .set_bit()
    });
    peripherals
        .CPU
        .mcucr
        .modify(|_, w| w.se().set_bit().sm().pdown().pud().clear_bit());

    // Calibrate oscillator to lower frequency for additional 3mA. Writing `255` runs things at
    // 200% performance, and eats nearly 10mA more.
    peripherals.CPU.osccal.modify(|_, w| w.osccal().bits(0));
    // LED takes about 3mA.
    led.set_low();

    loop {
        points.iter().for_each(|p| {
            (0..511).for_each(|_| {
                led.set_low();
                arduino_hal::delay_us((*p).into());

                led.set_high();
                arduino_hal::delay_us((200 - p).into());
            });
        });
        led.set_low();
        arduino_hal::delay_us(2_000_000);
    }
}
