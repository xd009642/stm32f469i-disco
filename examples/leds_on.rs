#![no_std]
#![no_main]
#![feature(panic_implementation)]
#![feature(used)]
#![feature(asm)]
extern crate embedded_hal;
extern crate stm32f469xx_hal as hal;
extern crate stm32f469i_disco as bsp;
#[macro_use(entry, exception)]
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate panic_abort;


use cortex_m_rt::ExceptionFrame;
use bsp::leds::Led;
use hal::stm32f469xx;
use hal::gpio::*;
use hal::rcc::*;

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
    let gpiog = p.GPIOG.split(&mut rcc.ahb1);
    let gpiok = p.GPIOK.split(&mut rcc.ahb1);
    let gpiod = p.GPIOD.split(&mut rcc.ahb1);
    let mut led1 = Led::Led1(gpiog.pg6.into_output_pushpull());
    let mut led2 = Led::Led2(gpiod.pd4.into_output_pushpull());
    let mut led3 = Led::Led3(gpiod.pd5.into_output_pushpull());
    let mut led4 = Led::Led4(gpiok.pk3.into_output_pushpull());
    
    // Turn on PG6 PD4 PD5 PK3
    loop {   
        for _i in 1..10000 {
            led1.off();
            // Orange led
            led2.off();
            // Red
            led3.on();
            led4.on();
        }

        for _i in 1..10000 {
            led1.on();
            // Orange led
            led2.on();
            // Red
            led3.off();

            led4.off();
        }
    }
}
