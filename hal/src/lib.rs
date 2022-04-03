#![no_std]
#![allow(dead_code)]

#[cfg(not(feature = "device"))]
compile_error! {
    "The HAL must be built for a specific device or as a library."
}

#[cfg(feature = "atsam3a4c")]
pub use atsam3a4c as pac;

#[cfg(feature = "atsam3a8c")]
pub use atsam3a8c as pac;

#[cfg(feature = "atsam3n00a")]
pub use atsam3n00a as pac;

#[cfg(feature = "atsam3n00b")]
pub use atsam3n00b as pac;

#[cfg(feature = "atsam3n0a")]
pub use atsam3n0a as pac;

#[cfg(feature = "atsam3n0b")]
pub use atsam3n0b as pac;

#[cfg(feature = "atsam3n0c")]
pub use atsam3n0c as pac;

#[cfg(feature = "atsam3n1a")]
pub use atsam3n1a as pac;

#[cfg(feature = "atsam3n1b")]
pub use atsam3n1b as pac;

#[cfg(feature = "atsam3n1c")]
pub use atsam3n1c as pac;

#[cfg(feature = "atsam3n2a")]
pub use atsam3n2a as pac;

#[cfg(feature = "atsam3n2b")]
pub use atsam3n2b as pac;

#[cfg(feature = "atsam3n2c")]
pub use atsam3n2c as pac;

#[cfg(feature = "atsam3n4a")]
pub use atsam3n4a as pac;

#[cfg(feature = "atsam3n4b")]
pub use atsam3n4b as pac;

#[cfg(feature = "atsam3n4c")]
pub use atsam3n4c as pac;

#[cfg(feature = "atsam3s1a")]
pub use atsam3s1a as pac;

#[cfg(feature = "atsam3s1b")]
pub use atsam3s1b as pac;

#[cfg(feature = "atsam3s1c")]
pub use atsam3s1c as pac;

#[cfg(feature = "atsam3s2a")]
pub use atsam3s2a as pac;

#[cfg(feature = "atsam3s2b")]
pub use atsam3s2b as pac;

#[cfg(feature = "atsam3s2c")]
pub use atsam3s2c as pac;

#[cfg(feature = "atsam3s4a")]
pub use atsam3s4a as pac;

#[cfg(feature = "atsam3s4b")]
pub use atsam3s4b as pac;

#[cfg(feature = "atsam3s4c")]
pub use atsam3s4c as pac;

#[cfg(feature = "atsam3s8b")]
pub use atsam3s8b as pac;

#[cfg(feature = "atsam3s8c")]
pub use atsam3s8c as pac;

#[cfg(feature = "atsam3sd8b")]
pub use atsam3sd8b as pac;

#[cfg(feature = "atsam3sd8c")]
pub use atsam3sd8c as pac;

#[cfg(feature = "atsam3u1c")]
pub use atsam3u1c as pac;

#[cfg(feature = "atsam3u1e")]
pub use atsam3u1e as pac;

#[cfg(feature = "atsam3u2c")]
pub use atsam3u2c as pac;

#[cfg(feature = "atsam3u2e")]
pub use atsam3u2e as pac;

#[cfg(feature = "atsam3u4c")]
pub use atsam3u4c as pac;

#[cfg(feature = "atsam3u4e")]
pub use atsam3u4e as pac;

#[cfg(feature = "atsam3x4c")]
pub use atsam3x4c as pac;

#[cfg(feature = "atsam3x4e")]
pub use atsam3x4e as pac;

#[cfg(feature = "atsam3x8c")]
pub use atsam3x8c as pac;

#[cfg(feature = "atsam3x8e")]
pub use atsam3x8e as pac;

#[cfg(feature = "atsam3x8h")]
pub use atsam3x8h as pac;

#[cfg(feature = "device")]
pub mod dacc;
#[cfg(feature = "device")]
pub mod peripheral_id;
#[cfg(feature = "device")]
pub mod pio;
#[cfg(feature = "device")]
mod pmc;

#[cfg(all(feature = "device", feature = "unproven"))]
/// Bias current control settings for DACCs and ADCs.
///
/// # Warning
///
/// None of the SAM3 chip families except for SAM3U have documentation that states what the values
/// for `IBCTLCH0`, `IBCTLCH1`, or `IBCTLDACCORE` can be, or what they mean. However, the docs for
/// the SAM3U chips have [something](https://github.com/arduino/ArduinoCore-sam/blob/master/system/CMSIS/Device/ATMEL/sam3u/html/ADC12B.html#L1629)
/// for IBCTL, and it's just assumed that they mean the same thing on other chip families.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum CurrentBias {
    /// `typical - 20%`
    M20 = 0b00,
    /// No change
    Typ = 0b01,
    /// `typical + 20%`
    P20 = 0b10,
    /// `typical + 40%`
    P40 = 0b11,
}
