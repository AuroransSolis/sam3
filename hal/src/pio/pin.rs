//! PIO pin definition + multi-drive and pad resistor configuration traits
//!
//! This module contains the definition of the `Pin` type, which is generic over all of the lines
//! and PIO controllers. It also contains the traits for configuring multi-drive and pad resistors
//! for each pin.
//!
//! Relevant manual sections:
//! - SAM3A, SAM3X: [manual][ax]
//!   - Multi-drive: page 623 (31.5.6)
//!   - Pad resistor: page 622 (31.5.1)
//! - SAM3N: [manual][n]
//!   - Multi-drive: page 381 (27.5.6)
//!   - Pad resistor: page 380 (27.5.1)
//! - SAM3S1, SAM3S2, SAM3S4: [manual][s124]
//!   - Multi-drive: page 472 (29.5.6)
//!   - Pad resistor: page 471 (29.5.1)
//! - SAM3S8, SAM3SD8: [manual][sd8]
//!   - Multi-drive: page 481 (28.5.6)
//!   - Pad resistor: page 480 (28.5.1)
//! - SAM3U: [manual][u]
//!   - Multi-drive: page 499 (29.5.6)
//!   - Pad resistor: page 497 (29.5.1)
//!
//! [ax]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf
//! [n]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11011-32-bit-Cortex-M3-Microcontroller-SAM3N_Datasheet.pdf
//! [s124]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6500-32-bit-Cortex-M3-Microcontroller-SAM3S4-SAM3S2-SAM3S1_Datasheet.pdf
//! [sd8]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11090-32-bit%20Cortex-M3-Microcontroller-SAM-3S8-SD8_Datasheet.pdf
//! [u]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6430-32-bit-Cortex-M3-Microcontroller-SAM3U4-SAM3U2-SAM3U1_Datasheet.pdf

use crate::{
    pio::{
        filter::InputFilterCfg, interrupt::InterruptCfg, peripheral::PioControlCfg, structure::*,
        PioError,
    },
    structure::*,
    write_protect::*,
};
use core::marker::PhantomData;

#[allow(clippy::module_name_repetitions)]
/// Trait implemented on all `Pin` types denoting which PIO controlls the pin and what its mask is
/// for various configuration options.
pub trait PinId {
    type Controller: PioRegisters;
    const MASK: u32;
}

/// A type representing an individual pin controlled by a given PIO controller.
///
/// # Configuration
///
/// Each pin has the following configurable parameters:
/// - Pad resistor (pull-up or pull-up/pull-down depending on chip): [`ConfigurePadResistor`]
/// - PIO or peripheral control: [`ConfigurePioControl`][cpc]
/// - Peripheral selection (A/B, A/B/C, or A/B/C/D depending on chip and PIO controller):
///   [`ConfigureFunctionSelect`][cfs], [`Pin::select_peripheral`][psp]
/// - Synchronous data output: [`ConfigureOutputSyncWrite`][cosw]
/// - Multi-drive control: [`ConfigureMultiDrive`][cmd]
/// - PIO output: [`ConfigurePioOutput`][cpo]
/// - Output write control: [`ConfigureOutputWrite`][cow]
/// - Input glitch and debouncing filters: [`ConfigureInputFilter`][cif]
/// - Input filter clock: [`ConfigureInputFilterClock`][cifc]
/// - Interrupts: [`ConfigureInterrupt`][ci]
/// - Additional interrupt modes: [`ConfigureAdditionalInterruptModes`][caim]
/// - Input edge/level interrupt: [`ConfigureEdgeLevel`][cel]
/// - Fall/Rise - Low/High interrupt: [`ConfigureFallRiseLowHigh`][cfrlh]
///
/// [cpc]: crate::pio::peripheral::ConfigurePioControl
/// [cfs]: crate::pio::peripheral::ConfigureFunctionSelect
/// [psp]: crate::pio::pin::Pin::select_peripheral
/// [cosw]: crate::pio::peripheral::ConfigureOutputSyncWrite
/// [cmd]: crate::pio::pin::ConfigureMultiDriver
/// [cpo]: crate::pio::peripheral::ConfigurePioOutput
/// [cow]: crate::pio::peripheral::ConfigureOutputWrite
/// [cif]: crate::pio::filter::ConfigureInputFilter
/// [cifc]: crate::pio::filter::ConfigureInputFilterClock
/// [ci]: crate::pio::interrupt::ConfigureInterrupt
/// [caim]: crate::pio::interrupt::ConfigureAdditionalInterruptModes
/// [cel]: crate::pio::interrupt::ConfigureEdgeLevel
/// [cfrlh]: crate::pio::interrupt::ConfigureFallRiseLowHigh
///
/// Please refer to the relevant modules for additional information on each of the traits listed
/// above and their methods.
pub struct Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    _pio: PhantomData<Pio>,
    _pid: PhantomData<Pid>,
    _mdvr: PhantomData<Mdvr>,
    _pioc: PhantomData<Pioc>,
    _padr: PhantomData<Padr>,
    _irpt: PhantomData<Irpt>,
    _filt: PhantomData<Filt>,
}

impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt> Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    pub(crate) unsafe fn new() -> Self {
        Pin {
            _pio: PhantomData,
            _pid: PhantomData,
            _mdvr: PhantomData,
            _pioc: PhantomData,
            _padr: PhantomData,
            _irpt: PhantomData,
            _filt: PhantomData,
        }
    }
}

/// Marker trait denoting that a pin's configuration is complete.
pub trait Configured {}

impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt> Configured
    for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg + Configured,
    Pioc: PioControlCfg + Configured,
    Padr: PadResistorCfg + Configured,
    Irpt: InterruptCfg + Configured,
    Filt: InputFilterCfg + Configured,
{
}

/// Type marking an option for a pin as unconfigured.
pub struct Unconfigured;

/// Marker trait for multidrive configuration options.
pub trait MultiDriverCfg {}

impl MultiDriverCfg for Unconfigured {}

/// Disable multidrive on this PIO line.
pub struct MultiDriverDisabled;
impl MultiDriverCfg for MultiDriverDisabled {}

/// Enable multidrive on this PIO line. When configured in this state, drivers should only drive the
/// line low. Additionally, a pull-up resistor is generally required to ensure that the line can
/// achieve a high level.
pub struct MultiDriverEnabled;
impl MultiDriverCfg for MultiDriverEnabled {}

/// Marker trait for pad resistor configuration options.
pub trait PadResistorCfg {}

impl PadResistorCfg for Unconfigured {}

/// Enable the pull-up resistor on an I/O line.
pub struct PullupEnabled;

impl PadResistorCfg for PullupEnabled {}

#[cfg(feature = "ppd")]
/// Enable the pull-down resistor on an I/O line.
pub struct PulldownEnabled;

#[cfg(feature = "ppd")]
impl PadResistorCfg for PulldownEnabled {}

#[cfg(feature = "ppd")]
/// Disable the pull-up and pull-down resistors on an I/O line.
pub struct PdPuDisabled;

#[cfg(feature = "ppd")]
impl PadResistorCfg for PdPuDisabled {}

#[cfg(not(feature = "ppd"))]
/// Disable the pull-down resistor on an I/O line.
pub struct PullupDisabled;

#[cfg(not(feature = "ppd"))]
impl PadResistorCfg for PullupDisabled {}

#[cfg(not(feature = "ppd"))]
/// Pull-up Resistor Control
///
/// Each I/O line is designed with an embedded pull-up resistor. The pull-up resistor can be enabled
/// or disabled by writing respectively `PIO_PUER` (Pull-up Enable Register) and `PIO_PUDR` (Pull-up
/// Disable Register). Writing in these registers results in setting or clearing the corresponding
/// bit in `PIO_PUSR` (Pull-up Status Register). Reading a 1 in `PIO_PUSR` means the pull-up is
/// disabled and reading a 0 means the pull-up is enabled.
///
/// Control of the pull-up resistor is possible regardless of the configuration of the I/O line.
///
/// After reset, all of the pull-ups are enabled, i.e. `PIO_PUSR` resets at the value `0x0`.
pub trait ConfigurePadResistor: Sized {
    type Disabled;
    type Enabled;

    /// Disables the pull-up resistor on this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn disable_pullup_resistor(self) -> Result<Self::Disabled, (Self, PioError)>;
    /// Disables the pull-up resistor on this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its pull-up resistor disabled, but
    /// this may be at odds with the actual configuration state of the PIO controller. Writes to
    /// the configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    unsafe fn disable_pullup_resistor_unchecked(self) -> Self::Disabled;
    /// Enables the pull-up resistor on this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn enable_pullup_resistor(self) -> Result<Self::Enabled, (Self, PioError)>;
    /// Enables the pull-up resistor on this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its pull-up resistor disabled, but
    /// this may be at odds with the actual configuration state of the PIO controller. Writes to
    /// the configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Enabled;
}

