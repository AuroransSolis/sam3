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
use pioa::PioA;
use piob::PioB;
#[cfg(feature = "pioc")]
use pioc::PioC;
#[cfg(feature = "piod")]
use piod::PioD;
#[cfg(feature = "pioe")]
use pioe::PioE;
#[cfg(feature = "piof")]
use piof::PioF;
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

#[allow(clippy::module_name_repetitions)]
pub struct PioControllers {
    pioa: PioA,
    piob: PioB,
    #[cfg(feature = "pioc")]
    pioc: PioC,
    #[cfg(feature = "piod")]
    piod: PioD,
    #[cfg(feature = "pioe")]
    piof: PioF,
    #[cfg(feature = "piof")]
    pioe: PioE,
}

macro_rules! def_pioc {
    ($(
        $pio:ident($inner:ty) => {
            $(
                $(#[$meta:meta])*
                $pio_abbv:tt: $num:literal
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
                [<$pio:lower>]: $inner,
                $(
                    $(#[$meta])*
                    [<$pio_abbv:snake $num:snake>]: [<$pio_abbv:camel $num:camel>],
                )+

            }

            impl crate::write_protect::WriteProtectKey for $pio {
                const WPKEY: u32 = 0x50494F;
            }

            // impl crate::pio::IsPio for $pio {
            //     type RegType = crate::pac::[<$pio:lower>]::RegisterBlock;
            //     const PTR: *const Self::RegType = crate::pac::[<$pio:upper>]::PTR;
            // }
        }
    )+};
}

pub(crate) use def_pioc;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy)]
pub enum PioError {
    LineLocked,
    WriteProtected,
}

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
