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

pub trait Pioc {
    type RegType;
    const PTR: *const Self::RegType;
}

// pub struct PioControllers {
//     pioa: PIOA,
//     piob: PIOB,
//     pioc: PIOC,
//     #[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
//     piod: PIOD,
//     #[cfg(feature = "sam3x8h")]
//     piof: PIOF,
//     #[cfg(feature = "sam3x8h")]
//     pioe: PIOE,
// }

macro_rules! def_pioc {
    ($(
        $pio:ident($inner:ty) => {
            $(
                $(#[$meta:meta])*
                $pin_id:ident
            ),+$(,)?
        }
    ),+$(,)?) => {$(
        $(
            $(#[$meta])*
            pub struct $pin_id;
            impl crate::pio::pin::PinId for $pin_id {}
        )+

        paste::paste! {
            pub struct $pio {
                [<$pio:snake>]: $inner,
                $(
                    $(#[$meta])*
                    [<$pin_id:snake>]: [<$pin_id:camel>],
                )+
            }

            impl crate::pio::Pioc for $pio {
                type RegType = crate::pac::[<$pio:lower>]::RegisterBlock;
                const PTR: *const Self::RegType = crate::pac::[<$pio:upper>]::PTR;
            }
        }
    )+};
}

pub(crate) use def_pioc;
