// pub mod dynpin;
pub mod filter;
pub mod interrupt;
pub mod peripheral;
pub mod pin;
#[cfg(feature = "pioa")]
pub mod pioa;
#[cfg(feature = "piob")]
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

use crate::write_protect::{WriteProtect, WriteProtectKey};
#[cfg(feature = "pioa")]
use pioa::PioA;
#[cfg(feature = "piob")]
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

#[cfg(feature = "pioa")]
impl WriteProtectKey for crate::pac::PIOA {
    const WPKEY: u32 = 0x50494F;
}

#[cfg(feature = "piob")]
impl WriteProtectKey for crate::pac::PIOB {
    const WPKEY: u32 = 0x50494F;
}

#[cfg(feature = "pioc")]
impl WriteProtectKey for crate::pac::PIOC {
    const WPKEY: u32 = 0x50494F;
}

#[cfg(feature = "piod")]
impl WriteProtectKey for crate::pac::PIOD {
    const WPKEY: u32 = 0x50494F;
}

#[cfg(feature = "pioe")]
impl WriteProtectKey for crate::pac::PIOE {
    const WPKEY: u32 = 0x50494F;
}

#[cfg(feature = "piof")]
impl WriteProtectKey for crate::pac::PIOF {
    const WPKEY: u32 = 0x50494F;
}

impl<Pio: PioRegisters + WriteProtectKey> WriteProtect for Pio {
    fn enable_writeprotect(&mut self) {
        self._wpmr()
            .write(|w| unsafe { w.wpkey().bits(Self::WPKEY).wpen().set_bit() })
    }

    fn disable_writeprotect(&mut self) {
        self._wpmr()
            .write(|w| unsafe { w.wpkey().bits(Self::WPKEY).wpen().clear_bit() });
    }

    fn writeprotect_enabled(&self) -> bool {
        self._wpmr().read().wpen().bit()
    }

    fn writeprotect_error(&self) -> bool {
        self._wpsr().read().wpvs().bit()
    }

    unsafe fn writeprotect_error_addr_unchecked(&self) -> u16 {
        self._wpsr().read().wpvsrc().bits()
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct PioControllers {
    #[cfg(feature = "pioa")]
    pioa: PioA,
    #[cfg(feature = "piob")]
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

            // crate::write_protect::wp_impl! {
            //     ///   - PIO Enable Register (`PIO_PER`)
            //     ///   - PIO Disable Register (`PIO_PDR`)
            //     ///   - Output Enable Register (`PIO_OER`)
            //     ///   - Output Disable Register (`PIO_ODR`)
            //     ///   - Input Filter Enable Register (`PIO_IFER`)
            //     ///   - Input Filter Disable Register (`PIO_IDER`)
            //     ///   - Multi-driver Enable Register (`PIO_MDER`)
            //     ///   - Multi-driver Disable Register (`PIO_MDDR`)
            //     ///   - Pull Up Enable Register (`PIO_PUER`)
            //     ///   - Pull Up Disable Register (`PIO_PUDR`)
            //     ///   - Peripheral AB Select Register (`PIO_ABSR`)
            //     ///   - Output Write Enable Register (`PIO_OWER`)
            //     ///   - Output Write Disable Register (`PIO_OWDR`)
            //     $pio => [<$pio:lower>](wpvs, wpvsrc<u16>): b"PIO",
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
