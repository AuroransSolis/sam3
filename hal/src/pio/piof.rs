use crate::{
    pac::PIOF,
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::def_peripheral_multiplex},
};
use seq_macro::seq;

seq! {N in 0..6 {
    def_pioc! {
        PioF(PIOF) => {
            #(Pf: N,)*
        }
    }
}}

impl_peripheral_absel! {
    PioF {
        Pf0: asel,
        Pf1: asel,
        Pf2: asel,
        Pf3: asel,
        Pf4: asel,
        Pf5: asel,
    }
}

def_peripheral_multiplex! {
    PioF {
        Pf0: [asel SPI1NPCS1];
        Pf1: [asel SPI1NPCS2];
        Pf2: [asel SPI1NPCS3];
        Pf3: [asel PwmH3];
        Pf4: [asel CTS3];
        Pf5: [asel RTS3];
    }
}
