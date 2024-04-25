//! Wrapper struct for [`PIOA`] and its pins.
#![allow(unused_imports)]

#[cfg(feature = "3fn")]
use crate::pio::peripheral::PeripheralC;
#[cfg(feature = "4fn")]
use crate::pio::peripheral::PeripheralD;
use crate::{
    pac::PIOA,
    pio::peripheral::{PeripheralA, PeripheralB},
};
use seq_macro::seq;

seq! {N in 0..32 {
    crate::pio::def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa0, peripherals: [A, B]],
        [pin: Pa1, peripherals: [A, B]],
        [pin: Pa2, peripherals: [A, B]],
        [pin: Pa3, peripherals: [A, B]],
        [pin: Pa4, peripherals: [A, B]],
        [pin: Pa5, peripherals: [A, B]],
        [pin: Pa6, peripherals: [A, B]],
        [pin: Pa7, peripherals: [A, B]],
        [pin: Pa8, peripherals: [A, B]],
        [pin: Pa9, peripherals: [A, B]],
        [pin: Pa10, peripherals: [A, B]],
        [pin: Pa11, peripherals: [A, B]],
        [pin: Pa12, peripherals: [A, B]],
        [pin: Pa13, peripherals: [A, B]],
        [pin: Pa14, peripherals: [A, B]],
        [pin: Pa15, peripherals: [A, B]],
        [pin: Pa16, peripherals: [A, B]],
        [pin: Pa17, peripherals: [A, B]],
        [pin: Pa18, peripherals: [A, B]],
        [pin: Pa19, peripherals: [A, B]],
        [pin: Pa20, peripherals: [A, B]],
        [pin: Pa21, peripherals: [A, B]],
        [pin: Pa22, peripherals: [A, B]],
        [pin: Pa23, peripherals: [A, B]],
        [pin: Pa24, peripherals: [A, B]],
        [pin: Pa25, peripherals: [A, B]],
        [pin: Pa26, peripherals: [A, B]],
        [pin: Pa27, peripherals: [A, B]],
        [pin: Pa28, peripherals: [A, B]],
        [pin: Pa29, peripherals: [A, B]],
    ],
}

#[cfg(feature = "sam3x217")]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa30, peripherals: [A, B]],
        [pin: Pa31, peripherals: [A, B]],
    ],
}

#[cfg(feature = "sam3n")]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa0, peripherals: [A, B]],
        [pin: Pa1, peripherals: [A, B]],
        [pin: Pa2, peripherals: [A, B, C]],
        [pin: Pa3, peripherals: [A, B]],
        [pin: Pa4, peripherals: [A, B]],
        [pin: Pa5, peripherals: [A, B]],
        [pin: Pa6, peripherals: [A, B]],
        [pin: Pa7, peripherals: [A, B]],
        [pin: Pa8, peripherals: [A, B]],
        [pin: Pa9, peripherals: [A, B]],
        [pin: Pa10, peripherals: [A, B]],
        [pin: Pa11, peripherals: [A, B]],
        [pin: Pa12, peripherals: [A, B]],
        [pin: Pa13, peripherals: [A, B]],
        [pin: Pa14, peripherals: [A, B]],
        [pin: Pa15, peripherals: [B]],
        [pin: Pa16, peripherals: [B]],
        [pin: Pa17, peripherals: [B]],
        [pin: Pa18, peripherals: [B]],
    ],
}

#[cfg(any(feature = "sam3n64", feature = "sam3n100"))]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa21, peripherals: [A, B]],
        [pin: Pa22, peripherals: [A, B]],
        [pin: Pa23, peripherals: [A, B]],
        [pin: Pa24, peripherals: [A, B]],
        [pin: Pa25, peripherals: [A, B]],
        [pin: Pa26, peripherals: [B]],
        [pin: Pa27, peripherals: [B]],
        [pin: Pa28, peripherals: [B]],
        [pin: Pa29, peripherals: [B]],
        [pin: Pa30, peripherals: [B]],
        [pin: Pa31, peripherals: [A, B]],
    ],
}

#[cfg(feature = "sam3s")]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa0, peripherals: [A, B, C]],
        [pin: Pa1, peripherals: [A, B, C]],
        [pin: Pa2, peripherals: [A, B, C]],
        [pin: Pa3, peripherals: [A, B]],
        [pin: Pa4, peripherals: [A, B]],
        [pin: Pa5, peripherals: [A, B]],
        [pin: Pa6, peripherals: [A, B]],
        [pin: Pa7, peripherals: [A, B]],
        [pin: Pa8, peripherals: [A, B]],
        [pin: Pa9, peripherals: [A, B, C]],
        [pin: Pa10, peripherals: [A, B]],
        [pin: Pa11, peripherals: [A, B]],
        [pin: Pa12, peripherals: [A, B]],
        [pin: Pa13, peripherals: [A, B]],
        [pin: Pa14, peripherals: [A, B]],
        [pin: Pa15, peripherals: [A, B, C]],
        [pin: Pa16, peripherals: [A, B, C]],
        [pin: Pa17, peripherals: [A, B, C]],
        [pin: Pa18, peripherals: [A, B, C]],
        [pin: Pa19, peripherals: [A, B, C]],
        [pin: Pa20, peripherals: [A, B, C]],
    ],
}

