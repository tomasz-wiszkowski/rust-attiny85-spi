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
    let mut master = spi::MicroSPI::<spi::RoleMaster>::new(&usi, &portb);

    master.init();

    loop {
        // It's possible to configure prescaler to transmit data  at a lower rate.
        // This allows transmitting data over unreliable medium,
        // such as the 433MHz radio transitters/receivers.
        peripherals.CPU.clkpr.write(|w| w.clkpce().set_bit());
        peripherals.CPU.clkpr.write(|w| w.clkps().prescaler_8());
        master.write(b"bonanza!\n\r\0").unwrap();

        loop {
            let mut data_in = [0u8];
            master.read(&mut data_in).unwrap();
            if data_in[0] == 0x3a {
                break;
            }
        }

        // Sleep uses TC0. Reset prescaler for accurate sleep durations.
        peripherals.CPU.clkpr.write(|w| w.clkpce().set_bit());
        peripherals.CPU.clkpr.write(|w| w.clkps().prescaler_1());
        arduino_hal::delay_ms(1);
    }
}
