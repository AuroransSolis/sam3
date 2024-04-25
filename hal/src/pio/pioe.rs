//! Wrapper struct for [`PIOE`] and its pins.
#![allow(unused_imports)]

use crate::{
    pac::PIOE,
    pio::peripheral::{PeripheralA, PeripheralB},
};
use seq_macro::seq;

seq! {N in 0..32 {
    crate::pio::def_pioc! {
        PioE(PIOE) => {
            #(Pe: N,)*
        }
    }
}}

crate::pio::pin_peripherals! {
    pio: PIOE,
    pinopts: [
        [pin: Pe0, peripherals: [A]],
        [pin: Pe1, peripherals: [A]],
        [pin: Pe2, peripherals: [A]],
        [pin: Pe3, peripherals: [A]],
        [pin: Pe4, peripherals: [A]],
        [pin: Pe5, peripherals: [A]],
        [pin: Pe6, peripherals: [A]],
        [pin: Pe9, peripherals: [A]],
        [pin: Pe10, peripherals: [A]],
        [pin: Pe11, peripherals: [A]],
        [pin: Pe12, peripherals: [A]],
        [pin: Pe13, peripherals: [A]],
        [pin: Pe14, peripherals: [A]],
        [pin: Pe15, peripherals: [A]],
        [pin: Pe16, peripherals: [A, B]],
        [pin: Pe17, peripherals: [A]],
        [pin: Pe18, peripherals: [A, B]],
        [pin: Pe19, peripherals: [A]],
        [pin: Pe20, peripherals: [A, B]],
        [pin: Pe21, peripherals: [A]],
        [pin: Pe22, peripherals: [A, B]],
        [pin: Pe23, peripherals: [A]],
        [pin: Pe24, peripherals: [A, B]],
        [pin: Pe25, peripherals: [A]],
        [pin: Pe26, peripherals: [A, B]],
        [pin: Pe27, peripherals: [A, B]],
        [pin: Pe28, peripherals: [A]],
        [pin: Pe29, peripherals: [A]],
        [pin: Pe30, peripherals: [A]],
        [pin: Pe31, peripherals: [A]],
    ],
}