#[cfg(any(feature = "sam3s64", feature = "sam3s100"))]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa21, peripherals: [A, B]],
        [pin: Pa22, peripherals: [A, B, C]],
        [pin: Pa23, peripherals: [A, B, C]],
        [pin: Pa24, peripherals: [A, B, C]],
        [pin: Pa25, peripherals: [A, B, C]],
        [pin: Pa26, peripherals: [A, B, C]],
        [pin: Pa27, peripherals: [A, B, C]],
        [pin: Pa28, peripherals: [A, B, C]],
        [pin: Pa29, peripherals: [A, B, C]],
        [pin: Pa30, peripherals: [A, B, C]],
        [pin: Pa31, peripherals: [A, B, C]],
    ],
}

#[cfg(feature = "sam3s8")]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa0, peripherals: [A, B, C]],
        [pin: Pa1, peripherals: [A, B, C]],
        [pin: Pa2, peripherals: [A, B, C]],
        [pin: Pa3, peripherals: [A, B]],
        [pin: Pa4, peripherals: [A, B]],
        [pin: Pa5, peripherals: [A, B]],
        [pin: Pa6, peripherals: [A, B]],
        [pin: Pa7, peripherals: [A, B]],
        [pin: Pa8, peripherals: [A, B, C]],
        [pin: Pa9, peripherals: [A, B]],
        [pin: Pa10, peripherals: [A, B]],
        [pin: Pa11, peripherals: [A, B]],
        [pin: Pa12, peripherals: [A, B]],
        [pin: Pa13, peripherals: [A, B]],
        [pin: Pa14, peripherals: [A, B]],
        [pin: Pa15, peripherals: [A, B, C, D]],
        [pin: Pa16, peripherals: [A, B, C, D]],
        [pin: Pa17, peripherals: [A, B, C]],
        [pin: Pa18, peripherals: [A, B, C]],
        [pin: Pa19, peripherals: [A, B, C]],
        [pin: Pa20, peripherals: [A, B, C]],
    ],
}

#[cfg(any(feature = "sam3s864", feature = "sam3s8100"))]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa21, peripherals: [A, B]],
        [pin: Pa22, peripherals: [A, B, C]],
        [pin: Pa23, peripherals: [A, B, C, D]],
        [pin: Pa24, peripherals: [A, B, C, D]],
        [pin: Pa25, peripherals: [A, B, C, D]],
        [pin: Pa26, peripherals: [A, B, C, D]],
        [pin: Pa27, peripherals: [A, B, C, D]],
        [pin: Pa28, peripherals: [A, B, C, D]],
        [pin: Pa29, peripherals: [A, B, C, D]],
        [pin: Pa30, peripherals: [A, B, C, D]],
        [pin: Pa31, peripherals: [A, B, C, D]],
    ],
}

#[cfg(feature = "sam3u")]
crate::pio::pin_peripherals! {
    pio: PIOA,
    pinopts: [
        [pin: Pa0, peripherals: [A, B]],
        [pin: Pa1, peripherals: [A, B]],
        [pin: Pa2, peripherals: [A, B]],
        [pin: Pa3, peripherals: [A, B]],
        [pin: Pa4, peripherals: [A, B]],
        [pin: Pa5, peripherals: [A, B]],
        [pin: Pa6, peripherals: [A, B]],
        [pin: Pa7, peripherals: [A, B]],
        [pin: Pa8, peripherals: [A, B]],
        [pin: Pa9, peripherals: [A, B]],
        [pin: Pa10, peripherals: [A, B]],
        [pin: Pa11, peripherals: [A, B]],
        [pin: Pa12, peripherals: [A, B]],
        [pin: Pa13, peripherals: [A]],
        [pin: Pa14, peripherals: [A]],
        [pin: Pa15, peripherals: [A, B]],
        [pin: Pa16, peripherals: [A, B]],
        [pin: Pa17, peripherals: [A, B]],
        [pin: Pa18, peripherals: [A, B]],
        [pin: Pa19, peripherals: [A, B]],
        [pin: Pa20, peripherals: [A, B]],
        [pin: Pa21, peripherals: [A, B]],
        [pin: Pa22, peripherals: [A, B]],
        [pin: Pa23, peripherals: [A, B]],
        [pin: Pa24, peripherals: [A, B]],
        [pin: Pa25, peripherals: [A, B]],
        [pin: Pa26, peripherals: [A, B]],
        [pin: Pa27, peripherals: [A, B]],
        [pin: Pa28, peripherals: [A, B]],
        [pin: Pa29, peripherals: [A, B]],
        [pin: Pa30, peripherals: [A, B]],
        [pin: Pa31, peripherals: [A, B]],
    ],
}
