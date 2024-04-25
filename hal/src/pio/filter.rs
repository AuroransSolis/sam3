//! PIO line filtering
//!
//! PIO lines can be filtered in three ways:
//! 1. No filtering
//! 2. Programmable glitch filtering
//! 3. Debouncing
//!
//! It's difficult to communincate the exact nature of the debouncer and glitch filter through just
//! text, so please refer to the appropriate documentation for your chip for information on this
//! functionality:
//! - SAM3A, SAM3X: [manual][ax], pages 624-626
//! - SAM3N: [manual][n], pages 382-384
//! - SAM3S1, SAM3S2, SAM3S4: [manual][s124], pages 473-475
//! - SAM3S8, SAM3SD8: [manual][sd8], pages 482-484
//! - SAM3U: [manual][u], pages 500-502
//!
//! [ax]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf
//! [n]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11011-32-bit-Cortex-M3-Microcontroller-SAM3N_Datasheet.pdf
//! [s124]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6500-32-bit-Cortex-M3-Microcontroller-SAM3S4-SAM3S2-SAM3S1_Datasheet.pdf
//! [sd8]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11090-32-bit%20Cortex-M3-Microcontroller-SAM-3S8-SD8_Datasheet.pdf
//! [u]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6430-32-bit-Cortex-M3-Microcontroller-SAM3U4-SAM3U2-SAM3U1_Datasheet.pdf
//!
//! Note that for SAM3A, SAM3U, and SAM3X the slow clock is referred to as the (divided) system
//! clock.

use crate::{
    pio::{
        interrupt::InterruptCfg,
        peripheral::PioControlCfg,
        pin::{MultiDriverCfg, PadResistorCfg, Pin, PinId, Unconfigured},
        PioRegisters,
    },
    structure::*,
};
use core::marker::PhantomData;

#[allow(clippy::module_name_repetitions)]
/// Marker trait for input filter configuration options.
pub trait InputFilterCfg {}

impl InputFilterCfg for Unconfigured {}

#[allow(clippy::module_name_repetitions)]
/// Disable the glitch input filter on an I/O line.
pub struct InputFilterDisabled;

impl InputFilterCfg for InputFilterDisabled {}

#[allow(clippy::module_name_repetitions)]
/// Enable the glitch input filter on an I/O line.
pub struct InputFilterEnabled<Flck: InputFilterClockCfg> {
    _flck: PhantomData<Flck>,
}

impl<Flck: InputFilterClockCfg> InputFilterCfg for InputFilterEnabled<Flck> {}

/// # Input filter configuration
///
/// For further details, refer to module-level documentation for links to the relevant device
/// manuals.
pub trait ConfigureInputFilter {
    type Disabled: ConfigureInputFilter;
    type Enabled: ConfigureInputFilter;

    /// Disable the input filter for this pin.
    fn disable_input_filter(self) -> Self::Disabled;
    /// Disable the input filter without waiting for the status register to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its input filter disabled, but the
    /// trigger won't be disabled until the corresponding bit in `PIO_IFSR` is cleared.
    unsafe fn disable_input_filter_unchecked(self) -> Self::Disabled;
    /// Enable the input filter for this pin.
    fn enable_input_filter(self) -> Self::Enabled;
    /// Enable the input filter without waiting for the status register to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has its input filter enabled, but the
    /// trigger won't be enabled until the corresponding bit in `PIO_IFSR` is set.
    unsafe fn enable_input_filter_unchecked(self) -> Self::Enabled;
}

#[allow(clippy::module_name_repetitions)]
/// Marker trait for input filter clock configuration options.
pub trait InputFilterClockCfg {}

impl InputFilterClockCfg for Unconfigured {}

#[allow(clippy::module_name_repetitions)]
/// Set an I/O line's input filter to use the system clock glitch filter.
pub struct SlowClock;

impl InputFilterClockCfg for SlowClock {}

#[allow(clippy::module_name_repetitions)]
/// Set an I/O line's input filter to use the debouncing filter.
pub struct Debouncing;

impl InputFilterClockCfg for Debouncing {}

/// # Input filter clock configuration
///
/// For further details, refer to the module-level documentation for links to the relevant device
/// manuals.
pub trait ConfigureInputFilterClock {
    type Debouncing: ConfigureInputFilterClock;
    type SlowClock: ConfigureInputFilterClock;

