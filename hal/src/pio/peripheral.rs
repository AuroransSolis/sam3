//! PIO peripheral configuration
//!
//! # Pin Multiplexing
//!
//! Each pin is configurable, according to product definition as either a general-purpose I/O line
//! only, or as an I/O line multiplexed with one or two peripheral I/Os. As the multiplexing is
//! hardware defined and thus product-dependent, the hardware designer and programmer must carefully
//! determine the configuration of the PIO controllers required by their application. When an I/O
//! line is general-purpose only, i.e. not multiplexed with any peripheral I/O, programming of the
//! PIO Controller regarding the assignment to a peripheral has no effect and only the PIO
//! Controller can control how the pin is driven by the product.
//!
//! Relevant manual sections:
//! - SAM3A, SAM3X: [manual][ax], pages 622-624 (31.5.2, 31.5.3, 31.5.4, 31.5.5, 31.5.7)
//! - SAM3N: [manual][n], pages 380-382 (27.5.2, 27.5.3, 27.5.4, 27.5.5, 27.5.7)
//! - SAM3S1, SAM3S2, SAM3S4: [manual][n], pages 471-473 (29.5.2, 29.5.3, 29.5.4, 29.5.5, 29.5.7)
//! - SAM3S8, SAM3SD8: [manual][sd8], pages 480-482 (28.5.2, 28.5.3, 28.5.4, 28.5.5, 28.5.7)
//! - SAM3U: [manual][u], pages 498-499 (29.5.2, 29.5.3, 29.5.4, 29.5.5, 29.5.7)
//!
//! [ax]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf
//! [n]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11011-32-bit-Cortex-M3-Microcontroller-SAM3N_Datasheet.pdf
//! [s124]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6500-32-bit-Cortex-M3-Microcontroller-SAM3S4-SAM3S2-SAM3S1_Datasheet.pdf
//! [sd8]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11090-32-bit%20Cortex-M3-Microcontroller-SAM-3S8-SD8_Datasheet.pdf
//! [u]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6430-32-bit-Cortex-M3-Microcontroller-SAM3U4-SAM3U2-SAM3U1_Datasheet.pdf

use crate::{
    pio::{
        filter::InputFilterCfg,
        interrupt::InterruptCfg,
        pin::{Configured, MultiDriverCfg, PadResistorCfg, Pin, PinId, Unconfigured},
        structure::*,
        PioError,
    },
    structure::*,
    write_protect::*,
};
use core::marker::PhantomData;

/// Marker trait for line control configuration options.
pub trait PioControlCfg {}

impl PioControlCfg for Unconfigured {}

#[allow(clippy::module_name_repetitions)]
/// Allow the peripheral to control this I/O line.
pub struct PeripheralControlled<Psel: PeripheralSelectCfg> {
    _psel: PhantomData<Psel>,
}

impl<Psel: PeripheralSelectCfg> PioControlCfg for PeripheralControlled<Psel> {}
impl<Psel: PeripheralSelectCfg + Configured> Configured for PeripheralControlled<Psel> {}

/// Allow the PIO controller to control this I/O.
pub struct PioControlled<Otpt: OutputCfg> {
    _otpt: PhantomData<Otpt>,
}

impl<Otpt: OutputCfg> PioControlCfg for PioControlled<Otpt> {}
impl<Otpt: OutputCfg + Configured> Configured for PioControlled<Otpt> {}

/// # I/O Line or Peripheral Function Selection
///
/// When a pin is multiplexed with one or two peripheral functions, the selection is controlled with
/// the registers `PIO_PER` (PIO Enable Register) and `PIO_PDR` (PIO Disable Register). The register
/// `PIO_PSR` (PIO Status Register) is the result of the set and clear registers and indicates
/// whether the pin is controlled by the corresponding peripheral or by the PIO Controller. A value
/// of 0 indicates the pin is controlled by the corresponding on-chip peripheral selected in the
/// `PIO_ABSR` (AB Select Register). A value of 1 indicates the pin is controlled by the PIO
/// controller.
///
/// If a pin is used as a general purpose I/O line (not multiplexed with an on-chip peripheral),
/// `PIO_PER` and `PIO_PDR` have no effect and `PIO_PSR` returns 1 for the corresponding bit.
///
/// After reset, most generally, the I/O lines are controlled by the PIO controller, i.e. `PIO_PSR`
/// resets at 1. Howeever, in some events, it is important that PIO lines are controlled by the
/// peripheral (as in the case of memory chip seelect lines that must be driven inactive after reset
/// or for address lines that must be driven low for booting out of an external memory). Thus, the
/// reset value of `PIO_PSR` is defined at the product level, depending on the multiplexing of the
/// device.
pub trait ConfigurePioControl: Sized {
    type Pio: ConfigurePioControl;
    type Peripheral: ConfigurePioControl;

    /// Enable PIO control of this pin. Waits for `PIO_PSR` to update accordingly.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)>;
    /// Enable PIO control of this pin without waiting for `PIO_PSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin is PIO controlled, but this may be at
    /// odds with the actual configuration state of the PIO controller. Writes to the configuration
    /// may fail silently if the PIO line is locked or write protection is enabled.
    unsafe fn pio_controlled_unchecked(self) -> Self::Pio;
    /// Enable peripheral control of this pin. Waits for `PIO_PSR` to update accordingly.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)>;
    /// Enable peripheral control of this pin without waiting for `PIO_PSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin is peripheral controlled, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is enabled.
    unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral;
}

// When the I/O line is controlled by the PIO controller, the pin can be configured to be driven.
// This is done by writing `PIO_OER` (Output Enable Register) and `PIO_ODR` (Output Disable
// Register). The results of these write operations are detected in `PIO_OSR` (Output Status
// Register). When a bit in this register is at 0, the corresponding I/O line is used as an input
// only. When the bit is at 1, the corresponding I/O line is driven by the PIO controller.

#[allow(clippy::module_name_repetitions)]
/// Marker trait for peripheral selection configuration options.
pub trait PeripheralSelectCfg {}

impl PeripheralSelectCfg for Unconfigured {}

#[allow(clippy::module_name_repetitions)]
/// Allow output from peripheral A to drive the I/O line.
pub struct PeripheralA;

impl PeripheralSelectCfg for PeripheralA {}
impl Configured for PeripheralA {}

#[allow(clippy::module_name_repetitions)]
/// Allow output from peripheral B to drive the I/O line.
pub struct PeripheralB;

impl PeripheralSelectCfg for PeripheralB {}
impl Configured for PeripheralB {}

#[cfg(any(feature = "3fn", feature = "4fn"))]
mod c {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[allow(clippy::module_name_repetitions)]
    /// Allow output from peripheral C to drive the I/O line.
    pub struct PeripheralC;

