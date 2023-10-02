//! PIO peripheral configuration
//!
//!

use crate::pio::{
    filter::InputFilterCfg,
    interrupt::InterruptCfg,
    pin::{Configured, Pin, PinId, PullupResistorCfg, Unconfigured},
    pioa::PioA,
    piob::PioB,
    IsPio, PioError,
};
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use crate::pio::{pioc::PioC, piod::PioD};
#[cfg(feature = "sam3x8h")]
use crate::pio::{pioe::PioE, piof::PioF};
use core::marker::PhantomData;

pub trait MultiDriverCfg {}

impl MultiDriverCfg for Unconfigured {}

/// Disable multidrive on this PIO line.
pub struct MultiDriverDisabled<Line: PioControl> {
    _line: PhantomData<Line>,
}

impl<Line: PioControl> MultiDriverCfg for MultiDriverDisabled<Line> {}
impl<Line: PioControl + Configured> Configured for MultiDriverDisabled<Line> {}

/// Enable multidrive on this PIO line. When configured in this state, drivers should only drive the
/// line low. Additionally, a pull-up resistor is generally required to ensure that the line can
/// achieve a high level.
pub struct MultiDriverEnabled<Psel: PioControl, Otpt: OutputCfg> {
    _psel: PhantomData<Psel>,
    _otpt: PhantomData<Otpt>,
}

