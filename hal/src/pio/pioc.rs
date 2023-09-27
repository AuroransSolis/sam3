#![allow(unused_imports)]

use crate::{
    pac::{pioc, PIOC},
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..31 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

impl_peripheral_absel! {
    PioC {
        Pc0: noab,
        Pc1: noab,
        Pc2: absel,
        Pc3: absel,
        Pc4: absel,
        Pc5: absel,
        Pc6: absel,
        Pc7: absel,
        Pc8: absel,
        Pc9: absel,
        Pc10: absel,
        Pc11: absel,
        Pc12: absel,
        Pc13: absel,
        Pc14: absel,
        Pc15: absel,
        Pc16: absel,
        Pc17: absel,
        Pc18: absel,
        Pc19: absel,
        Pc20: absel,
        Pc21: absel,
        Pc22: absel,
        Pc23: absel,
        Pc24: absel,
        Pc25: absel,
        Pc26: absel,
        Pc27: absel,
        Pc28: absel,
        Pc29: absel,
        Pc30: absel,
    }
}
