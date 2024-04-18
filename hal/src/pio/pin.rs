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
pub trait PinId {
    type Controller: PioRegisters;
    const MASK: u32;
}

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

pub trait Configured {}

// #[rustfmt::skip]
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

pub struct Unconfigured;

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

pub trait PadResistorCfg {}

impl PadResistorCfg for Unconfigured {}

/// Enable the pull-up resistor on an I/O line.
pub struct PullupEnabled;

impl PadResistorCfg for PullupEnabled {}

#[cfg(feature = "ppd")]
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
pub trait ConfigurePadResistor: Sized {
    type Disabled;
    type Enabled;

    fn disable_pullup_resistor(self) -> Result<Self::Disabled, (Self, PioError)>;
    unsafe fn disable_pullup_resistor_unchecked(self) -> Self::Disabled;
    fn enable_pullup_resistor(self) -> Result<Self::Enabled, (Self, PioError)>;
    unsafe fn enable_pullup_resistor_unchecked(self) -> Self::Enabled;
}

#[cfg(feature = "ppd")]
pub trait ConfigurePadResistor: Sized {
    type Disabled;
    type Pulldown;
    type Pullup;

    fn disable_pdpu_resistors(self) -> Result<Self::Disabled, (Self, PioError)>;
    unsafe fn disable_pdpu_resistors_unchecked(self) -> Self::Disabled;
    fn enable_pulldown_resistor(self) -> Result<Self::Pulldown, (Self, PioError)>;
    unsafe fn enable_pulldown_resistor_unchecked(self) -> Self::Pulldown;
    fn enable_pullup_resistor(self) -> Result<Self::Pullup, (Self, PioError)>;
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
                    return Err((self, PioError::WriteProtected))
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
                    return Err((self, PioError::WriteProtected))
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
                    return Err((self, PioError::WriteProtected))
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
                    return Err((self, PioError::WriteProtected))
                }
                let _ = self.disable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
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
                    return Err((self, PioError::WriteProtected))
                }
                if pioreg._pusr().read().bits() & Pid::MASK != 0 {
                    let _ = self.disable_pdpu_resistors_unchecked();
                    while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                }
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
                    return Err((self, PioError::WriteProtected))
                }
                if pioreg._pusr().read().bits() & Pid::MASK != 0 {
                    let self_copy: Self = Pin::new();
                    let _ = self_copy.disable_pdpu_resistors_unchecked();
                    while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                }
                if pioreg._ppdsr().read().bits() & Pid::MASK == 0 {
                    let _ = self.enable_pulldown_resistor_unchecked();
                    while pioreg._ppdsr().read().bits() & Pid::MASK == 0 {}
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
                    return Err((self, PioError::WriteProtected))
                }
                if pioreg._ppdsr().read().bits() & Pid::MASK != 0 {
                    let self_copy: Self = Pin::new();
                    let _ = self_copy.disable_pdpu_resistors_unchecked();
                    while pioreg._ppdsr().read().bits() & Pid::MASK != 0 {}
                }
                if pioreg._pusr().read().bits() & Pid::MASK == 0 {
                    let _ = self.enable_pullup_resistor_unchecked();
                    while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
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
                    return Err((self, PioError::WriteProtected))
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
                    return Err((self, PioError::WriteProtected))
                }
                let _ = self.enable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
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
                    return Err((self, PioError::WriteProtected))
                }
                let _ = self.disable_pdpu_resistors_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK != 0 {}
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
                    return Err((self, PioError::WriteProtected))
                }
                let self_copy: Self = Pin::new();
                let _ = self_copy.disable_pdpu_resistors_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK != 0 {}
                let _ = self.enable_pullup_resistor_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK == 0 {}
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
                    return Err((self, PioError::WriteProtected))
                }
                let _ = self.disable_pdpu_resistors_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
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
                    return Err((self, PioError::WriteProtected))
                }
                let self_copy: Self = Pin::new();
                let _ = self_copy.disable_pdpu_resistors_unchecked();
                while pioreg._pusr().read().bits() & Pid::MASK != 0 {}
                let _ = self.enable_pulldown_resistor_unchecked();
                while pioreg._ppdsr().read().bits() & Pid::MASK == 0 {}
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
            if Pio::Rb::writeprotect_enabled(&pioreg) {
                return Err((self, PioError::WriteProtected));
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
            if Pio::Rb::writeprotect_enabled(&pioreg) {
                return Err((self, PioError::WriteProtected));
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
            if Pio::Rb::writeprotect_enabled(&pioreg) {
                return Err((self, PioError::WriteProtected));
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
            if Pio::Rb::writeprotect_enabled(&pioreg) {
                return Err((self, PioError::WriteProtected));
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