impl<Line: PioControl, Otpt: OutputCfg> MultiDriverCfg for MultiDriverEnabled<Line, Otpt> {}
impl<Line, Otpt> Configured for MultiDriverEnabled<Line, Otpt>
where
    Line: PioControl + Configured,
    Otpt: OutputCfg + Configured,
{
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
pub trait ConfigureMultiDriver {
    type Enabled: ConfigureMultiDriver;
    type Disabled: ConfigureMultiDriver;

    /// Enable multi drive control of this pin. Waits for `PIO_MDSR` to update accordingly.
    fn enable_multi_driver(self) -> Self::Enabled;
    /// Enable multi-drive control of this pin without waiting for `PIO_MDSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that multi drive of this pin is enabled, but multi
    /// drive on this pin isn't actually enabled until the corresponding bit in `PIO_MDSR` is set.
    unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled;
    /// Disable multi drive control of this pin. Waits for `PIO_MDSR` to update accordingly.
    fn disable_multi_driver(self) -> Self::Disabled;
    /// Disable multi-drive control of this pin without waiting for `PIO_MDSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that multi drive of this pin is disabled, but multi
    /// drive on this pin isn't actually disabled until the corresponding bit in `PIO_MDSR` is
    /// cleared.
    unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled;
}

pub trait PioControl {}

impl PioControl for Unconfigured {}

#[allow(clippy::module_name_repetitions)]
/// Allow the peripheral to control this I/O line.
pub struct PeripheralControlled<Psel: PeripheralSelectCfg> {
    _psel: PhantomData<Psel>,
}

impl<Psel: PeripheralSelectCfg> PioControl for PeripheralControlled<Psel> {}
impl<Psel: PeripheralSelectCfg + Configured> Configured for PeripheralControlled<Psel> {}

/// Allow the PIO controller to control this I/O.
pub struct PioControlled<Otpt: OutputCfg> {
    _otpt: PhantomData<Otpt>,
}

impl<Otpt: OutputCfg> PioControl for PioControlled<Otpt> {}
impl<Otpt: OutputCfg + Configured> Configured for PioControlled<Otpt> {}

/// # I/O Line or Peripheral Function Selection
///
/// When a pin is multiplexed with one or two peripheral functions, the selection is controlled with
/// the registers `PIO_PER` (PIO Enable Register) and `PIO_PDR` (PIO Disable Register). The register
/// `PIO_PSR` (PIO Status Register) is the result of the set and clear registers and indicates
/// whether the pin is controlled by the corresponding peripheral or by the PIO Controller. A value
/// of 0 indicates the pin is controlled by the corresponding on-chip peripheral selected in the
/// `PIO_ABSR` (AB Select Register). A balue of 1 indicates the pin is controlled by the PIO
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

/// # Peripheral A or B Selection
///
/// The PIO Controller provides multiplexing of up to two peripheral functions on a single pin. The
/// selection is performed by writing `PIO_ABSR` (AB Select Register). For each pin, the
/// corresponding bit at level 0 means peripheral A is selected whereas the corresponding bit at
/// level 1 indicates that peripheral B is selected.
///
/// Note that multiplexing of peripheral lines A and B only affects the output line. The peripheral
/// input lines are always connected to the pin input.
///
/// After reset, `PIO_ABSR` is 0, thus indicating that all the PIO lines are configured on
/// peripheral A. However, peripheral A generally does not drive the pin as the PIO Controller
/// resets in I/O line mode.
///
/// Writing in `PIO_ABSR` manages the multiplexing regardless of the configuration of the pin.
/// However, assignment of a pin to a peripheral function requires a write in the peripheral
/// selection register (`PIO_ABSR`) in addition to a write in `PIO_PDR`.
pub trait ConfigureABSelect: Sized {
    type A: ConfigureABSelect;
    type B: ConfigureABSelect;

    /// Select peripheral A for this pin. Waits for `PIO_ABSR` to update accordingly.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn peripheral_a(self) -> Result<Self::A, (Self, PioError)>;
    /// Select peripheral A for this pin without waiting for `PIO_ABSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral A, but this may be at
    /// odds with the actual configuration state of the PIO controller. Writes to the configuration
    /// may fail silently if the PIO line is locked or write protection is enabled.
    unsafe fn peripheral_a_unchecked(self) -> Self::A;
    /// Enable select peripheral B for this pin. Waits for `PIO_ABSR` to update accordingly.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - This pin's bit is set in `PIO_LOCKSR`, denoting that a peripheral has locked its
    ///   configuration, and it cannot be changed until a hardware reset is given to the PIO
    ///   controller.
    /// - Write protection is enabled on the PIO controller. Write protection must first be
    ///   disabled for any pins within the controller to have their configurations modified.
    fn peripheral_b(self) -> Result<Self::B, (Self, PioError)>;
    /// Select peripheral B for this pin without waiting for `PIO_ABSR` to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin selects peripheral B, but this may be at
    /// odds with the actual configuration state of the PIO controller. Writes to the configuration
    /// may fail silently if the PIO line is locked or write protection is enabled.
    unsafe fn peripheral_b_unchecked(self) -> Self::B;
}

pub trait OutputCfg {}

impl OutputCfg for Unconfigured {}

/// Disable PIO control of the output.
pub struct OutputDisabled;

impl OutputCfg for OutputDisabled {}
impl Configured for OutputDisabled {}

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

/// # Output Control
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

macro_rules! impl_peripheral_cfgs {
    ($($pio:ty),+$(,)?) => {
        $(
            impl<Pid, Pupr, Irpt, Filt> ConfigureMultiDriver for
                Pin<$pio, Pid, Unconfigured, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<Unconfigured, Unconfigured>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Pin<$pio, Pid, MultiDriverDisabled<Unconfigured>, Pupr, Irpt, Filt>;

                fn enable_multi_driver(
                    self,
                ) -> Self::Enabled {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        if pioreg.mdsr.read().bits() & <Pid as PinId>::MASK == 0 {
                            let _ = self.enable_multi_driver_unchecked();
                            while pioreg.mdsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        }
                        Pin::new()
                    }
                }

                unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.mder.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_multi_driver(self) -> Self::Disabled {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        if pioreg.mdsr.read().bits() & <Pid as PinId>::MASK > 0 {
                            let _ = self.disable_multi_driver_unchecked();
                            while pioreg.mdsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                        }
                        Pin::new()
                    }
                }

                unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.mddr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Line, Pupr, Irpt, Filt> ConfigureMultiDriver for
                Pin<$pio, Pid, MultiDriverDisabled<Line>, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Line: PioControl,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<Line, Unconfigured>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Self;

                fn enable_multi_driver(
                    self,
                ) -> Self::Enabled {
                    unsafe {
                        let _ = self.enable_multi_driver_unchecked();
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        while pioreg.mdsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        Pin::new()
                    }
                }

                unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.mder.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_multi_driver(self) -> Self::Disabled {
                    self
                }

                unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled {
                    self
                }
            }

            impl<Pid, Psel, Otpt, Pupr, Irpt, Filt> ConfigureMultiDriver for
                Pin<$pio, Pid, MultiDriverEnabled<Psel, Otpt>, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Psel: PioControl,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Self;
                type Disabled = Pin<$pio, Pid, MultiDriverDisabled<Psel>, Pupr, Irpt, Filt>;

                fn enable_multi_driver(self) -> Self::Enabled {
                    self
                }

                unsafe fn enable_multi_driver_unchecked(self) -> Self::Enabled {
                    self
                }

                fn disable_multi_driver(self) -> Self::Disabled {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        let _ = self.disable_multi_driver_unchecked();
                        while pioreg.mdsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                        Pin::new()
                    }
                }

                unsafe fn disable_multi_driver_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.mddr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Pupr, Irpt, Filt> ConfigurePioOutput for
                Pin<$pio, Pid, MultiDriverDisabled<PioControlled<Unconfigured>>, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Unconfigured, Unconfigured>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputDisabled>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.osr.read().bits() & <Pid as PinId>::MASK == 0 {
                            unsafe {
                                let _ = self.enable_pio_output_unchecked();
                                while pioreg.osr.read().bits() & <Pid as PinId>::MASK == 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.oer.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.osr.read().bits() & <Pid as PinId>::MASK > 0 {
                            unsafe {
                                let _ = self.disable_pio_output_unchecked();
                                while pioreg.osr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.odr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Pupr, Irpt, Filt> ConfigurePioOutput for
                Pin<$pio, Pid, MultiDriverDisabled<PioControlled<OutputDisabled>>, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Unconfigured, Unconfigured>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Self;

                fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.enable_pio_output_unchecked();
                            while pioreg.osr.read().bits() & <Pid as PinId>::MASK == 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.oer.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
                    self
                }
            }

            impl<Pid, Sync, Outw, Pupr, Irpt, Filt> ConfigurePioOutput for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Sync: OutputSyncWriteCfg,
                Outw: OutputWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Self;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputDisabled>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
                    self
                }

                fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.disable_pio_output_unchecked();
                            while pioreg.osr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.odr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Otpt, Pupr, Irpt, Filt> ConfigurePioOutput for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<
                        PioControlled<OutputEnabled<Unconfigured, Unconfigured>>,
                        Otpt,
                    >,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputDisabled>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.osr.read().bits() & <Pid as PinId>::MASK == 0 {
                            unsafe {
                                let _ = self.enable_pio_output_unchecked();
                                while pioreg.osr.read().bits() & <Pid as PinId>::MASK == 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.oer.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.osr.read().bits() & <Pid as PinId>::MASK > 0 {
                            unsafe {
                                let _ = self.disable_pio_output_unchecked();
                                while pioreg.osr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.odr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Otpt, Pupr, Irpt, Filt> ConfigurePioOutput for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputDisabled>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<
                        PioControlled<OutputEnabled<Unconfigured, Unconfigured>>,
                        Otpt,
                    >,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Self;

                fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.enable_pio_output_unchecked();
                            while pioreg.osr.read().bits() & <Pid as PinId>::MASK == 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.oer.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
                    self
                }
            }

            impl<Pid, Sync, Outw, Otpt, Pupr, Irpt, Filt> ConfigurePioOutput for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputEnabled<Sync, Outw>>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Sync: OutputSyncWriteCfg,
                Outw: OutputWriteCfg,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Self;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputDisabled>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn enable_pio_output(self) -> Result<Self::Enabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn enable_pio_output_unchecked(self) -> Self::Enabled {
                    self
                }

                fn disable_pio_output(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.disable_pio_output_unchecked();
                            while pioreg.osr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn disable_pio_output_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.odr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Outw, Pupr, Irpt, Filt> ConfigureOutputSyncWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Unconfigured, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Outw: OutputWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = &*<$pio>::PTR;
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.owsr.read().bits() & <Pid as PinId>::MASK == 0 {
                            let _ = self.enable_output_sync_write_unchecked();
                            while pioreg.owsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        }
                        Ok(Pin::new())
                    }
                }

                unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.ower.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.owsr.read().bits() & <Pid as PinId>::MASK > 0 {
                            unsafe {
                                let _ = self.disable_output_sync_write_unchecked();
                                while pioreg.owsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.owdr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Outw, Pupr, Irpt, Filt> ConfigureOutputSyncWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Outw: OutputWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Self;

                unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = &*<$pio>::PTR;
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        let _ = self.enable_output_sync_write_unchecked();
                        while pioreg.owsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        Ok(Pin::new())
                    }
                }

                unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.ower.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
                    self
                }
            }

            impl<Pid, Outw, Pupr, Irpt, Filt> ConfigureOutputSyncWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Outw: OutputWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Self;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
                    self
                }

                fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.disable_output_sync_write_unchecked();
                            while pioreg.owsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.owdr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Outw, Otpt, Pupr, Irpt, Filt> ConfigureOutputSyncWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputEnabled<Unconfigured, Outw>>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Outw: OutputWriteCfg,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<
                        PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>,
                        Otpt,
                    >,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = &*<$pio>::PTR;
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.owsr.read().bits() & <Pid as PinId>::MASK == 0 {
                            let _ = self.enable_output_sync_write_unchecked();
                            while pioreg.owsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        }
                        Ok(Pin::new())
                    }
                }

                unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.ower.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.owsr.read().bits() & <Pid as PinId>::MASK > 0 {
                            unsafe {
                                let _ = self.disable_output_sync_write_unchecked();
                                while pioreg.owsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.owdr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Outw, Otpt, Pupr, Irpt, Filt> ConfigureOutputSyncWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<
                        PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>,
                        Otpt,
                    >,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Outw: OutputWriteCfg,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Disabled = Self;

                unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
                    let pioreg = &*<$pio>::PTR;
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        let _ = self.enable_output_sync_write_unchecked();
                        while pioreg.owsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        Ok(Pin::new())
                    }
                }

                unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.ower.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }

                fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
                    self
                }
            }

            impl<Pid, Outw, Otpt, Pupr, Irpt, Filt> ConfigureOutputSyncWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<PioControlled<OutputEnabled<SyncOutputEnabled, Outw>>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Outw: OutputWriteCfg,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Enabled = Self;
                type Disabled = Pin<
                    $pio,
                    Pid,
                    MultiDriverEnabled<
                        PioControlled<OutputEnabled<SyncOutputDisabled, Outw>>,
                        Otpt,
                    >,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                unsafe fn enable_output_sync_write(self) -> Result<Self::Enabled, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn enable_output_sync_write_unchecked(self) -> Self::Enabled {
                    self
                }

                fn disable_output_sync_write(self) -> Result<Self::Disabled, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.disable_output_sync_write_unchecked();
                            while pioreg.owsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn disable_output_sync_write_unchecked(self) -> Self::Disabled {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.owdr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }

            impl<Pid, Sync, Pupr, Irpt, Filt> ConfigureOutputWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, Unconfigured>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Sync: OutputSyncWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Set = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, SetOutput>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Clear = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, ClearOutput>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn set_output(self) -> Self::Set {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        if pioreg.odsr.read().bits() & <Pid as PinId>::MASK == 0 {
                            pioreg.sodr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                            while pioreg.odsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        }
                        Pin::new()
                    }
                }

                unsafe fn set_output_unchecked(self) -> Self::Set {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.sodr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    unsafe { Pin::new() }
                }

                fn clear_output(self) -> Self::Clear {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        if pioreg.odsr.read().bits() & <Pid as PinId>::MASK > 0 {
                            pioreg.codr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                            while pioreg.odsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                        }
                        Pin::new()
                    }
                }

                unsafe fn clear_output_unchecked(self) -> Self::Clear {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.codr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    unsafe { Pin::new() }
                }
            }

            impl<Pid, Sync, Pupr, Irpt, Filt> ConfigureOutputWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, SetOutput>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Sync: OutputSyncWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Set = Self;
                type Clear = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, ClearOutput>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn set_output(self) -> Self::Set {
                    self
                }

                unsafe fn set_output_unchecked(self) -> Self::Set {
                    self
                }

                fn clear_output(self) -> Self::Clear {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        pioreg.codr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                        while pioreg.odsr.read().bits() & <Pid as PinId>::MASK > 0 {}
                        Pin::new()
                    }
                }

                unsafe fn clear_output_unchecked(self) -> Self::Clear {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.codr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    unsafe { Pin::new() }
                }
            }

            impl<Pid, Sync, Pupr, Irpt, Filt> ConfigureOutputWrite for
                Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, ClearOutput>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pid: PinId<Controller = $pio>,
                Sync: OutputSyncWriteCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Set = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<OutputEnabled<Sync, SetOutput>>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Clear = Self;

                fn set_output(self) -> Self::Set {
                    unsafe {
                        let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                        pioreg.sodr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                        while pioreg.odsr.read().bits() & <Pid as PinId>::MASK == 0 {}
                        Pin::new()
                    }
                }

                unsafe fn set_output_unchecked(self) -> Self::Set {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.sodr.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    unsafe { Pin::new() }
                }

                fn clear_output(self) -> Self::Clear {
                    self
                }

                unsafe fn clear_output_unchecked(self) -> Self::Clear {
                    self
                }
            }
        )+
    };
}

