#![no_std]
#![allow(dead_code)]

#[cfg(not(feature = "device"))]
compile_error!(
    "The HAL must be built for a specific device or as a library."
);

#[cfg(feature = "atsam3x8e")]
pub use atsam3x8e as pac;

pub mod dacc;
pub mod peripheral_id;
mod pmc;
