#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

use cortex_m::peripheral::{syst::SystClkSource, NVIC};
use cortex_m_rt::entry;
use tomu::{
    efm32::{interrupt, Interrupt},
    prelude::*,
    Tomu,
};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();
    let red = tomu.led.red();

    tomu.watchdog.disable();

    let mut syst = tomu.delay.free();
    let mut nvic = tomu.NVIC;

    nvic.enable(Interrupt::USART0_RX);

    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(16777215); // 1s
    syst.enable_counter();
    red.off();

    loop {
        red.off();
        // busy wait until the timer wraps around
        while !syst.has_wrapped() {}
        red.on();
        // trigger the `USART0_RX` interrupt
        NVIC::pend(Interrupt::USART0_RX);
    }
}

#[interrupt]
fn USART0_RX() {
    // panic!();
}
