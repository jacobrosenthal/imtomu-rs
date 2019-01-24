#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use tomu_hal::prelude::*;

#[entry]
fn dogs() -> ! {
    let mut p = efm32::Peripherals::take().unwrap();
    let cp = efm32::CorePeripherals::take().unwrap();

    p.WDOG.disable();

    let gpio = p.GPIO.split(&mut p.CMU);
    let red = gpio.a0.into_push_pull_output();
    loop {}
}
