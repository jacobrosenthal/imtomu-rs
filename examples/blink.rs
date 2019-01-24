#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::watchdog::WatchdogDisable;
use tomu_hal::{
    delay::Delay,
    gpio::{pin::B7, OpenDrain, GPIO},
    prelude::*,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    let mut p = efm32::Peripherals::take().unwrap();
    let cp = efm32::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(p.WDOG).disable();

    let g = GPIO::new(&mut p.CMU);
    let mut red = g.split::<B7<OpenDrain>>();

    let cmu = p.CMU.constrain().freeze();
    let mut timer = Delay::new(cp.SYST, cmu);

    loop {
        red.set_high();
        timer.delay_ms(1000_u32);
        red.set_low();
        timer.delay_ms(1000_u32);
    }
}
