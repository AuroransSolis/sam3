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
#[cfg(feature = "pioa")]
use crate::pio::pioa::PioA;
#[cfg(feature = "piob")]
use crate::pio::piob::PioB;
#[cfg(feature = "pioc")]
use crate::pio::pioc::PioC;
#[cfg(feature = "piod")]
use crate::pio::piod::PioD;
#[cfg(feature = "pioe")]
use crate::pio::pioe::PioE;
#[cfg(feature = "piof")]
use crate::pio::piof::PioF;
use crate::pio::{
    filter::InputFilterCfg,
    peripheral::MultiDriverCfg,
    pin::{Configured, Pin, PinId, PullupResistorCfg, Unconfigured},
    IsPio,
};
use core::marker::PhantomData;

#[allow(clippy::module_name_repetitions)]
/// Marker trait for interrupt configuration types.
pub trait InterruptCfg {}

impl InterruptCfg for Unconfigured {}

#[allow(clippy::module_name_repetitions)]
/// Disable interrupts on an I/O line.
pub struct InterruptDisabled;

impl InterruptCfg for InterruptDisabled {}
impl Configured for InterruptDisabled {}

#[allow(clippy::module_name_repetitions)]
/// Enable interrupts on an I/O line.
pub struct InterruptEnabled<Aint> {
    _aint: PhantomData<Aint>,
}

impl<Aint: AdditionalInterruptModesCfg> InterruptCfg for InterruptEnabled<Aint> {}
impl<Aint: AdditionalInterruptModesCfg + Configured> Configured for InterruptEnabled<Aint> {}

#[allow(clippy::module_name_repetitions)]
/// # Input Edge/Level Interrupt
///
/// The PIO Controller can be programmed to generate an interrupt when it detects an edge or a level
/// on an I/O line. The Input Edge/Level Interrupt is controlled by writing `PIO_IER` (Interrupt
/// Enable Register) and `PIO_IDR` (Interrupt Disable Register). As Input change detection is
/// possible only by comparing two successive samplings of the input of the I/O line, the PIO
/// Controller clock must be enabled. The Input Change Interrupt is available, regardless of the
/// configuration of the I/O line, i.e. configured as an input only, controlled by the PIO
/// Controller or assigned to a peripheral function.
///
/// By default, the interrupt can be generated at any time an edge is detected on the input.
///
/// Additional Interrupt Modes can be configured through the [`ConfigureAdditionalInterruptModes`]
/// trait.
///
/// When an input Edge or Level is detected on an I/O line, the corresponding bit in the `PIO_ISR`
/// (Interrupt Status Register) is set. If the corresponding bit in `PIO_IMR` is set, the PIO
/// Controller interrupt line is asserted. The interrupt signals of the thirty-two channels are
/// ORed-wired together to generate a single interrupt signal to the Nested Vector Interrupt
/// Controller (NVIC).
///
/// When the software reads `PIO_ISR`, all the interrupts are automatically cleared. This signifies
/// that all the interrupts that are pending when `PIO_ISR` is read must be handled. When an
/// Interrupt is enabled on a "Level", the interrupt is generated as long as the interrupt source is
/// not cleared, even if some read accesses in `PIO_ISR` are performed.
pub trait ConfigureInterrupt {
    type Enabled;
    type Disabled;

    /// Enable interrupts on this pin. Waits for `PIO_IMR` to update accordingly.
    fn enable_interrupt(self) -> Self::Enabled;
    /// Enable interrupts on this pin without waiting for `PIO_IMR` to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that interrupts are enabled, the pin will not
    /// actually trigger an interrupt until the corresponding bit in `PIO_IMR` is set.
    unsafe fn enable_interrupt_unchecked(self) -> Self::Enabled;
    /// Disable interrupts on this pin. Waits for `PIO_IMR` to update accordingly.
    fn disable_interrupt(self) -> Self::Disabled;
    /// Disable interrupts on this pin without waiting for `PIO_IMR` to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that interrupts are disabled, the pin will
    /// continue triggering interrupts until the corresponding bit in `PIO_IMR` is cleared.
    unsafe fn disable_interrupt_unchecked(self) -> Self::Disabled;
}

/// Marker trait for additional interrupt modes configuration types.
pub trait AdditionalInterruptModesCfg {}

impl AdditionalInterruptModesCfg for Unconfigured {}

/// Disable additional interrupt modes from the event detector on an I/O line. This uses the default
/// edge detector for interrupt signals.
pub struct AdditionalInterruptModesDisabled;

