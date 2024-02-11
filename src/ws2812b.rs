#![allow(dead_code)]

use arduino_hal::pac::PORTB;
use core::arch::asm;
use core::marker::PhantomData;

pub trait LedType {}
pub struct LedType_WS2812B_GRB;
pub struct LedType_WS2812B_RGBW;

impl LedType for LedType_WS2812B_GRB {}
impl LedType for LedType_WS2812B_RGBW {}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    w: u8,
}

impl Color {
    pub fn color(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, w: 0x00 }
    }

    pub fn white(intensity: u8) -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            w: intensity,
        }
    }

    pub fn rgbwhite(intensity: u8) -> Self {
        Self {
            r: intensity,
            g: intensity,
            b: intensity,
            w: 0,
        }
    }
}

pub struct WS2812B<'a, Type: LedType> {
    portb: &'a PORTB,
    _type: PhantomData<Type>,
}

impl<'a> WS2812B<'a, LedType_WS2812B_GRB> {
    pub fn set_colors(&mut self, data: &[Color]) {
        data.into_iter().for_each(|c| {
            self.emit(c.g);
            self.emit(c.r);
            self.emit(c.b);
        });
    }
}

impl<'a> WS2812B<'a, LedType_WS2812B_RGBW> {
    pub fn set_colors(&mut self, data: &[Color]) {
        data.into_iter().for_each(|c| {
            self.emit(c.r);
            self.emit(c.g);
            self.emit(c.b);
            self.emit(c.w);
        });
    }
}

impl<'a, Type: LedType> WS2812B<'a, Type> {
    #[inline]
    pub fn new(portb: &'a arduino_hal::pac::PORTB) -> Self {
        Self {
            portb,
            _type: PhantomData {},
        }
    }

    pub fn init(&mut self) {
        // TODO: revisit pin usage.
        self.portb.ddrb.write(|w| w.pb1().set_bit());
        self.portb.portb.write(|w| w.pb1().set_bit());
    }

    fn emit(&mut self, mut v: u8) {
        (0..=7).for_each(|_| {
            self.portb.pinb.write(|w| w.pb1().set_bit());

            if v & 0x80 == 0 {
                unsafe {
                    asm!("nop");
                }
                self.portb.pinb.write(|w| w.pb1().set_bit());
                unsafe {
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                }
            } else {
                unsafe {
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                }
                self.portb.pinb.write(|w| w.pb1().set_bit());
                unsafe {
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                    asm!("nop");
                }
            }

            v = v.rotate_left(1);
        });
    }
}
