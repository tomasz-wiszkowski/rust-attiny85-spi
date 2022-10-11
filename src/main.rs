#![no_std]
#![no_main]

extern crate panic_halt;
mod spi;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let usi = peripherals.USI;
    let portb = peripherals.PORTB;
    let mut out = spi::MicroSPI::<spi::RoleMaster>::new(&usi, &portb);

    // Enable prescaler to transmit data at a lower rate.
    peripherals.CPU.clkpr.write(|w| { w.clkpce().set_bit() });
    peripherals.CPU.clkpr.write(|w| { w.clkps().prescaler_256() });

    out.init();

    loop {
        ufmt::uwrite!(&mut out, "Bonanza!\n\r").unwrap();
        arduino_hal::delay_ms(100);
    }
}