impl_peripheral_cfgs! {
    PioA, PioB,
}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_peripheral_cfgs! {
    PioC, PioD,
}

#[cfg(feature = "sam3x8h")]
impl_peripheral_cfgs! {
    PioE, PioF,
}

macro_rules! impl_peripheral_absel {
    ($pio:ty { $($pid:ty: $opt:tt),+$(,)? }) => {
        $(
            impl_peripheral_absel! {
                @impl $pio, $pid, $opt
            }
        )+
    };
    (@impl $pio:ty, $pid:ty, noab) => {
        const _: () = {
            use crate::pio::{
                filter::InputFilterCfg,
                interrupt::InterruptCfg,
                peripheral::{
                    ConfigurePioControl,
                    MultiDriverDisabled,
                    MultiDriverEnabled,
                    OutputCfg,
                    PioControlled,
                },
                pin::{Pin, PullupResistorCfg, Unconfigured},
                PioError,
            };

            impl<Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverDisabled<Unconfigured>, Pupr, Irpt, Filt>
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PioControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Self;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    unsafe { Ok(Pin::new()) }
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    Pin::new()
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverDisabled<PioControlled<Otpt>>, Pupr, Irpt, Filt>
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Self;
                type Peripheral = Self;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    self
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverEnabled<Unconfigured, Otpt>, Pupr, Irpt, Filt>
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PioControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Self;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    unsafe { Ok(Pin::new()) }
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    Pin::new()
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    self
                }
            }

            impl<Otpt1, Otpt2, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverEnabled<PioControlled<Otpt1>, Otpt2>, Pupr, Irpt, Filt>
            where
                Otpt1: OutputCfg,
                Otpt2: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Self;
                type Peripheral = Self;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    self
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    self
                }
            }
        };
    };
    (@impl $pio:ty, $pid:ty, asel) => {
        impl_peripheral_absel! {
            @peripheral $pio, $pid
        }

        const _: () = {
            use crate::pio::{
                filter::InputFilterCfg,
                interrupt::InterruptCfg,
                peripheral::{
                    ConfigureABSelect,
                    MultiDriverDisabled,
                    MultiDriverEnabled,
                    OutputCfg,
                    PeripheralA,
                    PeripheralControlled,
                },
                pin::{Pin, PinId, PullupResistorCfg, Unconfigured},
                IsPio,
                PioError,
            };

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralA>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK > 0 {
                            unsafe {
                                let _ = self.peripheral_a_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    const INV: u32 = !<$pid>::MASK;

                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() & INV));
                    Pin::new()
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralA>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralA>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK > 0 {
                            unsafe {
                                let _ = self.peripheral_a_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    const INV: u32 = !<$pid>::MASK;

                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() & INV));
                    Pin::new()
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralA>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }
        };
    };
    (@impl $pio:ty, $pid:ty, bsel) => {
        impl_peripheral_absel! {
            @peripheral $pio, $pid
        }

        const _: () = {
            use crate::pio::{
                filter::InputFilterCfg,
                interrupt::InterruptCfg,
                peripheral::{
                    ConfigureABSelect,
                    MultiDriverDisabled,
                    MultiDriverEnabled,
                    OutputCfg,
                    PeripheralB,
                    PeripheralControlled,
                },
                pin::{Unconfigured, Pin, PinId, PullupResistorCfg},
                IsPio,
                PioError,
            };

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralB>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK == 0 {
                            unsafe {
                                let _ = self.peripheral_b_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() | <$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralB>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralB>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK == 0 {
                            unsafe {
                                let _ = self.peripheral_b_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() | <$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralB>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }
        };
    };
    (@impl $pio:ty, $pid:ty, absel) => {
        impl_peripheral_absel! {
            @peripheral $pio, $pid
        }

        const _: () = {
            use crate::pio::{
                filter::InputFilterCfg,
                interrupt::InterruptCfg,
                peripheral::{
                    ConfigureABSelect,
                    MultiDriverDisabled,
                    MultiDriverEnabled,
                    OutputCfg,
                    PeripheralA,
                    PeripheralB,
                    PeripheralControlled,
                },
                pin::{Pin, PinId, PullupResistorCfg, Unconfigured},
                IsPio,
                PioError,
            };

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralA>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type B = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralB>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK > 0 {
                            unsafe {
                                let _ = self.peripheral_a_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    const INV: u32 = !<$pid>::MASK;

                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() & INV));
                    Pin::new()
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK == 0 {
                            unsafe {
                                let _ = self.peripheral_b_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() | <$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralA>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralB>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe { Ok(self.peripheral_b_unchecked()) }
                    }
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() | <$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralB>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<PeripheralA>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe { Ok(self.peripheral_a_unchecked()) }
                    }
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    const INV: u32 = !<$pid>::MASK;

                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() & INV));
                    Pin::new()
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralA>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type B = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralB>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK > 0 {
                            unsafe {
                                let _ = self.peripheral_a_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    const INV: u32 = !<$pid>::MASK;

                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() & INV));
                    Pin::new()
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.absr.read().bits() & <$pid>::MASK == 0 {
                            unsafe {
                                let _ = self.peripheral_b_unchecked();
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() | <$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralA>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Self;
                type B = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralB>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    self
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe { Ok(self.peripheral_b_unchecked()) }
                    }
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() | <$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigureABSelect
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralB>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type A = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<PeripheralA>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type B = Self;

                fn peripheral_a(self) -> Result<Self::A, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe { Ok(self.peripheral_a_unchecked()) }
                    }
                }

                unsafe fn peripheral_a_unchecked(self) -> Self::A {
                    const INV: u32 = !<$pid>::MASK;

                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.absr.modify(|r, w| w.bits(r.bits() & INV));
                    Pin::new()
                }

                fn peripheral_b(self) -> Result<Self::B, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_b_unchecked(self) -> Self::B {
                    self
                }
            }
        };
    };
    (@impl $pio:ty, $pid:ty, $unk:tt) => {
        compile_error!(concat!(
            "impl_peripheral_absel: @impl internal rule got unexpected option: `",
            stringify!($unk),
            "`",
        ));
    };
    (@peripheral $pio:ty, $pid:ty) => {
        const _: () = {
            use crate::pio::{
                filter::InputFilterCfg,
                interrupt::InterruptCfg,
                peripheral::{
                    ConfigurePioControl,
                    MultiDriverDisabled,
                    MultiDriverEnabled,
                    OutputCfg,
                    PeripheralControlled,
                    PeripheralSelectCfg,
                    PioControlled,
                },
                pin::{Pin, PinId, PullupResistorCfg, Unconfigured},
                IsPio,
                PioError,
            };

            impl<Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverDisabled<Unconfigured>, Pupr, Irpt, Filt>
            where
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PioControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.psr.read().bits() & <$pid>::MASK == 0 {
                            unsafe {
                                let _ = self.pio_controlled_unchecked();
                                while pioreg.psr.read().bits() & <$pid>::MASK == 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.per.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.psr.read().bits() & <$pid>::MASK > 0 {
                            unsafe {
                                let _ = self.peripheral_controlled_unchecked();
                                while pioreg.psr.read().bits() & <$pid>::MASK > 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.pdr.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverDisabled<PioControlled<Otpt>>, Pupr, Irpt, Filt>
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Self;
                type Peripheral = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    self
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.peripheral_controlled_unchecked();
                            while pioreg.psr.read().bits() & <$pid>::MASK > 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.pdr.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Psel, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PeripheralControlled<Psel>>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Psel: PeripheralSelectCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    $pid,
                    MultiDriverDisabled<PioControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Self;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.pio_controlled_unchecked();
                            while pioreg.psr.read().bits() & <$pid>::MASK == 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.per.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    self
                }
            }

            impl<Otpt, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverEnabled<Unconfigured, Otpt>, Pupr, Irpt, Filt>
            where
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PioControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.psr.read().bits() & <$pid>::MASK == 0 {
                            unsafe {
                                let _ = self.pio_controlled_unchecked();
                                while pioreg.psr.read().bits() & <$pid>::MASK == 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.per.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        if pioreg.psr.read().bits() & <$pid>::MASK > 0 {
                            unsafe {
                                let _ = self.peripheral_controlled_unchecked();
                                while pioreg.psr.read().bits() & <$pid>::MASK > 0 {}
                            }
                        }
                        unsafe { Ok(Pin::new()) }
                    }
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.pdr.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Otpt1, Otpt2, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<$pio, $pid, MultiDriverEnabled<PioControlled<Otpt1>, Otpt2>, Pupr, Irpt, Filt>
            where
                Otpt1: OutputCfg,
                Otpt2: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Self;
                type Peripheral = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<Unconfigured>, Otpt2>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    self
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.peripheral_controlled_unchecked();
                            while pioreg.psr.read().bits() & <$pid>::MASK > 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.pdr.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }
            }

            impl<Psel, Otpt, Pupr, Irpt, Filt> ConfigurePioControl
                for Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PeripheralControlled<Psel>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >
            where
                Psel: PeripheralSelectCfg,
                Otpt: OutputCfg,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    $pid,
                    MultiDriverEnabled<PioControlled<Unconfigured>, Otpt>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Self;

                fn pio_controlled(self) -> Result<Self::Pio, (Self, PioError)> {
                    let pioreg = unsafe { &*<$pio>::PTR };
                    if pioreg.locksr.read().bits() & <$pid>::MASK > 0 {
                        Err((self, PioError::LineLocked))
                    } else if pioreg.wpmr.read().wpen().bit() {
                        Err((self, PioError::WriteProtected))
                    } else {
                        unsafe {
                            let _ = self.pio_controlled_unchecked();
                            while pioreg.psr.read().bits() & <$pid>::MASK == 0 {}
                            Ok(Pin::new())
                        }
                    }
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.per.write_with_zero(|w| w.bits(<$pid>::MASK));
                    Pin::new()
                }

                fn peripheral_controlled(self) -> Result<Self::Peripheral, (Self, PioError)> {
                    Ok(self)
                }

                unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral {
                    self
                }
            }
        };
    };
}

pub(crate) use impl_peripheral_absel;
