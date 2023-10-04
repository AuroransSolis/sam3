use crate::{
    pac::PIOD,
    pio::{def_pioc, peripheral::impl_peripheral_fnsel},
};
use seq_macro::seq;

#[cfg(any(feature = "sam3x144"))]
seq! {N in 0..11 {
    def_pioc! {
        PioD(PIOD) => {
            #(Pd: N,)*
        }
    }
}}

#[cfg(feature = "sam3x217")]
seq! {N in 0..31 {
    def_pioc! {
        PioD(PIOD) => {
            #(Pd: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3x144", feature = "sam3x217"))]
impl_peripheral_fnsel! {
    PioD {
        Pd0: ab,
        Pd1: ab,
        Pd2: ab,
        Pd3: ab,
        Pd4: ab,
        Pd5: ab,
        Pd6: ab,
        Pd7: ab,
        Pd8: ab,
        Pd9: ab,
        Pd10: a,
    }
}

#[cfg(feature = "sam3x217")]
impl_peripheral_fnsel! {
    PioD {
        Pd11: a,
        Pd12: a,
        Pd13: a,
        Pd14: a,
        Pd15: a,
        Pd16: a,
        Pd17: a,
        Pd18: a,
        Pd19: a,
        Pd20: a,
        Pd21: a,
        Pd22: a,
        Pd23: a,
        Pd24: a,
        Pd25: a,
        Pd26: a,
        Pd27: a,
        Pd28: a,
        Pd29: a,
        Pd30: a,
    }
}

// def_peripheral_multiplex! {
//     PioD {
//         Pd0: [a Pd0A10, b MCDA4],
//         Pd1: [a Pd1A11, b MCDA5],
//         Pd2: [a Pd2A12, b MCDA6],
//         Pd3: [a Pd3A13, b MCDA7],
//         Pd4: [a Pd4A14, b TxD3],
//         Pd5: [a Pd5A15, b RxD3],
//         Pd6: [a Pd6A16Ba0, b PwmFi2],
//         Pd7: [a Pd7A17Ba0, b TioA8],
//         Pd8: [a A21NandALE, b TioB8],
//         Pd9: [a A22NandCLE, b TClk8],
//         Pd10: [a NWr1Nbs1],
//     }
// }

// #[cfg(feature = "sam3x8h")]
// def_peripheral_multiplex! {
//     PioD {
//         Pd11: [a Sda10],
//         Pd12: [a SDCS],
//         Pd13: [a SDCkE],
//         Pd14: [a SDWE],
//         Pd15: [a RAS],
//         Pd16: [a CAS],
//         Pd17: [a A5],
//         Pd18: [a A6],
//         Pd19: [a A7],
//         Pd20: [a A8],
//         Pd21: [a A9],
//         Pd22: [a Pd22A10],
//         Pd23: [a Pd23A11],
//         Pd24: [a Pd24A12],
//         Pd25: [a Pd25A13],
//         Pd26: [a Pd26A14],
//         Pd27: [a Pd27A15],
//         Pd28: [a Pd28A16Ba0],
//         Pd29: [a Pd29A17Ba1],
//         Pd30: [a A18],
//     }
// }