#[cfg(feature = "ppd")]
/// # Pull-up and Pull-down Resistor Control
///
/// Each I/O line is designed with an embedded pull-up resistor and an embedded pull-down resistor.
/// The pull-up resistor can be enabled or disabled by writing respectively `PIO_PUER` (Pull-up
/// Enable Register) and `PIO_PUDR` (Pull-up Disable Register). Writing in these registers results
/// in setting or clearing the corresponding bit in `PIO_PUSR` (Pull-up Status Register). Reading a
/// 1 in `PIO_PUSR` means the pull-up is disabled and reading a 0 means the pull-up is enabled. The
/// pull-down resistor can be enabled or disabled by writing respectively `PIO_PPDER` (Pull-down
/// enable register) and `PIO_PPDDR` (Pull-down Disable Register). Writing in these registers
/// results in setting or clearing the corresponding bit in `PIO_PPDSR` (Pull-down Status Register).
/// Reading a 1 in `PIO_PPDSR` means the pull-up is disabled and reading a 0 means the pull-down is
/// enabled.
///
/// Enabling the pull-down resistor while the pull-up resistor is still enabled is not possible. In
/// this case, the write of `PIO_PPDER` for the concerned I/O line is discarded. Likewise, enabling
/// the pull-up resistor while the pull-down resistor is still enabled is not possible. In this
/// case, the write of `PIO_PUER` for the concerned I/O line is discarded.
///
/// Control of the pull-up resistor is possible regardless of the configuration of the I/O line.
///
/// After reset, all of the pull-ups are enabled, i.e. `PIO_PUSR` resets at the value `0x0`, and all
/// the pull-downs are disabled, i.e. `PIO_PPDSR` resets at the value `0xFFFF_FFFF`.
pub trait ConfigurePadResistor: Sized {
    type Disabled;
    type Pulldown;
    type Pullup;

    /// Disables the pull-up or pull-down resistor on this pin, whichever is active.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's pull-up resistor is enabled and its bit is set in `PIO_LOCKSR`, denoting that a
    ///   peripheral has locked its configuration, and it cannot be changed until a hardware reset
    ///   is given to the PIO controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn disable_pdpu_resistors(self) -> Result<Self::Disabled, (Self, PioError)>;
    /// Disables the pull-up or pull-down resistor (whichever is active) on this pin without
    /// checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its pull-up and pull-down resistors
    /// are disabled, but this may be at odds with the actual configuration state of the PIO
    /// controller. Writes to the configuration may fail silently if the PIO line is locked or write
    /// protection is enabled.
    unsafe fn disable_pdpu_resistors_unchecked(self) -> Self::Disabled;
    /// Enables the pull-down resistor on this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if write protection is enabled on the PIO controller. Write
    /// protection must first be disabled for any pins within the controller to have their
    /// configurations modified.
    fn enable_pulldown_resistor(self) -> Result<Self::Pulldown, (Self, PioError)>;
    /// Enables the pull-down resistor on this pin without checking `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its pull-up and pull-down resistors
    /// are disabled, but this may be at odds with the actual configuration state of the PIO
    /// controller. Writes to the configuration may fail silently if write protection is enabled.
    unsafe fn enable_pulldown_resistor_unchecked(self) -> Self::Pulldown;
    /// Enables the pull-up resistor on this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn enable_pullup_resistor(self) -> Result<Self::Pullup, (Self, PioError)>;
    /// Enables the pull-up resistor on this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its pull-up and pull-down resistors
    /// are disabled, but this may be at odds with the actual configuration state of the PIO
    /// controller. Writes to the configuration may fail silently if the PIO line is locked or write
    /// protection is enabled.
    unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Pullup;
}

/// # Multi Drive Control (Open Drain)
///
/// Each I/O can be independently programmed in Open Drain by using the Multi Drive feature. This
/// feature permits several drivers to be connected on the I/O line which is driven low only by each
/// device. An external pull-up resistor (or enabling of the internal one) is generally required to
/// guarantee a high level on the line.
///
/// The Multi Drive feature is controlled by `PIO_MDER` (Multi-driver Enable Register) and
/// `PIO_MDDR` (Multi-driver Disable Register). The Multi Drive can be selected whether the I/O line
/// is controlled by the PIO controller or assigned to a peripheral function. `PIO_MDSR`
/// (Multi-driver Status Register) indicates the pins that are configured to support external
/// drivers.
///
/// After reset, the Multi Drive feature is disabled on all pins, i.e. `PIO_MDSR` resets at value
/// 0x0.
pub trait ConfigureMultiDriver: Sized {
    type Disabled: ConfigureMultiDriver;
    type Enabled: ConfigureMultiDriver;