    impl PeripheralSelectCfg for PeripheralC {}
    impl Configured for PeripheralC {}
}

#[cfg(any(feature = "3fn", feature = "4fn"))]
pub use c::PeripheralC;

#[cfg(feature = "4fn")]
mod d {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[allow(clippy::module_name_repetitions)]
    /// Allow output from peripheral D to drive the I/O line.
    pub struct PeripheralD;

    impl PeripheralSelectCfg for PeripheralD {}
    impl Configured for PeripheralD {}
}

#[cfg(feature = "4fn")]
pub use d::PeripheralD;

/// Marks whether a given peripheral exists for a given PIO + pin ID combination.
pub trait PeripheralExistsFor<Pio: PioRegisters, Pid: PinId<Controller = Pio>>:
    PeripheralSelectCfg
{
}

/// Marks it possible to execute a given function selection and provides methods to do so.
///
/// This is the preferred way to execute a function selection, since it should only be callable
/// on types that are able to be configured to a given peripheral.
pub trait ExecFnSel<Psel>: ConfigureFunctionSelect
where
    Self: Sized,
    Psel: PeripheralSelectCfg,
{
    type Configured;
    /// Selects the given peripheral for this pin. Waits for the peripheral selection register(s)
    /// to update.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn configure_function(self) -> Result<Self::Configured, (Self, PioError)>;
    /// Executes a given function selection without checking `PIO_LOCKSR`, `PIO_WPMR`, or waiting
    /// for the peripheral selection register(s) to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has selected the chosen peripheral, but
    /// this may be at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is enabled.
    unsafe fn configure_function_unchecked(self) -> Self::Configured;
}

impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt> ExecFnSel<PeripheralA>
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect,
    PeripheralA: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Configured = <Self as ConfigureFunctionSelect>::A;

    fn configure_function(self) -> Result<Self::Configured, (Self, PioError)> {
        unsafe { self.peripheral_a() }
    }

    unsafe fn configure_function_unchecked(self) -> Self::Configured {
        self.peripheral_a_unchecked()
    }
}

impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt> ExecFnSel<PeripheralB>
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect,
    PeripheralB: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Configured = <Self as ConfigureFunctionSelect>::B;

    fn configure_function(self) -> Result<Self::Configured, (Self, PioError)> {
        unsafe { self.peripheral_b() }
    }

    unsafe fn configure_function_unchecked(self) -> Self::Configured {
        self.peripheral_b_unchecked()
    }
}

#[cfg(feature = "3fn")]
impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt> ExecFnSel<PeripheralC>
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect,
    PeripheralC: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Configured = <Self as ConfigureFunctionSelect>::C;

    fn configure_function(self) -> Result<Self::Configured, (Self, PioError)> {
        unsafe { self.peripheral_c() }
    }

    unsafe fn configure_function_unchecked(self) -> Self::Configured {
        self.peripheral_c_unchecked()
    }
}

#[cfg(feature = "4fn")]
impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt> ExecFnSel<PeripheralD>
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect,
    PeripheralD: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Configured = <Self as ConfigureFunctionSelect>::D;

    fn configure_function(self) -> Result<Self::Configured, (Self, PioError)> {
        unsafe { self.peripheral_d() }
    }

    unsafe fn configure_function_unchecked(self) -> Self::Configured {
        self.peripheral_d_unchecked()
    }
}

impl<Pio, Pid, Mdvr, Psel1, Padr, Irpt, Filt>
    Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel1>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel1: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    /// A sort of equivalent to `ExecFnSel::configure_function`, except the generic is on the
    /// function instead of the trait.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    pub fn select_peripheral<Psel2>(
        self,
    ) -> Result<<Self as ExecFnSel<Psel2>>::Configured, (Self, PioError)>
    where
        Self: ExecFnSel<Psel2>,
        Psel2: PeripheralExistsFor<Pio, Pid>,
    {
        self.configure_function()
    }

    /// A sort of equivalent to `ExecFnSel::configure_function_unchecked`, except the generic is on
    /// the function instead of the trait.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral A, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    pub unsafe fn select_peripheral_unchecked<Psel2>(self) -> <Self as ExecFnSel<Psel2>>::Configured
    where
        Self: ExecFnSel<Psel2>,
        Psel2: PeripheralExistsFor<Pio, Pid>,
    {
        self.configure_function_unchecked()
    }
}

impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt>
    Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect + ExecFnSel<PeripheralA>,
    PeripheralA: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    /// Select peripheral A for this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    pub fn select_peripheral_a(
        self,
    ) -> Result<<Self as ExecFnSel<PeripheralA>>::Configured, (Self, PioError)> {
        self.configure_function()
    }

    /// Select peripheral A for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral A, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    pub unsafe fn select_peripheral_a_unchecked(
        self,
    ) -> <Self as ExecFnSel<PeripheralA>>::Configured {
        self.configure_function_unchecked()
    }
}

impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt>
    Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect + ExecFnSel<PeripheralB>,
    PeripheralB: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    /// Select peripheral B for this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    pub fn select_peripheral_b(
        self,
    ) -> Result<<Self as ExecFnSel<PeripheralB>>::Configured, (Self, PioError)> {
        self.configure_function()
    }

    /// Select peripheral B for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral B, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    pub unsafe fn select_peripheral_b_unchecked(
        self,
    ) -> <Self as ExecFnSel<PeripheralB>>::Configured {
        self.configure_function_unchecked()
    }
}

#[cfg(feature = "3fn")]
impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt>
    Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect + ExecFnSel<PeripheralC>,
    PeripheralC: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    /// Select peripheral C for this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    pub fn select_peripheral_c(
        self,
    ) -> Result<<Self as ExecFnSel<PeripheralC>>::Configured, (Self, PioError)> {
        self.configure_function()
    }

    /// Select peripheral C for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral C, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    pub unsafe fn select_peripheral_c_unchecked(
        self,
    ) -> <Self as ExecFnSel<PeripheralC>>::Configured {
        self.configure_function_unchecked()
    }
}

#[cfg(feature = "4fn")]
impl<Pio, Pid, Mdvr, Psel, Padr, Irpt, Filt>
    Pin<Pio, Pid, Mdvr, PeripheralControlled<Psel>, Padr, Irpt, Filt>
