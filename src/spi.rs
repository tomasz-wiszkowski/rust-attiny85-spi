#![allow(dead_code)]

use core::convert::Infallible;
use core::marker::PhantomData;

use arduino_hal::pac::usi::{USICR, USIDR, USISR};
use arduino_hal::pac::portb::DDRB;
use ufmt::uWrite;

pub trait Role {}
pub struct RoleMaster {}
pub struct RoleSlave {}

impl Role for RoleMaster {}
impl Role for RoleSlave {}

pub struct MicroSPI<'a, As: Role> {
    cr: &'a USICR,
    dr: &'a USIDR,
    sr: &'a USISR,
    ddrb: &'a DDRB,
    _as: PhantomData<As>
}

impl<'a, As: Role> MicroSPI<'a, As> {
    #[inline]
    pub fn new(usi: &'a arduino_hal::pac::USI, portb: &'a arduino_hal::pac::PORTB) -> Self {
        Self {
            cr: &usi.usicr,
            dr: &usi.usidr,
            sr: &usi.usisr,
            ddrb: &portb.ddrb,
            _as: PhantomData
        }
    }
}

impl<'a> MicroSPI<'a, RoleMaster> {
    #[inline]
    pub fn init(&mut self) {
        // Pin 1 is the MISO, we use it to send the data.
        // Pin 2 is the SCLK, used by us to send us clock pulses.
        self.ddrb.modify(|_r, w| w.pb1().set_bit().pb2().set_bit());
        // Three-wire mode SPI. TC0 is the clock source.
        self.cr.write(|w| w.usiwm().three_wire().usics().ext_pos());
    }
}

impl<'a> MicroSPI<'a, RoleSlave> {
    #[inline]
    pub fn init(&mut self) {
        // Pin 1 is the MISO, we use it to send the data.
        // Pin 2 is the SCLK, used by master to send us clock pulses.
        self.ddrb.modify(|_r, w| w.pb1().set_bit().pb2().clear_bit());
        // Three-wire mode SPI. External clock source.
        self.cr.write(|w| w.usiwm().three_wire().usics().ext_pos());
    }
}

impl<'a> uWrite for MicroSPI<'a, RoleMaster> {
    /// Type of errors reported by SPI.
    type Error = Infallible;

    /// Send formatted bytes to the other device via SPI.
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        text.as_bytes().iter().for_each(|c| {
            self.dr.write(|w| unsafe { w.bits(*c) });
            self.sr.modify(|_, w| w.usioif().set_bit());
            loop {
                self.cr.modify(|_, w| w.usitc().set_bit().usiclk().set_bit());
                if self.sr.read().usioif().bit_is_set() {
                    break;
                }
            }
        });
        Ok(())
    }
}

impl<'a> uWrite for MicroSPI<'a, RoleSlave> {
    /// Type of errors reported by SPI.
    type Error = Infallible;

    /// Send formatted bytes to the other device via SPI.
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        text.as_bytes().iter().for_each(|c| {
            self.dr.write(|w| unsafe { w.bits(*c) });
            self.sr.write(|w| w.usioif().set_bit());
            loop {
                if self.sr.read().usioif().bit_is_set() {
                    break;
                }
            }
        });
        Ok(())
    }
}

