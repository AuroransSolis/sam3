#![allow(unused_imports)]

use crate::{
    pac::{pioc, PIOC},
    pio::{def_pioc, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}
