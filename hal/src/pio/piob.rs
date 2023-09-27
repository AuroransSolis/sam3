#![allow(unused_imports)]

use crate::{
    pac::{piob, PIOB},
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3a")]
impl_peripheral_absel! {
    PioB {
        Pb0: asel,
        Pb1: asel,
        Pb2: asel,
        Pb3: asel,
        Pb4: asel,
        Pb5: asel,
        Pb6: asel,
        Pb7: asel,
        Pb8: asel,
        Pb9: asel,
    }
}

#[cfg(feature = "sam3x")]
impl_peripheral_absel! {
    PioB {
        Pb0: bsel,
        Pb1: bsel,
        Pb2: bsel,
        Pb3: bsel,
        Pb4: bsel,
        Pb5: bsel,
        Pb6: bsel,
        Pb7: bsel,
        Pb8: bsel,
        Pb9: bsel,
    }
}

impl_peripheral_absel! {
    PioB {
        Pb10: absel,
        Pb11: absel,
        Pb12: absel,
        Pb13: absel,
        Pb14: absel,
        Pb15: absel,
        Pb16: absel,
        Pb17: absel,
        Pb18: absel,
        Pb19: absel,
        Pb20: absel,
        Pb21: absel,
        Pb22: absel,
        Pb23: absel,
        Pb24: absel,
        Pb25: absel,
        Pb26: absel,
        Pb27: absel,
        Pb28: asel,
        Pb29: asel,
        Pb30: asel,
        Pb31: asel,
    }
}
