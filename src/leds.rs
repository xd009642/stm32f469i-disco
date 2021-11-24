use hal::gpio::*;
use embedded_hal::digital::OutputPin;

/// Green LED 
pub type LD1 = gpiog::PG6<Output<PushPull>>;
/// Orange LED 
pub type LD2 = gpiod::PD4<Output<PushPull>>;
/// Red LED
pub type LD3 = gpiod::PD5<Output<PushPull>>;
/// Blue LED
pub type LD4 = gpiok::PK3<Output<PushPull>>;


pub enum Led {
    Led1(LD1),
    Led2(LD2),
    Led3(LD3),
    Led4(LD4),
}

impl Led {
    
    pub fn on(&mut self) {
        use leds::Led::*;
        match *self {
            Led1(ref mut l) => l.set_low(),
            Led2(ref mut l) => l.set_low(),
            Led3(ref mut l) => l.set_low(),
            Led4(ref mut l) => l.set_low(),
        }
    }
    
    pub fn off(&mut self) {
        use leds::Led::*;
        match *self {
            Led1(ref mut l) => l.set_high(),
            Led2(ref mut l) => l.set_high(),
            Led3(ref mut l) => l.set_high(),
            Led4(ref mut l) => l.set_high(),
        }
    }
}

