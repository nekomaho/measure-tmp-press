//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::i2c::{self, Config};
use embassy_time::Timer;
use gpio::{Level, Output};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};

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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

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
        Text::with_baseline("led on!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        led.set_high();
        display.flush().unwrap();
        Timer::after_millis(1000).await;
        display.clear_buffer();
        display.flush().unwrap();

        info!("led off!");
        Text::with_baseline("led off!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        led.set_low();
        display.flush().unwrap();
        Timer::after_millis(1000).await;
        display.clear_buffer();
        display.flush().unwrap();
    }
}

