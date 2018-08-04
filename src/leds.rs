use hal::gpio::*;


struct Led1<MODE> {
    led: gpiog::PG6<Output<MODE>>,
}


struct Led2<MODE> {
    led: gpiod::PD4<Output<MODE>>,
}


struct Led3<MODE> {
    led: gpiod::PD5<Output<MODE>>,
}


struct Led4<MODE> {
    led: gpiok::PK3<Output<MODE>>,
}

