#![allow(unused_imports)]

// pub mod dynpin;
pub mod filter;
pub mod interrupt;
pub mod peripheral;
pub mod pin;
pub mod pioa;
pub mod piob;
pub mod pioc;
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
pub mod piod;
#[cfg(feature = "sam3x8h")]
pub mod pioe;
#[cfg(feature = "sam3x8h")]
pub mod piof;

use crate::{pac::pioa::RegisterBlock, write_protect::WriteProtect};
use pioa::PioA;
use piob::PioB;
use pioc::PioC;
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use piod::PioD;
#[cfg(feature = "sam3x8h")]
use pioe::PioE;
#[cfg(feature = "sam3x8h")]
use piof::PioF;

#[allow(clippy::module_name_repetitions)]
pub trait IsPio: WriteProtect {
    type RegType;
    const PTR: *const Self::RegType;
}

#[allow(clippy::module_name_repetitions)]
pub struct PioControllers {
    pioa: PioA,
    piob: PioB,
    pioc: PioC,
    #[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
    piod: PioD,
    #[cfg(feature = "sam3x8h")]
    piof: PioF,
    #[cfg(feature = "sam3x8h")]
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
                    type Controller = $pio;
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

            impl crate::pio::IsPio for $pio {
                type RegType = crate::pac::[<$pio:lower>]::RegisterBlock;
                const PTR: *const Self::RegType = crate::pac::[<$pio:upper>]::PTR;
            }

            crate::write_protect::wp_impl! {
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
                ///   - Peripheral AB Select Register (`PIO_ABSR`)
                ///   - Output Write Enable Register (`PIO_OWER`)
                ///   - Output Write Disable Register (`PIO_OWDR`)
                $pio => [<$pio:lower>](wpvs, wpvsrc<u16>): b"PIO",
            }
        }
    )+};
}

pub(crate) use def_pioc;

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
