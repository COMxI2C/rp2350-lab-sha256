//! ============================================================================
//! RP2350-LAB-SHA256
//! ----------------------------------------------------------------------------
//! Hardware-accelerated SHA-256 driver for the Raspberry Pi RP2350.
//!
//! This module provides a safe, minimal and reusable abstraction over the
//! RP2350's dedicated SHA-256 hardware accelerator.
//!
//! Repository:
//! https://github.com/COMxI2C/rp2350-lab-sha256
//!
//! Author:
//! Carlos Daniel Perdomo Vela
//! Embedded Software Engineer
//!
//! License:
//! MIT
//! ============================================================================

#![no_std]
#![no_main]

use embassy_rp as hal;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;
use embassy_rp::gpio::{Level, Output};

//Panic Handler
use {panic_probe as _};
// Defmt Logging
use defmt_rtt as _;//deferred format
use defmt::info;

//Import our SHA256 driver
mod drivers;
use drivers::sha256::Sha256;

/// Application image definition required by the RP2350 Boot ROM.
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_6, Level::Low);

    //Initialize the SHA256 hardware accelerator
    let mut sha = Sha256::new();

    loop{
        let message = b"abc";

        info!("Computing SHA256 digest for message: {:?}", message);

        let mut hasher = sha.start();
        hasher.update(message);

        let digest = hasher.finalize();

        info!(
            "Digest:\n{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
            digest[0],
            digest[1],
            digest[2],
            digest[3],
            digest[4],
            digest[5],
            digest[6],
            digest[7]
        );
        
        // TODO:
        // Consider exposing a true Disabled -> Enabled type-state transition.
        // The current implementation initializes the peripheral directly.

        led.toggle();

        Timer::after_secs(1).await;
    }   
}

/// Metadata displayed by `picotool info`.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"rp2350-lab-sha256"),
    embassy_rp::binary_info::rp_program_description!(
        c"Hardware-accelerated SHA-256 driver for RP2350"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
