#![no_std]
#![no_main]

mod error;
mod hardware;

use {
    defmt::{
        expect,
        info
    },
    defmt_rtt as _,
    panic_probe as _
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::RgbColor,
    draw_target::DrawTarget
};

use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};

use st7789v2_driver::{ST7789V2, VERTICAL};

use crate::error::Result;

// Defined elsewhere
/// Rust's `!` is unstable.  This is a locally-defined equivalent which is stable.
#[derive(Debug)]
pub enum Never {}

// We wrap the main code inside a secondary function to allow returning an error/result
// from task code, instead of a panic which is required in Embassy's main task.
#[expect(clippy::future_not_send, reason = "Safe in single-threaded, bare-metal embedded context")]
#[expect(clippy::items_after_statements, reason = "Keeps related code together")]
async fn inner_main(_spawner: Spawner) -> Result<Never> {
    info!("initializing hardware");

    // Initialize the board hardware that we're using.
    //
    // This takes care of GPIO states, serial ports, communication peripherals, etc.
    let mut hardware = hardware::Hardware::default();   

    // Initialize the display
    let mut display = ST7789V2::new(
        hardware.lcd_spi,
        hardware.lcd_dc,
        hardware.lcd_cs,
        hardware.lcd_rst,
        false,
        VERTICAL,
        240,
        240,
    );

    // Get a handle to the embassy-time Delay wrapper.
    let mut delay = Delay;

    // Initialize the ST7789V2 dispaly driver, passing in the embassy-time
    // delay implementation for STM32 for syncronizing the command delays.
    info!("initializing ST7789V2 driver");
    display.init(&mut delay).unwrap();

    // Clear the screen before turning on the backlight.
    info!("clearing the display");
    display.clear(Rgb565::BLACK).unwrap();

    info!("starting loop");
    loop {
        hardware.led.set_high();
        Timer::after_millis(300).await;
        hardware.led.set_low();
        Timer::after_millis(300).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // If the inner main returns an error then
    // something failed and we should panic.
    let err = inner_main(_spawner).await.unwrap_err();
    panic!("{err}");
}
