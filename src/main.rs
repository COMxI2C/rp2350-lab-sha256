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

//Panic Handler
use {panic_probe as _};
// Defmt Logging
use defmt_rtt as _;//deferred format,

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    loop{
        Timer::after_millis(100).await;
    }   
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"rp2350-lab-sha256"),
    embassy_rp::binary_info::rp_program_description!(
        c"sha256 example for rp2350 using embassy-rp"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
