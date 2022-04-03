pub mod peripherals;
pub mod pins;
pub mod pioa;
pub mod piob;
pub mod pioc;
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
pub mod piod;
#[cfg(feature = "sam3x8h")]
pub mod pioe;
#[cfg(feature = "sam3x8h")]
pub mod piof;

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use crate::pac::PIOD;
#[cfg(feature = "sam3x8h")]
use crate::pac::{PIOE, PIOF};
use crate::{
    pac::{PIOA, PIOB, PIOC},
    WPKEY,
};
use paste::paste;

pub trait IsPioc {
    type RegType;
    const PTR: *const Self::RegType;
}

macro_rules! impl_is_pioc {
    ($($pio:ty),+$(,)?) => {$(
        paste! {
            impl IsPioc for $pio {
                type RegType = crate::pac::[<$pio:lower>]::RegisterBlock;
                const PTR: *const Self::RegType = $pio::PTR;
            }
        }
    )+}
}

impl_is_pioc! {
    PIOA, PIOB, PIOC
}
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_is_pioc! {
    PIOD
}
#[cfg(feature = "sam3x8h")]
impl_is_pioc! {
    PIOE, PIOF
}

pub struct Pio {
    pioa: PIOA,
    piob: PIOB,
    pioc: PIOC,
    #[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
    piod: PIOD,
    #[cfg(feature = "sam3x8h")]
    piof: PIOF,
    #[cfg(feature = "sam3x8h")]
    pioe: PIOE,
}

pub struct Pioc<P> {
    pio: P,
}

macro_rules! common_pio_impls {
    ($($pio:ty),+$(,)?) => {$(
impl Pioc<$pio> {
    /// Enables write protection for the following registers:
    ///     - PIO Enable Register (`PER`)
    ///     - PIO Disable Register (`PDR`)
    ///     - Output Enable Register (`OER`)
    ///     - Output Disable Register (`ODR`)
    ///     - Input Filter Enable Register (`IFER`)
    ///     - Input Filter Disable Register (`IFDR`)
    ///     - Multi-driver Enable Register (`MDER`)
    ///     - Multi-driver Disable Register (`MDDR`)
    ///     - Pull Up Disable Register (`PUDR`)
    ///     - Pull Up Enable Register (`PUER`)
    ///     - Output Write Enable Register (`OWER`)
    ///     - Output Write Disable Register (`OWDR`)
    #[rustfmt::skip]
    pub fn enable_writeprotect(&mut self) {
        self.pio.wpmr.write(|wpmr| unsafe {
            wpmr
                .wpkey().bits(WPKEY)
                .wpen().set_bit()
        });
    }

    /// Disables write protection for the registers listed for [`enable_writeprotect`].
    #[rustfmt::skip]
    pub fn disable_writeprotect(&mut self) {
        self.pio.wpmr.write(|wpmr| unsafe {
            wpmr
                .wpkey().bits(WPKEY)
                .wpen().set_bit()
        });
    }

    /// Check whether write protection is currently enabled.
    pub fn writeprotect_enabled(&self) -> bool {
        self.pio.wpmr.read().wpen().bit()
    }
}
    )+}
}

common_pio_impls! {
    PIOA, PIOB, PIOC
}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
common_pio_impls! {
    PIOD
}

#[cfg(feature = "sam3x8h")]
common_pio_impls! {
    PIOE, PIOF
}