where
    Self: ConfigureFunctionSelect + ExecFnSel<PeripheralD>,
    PeripheralD: PeripheralExistsFor<Pio, Pid>,
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Psel: PeripheralSelectCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    /// Select peripheral D for this pin.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    pub fn select_peripheral_d(
        self,
    ) -> Result<<Self as ExecFnSel<PeripheralD>>::Configured, (Self, PioError)> {
        self.configure_function()
    }

    /// Select peripheral D for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral D, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if the PIO line is locked or write protection is
    /// enabled.
    pub unsafe fn select_peripheral_d_unchecked(
        self,
    ) -> <Self as ExecFnSel<PeripheralD>>::Configured {
        self.configure_function_unchecked()
    }
}

#[cfg(feature = "2fn")]
mod fn_ab {
    use crate::pio::PioError;

    /// # Peripheral A or B Selection
    ///
    /// The PIO Controller provides multiplexing of up to two peripheral functions on a single pin.
    /// The selection is performed by writing `PIO_ABSR` (AB Select Register). For each pin, the
    /// corresponding bit at level 0 means peripheral A is selected whereas the corresponding bit at
    /// level 1 indicates that peripheral B is selected.
    ///
    /// Note that multiplexing of peripheral lines A and B only affects the output line. The
    /// peripheral input lines are always connected to the pin input.
    ///
    /// After reset, `PIO_ABSR` is 0, thus indicating that all the PIO lines are configured on
    /// peripheral A. However, peripheral A generally does not drive the pin as the PIO Controller
    /// resets in I/O line mode.
    ///
    /// Writing in `PIO_ABSR` manages the multiplexing regardless of the configuration of the pin.
    /// However, assignment of a pin to a peripheral function requires a write in the peripheral
    /// selection register (`PIO_ABSR`) in addition to a write in `PIO_PDR`.
    /// ([`ConfigurePioControl::peripheral_controlled`][pc],
    /// [`ConfigurePioControl::peripheral_controlled_unchecked`][pcu]).
    ///
    /// [pc]: crate::pio::peripheral::ConfigurePioControl::peripheral_controlled
    /// [pcu]: crate::pio::peripheral::ConfigurePioControl::peripheral_controlled_unchecked
    pub trait ConfigureFunctionSelect2Fn: Sized {
        type A: ConfigureFunctionSelect2Fn;
        type B: ConfigureFunctionSelect2Fn;

        /// Select peripheral A for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)>;
        /// Select peripheral A for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral A, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_a_unchecked(self) -> Self::A;
        /// Enable select peripheral B for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)>;
        /// Select peripheral B for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral B, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_b_unchecked(self) -> Self::B;
    }
}

#[cfg(feature = "2fn")]
pub use fn_ab::ConfigureFunctionSelect2Fn as ConfigureFunctionSelect;

#[cfg(all(feature = "3fn", not(feature = "4fn")))]
mod fn_abc {
    use crate::pio::PioError;

    /// # Peripheral A or B or C Selection
    ///
    /// The PIO Controller provides multiplexing of up to three peripheral functions on a single
    /// pin. The selection is performed by writing to `PIO_ABCDSR0` and `PIO_ABCDSR1` (ABCD select
    /// registers).
    ///
    /// The following is a table denoting the peripheral selected for all the select register
    /// combinations:
    ///
    /// `PIO_ABCDSR0` | `PIO_ABCDSR1` | Selected
    /// --------------|---------------|------------
    ///             0 |             0 |           A
    ///             1 |             0 |           B
    ///             0 |             1 |           C
    ///             1 |             0 | Unspecified
    ///
    /// Note that multiplexing of peripheral lines A, B, and C only affects the output line. The
    /// peripheral input lines are always connected to the pin input.
    ///
    /// After reset, `PIO_ABCDSR0` and `PIO_ABCDSR1` are 0, thus indicating that all the PIO lines
    /// are configured on peripheral A. However, peripheral A generally does not drive the pin as
    /// the PIO controller resets in I/O line mode.
    ///
    /// Writing in `PIO_ABCDSR0` and `PIO_ABCDSR1` manages the multiplexing regardless of the
    /// configuration of the pin. However, an assigment of a pin to a peripheral function requires a
    /// write in the peripheral selection registers (`PIO_ABCDSR0` and `PIO_ABCDSR1`) in addition to
    /// a write in `PIO_PDR` ([`ConfigurePioControl::peripheral_controlled`][pc],
    /// [`ConfigurePioControl::peripheral_controlled_unchecked`][pcu]).
    ///
    /// [pc]: crate::pio::peripheral::ConfigurePioControl::peripheral_controlled
    /// [pcu]: crate::pio::peripheral::ConfigurePioControl::peripheral_controlled_unchecked
    pub trait ConfigureFunctionSelect3Fn: Sized {
        type A: ConfigureFunctionSelect3Fn;
        type B: ConfigureFunctionSelect3Fn;
        type C: ConfigureFunctionSelect3Fn;

        /// Select peripheral A for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)>;
        /// Select peripheral A for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral A, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_a_unchecked(self) -> Self::A;
        /// Select peripheral B for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)>;
        /// Select peripheral B for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral B, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_b_unchecked(self) -> Self::B;
        /// Select peripheral C for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)>;
        /// Select peripheral C for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral C, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_c_unchecked(self) -> Self::C;
    }
}

#[cfg(all(feature = "3fn", not(feature = "4fn")))]
pub use fn_abc::ConfigureFunctionSelect3Fn as ConfigureFunctionSelect;

#[cfg(feature = "4fn")]
mod fn_abcd {
    use crate::pio::PioError;

    /// # Peripheral A or B or C or D Selection
    ///
    /// The PIO Controller provides multiplexing of up to three peripheral functions on a single
    /// pin. The selection is performed by writing to `PIO_ABCDSR0` and `PIO_ABCDSR1` (ABCD select
    /// registers).
    ///
    /// The following is a table denoting the peripheral selected for all the select register
    /// combinations:
    ///
    /// `PIO_ABCDSR0` | `PIO_ABCDSR1` | Selected
    /// --------------|---------------|------------
    ///             0 |             0 |           A
    ///             1 |             0 |           B
    ///             0 |             1 |           C
    ///             1 |             0 |           D
    ///
    /// Note that multiplexing of peripheral lines A, B, C, and D only affects the output line. The
    /// peripheral input lines are always connected to the pin input.
    ///
    /// After reset, `PIO_ABCDSR0` and `PIO_ABCDSR1` are 0, thus indicating that all the PIO lines
    /// are configured on peripheral A. However, peripheral A generally does not drive the pin as
    /// the PIO controller resets in I/O line mode.
    ///
    /// Writing in `PIO_ABCDSR0` and `PIO_ABCDSR1` manages the multiplexing regardless of the
    /// configuration of the pin. However, an assigment of a pin to a peripheral function requires a
    /// write in the peripheral selection registers (`PIO_ABCDSR0` and `PIO_ABCDSR1`) in addition to
    /// a write in `PIO_PDR` ([`ConfigurePioControl::peripheral_controlled`],
    /// [`ConfigurePioControl::peripheral_controlled_unchecked`][pcu]).
    ///
    /// [pc]: crate::pio::peripheral::ConfigurePioControl::peripheral_controlled
    /// [pcu]: crate::pio::peripheral::ConfigurePioControl::peripheral_controlled_unchecked
    pub trait ConfigureFunctionSelect4Fn: Sized {
        type A: ConfigureFunctionSelect4Fn;
        type B: ConfigureFunctionSelect4Fn;
        type C: ConfigureFunctionSelect4Fn;
        type D: ConfigureFunctionSelect4Fn;

