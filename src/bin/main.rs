//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::time::Duration;
use core::cell::RefCell;
//use defmt::*;
use embassy_executor::Spawner;
//use embassy_rp::clocks::clk_adc_freq;
use embassy_rp::gpio;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::pwm::{PioPwm, PioPwmProgram};
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
use display_interface_spi::SPIInterface;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use mipidsi::models::GC9A01;
use mipidsi::options::{Orientation, Rotation};
use mipidsi::Builder;
use embassy_time::Delay;
use embedded_graphics::image::{Image, ImageRawLE};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
//use embedded_graphics::text::Text;

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
    //let mut lcd_rst = Output::new(p.PIN_13, Level::Low);
    //DEV_GPIO_Mode(LCD_DC_PIN, 1);
    //let mut lcd_dc = Output::new(p.PIN_8, Level::Low);
    //DEV_GPIO_Mode(LCD_CS_PIN, 1);
    //let mut lcd_cs = Output::new(p.PIN_9, Level::Low);
    //DEV_GPIO_Mode(LCD_BL_PIN, 1);
    //let mut lcd_backlight = Output::new(p.PIN_25, Level::Low);

    //DEV_Digital_Write(LCD_CS_PIN, 1);
    //lcd_cs.set_high();
    //DEV_Digital_Write(LCD_DC_PIN, 0);
    //lcd_dc.set_low();
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

    // SPI Config
    //spi_init(LCD_SPI_PORT, 40000 * 1000);
    //gpio_set_function(LCD_CLK_PIN, GPIO_FUNC_SPI);
    //gpio_set_function(LCD_MOSI_PIN, GPIO_FUNC_SPI);    

   // create SPI
   let mut display_config = spi::Config::default();
   display_config.frequency = 40000 * 1000;
   display_config.phase = spi::Phase::CaptureOnSecondTransition;
   display_config.polarity = spi::Polarity::IdleHigh;

   let p_clk = p.PIN_10;
   let p_dcx = p.PIN_8;
   let p_miso = p.PIN_12;
   let p_mosi = p.PIN_11;
   let p_rst = p.PIN_13;
   let p_display_cs = p.PIN_9;

   let spi = Spi::new_blocking(p.SPI1, p_clk, p_mosi, p_miso, display_config.clone());
   let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

   let display_spi = SpiDeviceWithConfig::new(&spi_bus, Output::new(p_display_cs, Level::High), display_config);

   let dcx = Output::new(p_dcx, Level::Low);
   let rst = Output::new(p_rst, Level::Low);
   // dcx: 0 = command, 1 = data

   // Enable LCD backlight
   //let _bl = Output::new(bl, Level::High);
   lcd_pwm_pio.write(Duration::from_micros(500));

   // display interface abstraction from SPI and DC
   let di = SPIInterface::new(display_spi, dcx);

   // Define the display from the display interface and initialize it
   let mut display = Builder::new(GC9A01, di)
       .display_size(240, 240)
       .reset_pin(rst)
       .orientation(Orientation::new().rotate(Rotation::Deg0))
       .init(&mut Delay)
       .unwrap();
   display.clear(Rgb565::BLACK).unwrap();

   let raw_image_data = ImageRawLE::new(include_bytes!("../../ferris.raw"), 86);
   let ferris = Image::new(&raw_image_data, Point::new(34, 68));

   // Display the image
   ferris.draw(&mut display).unwrap();
   

    let mut duration = 0;
    loop {
        duration = (duration + 1) % 1000;
        lcd_pwm_pio.write(Duration::from_micros(duration));
        Timer::after_millis(1).await;
    }
}
