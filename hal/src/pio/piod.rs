//! Wrapper struct for [`PIOD`] and its pins.
#![allow(unused_imports)]

use crate::{
    pac::PIOD,
    pio::peripheral::{PeripheralA, PeripheralB},
};
use seq_macro::seq;

seq! {N in 0..31 {
    crate::pio::def_pioc! {
        PioD(PIOD) => {
            #(Pd: N,)*
        }
    }
}}

crate::pio::pin_peripherals! {
    pio: PIOD,
    pinopts: [
        [pin: Pd0, peripherals: [A, B]],
        [pin: Pd1, peripherals: [A, B]],
        [pin: Pd2, peripherals: [A, B]],
        [pin: Pd3, peripherals: [A, B]],
        [pin: Pd4, peripherals: [A, B]],
        [pin: Pd5, peripherals: [A, B]],
        [pin: Pd6, peripherals: [A, B]],
        [pin: Pd7, peripherals: [A, B]],
        [pin: Pd8, peripherals: [A, B]],
        [pin: Pd9, peripherals: [A, B]],
        [pin: Pd10, peripherals: [A]],
    ],
}

#[cfg(feature = "sam3x217")]
crate::pio::pin_peripherals! {
    pio: PIOD,
    pinopts: [
        [pin: Pd11, peripherals: [A]],
        [pin: Pd12, peripherals: [A]],
        [pin: Pd13, peripherals: [A]],
        [pin: Pd14, peripherals: [A]],
        [pin: Pd15, peripherals: [A]],
        [pin: Pd16, peripherals: [A]],
        [pin: Pd17, peripherals: [A]],
        [pin: Pd18, peripherals: [A]],
        [pin: Pd19, peripherals: [A]],
        [pin: Pd20, peripherals: [A]],
        [pin: Pd21, peripherals: [A]],
        [pin: Pd22, peripherals: [A]],
        [pin: Pd23, peripherals: [A]],
        [pin: Pd24, peripherals: [A]],
        [pin: Pd25, peripherals: [A]],
        [pin: Pd26, peripherals: [A]],
        [pin: Pd27, peripherals: [A]],
        [pin: Pd28, peripherals: [A]],
        [pin: Pd29, peripherals: [A]],
        [pin: Pd30, peripherals: [A]],
    ],
}
