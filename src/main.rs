#![no_std]
#![no_main]

extern crate alloc;

use core::cell::RefCell;

use alloc::rc::Rc;
use embassy_executor::Spawner;
use embassy_rp::{config::Config, gpio::Flex};
use embassy_time::Timer;
use panic_probe as _;
use embedded_alloc::LlffHeap as Heap;
use rhai;

use crate::pins::{GpioPin, GpioPinId, GpioPins};

mod pins;

#[global_allocator]
static HEAP: Heap = Heap::empty();

unsafe fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

    #[allow(static_mut_refs)]
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    unsafe { init_heap() };
    let p = embassy_rp::init(Config::default());
    let mut gpio_pins = Rc::new(RefCell::new(pins::Pins { inner: [
        Flex::new(p.PIN_0), Flex::new(p.PIN_1),
        Flex::new(p.PIN_2), Flex::new(p.PIN_3),
        Flex::new(p.PIN_4), Flex::new(p.PIN_5),
        Flex::new(p.PIN_6), Flex::new(p.PIN_7),
        Flex::new(p.PIN_8), Flex::new(p.PIN_9),
        Flex::new(p.PIN_10), Flex::new(p.PIN_11),
        Flex::new(p.PIN_12), Flex::new(p.PIN_13),
        Flex::new(p.PIN_14), Flex::new(p.PIN_15),
        Flex::new(p.PIN_16), Flex::new(p.PIN_17),
        Flex::new(p.PIN_18), Flex::new(p.PIN_19),
        Flex::new(p.PIN_20), Flex::new(p.PIN_21),
        Flex::new(p.PIN_22), Flex::new(p.PIN_23),
        Flex::new(p.PIN_24), Flex::new(p.PIN_25),
        Flex::new(p.PIN_26), Flex::new(p.PIN_27),
        Flex::new(p.PIN_28), Flex::new(p.PIN_29)
    ]}));

    let mut r = rhai::Engine::new_raw();
    r.register_type_with_name::<GpioPins>("pins");
    r.register_type_with_name::<GpioPin>("Pin");
    r.register_fn("get_pin", |idx: i32| -> GpioPin {
        GpioPins::get_pin(idx)
    });
    
    {
        let gpio_pins = gpio_pins.clone();
        r.register_fn("into_output", move |pin: &mut GpioPin| {
            gpio_pins.borrow_mut()[pin.inner].set_as_output();
        });
    }
    {
        let gpio_pins = gpio_pins.clone();
        r.register_fn("set_high", move |pin: &mut GpioPin| {
            gpio_pins.borrow_mut()[pin.inner].set_high();
        });
    }

    r.run(include_str!("../scripts/gpio.rhai")).unwrap();
    loop {
        Timer::after_millis(1000).await;
    }
}