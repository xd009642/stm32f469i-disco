#![no_std]
#![no_main]
#![feature(panic_implementation)]
#![feature(used)]
#![feature(core_intrinsics)]
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

entry!(main);
exception!(HardFault, safe_state);
exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

fn safe_state(_ef: &ExceptionFrame) -> ! {
    loop {}
}

fn main() -> ! {
    let p = stm32f469xx::Peripherals::take().unwrap();
    let gpiog = p.GPIOG.split();
    let gpiok = p.GPIOK.split();
    let gpiod = p.GPIOD.split();
    let mut led1 = gpiog.pg6.into_output_pushpull().downgrade();
    let mut led2 = gpiod.pd4.into_output_pushpull().downgrade();
    let mut led3 = gpiod.pd5.into_output_pushpull().downgrade();
    let mut led4 = gpiok.pk3.into_output_pushpull().downgrade();
    
    // Turn on PG6 PD4 PD5 PK3
    loop {   
        for _i in 1..10000 {
            led1.set_low();
            // Orange led
            led2.set_high();
            // Red
            led3.set_high();
            led4.set_low();
        }

        for _i in 1..10000 {
            led1.set_high();
            // Orange led
            led2.set_low();
            // Red
            led3.set_low();

            led4.set_high();
        }
    }
}
