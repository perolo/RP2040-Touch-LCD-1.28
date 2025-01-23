//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::time::Duration;
//use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::pwm::{PioPwm, PioPwmProgram};
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

const REFRESH_INTERVAL: u64 = 20000;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    //let pins = p.pins;
//    let mut led = Output::new(p.PIN_25, Level::Low);
//    let backlight = pins.gpio2;

    //DEV_GPIO_Mode(LCD_RST_PIN, 1);
    let mut lcd_rst = Output::new(p.PIN_13, Level::Low);
    //DEV_GPIO_Mode(LCD_DC_PIN, 1);
    let mut lcd_dc = Output::new(p.PIN_8, Level::Low);
    //DEV_GPIO_Mode(LCD_CS_PIN, 1);
    let mut lcd_cs = Output::new(p.PIN_9, Level::Low);
    //DEV_GPIO_Mode(LCD_BL_PIN, 1);
    //let mut lcd_backlight = Output::new(p.PIN_25, Level::Low);

    //DEV_Digital_Write(LCD_CS_PIN, 1);
    lcd_cs.set_high();
    //DEV_Digital_Write(LCD_DC_PIN, 0);
    lcd_dc.set_low();
    //DEV_Digital_Write(LCD_BL_PIN, 1);
    //lcd_backlight.set_high();

    // PWM Config
    //gpio_set_function(LCD_BL_PIN, GPIO_FUNC_PWM);
    //slice_num = pwm_gpio_to_slice_num(LCD_BL_PIN);
    //pwm_set_wrap(slice_num, 100);
    //pwm_set_chan_level(slice_num, PWM_CHAN_B, 0);
    //pwm_set_clkdiv(slice_num, 50);
    //pwm_set_enabled(slice_num, true);


    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);

    // Note that PIN_25 is the led pin on the Pico
    let prg = PioPwmProgram::new(&mut common);
    let mut lcd_pwm_pio = PioPwm::new(&mut common, sm0, p.PIN_25, &prg);
    lcd_pwm_pio.set_period(Duration::from_micros(REFRESH_INTERVAL));
    lcd_pwm_pio.start();

    let mut duration = 0;
    loop {
        duration = (duration + 1) % 1000;
        lcd_pwm_pio.write(Duration::from_micros(duration));
        Timer::after_millis(1).await;
    }
}