        /// Select peripheral A for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)>;
        /// Select peripheral A for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral A, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_a_unchecked(self) -> Self::A;
        /// Select peripheral B for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)>;
        /// Select peripheral B for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral B, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_b_unchecked(self) -> Self::B;
        /// Select peripheral C for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)>;
        /// Select peripheral C for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral C, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_c_unchecked(self) -> Self::C;
        /// Select peripheral D for this pin.
        ///
        /// # Errors
        ///
        /// This function can fail if:
        /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
        ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
        ///   controller.
        /// - Write protection is enabled on the PIO controller. Write protection must first be
        ///   disabled for any pins within the controller to have their configurations modified.
        ///
        /// # Safety
        ///
        /// The device manuals do not specify what happens in the event that an unmapped peripheral
        /// function is selected, and so this method is marked as unsafe. Please refer to
        /// [`ExecFnSel`][efs] for a safe method to configure pins.
        ///
        /// [efs]: crate::pio::peripheral::ExecFnSel
        unsafe fn peripheral_d(self) -> Result<Self::D, (Self, PioError)>;
        /// Select peripheral D for this pin without checking `PIO_LOCKSR` or `PIO_WPMR`.
        ///
        /// # Safety
        ///
        /// This function returns a type showing that this pin selects peripheral D, but this may be
        /// at odds with the actual configuration state of the PIO controller. Writes to the
        /// configuration may fail silently if the PIO line is locked or write protection is
        /// enabled.
        unsafe fn peripheral_d_unchecked(self) -> Self::D;
    }
}

#[cfg(feature = "4fn")]
pub use fn_abcd::ConfigureFunctionSelect4Fn as ConfigureFunctionSelect;

/// Marker trait for PIO output control configuration options.
pub trait OutputCfg {}

impl OutputCfg for Unconfigured {}

#[cfg(not(feature = "schmitt"))]
/// Disable PIO control of the output.
pub struct OutputDisabled;

#[cfg(feature = "schmitt")]
/// Disable PIO control of the output.
pub struct OutputDisabled<Schm: SchmittTriggerCfg> {
    _schm: PhantomData<Schm>,
}

#[cfg(not(feature = "schmitt"))]
impl OutputCfg for OutputDisabled {}
#[cfg(feature = "schmitt")]
impl<Schm: SchmittTriggerCfg> OutputCfg for OutputDisabled<Schm> {}

#[cfg(not(feature = "schmitt"))]
impl Configured for OutputDisabled {}
#[cfg(feature = "schmitt")]
impl<Schm: SchmittTriggerCfg + Configured> Configured for OutputDisabled<Schm> {}

/// Enable PIO control of the output.
pub struct OutputEnabled<Sync: OutputSyncWriteCfg, Outw: OutputWriteCfg> {
    _sync: PhantomData<Sync>,
    _outw: PhantomData<Outw>,
}

impl<Sync: OutputSyncWriteCfg, Outw: OutputWriteCfg> OutputCfg for OutputEnabled<Sync, Outw> {}
impl<Sync: OutputSyncWriteCfg + Configured, Outw: OutputWriteCfg + Configured> Configured
    for OutputEnabled<Sync, Outw>
{
}

/// # Output Control (enable/disable PIO driving output)
///
/// When the I/O line is assigned to a peripheral function, i.e. the corresponding bit in `PIO_PSR`
/// is at 0, the drive of the I/O line is controlled by the peripheral. Peripheral A or B depending
/// on the value in `PIO_ABSR` (AB Select Register) determines whether the pin is driven or not.
///
/// When the I/O line is controlled by the PIO controller, the pin can be configured to be driven.
/// This is done by writing `PIO_OER` (Output Enable Register) and `PIO_ODR` (Output Disable
/// Register). The results of these write operations are detected in `PIO_OSR` (Output Status
/// Register). When a bit in this register is at 0, the corresponding I/O line is used as an input
/// only. When the bit is at 1, the corresponding I/O line is driven by the PIO controller.
pub trait ConfigurePioOutput: Sized {
    type Enabled: ConfigurePioOutput;
    type Disabled: ConfigurePioOutput;

    /// Enables PIO output for this pin. Waits for `PIO_OSR` to update accordingly.
    ///
    /// # Errors
    ///
    /// This function can fail if write protection is enabled on the PIO controller. Write
    /// protection must first be disabled for any pins within the controller to have their
    /// configurations modified.
    fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)>;
    /// Enables PIO output for this pin without waiting for `PIO_OSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has PIO output enabled, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if write protection is enabled.
    unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled;
    /// Disables PIO output for this pin. Waits for `PIO_OSR` to update accordingly.
    ///
    /// # Errors
    ///
    /// This function can fail if write protection is enabled on the PIO controller. Write
    /// protection must first be disabled for any pins within the controller to have their
    /// configurations modified.
    fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)>;
    /// Disables PIO output for this pin without waiting for `PIO_OSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has PIO output disabled, but this may be
    /// at odds with the actual configuration state of the PIO controller. Writes to the
    /// configuration may fail silently if write protection is enabled.
    unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled;
}

/// Marker trait for synchronous output writing configuration options.
pub trait OutputSyncWriteCfg {}

impl OutputSyncWriteCfg for Unconfigured {}

/// Allow the output of this PIO line to be set or cleared synchronously with other PIO lines
/// by writing to `PIO_ODSR`.
pub struct SyncOutputEnabled;

impl OutputSyncWriteCfg for SyncOutputEnabled {}
impl Configured for SyncOutputEnabled {}

/// Disable synchronously setting the output of this PIO line with others.
pub struct SyncOutputDisabled;

impl OutputSyncWriteCfg for SyncOutputDisabled {}
impl Configured for SyncOutputDisabled {}

