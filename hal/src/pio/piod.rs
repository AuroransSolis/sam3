use crate::{
    pac::PIOD,
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::def_peripheral_multiplex},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioD(PIOD) => {
            #(Pd: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
impl_peripheral_absel! {
    PioD {
        Pd0: absel,
        Pd1: absel,
        Pd2: absel,
        Pd3: absel,
        Pd4: absel,
        Pd5: absel,
        Pd6: absel,
        Pd7: absel,
        Pd8: absel,
        Pd9: absel,
    }
}

#[cfg(feature = "sam3x8h")]
impl_peripheral_absel! {
    PioD {
        Pd10: asel,
        Pd11: asel,
        Pd12: asel,
        Pd13: asel,
        Pd14: asel,
        Pd15: asel,
        Pd16: asel,
        Pd17: asel,
        Pd18: asel,
        Pd19: asel,
        Pd20: asel,
        Pd21: asel,
        Pd22: asel,
        Pd23: asel,
        Pd24: asel,
        Pd25: asel,
        Pd26: asel,
        Pd27: asel,
        Pd28: asel,
        Pd29: asel,
        Pd30: asel,
    }
}

def_peripheral_multiplex! {
    PioD {
        Pd0: [Pd0A10, MCDA4];
        Pd1: [Pd1A11, MCDA5];
        Pd2: [Pd2A12, MCDA6];
        Pd3: [Pd3A13, MCDA7];
        Pd4: [Pd4A14, TxD3];
        Pd5: [Pd5A15, RxD3];
        Pd6: [Pd6A16Ba0, PwmFi2];
        Pd7: [Pd7A17Ba0, TioA8];
        Pd8: [A21NandALE, TioB8];
        Pd9: [A22NandCLE, TClk8];
        Pd10: [asel NWr1Nbs1];
    }
}

#[cfg(feature = "sam3x8h")]
def_peripheral_multiplex! {
    PioD {
        Pd11: [asel Sda10];
        Pd12: [asel SDCS];
        Pd13: [asel SDCkE];
        Pd14: [asel SDWE];
        Pd15: [asel RAS];
        Pd16: [asel CAS];
        Pd17: [asel A5];
        Pd18: [asel A6];
        Pd19: [asel A7];
        Pd20: [asel A8];
        Pd21: [asel A9];
        Pd22: [asel Pd22A10];
        Pd23: [asel Pd23A11];
        Pd24: [asel Pd24A12];
        Pd25: [asel Pd25A13];
        Pd26: [asel Pd26A14];
        Pd27: [asel Pd27A15];
        Pd28: [asel Pd28A16Ba0];
        Pd29: [asel Pd29A17Ba1];
        Pd30: [asel A18];
    }
}
