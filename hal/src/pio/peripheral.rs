//! PIO peripheral configuration
//!
//!

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use crate::pio::PioD;
use crate::pio::{
    filter::InputFilterCfg,
    interrupt::InterruptCfg,
    pin::{Configured, Pin, PinId, PullupResistorCfg, Unconfigured},
    pioa::PioA,
    piob::PioB,
    pioc::PioC,
    IsPio,
};
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
pub struct MultiDriverEnabled<Psel: PioControl, Sync: OutputSyncWriteCfg> {
    _psel: PhantomData<Psel>,
    _sync: PhantomData<Sync>,
}

impl<Line: PioControl, Sync: OutputSyncWriteCfg> MultiDriverCfg for MultiDriverEnabled<Line, Sync> {}
impl<Line, Sync> Configured for MultiDriverEnabled<Line, Sync>
where
    Line: PioControl + Configured,
    Sync: OutputSyncWriteCfg + Configured,
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
    type Enabled;
    type Disabled;

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

pub trait ConfigurePioControl: Sized {
    type Pio;
    type Peripheral;

    fn pio_controlled(self) -> Result<Self::Pio, Self>;
    unsafe fn pio_controlled_unchecked(self) -> Self::Pio;
    fn peripheral_controlled(self) -> Result<Self::Peripheral, Self>;
    unsafe fn peripheral_controlled_unchecked(self) -> Self::Peripheral;
}

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

pub trait OutputCfg {}

impl OutputCfg for Unconfigured {}

/// Disable PIO control of the output.
pub struct OutputDisabled;

impl OutputCfg for OutputDisabled {}
impl Configured for OutputDisabled {}

/// Enable PIO control of the output.
pub struct OutputEnabled<Outw: OutputWriteCfg, Sync: OutputSyncWriteCfg> {
    _outw: PhantomData<Outw>,
    _sync: PhantomData<Sync>,
}

impl<Outw: OutputWriteCfg, Sync: OutputSyncWriteCfg> OutputCfg for OutputEnabled<Outw, Sync> {}
impl<Outw, Sync> Configured for OutputEnabled<Outw, Sync>
where
    Outw: OutputWriteCfg + Configured,
    Sync: OutputSyncWriteCfg + Configured,
{
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

            impl<Pid, Psel, Sync, Pupr, Irpt, Filt> ConfigureMultiDriver for
                Pin<$pio, Pid, MultiDriverEnabled<Psel, Sync>, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Psel: PioControl,
                Sync: OutputSyncWriteCfg,
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

            impl<Pid, Pupr, Irpt, Filt> ConfigurePioControl for Pin<$pio, Pid, MultiDriverDisabled<Unconfigured>, Pupr, Irpt, Filt>
            where
                Pid: PinId<Controller = $pio>,
                Pupr: PullupResistorCfg,
                Irpt: InterruptCfg,
                Filt: InputFilterCfg,
            {
                type Pio = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PioControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;
                type Peripheral = Pin<
                    $pio,
                    Pid,
                    MultiDriverDisabled<PeripheralControlled<Unconfigured>>,
                    Pupr,
                    Irpt,
                    Filt,
                >;

                fn pio_controlled(self) -> Result<Self::Pio, Self> {
                    if 
                }

                unsafe fn pio_controlled_unchecked(self) -> Self::Pio {
                    let pioreg = &mut *(<$pio>::PTR as *mut <$pio as IsPio>::RegType);
                    pioreg.per.write_with_zero(|w| w.bits(<Pid as PinId>::MASK));
                    Pin::new()
                }
            }
        )+
    };
}

impl_peripheral_cfgs! {
    PioA, PioB, PioC,
}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_peripheral_cfgs! {
    PioD,
}

#[cfg(feature = "sam3x8h")]
impl_peripheral_cfgs! {
    PioE, PioF,
}
