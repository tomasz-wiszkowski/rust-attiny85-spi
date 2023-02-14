#![no_std]
#![no_main]

extern crate panic_halt;
use attiny85::spi::{self, SPIRead, SPIWrite, SPI};

// Example master SPI implementation.
// Works with example slave SPI implementation.
//
// IMPORTANT:
// Please be sure to flash both of the devices before wiring them up.
//
// Wiring diagram:
// - GND  --- GND
// - SCLK --- SCLK (PB2 --- PB2)
// - MOSI <-> MISO (PB1 <-> PB0), cross-connect
//
#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let usi = peripherals.USI;
    let portb = peripherals.PORTB;
    let mut slave = spi::MicroSPI::<spi::RoleSlave>::new(&usi, &portb);

    slave.init();

    loop {
        let mut data_in = [0u8];
        // Note: pre-scaler speed shouldn't matter.
        slave.read(&mut data_in).unwrap();

        if data_in[0] == 0 {
            slave.write(&[0x3a]).unwrap();
        }
    }
}
