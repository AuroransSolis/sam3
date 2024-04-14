#![no_std]
#![allow(dead_code)]
//! # SAM3
//!
//! This is a hardware abstraction layer (HAL) over each of the following families of Microchip
//! MCUs (configurations listed in parenthesis):
//!   - **ATSAM3A**, **ATSAM3X** (ATSAM3A4C, ATSAM3A8C, ATSAM3X4C, ATSAM3X4E, ATSAM3X8C, ATSAM3X8E,
//!     ATSAM3X8H)
//!   - **ATSAM3N** (ATSAM3N00A, ATSAM3N00B, ATSAM3N0A, ATSAM3N0B, ATSAM3N0C, ATSAM3N1A, ATSAM3N1B,
//!     ATSAM3N1C, ATSAM3N2A, ATSAM3N2B, ATSAM3N2C, ATSAM3N4A, ATSAM3N4B, ATSAM3N4C)
//!   - **ATSAM3S1**, **ATSAM3S2**, **ATSAM3S4** (ATSAM3S1A, ATSAM3S1B, ATSAM3S1C, ATSAM3S2A,
//!     ATSAM3S2B, ATSAM3S2C, ATSAM3S4A, ATSAM3S4B, ATSAM3S4C)
//!   - **ATSAM3S8**, **ATSAM3SD8** (ATSAM3S8B, ATSAM3S8C, ATSAM3SD8B, ATSAM3SD8C)
//!   - **ATSAM3U** (ATSAM3U1C, ATSAM3U1E, ATSAM3U2C, ATSAM3U2E, ATSAM3U4C, ATSAM3U4E)
//!
//! This project relies on some outside information under a different license (Apache), retrieved
//! from [Atmel][atmel-src]. The SVD files for each of the families listed above are depended on for
//! generating the PAC (peripheral access control) crates, and the linker scripts are used to create
//! the `memory.x` and `device.x` files that the [`cortex-m-rt`][cortex-m-rt] depends on to compile
//! device binaries.
//!
//! Additionally, documentation for each peripheral, register, and on-chip function has been copied
//! over (excluding diagrams) from the device manuals. Extra sections with per-family or per-chip
//! information have been added as necessary. However, should you prefer to read the manuals rather
//! than the transcriptions here, you can find them here:
//!   - **ATSAM3A**, **ATSAM3X**: [manual][atsam3ax-manual]
//!   - **ATSAM3N**: [manual][atsam3n-manual]
//!   - **ATSAM3S1**, **ATSAM3S2**, **ATSAM3S4**: [manual][atsam3s124-manual]
//!   - **ATSAM3S8**, **ATSAM3SD8**: [manual][atsam3sd8-manual]
//!   - **ATSAM3U**: [manual][atsam3u-manual]
//!
//! There are errata for each one towards the end of the manuals.
//!
//! [atmel-src]: http://packs.download.atmel.com/
//! [cortex-m-rt]: https://crates.io/crates/cortex-m-rt
//! [atsam3ax-manual]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf
//! [atsam3n-manual]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11011-32-bit-Cortex-M3-Microcontroller-SAM3N_Datasheet.pdf
//! [atsam3s124-manual]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6500-32-bit-Cortex-M3-Microcontroller-SAM3S4-SAM3S2-SAM3S1_Datasheet.pdf
//! [atsam3sd8-manual]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11090-32-bit%20Cortex-M3-Microcontroller-SAM-3S8-SD8_Datasheet.pdf
//! [atsam3u-manual]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6430-32-bit-Cortex-M3-Microcontroller-SAM3U4-SAM3U2-SAM3U1_Datasheet.pdf

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

#[cfg(all(feature = "device", any(feature = "sam3a", feature = "sam3x")))]
pub mod dacc;
#[cfg(feature = "device")]
pub mod peripheral_id;
#[cfg(feature = "device")]
pub mod pio;
#[cfg(all(feature = "device", any(feature = "sam3a", feature = "sam3x")))]
mod pmc;
#[cfg(feature = "device")]
pub mod write_protect;

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
