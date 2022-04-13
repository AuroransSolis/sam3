// pub mod dynpin;
pub mod peripherals;
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

use pioa::PioA;
use piob::PioB;
use pioc::PioC;
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use piod::PioD;
#[cfg(feature = "sam3x8h")]
use pioe::PioE;
#[cfg(feature = "sam3x8h")]
use piof::PioF;

pub trait IsPio {
    type RegType;
    const PTR: *const Self::RegType;
}

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
                [<$pio:snake>]: $inner,
                $(
                    $(#[$meta])*
                    [<$pio_abbv:snake $num:snake>]: [<$pio_abbv:camel $num:camel>],
                )+
            }

            impl crate::pio::IsPio for $pio {
                type RegType = crate::pac::[<$pio:lower>]::RegisterBlock;
                const PTR: *const Self::RegType = crate::pac::[<$pio:upper>]::PTR;
            }
        }
    )+};
}

pub(crate) use def_pioc;
