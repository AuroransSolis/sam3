use crate::{
    pac::PIOF,
    pio::{def_pioc, peripheral::impl_peripheral_fnsel},
};
use seq_macro::seq;

seq! {N in 0..6 {
    def_pioc! {
        PioF(PIOF) => {
            #(Pf: N,)*
        }
    }
}}

impl_peripheral_fnsel! {
    PioF {
        Pf0: a,
        Pf1: a,
        Pf2: a,
        Pf3: a,
        Pf4: a,
        Pf5: a,
    }
}

// def_peripheral_multiplex! {
//     PioF {
//         Pf0: [a SPI1NPCS1],
//         Pf1: [a SPI1NPCS2],
//         Pf2: [a SPI1NPCS3],
//         Pf3: [a PwmH3],
//         Pf4: [a CTS3],
//         Pf5: [a RTS3],
//     }
// }
