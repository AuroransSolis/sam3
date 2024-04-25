//! Wrapper struct for [`PIOB`] and its pins.
#![allow(unused_imports)]

#[cfg(feature = "3fn")]
use crate::pio::peripheral::PeripheralC;
#[cfg(feature = "4fn")]
use crate::pio::peripheral::PeripheralD;
use crate::{
    pac::PIOB,
    pio::peripheral::{PeripheralA, PeripheralB},
};
use seq_macro::seq;

#[cfg(any(feature = "sam3a", feature = "sam3u", feature = "sam3x"))]
seq! {N in 0..32 {
    crate::pio::def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb0, peripherals: [A, B]],
        [pin: Pb1, peripherals: [A, B]],
        [pin: Pb2, peripherals: [A, B]],
        [pin: Pb3, peripherals: [A, B]],
        [pin: Pb4, peripherals: [A, B]],
        [pin: Pb5, peripherals: [A, B]],
        [pin: Pb6, peripherals: [A, B]],
        [pin: Pb7, peripherals: [A, B]],
        [pin: Pb8, peripherals: [A, B]],
        [pin: Pb9, peripherals: [A, B]],
        [pin: Pb10, peripherals: [A, B]],
        [pin: Pb11, peripherals: [A, B]],
        [pin: Pb12, peripherals: [A, B]],
        [pin: Pb13, peripherals: [A, B]],
        [pin: Pb14, peripherals: [A, B]],
        [pin: Pb15, peripherals: [A, B]],
        [pin: Pb16, peripherals: [A, B]],
        [pin: Pb17, peripherals: [A, B]],
        [pin: Pb18, peripherals: [A, B]],
        [pin: Pb19, peripherals: [A, B]],
        [pin: Pb20, peripherals: [A, B]],
        [pin: Pb21, peripherals: [A, B]],
        [pin: Pb22, peripherals: [A, B]],
        [pin: Pb23, peripherals: [A, B]],
        [pin: Pb24, peripherals: [A, B]],
        [pin: Pb25, peripherals: [A, B]],
        [pin: Pb26, peripherals: [A, B]],
        [pin: Pb27, peripherals: [A, B]],
        [pin: Pb28, peripherals: [A]],
        [pin: Pb29, peripherals: [A]],
        [pin: Pb30, peripherals: [A]],
        [pin: Pb31, peripherals: [A]],
    ],
}

#[cfg(feature = "sam3u")]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb0, peripherals: [A, B]],
        [pin: Pb1, peripherals: [A, B]],
        [pin: Pb2, peripherals: [A, B]],
        [pin: Pb3, peripherals: [A, B]],
        [pin: Pb4, peripherals: [A, B]],
        [pin: Pb5, peripherals: [A, B]],
        [pin: Pb6, peripherals: [A, B]],
        [pin: Pb7, peripherals: [A, B]],
        [pin: Pb8, peripherals: [A, B]],
        [pin: Pb9, peripherals: [A, B]],
        [pin: Pb10, peripherals: [A, B]],
        [pin: Pb11, peripherals: [A, B]],
        [pin: Pb12, peripherals: [A, B]],
        [pin: Pb13, peripherals: [A, B]],
        [pin: Pb14, peripherals: [A, B]],
        [pin: Pb15, peripherals: [A, B]],
        [pin: Pb16, peripherals: [A, B]],
        [pin: Pb17, peripherals: [A, B]],
        [pin: Pb18, peripherals: [A, B]],
        [pin: Pb19, peripherals: [A, B]],
        [pin: Pb20, peripherals: [A, B]],
        [pin: Pb21, peripherals: [A, B]],
        [pin: Pb22, peripherals: [A, B]],
        [pin: Pb23, peripherals: [A, B]],
        [pin: Pb24, peripherals: [A, B]],
    ],
}

#[cfg(feature = "sam3u144")]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb25, peripherals: [A, B]],
        [pin: Pb26, peripherals: [A, B]],
        [pin: Pb27, peripherals: [A, B]],
        [pin: Pb28, peripherals: [A, B]],
        [pin: Pb29, peripherals: [A]],
        [pin: Pb30, peripherals: [A]],
        [pin: Pb31, peripherals: [A]],
    ],
}

#[cfg(any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"))]
seq! {N in 0..15 {
    crate::pio::def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3n")]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb0, peripherals: [A]],
        [pin: Pb1, peripherals: [A]],
        [pin: Pb2, peripherals: [A, B]],
        [pin: Pb3, peripherals: [A, B]],
        [pin: Pb4, peripherals: [A, B]],
        [pin: Pb5, peripherals: [A]],
    ],
}

#[cfg(any(feature = "sam3n64", feature = "sam3n100"))]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb13, peripherals: [B]],
        [pin: Pb14, peripherals: [A, B]],
    ],
}

#[cfg(any(feature = "sam3s", feature = "sam3s8"))]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb0, peripherals: [A]],
        [pin: Pb1, peripherals: [A]],
        [pin: Pb2, peripherals: [A, B]],
        [pin: Pb3, peripherals: [A, B]],
        [pin: Pb4, peripherals: [A, B]],
        [pin: Pb5, peripherals: [A, B]],
    ],
}

#[cfg(any(
    feature = "sam3s64",
    feature = "sam3s100",
    feature = "sam3s864",
    feature = "sam3s8100"
))]
crate::pio::pin_peripherals! {
    pio: PIOB,
    pinopts: [
        [pin: Pb12, peripherals: [A]],
        [pin: Pb13, peripherals: [A, B]],
        [pin: Pb14, peripherals: [A, B]],
    ],
}
