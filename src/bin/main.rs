//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    //let pins = p.pins;
//    let mut led = Output::new(p.PIN_25, Level::Low);
//    let backlight = pins.gpio2;
    let mut backlight = Output::new(p.PIN_25, Level::Low);

    loop {
        info!("led on!");
        backlight.set_high();
        //led.set_high();
        Timer::after_secs(1).await;

        info!("led off!");
        backlight.set_low();
        //led.set_low();
        Timer::after_secs(1).await;
    }
}