/// # Synchronous Data Output
///
/// Clearing one (or more) PIO line(s) and setting another one (or more) PIO line(s) synchronously
/// cannot be done by using `PIO_SODR` and `PIO_CODR` registers. It requires two successive write
/// operations into two different registers. To overcome this, the PIO Controller offers a direct
/// control of PIO outputs by single write access to `PIO_ODSR` (Output Data Status Register). Only
/// bits unmasked by `PIO_OWSR` (Output Write Status Register) are written. The mask bits in
/// `PIO_OWSR` are set by writing to `PIO_OWER` (Output Write Enable Register) and cleared by
/// writing to `PIO_OWDR` (Output Write Disable Register).
///
/// After reset, the synchronous data output is disabled on all the I/O lines as `PIO_OWSR` resets
/// at `0x0`.
pub trait ConfigureOutputSyncWrite: Sized {
    type Enabled: ConfigureOutputSyncWrite;
    type Disabled: ConfigureOutputSyncWrite;

    /// Enables synchronous data output writing for this pin. Waits for `PIO_OWSR` to update
    /// accordingly.
    ///
    /// # Errors
    ///
    /// This function will fail unless write protection is disabled on the PIO controller. Write
    /// protection must be disabled for any pins within the controller to have their configurations
    /// modified.
    ///
    /// # Safety
    ///
    /// This function allows external control of this pin's output, and for that reason, it's marked
    /// `unsafe`. Mutable access to either this pin or the PIO controller driving it will allow
    /// modification of the output.
    unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)>;
    /// Enables synchronous data output writing for this pin without waiting for `PIO_OWSR` to
    /// update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has synchronous output writing enabled,
    /// but this may be at odds with the actual configuration state of the PIO controller. Writes to
    /// the configuration may fail silently if write protection is enabled. Furthermore, this is
    /// opting into having two owners of the same data (pin output), as mentioned in
    /// [`enable_output_sync_write`](ConfigureOutputSyncWrite::enable_output_sync_write).
    unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled;
    /// Disables synchronous data output writing for this pin. Waits for `PIO_OWSR` to update
    /// accordingly.
    ///
    /// # Errors
    ///
    /// This function will fail unless write protection is disabled on the PIO controller. Write
    /// protection must be disabled for any pins within the controller to have their configurations
    /// modified.
    fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)>;
    /// Disables synchronous data output writing for this pin without waiting for `PIO_OWSR` to
    /// update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has synchronous output writing disabled,
    /// but this may be at odds with the actual configuration state of the PIO controller. Writes to
    /// the configuration may fail silently if write protection is enabled.
    unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled;
}

/// Marker trait for PIO output configurations.
pub trait OutputWriteCfg {}

impl OutputWriteCfg for Unconfigured {}

/// Drive the output of an I/O line high.
pub struct SetOutput;

impl OutputWriteCfg for SetOutput {}
impl Configured for SetOutput {}

/// Pull the output of an I/O line low.
pub struct ClearOutput;

impl OutputWriteCfg for ClearOutput {}
impl Configured for ClearOutput {}

/// # Output Control (Set/Clear Output Data)
///
/// The level driven on an I/O line can be determined by writing in `PIO_SODR` (Set Output Data
/// Register) and `PIO_CODR` (Clear Output Data Register). These write operations respectively set
/// and clear `PIO_ODSR` (Output Data Status Register), which represents the data driven on the I/O
/// lines. Writing in `PIO_OER` and `PIO_ODR` manages `PIO_OSR` whether the pin is configured to be
/// controlled by the PIO controller or assigned to a peripheral function. This enables
/// configuration of the I/O line prior to setting it to be managed by the PIO Controller.
///
/// Similarly, writing in `PIO_SODR` and `PIO_CODR` affects `PIO_ODSR`. This is important as it
/// defines the first level driven on the I/O line.
pub trait ConfigureOutputWrite {
    type Set: ConfigureOutputWrite;
    type Clear: ConfigureOutputWrite;

    /// Set 1 as the first driven level for this pin I/O line. Waits for `PIO_ODSR` to update
    /// accordingly.
    fn set_output(self) -> Self::Set;
    /// Set 1 as the first driven level for this pin I/O line without waiting for `PIO_ODSR` to
    /// update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has set 1 as its first driven level, but
    /// the first driven level won't be set to 1 until the corresponding bit in `PIO_ODSR` is set.
    unsafe fn set_output_unchecked(self) -> Self::Set;
    /// Set 0 as the first driven level for this pin I/O line. Waits for `PIO_ODSR` to update
    /// accordingly.
    fn clear_output(self) -> Self::Clear;
    /// Set 0 as the first driven level for this pin I/O line without waiting for `PIO_ODSR` to
    /// update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has 0 set as its first driven level, but
    /// the first driven level won't be set to 0 until the corresponding bit in `PIO_ODSR` is
    /// cleared.
    unsafe fn clear_output_unchecked(self) -> Self::Clear;
}

#[cfg(feature = "schmitt")]
/// Marker trait for Schmitt input trigger configuration options.
pub trait SchmittTriggerCfg {}

#[cfg(feature = "schmitt")]
impl SchmittTriggerCfg for Unconfigured {}

#[cfg(feature = "schmitt")]
/// Disables the Schmitt input trigger for this pin.
pub struct SchmittDisabled;

#[cfg(feature = "schmitt")]
impl SchmittTriggerCfg for SchmittDisabled {}

#[cfg(feature = "schmitt")]
/// Enables the Schmitt input trigger for this pin.
pub struct SchmittEnabled;

#[cfg(feature = "schmitt")]
impl SchmittTriggerCfg for SchmittEnabled {}

#[cfg(feature = "schmitt")]
/// # Programmable Schmitt Trigger
///
/// It is possible to configure each input for the Schmitt Trigger. By default the Schmitt trigger
/// is active.
///
/// (Yes, this is actually all of the information given about this functionality in the manuals,
/// sorry about that.)
pub trait ConfigureSchmittTrigger {
    type Disabled;
    type Enabled;

    /// Disable the Schmitt trigger on this I/O line. Waits for `PIO_SCHMITT` to update accordingly.
    fn disable_schmitt_trigger(self) -> Self::Disabled;
    /// Disable the Schmitt trigger on this I/O line without waiting for `PIO_SCHMITT` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has the Schmitt trigger disabled, but the
    /// trigger won't be disabled until the corresponding bit in `PIO_SCHMITT` is set.
    unsafe fn disable_schmitt_trigger_unchecked(self) -> Self::Disabled;
    /// Enable the Schmitt trigger on this I/O line. Waits for `PIO_SCHMITT` to update accordingly.
    fn enable_schmitt_trigger(self) -> Self::Enabled;
    /// Enable the Schmitt trigger on this I/O line without waiting for `PIO_SCHMITT` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has the Schmitt trigger enabled, but the
    /// trigger won't be enabled until the corresponding bit in `PIO_SCHMITT` is clear.
    unsafe fn enable_schmitt_trigger_unchecked(self) -> Self::Enabled;
}

impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigurePioControl
    for Pin<Pio, Pid, Mdvr, Unconfigured, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Peripheral = Pin<Pio, Pid, Mdvr, PeripheralControlled<Unconfigured>, Padr, Irpt, Filt>;
    type Pio = Pin<Pio, Pid, Mdvr, PioControlled<Unconfigured>, Padr, Irpt, Filt>;

    fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            if pioreg._psr().read().bits() & Pid::MASK != 0 {
                let _ = self.peripheral_controlled_unchecked();
                while pioreg._psr().read().bits() & Pid::MASK != 0 {}
            }
            Ok(Pin::new())
        }
    }

    unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
        let pioreg = &*Pio::PTR;
        pioreg._pdr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            if pioreg._psr().read().bits() & Pid::MASK == 0 {
                let _ = self.pio_controlled_unchecked();
                while pioreg._psr().read().bits() & Pid::MASK == 0 {}
            }
            Ok(Pin::new())
        }
    }

    unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
        let pioreg = &*Pio::PTR;
        pioreg._per().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigurePioControl
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<Unconfigured>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Peripheral = Self;
    type Pio = Pin<Pio, Pid, Mdvr, PioControlled<Unconfigured>, Padr, Irpt, Filt>;

    fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
        self
    }

    fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            let _ = self.pio_controlled_unchecked();
            while pioreg._psr().read().bits() & Pid::MASK == 0 {}
            Ok(Pin::new())
        }
    }

    unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
        let pioreg = &*Pio::PTR;
        pioreg._per().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Otpt, Padr, Irpt, Filt> ConfigurePioControl
    for Pin<Pio, Pid, Mdvr, PioControlled<Otpt>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Otpt: OutputCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Peripheral = Pin<Pio, Pid, Mdvr, PeripheralControlled<Unconfigured>, Padr, Irpt, Filt>;
    type Pio = Self;

    fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._locksr().read().bits() & Pid::MASK == 0 {
                return Err((self, PioError::LineLocked));
            }
            let _ = self.peripheral_controlled_unchecked();
            while pioreg._psr().read().bits() & Pid::MASK != 0 {}
            Ok(Pin::new())
        }
    }

    unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
        let pioreg = &*Pio::PTR;
        pioreg._pdr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
        self
    }
}

impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureFunctionSelect
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<Unconfigured>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type A = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralA>, Padr, Irpt, Filt>;
    type B = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralB>, Padr, Irpt, Filt>;
    #[cfg(feature = "3fn")]
    type C = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralC>, Padr, Irpt, Filt>;
    #[cfg(feature = "4fn")]
    type D = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralD>, Padr, Irpt, Filt>;

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_a_unchecked())
    }

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        let pioreg = &*Pio::PTR;
        pioreg._absr().modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_b_unchecked())
    }

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        let pioreg = &*Pio::PTR;
        pioreg._absr().modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_a_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_b_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_c_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_c_unchecked(self) -> Self::C {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d(self) -> Result<Self::D, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_d_unchecked())
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d_unchecked(self) -> Self::D {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureFunctionSelect
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralA>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type A = Self;
    type B = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralB>, Padr, Irpt, Filt>;
    #[cfg(feature = "3fn")]
    type C = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralC>, Padr, Irpt, Filt>;
    #[cfg(feature = "4fn")]
    type D = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralD>, Padr, Irpt, Filt>;

    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        self
    }

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_b_unchecked())
    }

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        let pioreg = &*Pio::PTR;
        pioreg._absr().modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_b_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_c_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_c_unchecked(self) -> Self::C {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d(self) -> Result<Self::D, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_d_unchecked())
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d_unchecked(self) -> Self::D {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureFunctionSelect
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralB>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type A = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralA>, Padr, Irpt, Filt>;
    type B = Self;
    #[cfg(any(feature = "3fn", feature = "4fn"))]
    type C = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralC>, Padr, Irpt, Filt>;
    #[cfg(feature = "4fn")]
    type D = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralD>, Padr, Irpt, Filt>;

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_a_unchecked())
    }

    #[cfg(feature = "2fn")]
    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        let pioreg = &*Pio::PTR;
        pioreg._absr().modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        self
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_a_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_c_unchecked())
    }

    #[cfg(any(feature = "3fn", feature = "4fn"))]
    unsafe fn peripheral_c_unchecked(self) -> Self::C {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d(self) -> Result<Self::D, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_d_unchecked())
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d_unchecked(self) -> Self::D {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }
}

#[cfg(feature = "3fn")]
impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureFunctionSelect
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralC>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type A = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralA>, Padr, Irpt, Filt>;
    type B = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralB>, Padr, Irpt, Filt>;
    type C = Self;
    #[cfg(feature = "4fn")]
    type D = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralD>, Padr, Irpt, Filt>;

    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_a_unchecked())
    }

    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_b_unchecked())
    }

    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn peripheral_c_unchecked(self) -> Self::C {
        self
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d(self) -> Result<Self::D, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_d_unchecked())
    }

    #[cfg(feature = "4fn")]
    unsafe fn peripheral_d_unchecked(self) -> Self::D {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }
}

#[cfg(feature = "4fn")]
impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureFunctionSelect
    for Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralD>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type A = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralA>, Padr, Irpt, Filt>;
    type B = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralB>, Padr, Irpt, Filt>;
    type C = Pin<Pio, Pid, Mdvr, PeripheralControlled<PeripheralC>, Padr, Irpt, Filt>;
    type D = Self;

    unsafe fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_a_unchecked())
    }

    unsafe fn peripheral_a_unchecked(self) -> Self::A {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    unsafe fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_b_unchecked())
    }

    unsafe fn peripheral_b_unchecked(self) -> Self::B {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        Pin::new()
    }

    unsafe fn peripheral_c(self) -> Result<Self::C, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._locksr().read().bits() & Pid::MASK == 0 {
            return Err((self, PioError::LineLocked));
        }
        Ok(self.peripheral_c_unchecked())
    }

    unsafe fn peripheral_c_unchecked(self) -> Self::C {
        let pioreg = &*Pio::PTR;
        pioreg
            ._abcdsr0()
            .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
        pioreg
            ._abcdsr1()
            .modify(|r, w| w.bits(r.bits() | Pid::MASK));
        Pin::new()
    }

    unsafe fn peripheral_d(self) -> Result<Self::D, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn peripheral_d_unchecked(self) -> Self::D {
        self
    }
}

