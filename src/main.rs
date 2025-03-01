//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::i2c::{self, Config};
use embassy_time::Timer;
use embassy_time::Delay;
use gpio::{Level, Output};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_hal_1;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use bme280::i2c::BME280;
use {defmt_rtt as _, panic_probe as _};

use heapless::String;

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

fn f32_to_string(value: f32) -> String<16> {
    let mut buf = String::<16>::new(); // for buffer
    write!(&mut buf, "{:.2}", value).unwrap(); // format by under two digit of the decimal point
    buf
}

fn get_bme280_result<I2C>(bme280: &mut BME280<I2C>) -> String<128>
where
    I2C: embedded_hal_1::i2c::ErrorType, I2C: embedded_hal_1::i2c::I2c // measure ofr BME280 requires this setting
{
    let measurements = bme280.measure(&mut Delay).unwrap();
    let temperature: String<16>= f32_to_string(measurements.temperature);
    let humidity: String<16>= f32_to_string(measurements.humidity);
    let pressure: String<16> = f32_to_string(measurements.pressure/100.0);

    let mut result: String<128> = String::new(); // for display result as the String

    write!(&mut result, "{}{}\n", "t:", temperature.as_str()).unwrap();
    write!(&mut result, "{}{}\n", "h:", humidity.as_str()).unwrap();
    write!(&mut result, "{}{}", "p:", pressure.as_str()).unwrap();
    result
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    // For BME280
    let sda_bme280 = p.PIN_0;
    let scl_bme280 = p.PIN_1;

    let i2c_bme280 = i2c::I2c::new_blocking(p.I2C0, scl_bme280, sda_bme280, Config::default());
    let mut bme280 = BME280::new_primary(i2c_bme280);
    bme280.init(&mut Delay).unwrap();

    // For OLED
    let sda = p.PIN_2;
    let scl= p.PIN_3;

    let i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.init().unwrap();
    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();

    loop {
        info!("led on!");
        Text::with_baseline(get_bme280_result(&mut bme280).as_str(), Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        led.set_high();
        display.flush().unwrap();
        display.clear_buffer();

        Timer::after_millis(1000).await;

        info!("led off!");
        Text::with_baseline(get_bme280_result(&mut bme280).as_str(), Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        led.set_low();
        display.flush().unwrap();
        display.clear_buffer();

        Timer::after_millis(1000).await;
    }
}
