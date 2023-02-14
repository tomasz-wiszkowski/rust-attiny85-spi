#![allow(dead_code)]

use core::convert::Infallible;
use core::marker::PhantomData;

use arduino_hal::pac::portb::DDRB;
use arduino_hal::pac::usi::{USIBR, USICR, USIDR, USISR};

pub trait Role {}
pub struct RoleMaster {}
pub struct RoleSlave {}

impl Role for RoleMaster {}
impl Role for RoleSlave {}

pub struct MicroSPI<'a, As: Role> {
    cr: &'a USICR, // Control register.
    sr: &'a USISR, // Status register.
    br: &'a USIBR, // Input (buffer) register.
    dr: &'a USIDR, // Output (data) register.
    ddrb: &'a DDRB,
    _as: PhantomData<As>,
}

pub trait SPI {
    fn init(&mut self);
    fn exchange(&mut self, data: u8) -> u8;
}

pub trait SPIRead {
    fn read(&mut self, dat: &mut [u8]) -> Result<usize, Infallible>;
}

pub trait SPIWrite {
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible>;
}

impl<'a, As: Role> MicroSPI<'a, As> {
    #[inline]
    pub fn new(usi: &'a arduino_hal::pac::USI, portb: &'a arduino_hal::pac::PORTB) -> Self {
        Self {
            cr: &usi.usicr,
            br: &usi.usibr,
            dr: &usi.usidr,
            sr: &usi.usisr,
            ddrb: &portb.ddrb,
            _as: PhantomData,
        }
    }
}

impl<T: SPI> SPIRead for T {
    fn read(&mut self, data: &mut [u8]) -> Result<usize, Infallible> {
        data.iter_mut().for_each(|d| *d = self.exchange(0));
        Ok(data.len())
    }
}

impl<T: SPI> SPIWrite for T {
    /// Push 8 bits of data, ignoring what's received.
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        data.iter().for_each(|d| {
            self.exchange(*d);
        });
        Ok(data.len())
    }
}

/// Basic Three-wire SPI implementation. Uses pins
/// - PB0 as the MOSI/DI, to receive the data (always read),
/// - PB1 as the MISO/DO, to send the data (always write),
/// - PB2 as the SCLK, controlled by master to send clock pulses (role-dependant).
impl<'a> SPI for MicroSPI<'a, RoleMaster> {
    fn init(&mut self) {
        // Pin 0 is the MOSI/DI, used to receive the data. Set to read.
        // Pin 1 is the MISO/DO, used to send the data. Set to write.
        // Pin 2 is the SCLK, used by master to send us clock pulses. Set to write.
        self.ddrb
            .modify(|_r, w| w.pb0().clear_bit().pb1().set_bit().pb2().set_bit());
        // Three-wire mode SPI. TC0 is the clock source.
        self.cr.write(|w| w.usiwm().three_wire().usics().ext_neg());
    }

    /// Send or receive 8 bits of data.
    /// Emits 8 clock pulses, pushing supplied data on the MISO pin.
    /// Meanwhile collects MOSI data.
    ///
    /// Based on Chapter 15.3.2 of
    /// https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-2586-AVR-8-bit-Microcontroller-ATtiny25-ATtiny45-ATtiny85_Datasheet.pdf
    fn exchange(&mut self, data: u8) -> u8 {
        self.dr.write(|w| unsafe { w.bits(data) });
        self.sr.modify(|_, w| w.usioif().set_bit());
        loop {
            self.cr
                .modify(|_, w| w.usitc().set_bit().usiclk().set_bit());
            if self.sr.read().usioif().bit_is_set() {
                break;
            }
        }
        self.br.read().bits()
    }
}

impl<'a> SPI for MicroSPI<'a, RoleSlave> {
    fn init(&mut self) {
        // Pin 0 is the MOSI/DI, used to receive the data. Set to read.
        // Pin 1 is the MISO/DO, used to send the data. Set to write.
        // Pin 2 is the SCLK, used by master to send us clock pulses. Set to read.
        self.ddrb
            .modify(|_r, w| w.pb0().clear_bit().pb1().set_bit().pb2().clear_bit());
        // Three-wire mode SPI. External clock source.
        self.cr.write(|w| w.usiwm().three_wire().usics().ext_neg());
    }

    /// Send or receive 8 bits of data.
    /// Emits 8 clock pulses, pushing supplied data on the MISO pin.
    /// Meanwhile collects MOSI data.
    ///
    /// Based on Chapter 15.3.3 of
    /// https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-2586-AVR-8-bit-Microcontroller-ATtiny25-ATtiny45-ATtiny85_Datasheet.pdf
    fn exchange(&mut self, data: u8) -> u8 {
        self.dr.write(|w| unsafe { w.bits(data) });
        self.sr.write(|w| w.usioif().set_bit());
        loop {
            if self.sr.read().usioif().bit_is_set() {
                break;
            }
        }
        self.br.read().bits()
    }
}
