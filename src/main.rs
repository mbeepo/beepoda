#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{config::Config, gpio::{self, Flex, Input, Level, Output, Pin, Pull}};
use panic_probe as _;
use rhai;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Config::default());
    let pins = [
        Gpio::new(p.PIN_0), Gpio::new(p.PIN_1),
        Gpio::new(p.PIN_2), Gpio::new(p.PIN_3),
        Gpio::new(p.PIN_4), Gpio::new(p.PIN_5),
        Gpio::new(p.PIN_6), Gpio::new(p.PIN_7),
        Gpio::new(p.PIN_8), Gpio::new(p.PIN_9),
        Gpio::new(p.PIN_10), Gpio::new(p.PIN_11),
        Gpio::new(p.PIN_12), Gpio::new(p.PIN_13),
        Gpio::new(p.PIN_14), Gpio::new(p.PIN_15),
        Gpio::new(p.PIN_16), Gpio::new(p.PIN_17),
        Gpio::new(p.PIN_18), Gpio::new(p.PIN_19),
        Gpio::new(p.PIN_20), Gpio::new(p.PIN_21),
        Gpio::new(p.PIN_22), Gpio::new(p.PIN_23),
        Gpio::new(p.PIN_24), Gpio::new(p.PIN_25),
        Gpio::new(p.PIN_26), Gpio::new(p.PIN_27),
        Gpio::new(p.PIN_28), Gpio::new(p.PIN_29)
    ];
}

pub struct Gpio<'a> {
    inner: Flex<'a>,
}

impl<'a> Gpio<'a> {
    pub fn new<P: Pin>(pin: P) -> Self {
        let inner = Flex::new(pin);
        Self { inner }
    }

    pub fn into_output(&mut self) -> &mut Self {
        self.inner.set_as_output();
        self
    }

    pub fn set_high(&mut self) -> &mut Self {
        self.inner.set_high();
        self
    }

    pub fn set_low(&mut self) -> &mut Self {
        self.inner.set_low();
        self
    }

    pub fn toggle(&mut self) -> &mut Self {
        self.inner.toggle();
        self
    }

    pub fn into_input(&mut self, pull: Pull) -> &mut Self {
        self.inner.set_as_input();
        self.inner.set_pull(pull);
        self
    }

    pub fn get_level(&self) -> Level {
        self.inner.get_level()
    }

    pub fn is_low(&self) -> bool {
        self.inner.is_low()
    }

    pub fn is_high(&self) -> bool {
        self.inner.is_high()
    }
}