#[cfg(not(feature = "schmitt"))]
const _: () = {
    impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigurePioOutput
        for Pin<Pio, Pid, Mdvr, PioControlled<Unconfigured>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled>, Padr, Irpt, Filt>;
        type Enabled = Pin<
            Pio,
            Pid,
            Mdvr,
            PioControlled<OutputEnabled<Unconfigured, Unconfigured>>,
            Padr,
            Irpt,
            Filt,
        >;

        fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._osr().read().bits() & Pid::MASK != 0 {
                    let _ = self.disable_pio_output_unchecked();
                    while pioreg._osr().read().bits() & Pid::MASK != 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._odr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._osr().read().bits() & Pid::MASK == 0 {
                    let _ = self.enable_pio_output_unchecked();
                    while pioreg._osr().read().bits() & Pid::MASK == 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg._oer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigurePioOutput
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Self;
        type Enabled = Pin<
            Pio,
            Pid,
            Mdvr,
            PioControlled<OutputEnabled<Unconfigured, Unconfigured>>,
            Padr,
            Irpt,
            Filt,
        >;

        fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
            self
        }

        fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                let _ = self.enable_pio_output_unchecked();
                while pioreg._osr().read().bits() & Pid::MASK == 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg._oer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Sync, Outw, Padr, Irpt, Filt> ConfigurePioOutput
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, Outw>>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Sync: OutputSyncWriteCfg,
        Outw: OutputWriteCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled>, Padr, Irpt, Filt>;
        type Enabled = Self;

        fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                let _ = self.disable_pio_output_unchecked();
                while pioreg._osr().read().bits() & Pid::MASK != 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._odr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
            self
        }
    }
};

#[cfg(feature = "schmitt")]
const _: () = {
    impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigurePioOutput
        for Pin<Pio, Pid, Mdvr, PioControlled<Unconfigured>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled =
            Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<Unconfigured>>, Padr, Irpt, Filt>;
        type Enabled = Pin<
            Pio,
            Pid,
            Mdvr,
            PioControlled<OutputEnabled<Unconfigured, Unconfigured>>,
            Padr,
            Irpt,
            Filt,
        >;

        fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._osr().read().bits() & Pid::MASK != 0 {
                    let _ = self.disable_pio_output_unchecked();
                    while pioreg._osr().read().bits() & Pid::MASK != 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._odr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                if pioreg._osr().read().bits() & Pid::MASK == 0 {
                    let _ = self.enable_pio_output_unchecked();
                    while pioreg._osr().read().bits() & Pid::MASK == 0 {}
                }
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg._oer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Schm, Padr, Irpt, Filt> ConfigurePioOutput
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<Schm>>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Schm: SchmittTriggerCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Self;
        type Enabled = Pin<
            Pio,
            Pid,
            Mdvr,
            PioControlled<OutputEnabled<Unconfigured, Unconfigured>>,
            Padr,
            Irpt,
            Filt,
        >;

        fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
            self
        }

        fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                let _ = self.enable_pio_output_unchecked();
                while pioreg._osr().read().bits() & Pid::MASK == 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg._oer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Sync, Outw, Padr, Irpt, Filt> ConfigurePioOutput
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, Outw>>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pio::Rb: WriteProtect,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Sync: OutputSyncWriteCfg,
        Outw: OutputWriteCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled =
            Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<Unconfigured>>, Padr, Irpt, Filt>;
        type Enabled = Self;

        fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
            unsafe {
                let pioreg = &*Pio::PTR;
                if Pio::Rb::writeprotect_enabled(pioreg) {
                    return Err((self, PioError::WriteProtected));
                }
                let _ = self.disable_pio_output_unchecked();
                while pioreg._osr().read().bits() & Pid::MASK != 0 {}
                Ok(Pin::new())
            }
        }

        unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg._odr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
            Ok(self)
        }

        unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
            self
        }
    }

    impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureSchmittTrigger
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<Unconfigured>>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled =
            Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<SchmittDisabled>>, Padr, Irpt, Filt>;
        type Enabled =
            Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<SchmittEnabled>>, Padr, Irpt, Filt>;

        fn disable_schmitt_trigger(self) -> Self::Disabled {
            unsafe {
                let pioreg = &*Pio::PTR;
                if pioreg._schmitt().read().bits() & Pid::MASK == 0 {
                    let _ = self.disable_schmitt_trigger_unchecked();
                    while pioreg._schmitt().read().bits() & Pid::MASK == 0 {}
                }
                Pin::new()
            }
        }

        unsafe fn disable_schmitt_trigger_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg
                ._schmitt()
                .modify(|r, w| w.bits(r.bits() | Pid::MASK));
            Pin::new()
        }

        fn enable_schmitt_trigger(self) -> Self::Enabled {
            unsafe {
                let pioreg = &*Pio::PTR;
                if pioreg._schmitt().read().bits() & Pid::MASK != 0 {
                    let _ = self.enable_schmitt_trigger_unchecked();
                    while pioreg._schmitt().read().bits() & Pid::MASK != 0 {}
                }
                Pin::new()
            }
        }

        unsafe fn enable_schmitt_trigger_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg
                ._schmitt()
                .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureSchmittTrigger
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<SchmittDisabled>>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled = Self;
        type Enabled =
            Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<SchmittEnabled>>, Padr, Irpt, Filt>;

        fn disable_schmitt_trigger(self) -> Self::Disabled {
            self
        }

        unsafe fn disable_schmitt_trigger_unchecked(self) -> Self::Disabled {
            self
        }

        fn enable_schmitt_trigger(self) -> Self::Enabled {
            unsafe {
                let pioreg = &*Pio::PTR;
                let _ = self.enable_schmitt_trigger_unchecked();
                while pioreg._schmitt().read().bits() & Pid::MASK != 0 {}
                Pin::new()
            }
        }

        unsafe fn enable_schmitt_trigger_unchecked(self) -> Self::Enabled {
            let pioreg = &*Pio::PTR;
            pioreg
                ._schmitt()
                .modify(|r, w| w.bits(r.bits() & !Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Padr, Irpt, Filt> ConfigureSchmittTrigger
        for Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<SchmittEnabled>>, Padr, Irpt, Filt>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
        Filt: InputFilterCfg,
    {
        type Disabled =
            Pin<Pio, Pid, Mdvr, PioControlled<OutputDisabled<SchmittDisabled>>, Padr, Irpt, Filt>;
        type Enabled = Self;

        fn disable_schmitt_trigger(self) -> Self::Disabled {
            unsafe {
                let pioreg = &*Pio::PTR;
                let _ = self.disable_schmitt_trigger_unchecked();
                while pioreg._schmitt().read().bits() & Pid::MASK == 0 {}
                Pin::new()
            }
        }

        unsafe fn disable_schmitt_trigger_unchecked(self) -> Self::Disabled {
            let pioreg = &*Pio::PTR;
            pioreg
                ._schmitt()
                .modify(|r, w| w.bits(r.bits() | Pid::MASK));
            Pin::new()
        }

        fn enable_schmitt_trigger(self) -> Self::Enabled {
            self
        }

        unsafe fn enable_schmitt_trigger_unchecked(self) -> Self::Enabled {
            self
        }
    }
};

impl<Pio, Pid, Mdvr, Outw, Padr, Irpt, Filt> ConfigureOutputSyncWrite
    for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Unconfigured, Outw>>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Outw: OutputWriteCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Disabled = Pin<
        Pio,
        Pid,
        Mdvr,
        PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>,
        Padr,
        Irpt,
        Filt,
    >;
    type Enabled = Pin<
        Pio,
        Pid,
        Mdvr,
        PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>,
        Padr,
        Irpt,
        Filt,
    >;

    fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            if pioreg._owsr().read().bits() & Pid::MASK != 0 {
                let _ = self.disable_output_sync_write_unchecked();
                while pioreg._owsr().read().bits() & Pid::MASK != 0 {}
            }
            Ok(Pin::new())
        }
    }

    unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
        let pioreg = &*Pio::PTR;
        pioreg._owdr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        if pioreg._owsr().read().bits() & Pid::MASK == 0 {
            let _ = self.enable_output_sync_write_unchecked();
            while pioreg._owsr().read().bits() & Pid::MASK == 0 {}
        }
        Ok(Pin::new())
    }

    unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
        let pioreg = &*Pio::PTR;
        pioreg._ower().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Outw, Padr, Irpt, Filt> ConfigureOutputSyncWrite
    for Pin<
        Pio,
        Pid,
        Mdvr,
        PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>,
        Padr,
        Irpt,
        Filt,
    >
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Outw: OutputWriteCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Disabled = Self;
    type Enabled = Pin<
        Pio,
        Pid,
        Mdvr,
        PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>,
        Padr,
        Irpt,
        Filt,
    >;

    fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
        self
    }

    unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
        let pioreg = &*Pio::PTR;
        if Pio::Rb::writeprotect_enabled(pioreg) {
            return Err((self, PioError::WriteProtected));
        }
        let _ = self.enable_output_sync_write_unchecked();
        while pioreg._owsr().read().bits() & Pid::MASK == 0 {}
        Ok(Pin::new())
    }

    unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
        let pioreg = &*Pio::PTR;
        pioreg._ower().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Outw, Padr, Irpt, Filt> ConfigureOutputSyncWrite
    for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pio::Rb: WriteProtect,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Outw: OutputWriteCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Disabled = Pin<
        Pio,
        Pid,
        Mdvr,
        PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>,
        Padr,
        Irpt,
        Filt,
    >;
    type Enabled = Self;

    fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
        unsafe {
            let pioreg = &*Pio::PTR;
            if Pio::Rb::writeprotect_enabled(pioreg) {
                return Err((self, PioError::WriteProtected));
            }
            let _ = self.disable_output_sync_write_unchecked();
            while pioreg._owsr().read().bits() & Pid::MASK != 0 {}
            Ok(Pin::new())
        }
    }

    unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
        let pioreg = &*Pio::PTR;
        pioreg._owdr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
        Ok(self)
    }

    unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
        self
    }
}