impl Configured for AdditionalInterruptModesDisabled {}
impl AdditionalInterruptModesCfg for AdditionalInterruptModesDisabled {}

/// # Input Edge/Level Interrupt (Additional Interrupt Modes)
///
/// Some additional interrupt modes can be enabled/disabled by writing in the `PIO_AIMER`
/// (Additional Interrupt Modes Enable Register) and `PIO_AIMDR` (Additional Interrupt Modes Disable
/// Register). the current state of this selection can be read through the `PIO_AIMMR` (Additional
/// Interrupt Modes Mask Register).
///
/// These Additional Modes are:
/// - Rising Edge Detection
/// - Falling Edge Detection
/// - Low Level Detection
/// - High Level Detection
///
/// In order to select an Additional Interrupt Mode:
/// - The type of the event detection (Edge or Level) must be selected by writing in the set of
///   registers; `PIO_ESR` (Edge Select Register) and `PIO_LSR` (Level Select Register) which enable
///   respectively, the Edge and Level Detection. The current status of this selection is accessible
///   through the `PIO_ELSR` (Edge/Level Status Register).
/// - The Polarity of the event detection (Rising/Falling Edge or High/Low Level) must be selected
///   by writing in the set of registers; `PIO_FELLSR` (Falling Edge/Low Level Select Register) and
///   `PIO_REHLSR` (Rising Edge/High Level Select Register) which allow to select Falling or Rising
///   Edge (if Edge is selected in the `PIO_ELSR`) or High or Low Level Detection (if Level is
///   selected in the `PIO_ELSR`). The current status of this selection is accessible through the
///   `PIO_FRLHSR` (Fall/Rise - Low/High Status Register).
pub trait ConfigureAdditionalInterruptModes {
    type Enabled;
    type Disabled;

    /// Enable additional interrupt modes on this pin. Waits for `PIO_AIMMR` to update accordingly.
    fn enable_additional_interrupt_modes(self) -> Self::Enabled;
    /// Enable additional interrupt modes on this pin without waiting for `PIO_AIMMR` to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that additional interrupt modes are enabled, the
    /// pin will not actually trigger an interrupt from any of the additional modes until the
    /// corresponding bit in `PIO_AIMMR` is set.
    unsafe fn enable_additional_interrupt_modes_unchecked(self) -> Self::Enabled;
    /// Disable additional interrupt modes on this pin. Waits for `PIO_AIMMR` to update accordingly.
    fn disable_additional_interrupt_modes(self) -> Self::Disabled;
    /// Disable additional interrupt modes on this pin without waiting for `PIO_AIMMR` to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that additional interrupt modes are disabled, the
    /// pin will still trigger an interrupt from the configured additional interrupt mode until the
    /// corresponding bit in `PIO_AIMMR` is cleared.
    unsafe fn disable_additional_interrupt_modes_unchecked(self) -> Self::Disabled;
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

/// Marker trait for edge/level configuration types.
pub trait EdgeLevelCfg {}

impl EdgeLevelCfg for Unconfigured {}

/// Configure an I/O line's event detector to detect edges.
pub struct DetectEdges;

impl Configured for DetectEdges {}
impl EdgeLevelCfg for DetectEdges {}

/// # Input Edge/Level Interrupt (Edge/Level)
///
/// Determines whether an interrupt is triggered on a falling edge/rising edge (depending on
/// fall/rise - low/high configuration) or low level/high level (depending on fall/rise - low/high
/// configuration).
pub trait ConfigureEdgeLevel {
    type Edge;
    type Level;

    /// Trigger interrupts on edges for this pin. Waits for `PIO_ELSR` to update accordingly.
    fn detect_edges(self) -> Self::Edge;
    /// Trigger interrupts on edges for this pin without waiting for `PIO_ELSR` to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that interrupts are triggered on edges, the pin
    /// will not actually trigger an interrupt on an edge until the corresponding bit in `PIO_ELSR`
    /// is cleared.
    unsafe fn detect_edges_unchecked(self) -> Self::Edge;
    /// Trigger interrupts on levels for this pin. Waits for `PIO_ELSR` to update accordingly.
    fn detect_levels(self) -> Self::Level;
    /// Trigger interrupts on levels for this pin without waiting for `PIO_ELSR` to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that interrupts are triggered on levels, the pin
    /// will not actually trigger an interrupt on a level until the corresponding bit in `PIO_ELSR`
    /// is set.
    unsafe fn detect_levels_unchecked(self) -> Self::Level;
}

/// Configure an I/O line's event detector to detect levels.
pub struct DetectLevels;

impl Configured for DetectLevels {}
impl EdgeLevelCfg for DetectLevels {}

/// Marker trait for fall/rise - low/high configuration types.
pub trait FallRiseLowHighCfg {}

impl FallRiseLowHighCfg for Unconfigured {}

/// Configure an I/O line's event detector to detect falling edges or low levels depending
/// on whether it is configured to detect edges or levels.
pub struct DetectFallingEdgeLowLevel;

impl Configured for DetectFallingEdgeLowLevel {}
impl FallRiseLowHighCfg for DetectFallingEdgeLowLevel {}

/// # Input Edge/Level Interrupt (Fall/Rise - Low/High)
///
/// Determines whether an interrupt is triggered on a falling edge/low level (depending on
/// edge/level configuration) or rising edge/high level (depending on edge/level configuration).
pub trait ConfigureFallRiseLowHigh {
    type Fell;
    type Rehl;

