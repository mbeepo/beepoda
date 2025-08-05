use core::ops::{Index, IndexMut};

use embassy_rp::gpio::Flex;

pub struct Pins<'a> {
    pub(crate) inner: [Flex<'a>; 30],
}

impl<'a> Index<GpioPinId> for Pins<'a> {
    type Output = Flex<'a>;

    fn index(&self, index: GpioPinId) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl<'a> IndexMut<GpioPinId> for Pins<'a> {
    fn index_mut(&mut self, index: GpioPinId) -> &mut Self::Output {
        &mut self.inner[index as usize]
    }
}

impl<'a> Index<GpioPin> for Pins<'a> {
    type Output = Flex<'a>;

    fn index(&self, index: GpioPin) -> &Self::Output {
        &self.inner[index.inner as usize]
    }
}

impl<'a> IndexMut<GpioPin> for Pins<'a> {
    fn index_mut(&mut self, index: GpioPin) -> &mut Self::Output {
        &mut self.inner[index.inner as usize]
    }
}

#[derive(Clone, Copy)]
pub enum GpioPinId {
    Pin0 = 0,
    Pin1,  Pin2,  Pin3,  Pin4,
    Pin5,  Pin6,  Pin7,  Pin8,  Pin9,
    Pin10, Pin11, Pin12, Pin13, Pin14,
    Pin15, Pin16, Pin17, Pin18, Pin19,
    Pin20, Pin21, Pin22, Pin23, Pin24,
    Pin25, Pin26, Pin27, Pin28, Pin29,
    InvalidPin,
}

#[derive(Clone, Copy)]
pub struct GpioPins {}

impl GpioPins {
    pub fn get_pin(idx: i32) -> GpioPin {
        use GpioPinId::*;
        GpioPin { 
            inner: match idx {
                0 =>  Pin0,   1 => Pin1,   2 => Pin2,   3 => Pin3,   4 => Pin4,
                5 =>  Pin5,   6 => Pin6,   7 => Pin7,   8 => Pin8,   9 => Pin9,
                10 => Pin10, 11 => Pin11, 12 => Pin12, 13 => Pin13, 14 => Pin14,
                15 => Pin15, 16 => Pin16, 17 => Pin17, 18 => Pin18, 19 => Pin19,
                20 => Pin20, 21 => Pin21, 22 => Pin22, 23 => Pin23, 24 => Pin24,
                25 => Pin25, 26 => Pin26, 27 => Pin27, 28 => Pin28, 29 => Pin29,
                _ => InvalidPin
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct GpioPin {
    pub inner: GpioPinId,
}