    /// Disable multi drive control of this pin. Waits for `PIO_MDSR` to update accordingly.
    fn disable_multi_driver(self) -> Result<Self::Disabled, (Self, PioError)>;
    /// Disable multi-drive control of this pin without waiting for `PIO_MDSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that multi drive of this pin is disabled, but multi
    /// drive on this pin isn't actually disabled until the corresponding bit in `PIO_MDSR` is
    /// cleared.
    unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled;
    /// Enable multi drive control of this pin. Waits for `PIO_MDSR` to update accordingly.
    fn enable_multi_driver(self) -> Result<Self::Enabled, (Self, PioError)>;
    /// Enable multi-drive control of this pin without waiting for `PIO_MDSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that multi drive of this pin is enabled, but multi
    /// drive on this pin isn't actually enabled until the corresponding bit in `PIO_MDSR` is set.
    unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled;
}

#[cfg(not(feature = "ppd"))]
const _: () = {
    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, Unconfigured, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, Pioc, PullupDisabled, Irpt, Filt>;
        type Enabled = Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>;

        fn disable_pullup_resistor(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                if pioreg._pusr().read().bits() & Pid::MASK != 0 {
                    let _ = self.disable_pullup_resistor_unchecked();
                    while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pullup_resistor_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._pudr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pullup_resistor(self) -> Result<Self::Enabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                if pioreg._pusr().read().bits() & Pid::MASK == 0 {
                    let _ = self.enable_pullup_resistor_unchecked();
                    while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg._puer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, PullupDisabled, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Self;
        type Enabled = Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>;

        fn disable_pullup_resistor(self) -> Result<Self::Disabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn disable_pullup_resistor_unchecked(self) -> Self::Disabled {
            self
        }

        fn enable_pullup_resistor(self) -> Result<Self::Enabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let _ = self.enable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg._puer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, Pioc, PullupDisabled, Irpt, Filt>;
        type Enabled = Self;

        fn disable_pullup_resistor(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let _ = self.disable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pullup_resistor_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._pudr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pullup_resistor(self) -> Result<Self::Enabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Enabled {
            self
        }
    }
};

#[cfg(feature = "ppd")]
const _: () = {
    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, Unconfigured, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, Pioc, PdPuDisabled, Irpt, Filt>;
        type Pulldown = Pin<Pio, Pid, Mdvr, Pioc, PulldownEnabled, Irpt, Filt>;
        type Pullup = Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>;

        fn disable_pdpu_resistors(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                if pioreg._pusr().read().bits() & Pid::MASK == 0 {
                    pioreg._pudr().write_with_zero(|w| w.bits(Pid::MASK));
                    while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                } else if pioreg._ppdsr().read().bits() & Pid::MASK == 0 {
                    pioreg._ppddr().write_with_zero(|w| w.bits(Pid::MASK));
                    while pioreg._ppdsr().read().bits() & Pid::MASK == 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pdpu_resistors_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._pudr().write_with_zero(|w| w.bits(Pid::MASK));
            pioreg._ppddr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pulldown_resistor(self) -> Result<Self::Pulldown, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                if pioreg._pusr().read().bits() & Pid::MASK == 0 {
                    pioreg._pudr().write_with_zero(|w| w.bits(Pid::MASK));
                    while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                }
                if pioreg._ppdsr().read().bits() & Pid::MASK != 0 {
                    let _ = self.enable_pulldown_resistor_unchecked();
                    while pioreg._ppdsr().read().bits() & Pid::MASK != 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pulldown_resistor_unchecked(self) -> Self::Pulldown {
            let pioreg = &*Pio::PTR;
            pioreg._ppder().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pullup_resistor(self) -> Result<Self::Pullup, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                if pioreg._ppdsr().read().bits() & Pid::MASK == 0 {
                    pioreg._ppddr().write_with_zero(|w| w.bits(Pid::MASK));
                    while pioreg._ppdsr().read().bits() & Pid::MASK == 0 {}
                }
                if pioreg._pusr().read().bits() & Pid::MASK != 0 {
                    let _ = self.enable_pullup_resistor_unchecked();
                    while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Pullup {
            let pioreg = &*Pio::PTR;
            pioreg._puer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, PdPuDisabled, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Self;
        type Pulldown = Pin<Pio, Pid, Mdvr, Pioc, PulldownEnabled, Irpt, Filt>;
        type Pullup = Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>;

        fn disable_pdpu_resistors(self) -> Result<Self::Disabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn disable_pdpu_resistors_unchecked(self) -> Self::Disabled {
            self
        }

        fn enable_pulldown_resistor(self) -> Result<Self::Pulldown, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let _ = self.enable_pulldown_resistor_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK != 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pulldown_resistor_unchecked(self) -> Self::Pulldown {
            let pioreg = &*Pio::PTR;
            pioreg._ppder().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pullup_resistor(self) -> Result<Self::Pullup, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let _ = self.enable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Pullup {
            let pioreg = &*Pio::PTR;
            pioreg._puer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, PulldownEnabled, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, Pioc, PdPuDisabled, Irpt, Filt>;
        type Pulldown = Self;
        type Pullup = Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>;

        fn disable_pdpu_resistors(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let _ = self.disable_pdpu_resistors_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK == 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pdpu_resistors_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._ppddr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pulldown_resistor(self) -> Result<Self::Pulldown, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn enable_pulldown_resistor_unchecked(self) -> Self::Pulldown {
            self
        }

        fn enable_pullup_resistor(self) -> Result<Self::Pullup, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let self_copy: Self = Pin::new();
                let _ = self_copy.disable_pdpu_resistors_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK == 0 {}
                let _ = self.enable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Pullup {
            let pioreg = &*Pio::PTR;
            let _ = self.disable_pdpu_resistors_unchecked();
            pioreg._puer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Irpt, Filt> ConfigurePadResistor
        for Pin<Pio, Pid, Mdvr, Pioc, PullupEnabled, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, Pioc, PdPuDisabled, Irpt, Filt>;
        type Pulldown = Pin<Pio, Pid, Mdvr, Pioc, PulldownEnabled, Irpt, Filt>;
        type Pullup = Self;

        fn disable_pdpu_resistors(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let _ = self.disable_pdpu_resistors_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pdpu_resistors_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._pudr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pulldown_resistor(self) -> Result<Self::Pulldown, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                    return Err((self, PioError::LineLocked));
                }
                let self_copy: Self = Pin::new();
                let _ = self_copy.disable_pdpu_resistors_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
                let _ = self.enable_pulldown_resistor_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK != 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pulldown_resistor_unchecked(self) -> Self::Pulldown {
            let pioreg = &*Pio::PTR;
            pioreg._ppder().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pullup_resistor(self) -> Result<Self::Pullup, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Pullup {
            self
        }
    }
};

impl<Pio, Pid, Pioc, Padr, Irpt, Filt> ConfigureMultiDriver
    for Pin<Pio, Pid, Unconfigured, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Disabled = Pin<Pio, Pid, MultiDriverDisabled, Pioc, Padr, Irpt, Filt>;
    type Enabled = Pin<Pio, Pid, MultiDriverEnabled, Pioc, Padr, Irpt, Filt>;

    fn disable_multi_driver(self) -> Result<Self::Disabled, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            if pioreg._mdsr().read().bits() & Pid::MASK != 0 {
                let _ = self.disable_multi_driver_unchecked();
                while pioreg._mdsr().read().bits() & Pid::MASK != 0 {}
            }
            Ok(Pin::new())
        }
    }

    unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled {
        let pioreg = &*Pio::PTR;
        pioreg._mddr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn enable_multi_driver(self) -> Result<Self::Enabled, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            if pioreg._mdsr().read().bits() & Pid::MASK == 0 {
                let _ = self.enable_multi_driver_unchecked();
                while pioreg._mdsr().read().bits() & Pid::MASK == 0 {}
            }
            Ok(Pin::new())
        }
    }

    unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled {
        let pioreg = &*Pio::PTR;
        pioreg._mder().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Pioc, Padr, Irpt, Filt> ConfigureMultiDriver
    for Pin<Pio, Pid, MultiDriverEnabled, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Disabled = Pin<Pio, Pid, MultiDriverDisabled, Pioc, Padr, Irpt, Filt>;
    type Enabled = Self;

    fn disable_multi_driver(self) -> Result<Self::Disabled, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            let _ = self.disable_multi_driver_unchecked();
            while pioreg._mdsr().read().bits() & Pid::MASK != 0 {}
            Ok(Pin::new())
        }
    }

    unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled {
        let pioreg = &*Pio::PTR;
        pioreg._mddr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn enable_multi_driver(self) -> Result<Self::Enabled, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled {
        self
    }
}

impl<Pio, Pid, Pioc, Padr, Irpt, Filt> ConfigureMultiDriver
    for Pin<Pio, Pid, MultiDriverDisabled, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Disabled = Self;
    type Enabled = Pin<Pio, Pid, MultiDriverEnabled, Pioc, Padr, Irpt, Filt>;

    fn disable_multi_driver(self) -> Result<Self::Disabled, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled {
        self
    }

    fn enable_multi_driver(self) -> Result<Self::Enabled, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            let _ = self.enable_multi_driver_unchecked();
            while pioreg._mdsr().read().bits() & Pid::MASK == 0 {}
            Ok(Pin::new())
        }
    }

    unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled {
        let pioreg = &*Pio::PTR;
        pioreg._mder().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

/// Make a digital reading of the pin from `PIO_PDSR`.
pub trait DigitalRead {
    fn digital_read(&self) -> bool;
}

impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt> DigitalRead
    for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    fn digital_read(&self) -> bool {
        unsafe {
            let pioreg = &*Pio::PTR;
            pioreg._pdsr().read().bits() & Pid::MASK != 0
        }
    }
}

/*
unsure if this is the right way to do it - maybe reverse?
what i mean by this is instead of

type Pwm0<Padr, Irpt, Filt> = Pin<PioA, Pa0, MultiDriverDisabled<...>, Padr, Irpt, Filt>;

instead have

pub trait Pwm0: ConfigurePioControl + ConfigureFunctionSelect {
    type Pwm0;

    fn to_pwm0(self) -> Result<Self::Pwm0, (Self, PioError)>;
    unsafe fn to_pwm0_unchecked(self) -> Self::Pwm0;
}

impl<Line, Padr, Irpt, Filt> Pwm0 for Pin<PioA, Pa0, MultiDriverDisabled<Line>, Padr, Irpt, Filt>
where
    Pin<PioA, Pa0, MultiDriverDisabled<Line>, Padr, Irpt, Filt>: ConfigurePioControl
        + ConfigureFunctionSelect,
{
    type Pwm0 =
        Pin<PioA, Pa0, MultiDriverDisabled<PeripheralControlled<PeripheralA>>, Padr, Irpt, Filt>;

    fn to_pwm0(self) -> Result<Self::Pwm0, (Self, PioError)> {
        self.peripheral_controlled()?.peripheral_a()
    }

    unsafe fn to_pwm0_unchecked(self) -> Self::Pwm0 {
        self.peripheral_controlled_unchecked().peripheral_a_unchecked()
    }
}

and so on

macro_rules! def_peripheral_multiplex {
    (
        $pio:ty {
            $($pid:ty: $opts:tt),+$(,)?
        }
    ) => {
        $(
            def_peripheral_multiplex! {
                @expand $pio, $pid, $opts
            }
        )+
    };
    (@expand $pio:ty, $pid:ty, [$($sel:tt $name:ident),+$(,)?]) => {
        $(
            def_peripheral_multiplex! {
                @def $pio, $pid, $sel, $name
            }
        )+
    };
    (@def $pio:ty, $pid:ty, $suffix:tt, $name:ident) => {
        paste::paste! {
            pub type $name<Padr, Irpt, Filt> = crate::pio::pin::Pin<
                $pio,
                $pid,
                crate::pio::peripheral::MultiDriverDisabled<
                    crate::pio::peripheral::PeripheralControlled<
                        crate::pio::peripheral::[<Peripheral $suffix:upper>]
                    >,
                >,
                Padr,
                Irpt,
                Filt,
            >;
            pub type [<$name MD>]<Otpt, Padr, Irpt, Filt> = crate::pio::pin::Pin<
                $pio,
                $pid,
                crate::pio::peripheral::MultiDriverEnabled<
                    crate::pio::peripheral::PeripheralControlled<
                        crate::pio::peripheral::[<Peripheral $suffix:upper>]
                    >,
                    Otpt,
                >,
                Padr,
                Irpt,
                Filt,
            >;
        }
    };
}

pub(crate) use def_peripheral_multiplex;
*/
