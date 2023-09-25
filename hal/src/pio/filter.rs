//! PIO line filtering
//!
//! ADD IN `PIO_IFDGSR` AND `PIO_SCDR`
//!
//! PIO lines can be filtered in three ways:
//! 1. No filtering
//! 2. Programmable glitch filtering
//! 3. Debouncing
//!
//! It's difficult to communincate the exact nature of the debouncer and glitch filter through just
//! text, so please refer to the appropriate documentation for your chip for information on this
//! functionality:
//! - SAM3A, SAM3X: [manual](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf), pages 624-625
//! - SAM3N: [manual](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11011-32-bit-Cortex-M3-Microcontroller-SAM3N_Datasheet.pdf) pages 382-383
//! - SAM3S: [manual](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6500-32-bit-Cortex-M3-Microcontroller-SAM3S4-SAM3S2-SAM3S1_Datasheet.pdf) pages 473-474
//! - SAM3U: [manual](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6430-32-bit-Cortex-M3-Microcontroller-SAM3U4-SAM3U2-SAM3U1_Datasheet.pdf) pages 500-501

use core::marker::PhantomData;

#[allow(clippy::module_name_repetitions)]
pub trait InputFilterCfg {}
#[allow(clippy::module_name_repetitions)]
pub trait InputFilterClockCfg {}

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

#[allow(clippy::module_name_repetitions)]
/// Set an I/O line's input filter to use the system clock glitch filter.
pub struct SystemClockGlitchFilter;

impl InputFilterClockCfg for SystemClockGlitchFilter {}

#[allow(clippy::module_name_repetitions)]
/// Set an I/O line's input filter to use the debouncing filter.
pub struct DebouncingInputGlitchFilter;

impl InputFilterClockCfg for DebouncingInputGlitchFilter {}