impl<Pio, Pid, Mdvr, Sync, Padr, Irpt, Filt> ConfigureOutputWrite
    for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, Unconfigured>>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Sync: OutputSyncWriteCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Clear =
        Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, ClearOutput>>, Padr, Irpt, Filt>;
    type Set = Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, SetOutput>>, Padr, Irpt, Filt>;

    fn clear_output(self) -> Self::Clear {
        unsafe {
            let pioreg = &*Pio::PTR;
            if pioreg._odsr().read().bits() & Pid::MASK != 0 {
                let _ = self.clear_output_unchecked();
                while pioreg._odsr().read().bits() & Pid::MASK != 0 {}
            }
            Pin::new()
        }
    }

    unsafe fn clear_output_unchecked(self) -> Self::Clear {
        let pioreg = &*Pio::PTR;
        pioreg._codr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn set_output(self) -> Self::Set {
        unsafe {
            let pioreg = &*Pio::PTR;
            if pioreg._odsr().read().bits() & Pid::MASK == 0 {
                let _ = self.set_output_unchecked();
                while pioreg._odsr().read().bits() & Pid::MASK == 0 {}
            }
            Pin::new()
        }
    }

    unsafe fn set_output_unchecked(self) -> Self::Set {
        let pioreg = &*Pio::PTR;
        pioreg._sodr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Sync, Padr, Irpt, Filt> ConfigureOutputWrite
    for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, ClearOutput>>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Sync: OutputSyncWriteCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Clear = Self;
    type Set = Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, SetOutput>>, Padr, Irpt, Filt>;

    fn clear_output(self) -> Self::Clear {
        self
    }

    unsafe fn clear_output_unchecked(self) -> Self::Clear {
        self
    }

    fn set_output(self) -> Self::Set {
        unsafe {
            let pioreg = &*Pio::PTR;
            let _ = self.set_output_unchecked();
            while pioreg._odsr().read().bits() & Pid::MASK == 0 {}
            Pin::new()
        }
    }

    unsafe fn set_output_unchecked(self) -> Self::Set {
        let pioreg = &*Pio::PTR;
        pioreg._sodr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Sync, Padr, Irpt, Filt> ConfigureOutputWrite
    for Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, SetOutput>>, Padr, Irpt, Filt>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Sync: OutputSyncWriteCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    type Clear =
        Pin<Pio, Pid, Mdvr, PioControlled<OutputEnabled<Sync, ClearOutput>>, Padr, Irpt, Filt>;
    type Set = Self;

    fn clear_output(self) -> Self::Clear {
        unsafe {
            let pioreg = &*Pio::PTR;
            let _ = self.clear_output_unchecked();
            while pioreg._odsr().read().bits() & Pid::MASK != 0 {}
            Pin::new()
        }
    }

    unsafe fn clear_output_unchecked(self) -> Self::Clear {
        let pioreg = &*Pio::PTR;
        pioreg._codr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn set_output(self) -> Self::Set {
        self
    }

    unsafe fn set_output_unchecked(self) -> Self::Set {
        self
    }
}
