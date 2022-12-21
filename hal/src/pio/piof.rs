use crate::{
    pac::{piof, PIOF},
    pio::{def_pioc, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioF(PIOF) => {
            #(Pf: N,)*
        }
    }
}}
