#![allow(unused_imports)]

use crate::{
    pac::{piod, PIOD},
    pio::{def_pioc, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioD(PIOD) => {
            #(Pd: N,)*
        }
    }
}}
