//! Wrapper struct for [`PIOF`] and its pins.
#![allow(unused_imports)]

use crate::{pac::PIOF, pio::peripheral::PeripheralA};
use seq_macro::seq;

seq! {N in 0..6 {
    crate::pio::def_pioc! {
        PioF(PIOF) => {
            #(Pf: N,)*
        }
    }
}}

crate::pio::pin_peripherals! {
    pio: PIOF,
    pinopts: [
        [pin: Pf0, peripherals: [A]],
        [pin: Pf1, peripherals: [A]],
        [pin: Pf2, peripherals: [A]],
        [pin: Pf3, peripherals: [A]],
        [pin: Pf4, peripherals: [A]],
        [pin: Pf5, peripherals: [A]],
    ],
}
