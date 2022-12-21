#![allow(unused_imports)]

use crate::{
    pac::{piob, PIOB},
    pio::{def_pioc, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}
