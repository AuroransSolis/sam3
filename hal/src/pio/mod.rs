//! Pin and configuration definitions and peripheral mappings
//!
//! Relevant manual pages:
//! - SAM3A, SAM3X: [manual][ax], pages 618-675
//! - SAM3N: [manual][n] pages 376-437
//! - SAM3S1, SAM3S2, SAM3S4: [manual][s124] pages 467-538
//! - SAM3S8, SAM3SD8: [manual][sd8] pages 474-550
//! - SAM3U: [manual][u] pages 494-550
//!
//! [ax]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf
//! [n]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11011-32-bit-Cortex-M3-Microcontroller-SAM3N_Datasheet.pdf
//! [s124]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6500-32-bit-Cortex-M3-Microcontroller-SAM3S4-SAM3S2-SAM3S1_Datasheet.pdf
//! [sd8]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11090-32-bit%20Cortex-M3-Microcontroller-SAM-3S8-SD8_Datasheet.pdf
//! [u]: https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6430-32-bit-Cortex-M3-Microcontroller-SAM3U4-SAM3U2-SAM3U1_Datasheet.pdf

// pub mod dynpin;
pub mod filter;
pub mod interrupt;
pub mod peripheral;
pub mod pin;
pub mod pioa;
pub mod piob;
#[cfg(feature = "pioc")]
pub mod pioc;
#[cfg(feature = "piod")]
pub mod piod;
#[cfg(feature = "pioe")]
pub mod pioe;
#[cfg(feature = "piof")]
pub mod piof;
pub mod structure;

use crate::write_protect::{WpmrWpsrRegs, WriteProtect, WriteProtectKey};
#[allow(clippy::wildcard_imports)]
use structure::*;

impl<Pio: PioRegisters> WriteProtectKey for Pio {
    const WPKEY: u32 = 0x50494F;
}

impl<Pio: PioRegisters> WpmrWpsrRegs for Pio {
    type Wpmr = <Self as PioRegisters>::Wpmr;

    fn _wpmr(&self) -> &Self::Wpmr {
        self._wpmr()
    }

    type Wpsr = <Self as PioRegisters>::Wpsr;

    fn _wpsr(&self) -> &Self::Wpsr {
        self._wpsr()
    }
}

/// PIO types have write protection on the following fields:
///
///   - PIO Enable Register (`PIO_PER`)
///   - PIO Disable Register (`PIO_PDR`)
///   - Output Enable Register (`PIO_OER`)
///   - Output Disable Register (`PIO_ODR`)
///   - Input Filter Enable Register (`PIO_IFER`)
///   - Input Filter Disable Register (`PIO_IDER`)
///   - Multi-driver Enable Register (`PIO_MDER`)
///   - Multi-driver Disable Register (`PIO_MDDR`)
///   - Pull Up Enable Register (`PIO_PUER`)
///   - Pull Up Disable Register (`PIO_PUDR`)
#[cfg_attr(
    feature = "2fn",
    doc = "    - Peripheral AB Select Register (`PIO_ABSR`)"
)]
#[cfg_attr(
    any(feature = "3fn", feature = "4fn"),
    doc = "    - Peripheral ABCD Select Register 0 (`PIO_ABCDSR0`)"
)]
#[cfg_attr(
    any(feature = "3fn", feature = "4fn"),
    doc = "    - Peripheral ABCD Select Register 1 (`PIO_ABCDSR1`)"
)]
///   - Output Write Enable Register (`PIO_OWER`)
///   - Output Write Disable Register (`PIO_OWDR`)
impl<Pio: PioRegisters> WriteProtect for Pio {}

macro_rules! def_pioc {
    ($(
        $pio:ident($inner:ty) => {
            $(
                $(#[$meta:meta])*
                $pio_abbv:ident: $num:literal
            ),+$(,)?
        }
    ),+$(,)?) => {$(
        paste::paste! {
            $(
                $(#[$meta])*
                pub struct [<$pio_abbv:camel $num:camel>];
                impl crate::pio::pin::PinId for [<$pio_abbv $num>] {
                    type Controller = crate::pac::$inner;
                    const MASK: u32 = 1 << $num;
                }
            )+

            pub struct $pio {
                $(
                    $(#[$meta])*
                    pub [<$pio_abbv:snake $num:snake>]: crate::pio::pin::Pin<
                        crate::pac::[<$pio:upper>],
                        [<$pio_abbv:camel $num:camel>],
                        crate::pio::pin::Unconfigured,
                        crate::pio::pin::Unconfigured,
                        crate::pio::pin::Unconfigured,
                        crate::pio::pin::Unconfigured,
                        crate::pio::pin::Unconfigured,
                    >,
                )+
            }

            impl $pio {
                pub fn [<from_ $pio:lower>](_pio: crate::pac::[<$pio:upper>]) -> Self {
                    $pio {
                        $(
                            [<$pio_abbv:snake $num:snake>]: unsafe { crate::pio::pin::Pin::new() },
                        )+
                    }
                }

                pub(crate) const fn inner(&self) -> &crate::pac::[<$pio:lower>]::RegisterBlock {
                    unsafe { &*crate::pac::[<$pio:upper>]::PTR }
                }
            }

            const _: () = {
                use crate::write_protect::{WriteProtect, WriteProtectKey, WpmrWpsrRegs};

                impl WriteProtectKey for $pio {
                    const WPKEY: u32 = crate::pac::[<$pio:upper>]::WPKEY;
                }

                impl WpmrWpsrRegs for $pio {
                    type Wpmr = <crate::pac::[<$pio:upper>] as WpmrWpsrRegs>::Wpmr;
                    fn _wpmr(&self) -> &Self::Wpmr {
                        self.inner()._wpmr()
                    }
                    type Wpsr = <crate::pac::[<$pio:upper>] as WpmrWpsrRegs>::Wpsr;
                    fn _wpsr(&self) -> &Self::Wpsr {
                        self.inner()._wpsr()
                    }
                }

                impl WriteProtect for $pio {}
            };
        }
    )+};
}

pub(crate) use def_pioc;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug)]
/// Type returned in case a configuration change fails.
pub enum PioError {
    LineLocked,
    WriteProtected,
}

macro_rules! pin_peripherals {
    (
        pio: $pio:ty,
        pinopts: [$($pinopts:tt),+$(,)?],
    ) => {
        $(
            crate::pio::pin_peripherals! {
                @pin
                pio: $pio,
                pinopts: $pinopts,
            }
        )+
    };
    (
        @pin
        pio: $pio:ty,
        pinopts: [
            pin: $pin:ty,
            peripherals: [$($peripheral:ident),+]$(,)?
        ],
    ) => {
        $(
            crate::pio::pin_peripherals! {
                @impl
                pio: $pio,
                pin: $pin,
                peripheral: $peripheral,
            }
        )+
    };
    (
        @impl
        pio: $pio:ty,
        pin: $pin:ty,
        peripheral: $peripheral:ident,
    ) => {
        paste::paste! {
            impl crate::pio::peripheral::PeripheralExistsFor<$pio, $pin>
                for [<Peripheral $peripheral>] {}
        }
    };
}

pub(crate) use pin_peripherals;

// impl<Pio: IsPio> Pio {
//     pub fn status_reg(&self) -> u32 {
//         unsafe {
//             (&*(<Pio as IsPio>::PTR as *const RegisterBlock))
//                 .psr
//                 .read()
//                 .bits()
//         }
//     }
// }
