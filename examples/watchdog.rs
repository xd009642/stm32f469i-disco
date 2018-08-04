#![no_std]
#![no_main]
#![feature(panic_implementation)]
#![feature(used)]
#![feature(asm)]
extern crate embedded_hal;
extern crate stm32f469xx_hal as hal;
#[macro_use(entry, exception)]
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate panic_abort;

use cortex_m_rt::ExceptionFrame;

use embedded_hal::digital::OutputPin;
use hal::stm32f469xx;
use hal::gpio::*;
use hal::rcc::*;
use hal::watchdog::*;

entry!(main);
exception!(HardFault, safe_state);
exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

fn safe_state(_ef: &ExceptionFrame) -> ! {
    loop {}
}

fn main() -> ! {
    let p = stm32f469xx::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let gpiod = p.GPIOD.split(&mut rcc.ahb1);
    let mut wwdg = p.WWDG.constrain(&mut rcc.apb1);
    let mut led2 = gpiod.pd4.into_output_pushpull().downgrade();
    for _i in 1..1000 {
        led2.set_low();
    }
    led2.set_high();
    wwdg.start(2);
    loop {
        // This pause fits in window to kick watchdog. 10000 will cause reset.
        for i in 1..1000 {
        }
        wwdg.kick();
    }
}