    /// Select the debouncing filter clock for this pin.
    fn debouncing_filter(self) -> Self::Debouncing;
    /// Select the debouncing filter clock without waiting for the status register to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has the debouncing clock selected, but
    /// the clock isn't actually selected until the corresponding bit in `
    #[cfg_attr(
        any(feature = "sam3a", feature = "sam3u", feature = "sam3x"),
        doc = "PIO_IFDGSR"
    )]
    #[cfg_attr(
        any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"),
        doc = "PIO_IFSCSR"
    )]
    /// ` is cleared.
    unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing;
    /// Select the slow clock filter clock for this pin.
    fn slow_clock_filter(self) -> Self::SlowClock;
    /// Select the slow clock filter clock without waiting for the status register to update.
    ///
    /// # Safety
    ///
    /// This function returns a type showing that this pin has the slow clock selected, but the
    /// clock isn't actually selected until the corresponding bit in `
    #[cfg_attr(
        any(feature = "sam3a", feature = "sam3u", feature = "sam3x"),
        doc = "PIO_IFDGSR"
    )]
    #[cfg_attr(
        any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"),
        doc = "PIO_IFSCSR"
    )]
    /// ` is set.
    unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock;
}

impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilter
    for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Unconfigured>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
{
    type Disabled = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterDisabled>;
    type Enabled = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Unconfigured>>;

    fn disable_input_filter(self) -> Self::Disabled {
        unsafe {
            let pioreg = &*Pio::PTR;
            if pioreg._ifsr().read().bits() & Pid::MASK != 0 {
                let _ = self.disable_input_filter_unchecked();
                while pioreg._ifsr().read().bits() & Pid::MASK != 0 {}
            }
            Pin::new()
        }
    }

    unsafe fn disable_input_filter_unchecked(self) -> Self::Disabled {
        let pioreg = &*Pio::PTR;
        pioreg._ifdr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn enable_input_filter(self) -> Self::Enabled {
        unsafe {
            let pioreg = &*Pio::PTR;
            if pioreg._ifsr().read().bits() & Pid::MASK == 0 {
                let _ = self.enable_input_filter_unchecked();
                while pioreg._ifsr().read().bits() & Pid::MASK == 0 {}
            }
            Pin::new()
        }
    }

    unsafe fn enable_input_filter_unchecked(self) -> Self::Enabled {
        let pioreg = &*Pio::PTR;
        pioreg._ifer().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilter
    for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterDisabled>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
{
    type Disabled = Self;
    type Enabled = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Unconfigured>>;

    fn disable_input_filter(self) -> Self::Disabled {
        self
    }

    unsafe fn disable_input_filter_unchecked(self) -> Self::Disabled {
        self
    }

    fn enable_input_filter(self) -> Self::Enabled {
        unsafe {
            let pioreg = &*Pio::PTR;
            let _ = self.enable_input_filter_unchecked();
            while pioreg._ifsr().read().bits() & Pid::MASK == 0 {}
            Pin::new()
        }
    }

    unsafe fn enable_input_filter_unchecked(self) -> Self::Enabled {
        let pioreg = &*Pio::PTR;
        pioreg._ifer().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }
}

impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt, Flck> ConfigureInputFilter
    for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Flck>>
where
    Pio: PioRegisters,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pioc: PioControlCfg,
    Padr: PadResistorCfg,
    Irpt: InterruptCfg,
    Flck: InputFilterClockCfg,
{
    type Disabled = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterDisabled>;
    type Enabled = Self;

    fn disable_input_filter(self) -> Self::Disabled {
        unsafe {
            let pioreg = &*Pio::PTR;
            let _ = self.disable_input_filter_unchecked();
            while pioreg._ifsr().read().bits() & Pid::MASK != 0 {}
            Pin::new()
        }
    }

    unsafe fn disable_input_filter_unchecked(self) -> Self::Disabled {
        let pioreg = &*Pio::PTR;
        pioreg._ifdr().write_with_zero(|w| w.bits(Pid::MASK));
        Pin::new()
    }

    fn enable_input_filter(self) -> Self::Enabled {
        self
    }

    unsafe fn enable_input_filter_unchecked(self) -> Self::Enabled {
        self
    }
}

#[cfg(any(feature = "sam3a", feature = "sam3u", feature = "sam3x"))]
const _: () = {
    impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilterClock
        for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Unconfigured>>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
    {
        type Debouncing = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Debouncing>>;
        type SlowClock = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<SlowClock>>;

        fn debouncing_filter(self) -> Self::Debouncing {
            unsafe {
                let pioreg = &*Pio::PTR;
                if pioreg._ifdgsr().read().bits() & Pid::MASK != 0 {
                    let _ = self.debouncing_filter_unchecked();
                    while pioreg._ifdgsr().read().bits() & Pid::MASK != 0 {}
                }
                Pin::new()
            }
        }

        unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing {
            let pioreg = &*Pio::PTR;
            pioreg._difsr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn slow_clock_filter(self) -> Self::SlowClock {
            unsafe {
                let pioreg = &*Pio::PTR;
                if pioreg._ifdgsr().read().bits() & Pid::MASK == 0 {
                    let _ = self.slow_clock_filter_unchecked();
                    while pioreg._ifdgsr().read().bits() & Pid::MASK == 0 {}
                }
                Pin::new()
            }
        }

        unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock {
            let pioreg = &*Pio::PTR;
            pioreg._scifsr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilterClock
        for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Debouncing>>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
    {
        type Debouncing = Self;
        type SlowClock = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<SlowClock>>;

        fn debouncing_filter(self) -> Self::Debouncing {
            self
        }

        unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing {
            self
        }

        fn slow_clock_filter(self) -> Self::SlowClock {
            unsafe {
                let pioreg = &*Pio::PTR;
                let _ = self.slow_clock_filter_unchecked();
                while pioreg._ifdgsr().read().bits() & Pid::MASK == 0 {}
                Pin::new()
            }
        }

        unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock {
            let pioreg = &*Pio::PTR;
            pioreg._scifsr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilterClock
        for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<SlowClock>>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
    {
        type Debouncing = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Debouncing>>;
        type SlowClock = Self;

        fn debouncing_filter(self) -> Self::Debouncing {
            unsafe {
                let pioreg = &*Pio::PTR;
                let _ = self.debouncing_filter_unchecked();
                while pioreg._ifdgsr().read().bits() & Pid::MASK != 0 {}
                Pin::new()
            }
        }

        unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing {
            let pioreg = &*Pio::PTR;
            pioreg._difsr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn slow_clock_filter(self) -> Self::SlowClock {
            self
        }

        unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock {
            self
        }
    }
};

#[cfg(any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"))]
const _: () = {
    impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilterClock
        for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Unconfigured>>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
    {
        type Debouncing = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Debouncing>>;
        type SlowClock = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<SlowClock>>;

        fn debouncing_filter(self) -> Self::Debouncing {
            unsafe {
                let pioreg = &*Pio::PTR;
                if pioreg._ifscsr().read().bits() & Pid::MASK != 0 {
                    let _ = self.debouncing_filter_unchecked();
                    while pioreg._ifscsr().read().bits() & Pid::MASK != 0 {}
                }
                Pin::new()
            }
        }

        unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing {
            let pioreg = &*Pio::PTR;
            pioreg._ifscdr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn slow_clock_filter(self) -> Self::SlowClock {
            unsafe {
                let pioreg = &*Pio::PTR;
                if pioreg._ifscsr().read().bits() & Pid::MASK == 0 {
                    let _ = self.slow_clock_filter_unchecked();
                    while pioreg._ifscsr().read().bits() & Pid::MASK == 0 {}
                }
                Pin::new()
            }
        }

        unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock {
            let pioreg = &*Pio::PTR;
            pioreg._ifscer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilterClock
        for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Debouncing>>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
    {
        type Debouncing = Self;
        type SlowClock = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<SlowClock>>;

        fn debouncing_filter(self) -> Self::Debouncing {
            self
        }

        unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing {
            self
        }

        fn slow_clock_filter(self) -> Self::SlowClock {
            unsafe {
                let pioreg = &*Pio::PTR;
                let _ = self.slow_clock_filter_unchecked();
                while pioreg._ifscsr().read().bits() & Pid::MASK == 0 {}
                Pin::new()
            }
        }

        unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock {
            let pioreg = &*Pio::PTR;
            pioreg._ifscer().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }
    }

    impl<Pio, Pid, Mdvr, Pioc, Padr, Irpt> ConfigureInputFilterClock
        for Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<SlowClock>>
    where
        Pio: PioRegisters,
        Pid: PinId<Controller = Pio>,
        Mdvr: MultiDriverCfg,
        Pioc: PioControlCfg,
        Padr: PadResistorCfg,
        Irpt: InterruptCfg,
    {
        type Debouncing = Pin<Pio, Pid, Mdvr, Pioc, Padr, Irpt, InputFilterEnabled<Debouncing>>;
        type SlowClock = Self;

        fn debouncing_filter(self) -> Self::Debouncing {
            unsafe {
                let pioreg = &*Pio::PTR;
                let _ = self.debouncing_filter_unchecked();
                while pioreg._ifscsr().read().bits() & Pid::MASK != 0 {}
                Pin::new()
            }
        }

        unsafe fn debouncing_filter_unchecked(self) -> Self::Debouncing {
            let pioreg = &*Pio::PTR;
            pioreg._ifscdr().write_with_zero(|w| w.bits(Pid::MASK));
            Pin::new()
        }

        fn slow_clock_filter(self) -> Self::SlowClock {
            self
        }

        unsafe fn slow_clock_filter_unchecked(self) -> Self::SlowClock {
            self
        }
    }
};