    /// Trigger interrupts on falling edges/low levels for this pin. Waits for `PIO_FRLHSR` to
    /// update accordingly.
    fn detect_falling_edge_low_level(self) -> Self::Fell;
    /// Trigger interrupts on falling edges/low levels for this pin without waiting for `PIO_FRLHSR`
    /// to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that interrupts are triggered on falling
    /// edges/rising levels, the pin will not actually trigger an interrupt on either until the
    /// corresponding bit in `PIO_FRLHSR` is cleared.
    unsafe fn detect_falling_edge_low_level_unchecked(self) -> Self::Fell;
    /// Trigger interrupts on rising edges/high levels for this pin. Waits for `PIO_FRLHSR` to
    /// update accordingly.
    fn detect_rising_edge_high_level(self) -> Self::Rehl;
    /// Trigger interrupts on rising edges/high levels for this pin without waiting for `PIO_FRLHSR`
    /// to update.
    ///
    /// # Safety
    ///
    /// While this function returns a type showing that interrupts are triggered on rising
    /// edges/falling levels, the pin will not actually trigger an interrupt on either until the
    /// corresponding bit in `PIO_FRLHSR` is set.
    unsafe fn detect_rising_edge_high_level_unchecked(self) -> Self::Rehl;
}

/// Configure an I/O line's event detector to detect rising edges or high levels depending
/// on whether it is configured to detect edges or levels.
pub struct DetectRisingEdgeHighLevel;

impl Configured for DetectRisingEdgeHighLevel {}
impl FallRiseLowHighCfg for DetectRisingEdgeHighLevel {}

macro_rules! impl_interrupt_cfgs {
    ($pio:ty) => {
        impl<Pid, Mdvr, Pupr, Filt> ConfigureInterrupt
            for Pin<$pio, Pid, Mdvr, Pupr, Unconfigured, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Filt: InputFilterCfg,
        {
            type Enabled = Pin<$pio, Pid, Mdvr, Pupr, InterruptEnabled<Unconfigured>, Filt>;
            type Disabled = Pin<$pio, Pid, Mdvr, Pupr, InterruptDisabled, Filt>;

            fn enable_interrupt(self) -> Self::Enabled {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.imr.read().bits() & <Pid as PinId>::MASK == 0 {
                        let _ = self.enable_interrupt_unchecked();
                        while pioreg.imr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn enable_interrupt_unchecked(self) -> Self::Enabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.ier.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn disable_interrupt(self) -> Self::Disabled {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.imr.read().bits() & <Pid as PinId>::MASK != 0 {
                        let _ = self.disable_interrupt_unchecked();
                        while pioreg.imr.read().bits() & <Pid as PinId>::MASK != 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn disable_interrupt_unchecked(self) -> Self::Disabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.idr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Aint, Filt> ConfigureInterrupt
            for Pin<$pio, Pid, Mdvr, Pupr, InterruptEnabled<Aint>, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Aint: AdditionalInterruptModesCfg,
            Filt: InputFilterCfg,
        {
            type Enabled = Self;
            type Disabled = Pin<$pio, Pid, Mdvr, Pupr, InterruptDisabled, Filt>;

            fn enable_interrupt(self) -> Self::Enabled {
                self
            }

            unsafe fn enable_interrupt_unchecked(self) -> Self::Enabled {
                self
            }

            fn disable_interrupt(self) -> Self::Disabled {
                unsafe {
                    let _ = self.disable_interrupt_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.imr.read().bits() & <Pid as PinId>::MASK != 0 {}
                    Pin::new()
                }
            }

            unsafe fn disable_interrupt_unchecked(self) -> Self::Disabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.idr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Filt> ConfigureInterrupt
            for Pin<$pio, Pid, Mdvr, Pupr, InterruptDisabled, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Filt: InputFilterCfg,
        {
            type Enabled = Pin<$pio, Pid, Mdvr, Pupr, InterruptEnabled<Unconfigured>, Filt>;
            type Disabled = Self;

            fn enable_interrupt(self) -> Self::Enabled {
                unsafe {
                    let _ = self.enable_interrupt_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.imr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    Pin::new()
                }
            }

            unsafe fn enable_interrupt_unchecked(self) -> Self::Enabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.ier.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn disable_interrupt(self) -> Self::Disabled {
                self
            }

            unsafe fn disable_interrupt_unchecked(self) -> Self::Disabled {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Filt> ConfigureAdditionalInterruptModes
            for Pin<$pio, Pid, Mdvr, Pupr, InterruptEnabled<Unconfigured>, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Filt: InputFilterCfg,
        {
            type Enabled = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesDisabled>,
                Filt,
            >;
            type Disabled = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Unconfigured, Unconfigured>>,
                Filt,
            >;

            fn enable_additional_interrupt_modes(self) -> Self::Enabled {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.aimmr.read().bits() & <Pid as PinId>::MASK == 0 {
                        let _ = self.enable_additional_interrupt_modes_unchecked();
                        while pioreg.aimmr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn enable_additional_interrupt_modes_unchecked(self) -> Self::Enabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .aimer
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn disable_additional_interrupt_modes(self) -> Self::Disabled {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.aimmr.read().bits() & <Pid as PinId>::MASK != 0 {
                        let _ = self.disable_additional_interrupt_modes_unchecked();
                        while pioreg.aimmr.read().bits() & <Pid as PinId>::MASK != 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn disable_additional_interrupt_modes_unchecked(self) -> Self::Disabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .aimdr
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Frlh, Filt> ConfigureAdditionalInterruptModes
            for Pin<
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
            type Enabled = Self;
            type Disabled = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesDisabled>,
                Filt,
            >;

            fn enable_additional_interrupt_modes(self) -> Self::Enabled {
                self
            }

            unsafe fn enable_additional_interrupt_modes_unchecked(self) -> Self::Enabled {
                self
            }

            fn disable_additional_interrupt_modes(self) -> Self::Disabled {
                unsafe {
                    let _ = self.disable_additional_interrupt_modes_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.aimmr.read().bits() & <Pid as PinId>::MASK > 0 {}
                    Pin::new()
                }
            }

            unsafe fn disable_additional_interrupt_modes_unchecked(self) -> Self::Disabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .aimdr
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Filt> ConfigureAdditionalInterruptModes
            for Pin<$pio, Pid, Mdvr, Pupr, InterruptEnabled<AdditionalInterruptModesDisabled>, Filt>
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Filt: InputFilterCfg,
        {
            type Enabled = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Unconfigured, Unconfigured>>,
                Filt,
            >;
            type Disabled = Self;

            fn enable_additional_interrupt_modes(self) -> Self::Enabled {
                unsafe {
                    let _ = self.enable_additional_interrupt_modes_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.aimmr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    Pin::new()
                }
            }

            unsafe fn enable_additional_interrupt_modes_unchecked(self) -> Self::Enabled {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .aimer
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn disable_additional_interrupt_modes(self) -> Self::Disabled {
                self
            }

            unsafe fn disable_additional_interrupt_modes_unchecked(self) -> Self::Disabled {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Frlh, Filt> ConfigureEdgeLevel
            for Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Unconfigured, Frlh>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Frlh: FallRiseLowHighCfg,
            Filt: InputFilterCfg,
        {
            type Edge = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectEdges, Frlh>>,
                Filt,
            >;
            type Level = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectLevels, Frlh>>,
                Filt,
            >;

            fn detect_edges(self) -> Self::Edge {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.elsr.read().bits() & <Pid as PinId>::MASK > 0 {
                        let _ = self.detect_edges_unchecked();
                        while pioreg.elsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn detect_edges_unchecked(self) -> Self::Edge {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.esr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn detect_levels(self) -> Self::Level {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.elsr.read().bits() & <Pid as PinId>::MASK == 0 {
                        let _ = self.detect_levels_unchecked();
                        while pioreg.elsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn detect_levels_unchecked(self) -> Self::Level {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.lsr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Frlh, Filt> ConfigureEdgeLevel
            for Pin<
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
            type Edge = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectEdges, Frlh>>,
                Filt,
            >;
            type Level = Self;

            fn detect_edges(self) -> Self::Edge {
                unsafe {
                    let _ = self.detect_edges_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.elsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                    Pin::new()
                }
            }

            unsafe fn detect_edges_unchecked(self) -> Self::Edge {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.esr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn detect_levels(self) -> Self::Level {
                self
            }

            unsafe fn detect_levels_unchecked(self) -> Self::Level {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Frlh, Filt> ConfigureEdgeLevel
            for Pin<
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
            type Edge = Self;
            type Level = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<DetectLevels, Frlh>>,
                Filt,
            >;

            fn detect_edges(self) -> Self::Edge {
                self
            }

            unsafe fn detect_edges_unchecked(self) -> Self::Edge {
                self
            }

            fn detect_levels(self) -> Self::Level {
                unsafe {
                    let _ = self.detect_levels_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.elsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    Pin::new()
                }
            }

            unsafe fn detect_levels_unchecked(self) -> Self::Level {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg.lsr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Filt> ConfigureFallRiseLowHigh
            for Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, Unconfigured>>,
                Filt,
            >
        where
            Pid: PinId<Controller = $pio>,
            Mdvr: MultiDriverCfg,
            Pupr: PullupResistorCfg,
            Edlv: EdgeLevelCfg,
            Filt: InputFilterCfg,
        {
            type Fell = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectFallingEdgeLowLevel>>,
                Filt,
            >;
            type Rehl = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectRisingEdgeHighLevel>>,
                Filt,
            >;

            fn detect_falling_edge_low_level(self) -> Self::Fell {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK > 0 {
                        let _ = self.detect_falling_edge_low_level_unchecked();
                        while pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn detect_falling_edge_low_level_unchecked(self) -> Self::Fell {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .fellsr
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn detect_rising_edge_high_level(self) -> Self::Rehl {
                unsafe {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    if pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK == 0 {
                        let _ = self.detect_rising_edge_high_level_unchecked();
                        while pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    }
                    Pin::new()
                }
            }

            unsafe fn detect_rising_edge_high_level_unchecked(self) -> Self::Rehl {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .rehlsr
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Filt> ConfigureFallRiseLowHigh
            for Pin<
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
            type Fell = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectFallingEdgeLowLevel>>,
                Filt,
            >;
            type Rehl = Self;

            fn detect_falling_edge_low_level(self) -> Self::Fell {
                unsafe {
                    let _ = self.detect_falling_edge_low_level_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                    Pin::new()
                }
            }

            unsafe fn detect_falling_edge_low_level_unchecked(self) -> Self::Fell {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .fellsr
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }

            fn detect_rising_edge_high_level(self) -> Self::Rehl {
                self
            }

            unsafe fn detect_rising_edge_high_level_unchecked(self) -> Self::Rehl {
                self
            }
        }

        impl<Pid, Mdvr, Pupr, Edlv, Filt> ConfigureFallRiseLowHigh
            for Pin<
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
            type Fell = Self;
            type Rehl = Pin<
                $pio,
                Pid,
                Mdvr,
                Pupr,
                InterruptEnabled<AdditionalInterruptModesEnabled<Edlv, DetectRisingEdgeHighLevel>>,
                Filt,
            >;

            fn detect_falling_edge_low_level(self) -> Self::Fell {
                self
            }

            unsafe fn detect_falling_edge_low_level_unchecked(self) -> Self::Fell {
                self
            }

            fn detect_rising_edge_high_level(self) -> Self::Rehl {
                unsafe {
                    let _ = self.detect_rising_edge_high_level_unchecked();
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    while pioreg.frlhsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                    Pin::new()
                }
            }

            unsafe fn detect_rising_edge_high_level_unchecked(self) -> Self::Rehl {
                let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                pioreg
                    .rehlsr
                    .write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                Pin::new()
            }
        }
    };
}

#[cfg(feature = "pioa")]
impl_interrupt_cfgs! {
    PioA
}

#[cfg(feature = "piob")]
impl_interrupt_cfgs! {
    PioB
}

#[cfg(feature = "pioc")]
impl_interrupt_cfgs! {
    PioC
}

#[cfg(feature = "piod")]
impl_interrupt_cfgs! {
    PioD
}

#[cfg(feature = "pioe")]
impl_interrupt_cfgs! {
    PioE
}

#[cfg(feature = "piof")]
impl_interrupt_cfgs! {
    PioF
}
