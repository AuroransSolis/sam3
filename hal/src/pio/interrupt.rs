//! PIO interrupts
//!
//! By default, a PIO bank's interrupts can only be triggered on edge detections. However, it may be
//! configured on a per-pin basis to accept other interrupt modes. These modes are:
//! - Level detection
//!     - High level detection
//!     - Low level detection
//! - Edge detection
//!     - Rising edge detection
//!     - Falling edge detection
//!
//! Please note that the PIO interrupt signal for the entire PIO bank is fired if the interrupt
//! condition for any of the individual lines in that PIO bank is met.

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use super::piod::PioD;
use super::{
    filter::*,
    peripheral::*,
    pin::{Configured, Pin, PinId, PullupResistorCfg, Unconfigured},
    pioa::PioA,
    piob::PioB,
    pioc::PioC,
    IsPio,
};
#[cfg(feature = "sam3x8h")]
use super::{pioe::PioE, piof::PioF};
use core::marker::PhantomData;
use paste::paste;
use seq_macro::seq;

pub trait InterruptCfg {}

/// Disable interrupts on an I/O line.
pub struct InterruptDisabled;

impl InterruptCfg for InterruptDisabled {}
impl Configured for InterruptDisabled {}

/// Enable interrupts on an I/O line.
pub struct InterruptEnabled<Aint> {
    _aint: PhantomData<Aint>,
}

impl<Aint: AdditionalInterruptModesCfg> InterruptCfg for InterruptEnabled<Aint> {}
impl<Aint: AdditionalInterruptModesCfg + Configured> Configured for InterruptEnabled<Aint> {}

macro_rules! impl_intcfg {
    ($($pio:ty),+$(,)?) => {$(
        impl<Pid, Mdvr, Pupr, Aint, Filt> Pin<$pio, Pid, Mdvr, Pupr, InterruptEnabled<Aint>, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Aint: AdditionalInterruptModesCfg,
            Filt: InputFilterCfg,
        {
            pub fn interrupt_disabled(self) -> Pin<$pio, Pid, Mdvr, Pupr, InterruptDisabled, Filt> {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .idr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.imr.read().bits() & <Pid as PinId>::MASK != 0 {}
                unsafe { Pin::new() }
            }
        }

        impl<Pid, Mdvr, Pupr, Filt> Pin<$pio, Pid, Mdvr, Pupr, InterruptDisabled, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Filt: InputFilterCfg,
        {
            pub fn interrupt_enabled(self) -> Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<Unconfigured>,
                Filt
            > {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .ier
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.imr.read().bits() & <Pid as PinId>::MASK == 0 {}
                unsafe { Pin::new() }
            }
        }
    )+};
}

impl_intcfg! {
    PioA, PioB, PioC,
}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_intcfg! {
    PioD,
}

#[cfg(feature = "sam3x8h")]
impl_intcfg! {
    PioE, PioF,
}

pub trait AdditionalInterruptModesCfg {}

impl AdditionalInterruptModesCfg for Unconfigured {}

/// Disable additional interrupt modes from the event detector on an I/O line. This uses the default
/// edge detector for interrupt signals.
pub struct AdditionalInterruptModesDisabled;

impl Configured for AdditionalInterruptModesDisabled {}
impl AdditionalInterruptModesCfg for AdditionalInterruptModesDisabled {}

pub trait ApplyAdditionalInterruptModesDisabled {
    type Output;

    fn additional_interrupt_modes_disabled(self) -> Self::Output;
}

/// Enable additional interrupt modes from the event detector on an I/O line. The mode is determined
/// by the `Edlv` type and which kind of edge or level is detected is determined by the `Frlh` type.
pub struct AdditionalInterruptModesEnabled<Edlv, Frlh> {
    _edlv: PhantomData<Edlv>,
    _frlh: PhantomData<Frlh>,
}

#[rustfmt::skip]
impl<Edlv: Configured, Frlh: Configured> Configured
    for AdditionalInterruptModesEnabled<Edlv, Frlh>
{}
#[rustfmt::skip]
impl<Edlv: EdgeLevelCfg, Frlh: FallRiseLowHighCfg>
    AdditionalInterruptModesCfg for AdditionalInterruptModesEnabled<Edlv, Frlh>
{}

pub trait ApplyAdditionalInterruptModesEnabled {
    type Output;

    fn additional_interrupt_modes_enabled(
        self,
    ) -> <Self as ApplyAdditionalInterruptModesEnabled>::Output;
}

type AimeAlias<Pio, Pid, Mdvr, Pupr, Filt> = Pin<
    Pio,
    Pid,
    Mdvr,
    Pupr,
    InterruptEnabled<AdditionalInterruptModesEnabled<Unconfigured, Unconfigured>>,
    Filt,
>;

