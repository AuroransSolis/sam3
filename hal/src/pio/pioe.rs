use crate::{
    pac::PIOE,
    pio::{def_pioc, peripheral::impl_peripheral_fnsel},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioE(PIOE) => {
            #(Pe: N,)*
        }
    }
}}

impl_peripheral_fnsel! {
    PioE {
        Pe0: a,
        Pe1: a,
        Pe2: a,
        Pe3: a,
        Pe4: a,
        Pe5: a,
        Pe6: a,
        Pe9: a,
        Pe10: a,
        Pe11: a,
        Pe12: a,
        Pe13: a,
        Pe14: a,
        Pe15: a,
        Pe16: ab,
        Pe17: a,
        Pe18: ab,
        Pe19: a,
        Pe20: ab,
        Pe21: a,
        Pe22: ab,
        Pe23: a,
        Pe24: ab,
        Pe25: a,
        Pe26: ab,
        Pe27: ab,
        Pe28: a,
        Pe29: a,
        Pe30: a,
        Pe31: a,
    }
}

// def_peripheral_multiplex! {
//     PioE {
//         Pe0: [a A19],
//         Pe1: [a A20],
//         Pe2: [a A21NandALE],
//         Pe3: [a A22NandCLE],
//         Pe4: [a A23],
//         Pe5: [a NCS4],
//         Pe6: [a NCS5],
//         Pe9: [a TioA3],
//         Pe10: [a TioB3],
//         Pe11: [a TioA4],
//         Pe12: [a TioB4],
//         Pe13: [a TioA5],
//         Pe14: [a TioB5],
//         Pe15: [a PwmH0],
//         Pe16: [a PwmH1, b SCk3],
//         Pe17: [a PwmL2],
//         Pe18: [a PwmL0, b NCS6],
//         Pe19: [a PwmL4],
//         Pe20: [a PwmH4, b MCCdB],
//         Pe21: [a PwmL5],
//         Pe22: [a PwmH5, b MCDb0],
//         Pe23: [a PwmL6],
//         Pe24: [a PwmH6, b MCDb1],
//         Pe25: [a PwmL7],
//         Pe26: [a PwmH7, b MCDb2],
//         Pe27: [a NCS7, b MCDb3],
//         Pe28: [a SPI1MISO],
//         Pe29: [a SPI1MOSI],
//         Pe30: [a SPI1SPCk],
//         Pe31: [a SPI1NPCS0],
//     }
// }
