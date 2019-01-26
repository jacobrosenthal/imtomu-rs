#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::watchdog::WatchdogDisable;
use tomu_hal::{delay::Delay, prelude::*, watchdog::Watchdog};

#[entry]
fn main() -> ! {
    let mut p = efm32::Peripherals::take().unwrap();
    let cp = efm32::CorePeripherals::take().unwrap();

    Watchdog::new(p.WDOG).disable();

    let gpio = p.GPIO.split(&mut p.CMU);
    let mut red = gpio.a0.into_output_open_drain();

    let cmu = p.CMU.constrain().freeze();
    let mut timer = Delay::new(cp.SYST, cmu);

    loop {
        red.set_high();
        timer.delay_ms(1000_u32);
        red.set_low();
        timer.delay_ms(1000_u32);
    }
}