macro_rules! impl_aimcfg {
    ($($pio:ty),+$(,)?) => {$(
        impl<Pid, Mdvr, Pupr, Aint, Filt> ApplyAdditionalInterruptModesDisabled for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<Aint>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Aint: AdditionalInterruptModesCfg,
            Filt: InputFilterCfg,
        {
            type Output = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesDisabled>,
                Filt
            >;

            default fn additional_interrupt_modes_disabled(self) -> Self::Output {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .aimdr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.aimmr.read().bits() & <Pid as PinId>::MASK != 0 {}
                unsafe { Pin::new() }
            }
        }

        impl<Pid, Mdvr, Pupr, Filt> ApplyAdditionalInterruptModesDisabled for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesDisabled>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Filt: InputFilterCfg,
        {
            fn additional_interrupt_modes_disabled(self) -> Self {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Aint, Filt> ApplyAdditionalInterruptModesEnabled for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<Aint>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Aint: AdditionalInterruptModesCfg,
            Filt: InputFilterCfg,
        {
            default type Output = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Unconfigured, Unconfigured>>,
                Filt,
            >;

            default fn additional_interrupt_modes_enabled(self) -> Self::Output {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .aimdr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.aimmr.read().bits() & <Pid as PinId>::MASK == 0 {}
                //   ????
                //  ??????
                // ??    ??
                // ??    ??
                //      ??
                //     ??
                //    ??
                // 
                //    ??
                //    ??
                unsafe {
                    let tmp: AimeAlias<$pio, Pid, Mdvr, Pupr, Filt> = Pin::new();
                    (&tmp as *const AimeAlias<$pio, Pid, Mdvr, Pupr, Filt>)
                        .cast::<Self::Output>()
                        .read()
                }
                // why
                // FIXME
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Frlh, Filt> ApplyAdditionalInterruptModesEnabled for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            type Output = Self;

            fn additional_interrupt_modes_enabled(self) -> Self {
                self
            }
        }
    )+};
}

// impl_aimcfg! {
//     PioA, PioB, PioC,
// }

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_aimcfg! {
    PioD,
}

#[cfg(feature = "sam3x8h")]
impl_aimcfg! {
    PioE, PioF,
}

pub trait EdgeLevelCfg {}

impl EdgeLevelCfg for Unconfigured {}

/// Configure an I/O line's event detector to detect edges.
pub struct DetectEdges;

impl Configured for DetectEdges {}
impl EdgeLevelCfg for DetectEdges {}

pub trait ApplyDetectEdges {
    type Output;

    fn detect_edges(self) -> Self::Output;
}

/// Configure an I/O line's event detector to detect levels.
pub struct DetectLevels;

impl Configured for DetectLevels {}
impl EdgeLevelCfg for DetectLevels {}

pub trait ApplyDetectLevels {
    type Output;

    fn detect_levels(self) -> Self::Output;
}

macro_rules! impl_elcfg {
    ($($pio:ty),+$(,)?) => {$(
        impl<Pid, Mdvr, Pupr, Edlv, Frlh, Filt> ApplyDetectEdges for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            type Output = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectEdges, Frlh>>,
                Filt,
            >;

            default fn detect_edges(self) -> Self::Output {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .esr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.elsr.read().bits() & <Pid as PinId>::MASK != 0 {}
                unsafe { Pin::new() }
            }
        }

        impl<Pid, Mdvr, Pupr, Frlh, Filt> ApplyDetectEdges for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectEdges, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            fn detect_edges(self) -> Self {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Frlh, Filt> ApplyDetectLevels for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            type Output = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectLevels, Frlh>>,
                Filt,
            >;

            default fn detect_levels(self) -> Self::Output {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .lsr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.elsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                unsafe { Pin::new() }
            }
        }

        impl<Pid, Mdvr, Pupr, Frlh, Filt> ApplyDetectLevels for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectLevels, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            fn detect_levels(self) -> Self {
                self
            }
        }
    )+};
}

impl_elcfg! {
    PioA, PioB, PioC,
}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_elcfg! {
    PioD,
}

#[cfg(feature = "sam3x8h")]
impl_elcfg! {
    PioE, PioF,
}

pub trait FallRiseLowHighCfg {}

impl FallRiseLowHighCfg for Unconfigured {}

/// Configure an I/O line's event detector to detect falling edges or low levels depending
/// on whether it is configured to detect edges or levels.
pub struct DetectFallingEdgeLowLevel;

impl Configured for DetectFallingEdgeLowLevel {}
impl FallRiseLowHighCfg for DetectFallingEdgeLowLevel {}

pub trait ApplyDetectFallingEdgeLowLevel {
    type Output;

    fn detect_falling_edge_low_level(self) -> Self::Output;
}

/// Configure an I/O line's event detector to detect rising edges or high levels depending
/// on whether it is configured to detect edges or levels.
pub struct DetectRisingEdgeHighLevel;

impl Configured for DetectRisingEdgeHighLevel {}
impl FallRiseLowHighCfg for DetectRisingEdgeHighLevel {}

pub trait ApplyDetectRisingEdgeHighLevel {
    type Output;

    fn detect_rising_edge_high_level(self) -> Self::Output;
}

macro_rules! impl_flrhcfg {
    ($($pio:ty),+$(,)?) => {$(
        impl<Pid, Mdvr, Pupr, Edlv, Frlh, Filt> ApplyDetectFallingEdgeLowLevel for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            type Output = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectFallingEdgeLowLevel>>,
                Filt,
            >;

            default fn detect_falling_edge_low_level(self) -> Self::Output {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .fellsr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK != 0 {}
                unsafe { Pin::new() }
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Filt> ApplyDetectFallingEdgeLowLevel for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectFallingEdgeLowLevel>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Filt: InputFilterCfg,
        {
            fn detect_falling_edge_low_level(self) -> Self {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Frlh, Filt> ApplyDetectRisingEdgeHighLevel for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            type Output = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectRisingEdgeHighLevel>>,
                Filt,
            >;

            default fn detect_rising_edge_high_level(self) -> Self::Output {
                let pioreg = unsafe { &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType) };
                unsafe {
                    pioreg
                        .rehlsr
                        .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                }
                while pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                unsafe { Pin::new() }
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Filt> ApplyDetectRisingEdgeHighLevel for
            Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectRisingEdgeHighLevel>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Filt: InputFilterCfg,
        {
            fn detect_rising_edge_high_level(self) -> Self {
                self
            }
        }
    )+};
}

impl_flrhcfg! {
    PioA, PioB, PioC,
}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_flrhcfg! {
    PioD,
}

#[cfg(feature = "sam3x8h")]
impl_flrhcfg! {
    PioE, PioF,
}
