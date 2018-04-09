#![no_std]

extern crate embedded_hal;
extern crate stm32f469xx_hal as hal;

use embedded_hal::digital::OutputPin;
use hal::stm32f469xx;
use hal::gpio::*;

fn main() {
    let p = stm32f469xx::Peripherals::take().unwrap();

    let gpiod = p.GPIOD.split();

    // Turn on PG6 PD4 PD5 PK3
    
    // Orange led
    let mut led2 = gpiod.pd4.into_output_pushpull();
    led2.set_high();
    // Red
    let mut led3 = gpiod.pd5.into_output_pushpull();
    led3.set_high();
}
