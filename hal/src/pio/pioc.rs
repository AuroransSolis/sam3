use crate::{
    pac::PIOC,
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::def_peripheral_multiplex},
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

def_peripheral_multiplex! {
    PioC {
        Pc2: [D0, PwmL0];
        Pc3: [D1, PwmH0];
        Pc4: [D2, PwmL1];
        Pc5: [D3, PwmH1];
        Pc6: [D4, PwmL2];
        Pc7: [D5, PwmH2];
        Pc8: [D6, PwmL3];
        Pc9: [D7, PwmH3];
        Pc10: [D8, ECRS];
        Pc11: [D9, ERx2];
        Pc12: [D10, ERx3];
        Pc13: [D11, ECol];
        Pc14: [D12, ERxCk];
        Pc15: [D13, ETx2];
        Pc16: [D14, ETx3];
        Pc17: [D15, ETxEr];
        Pc18: [NWr0NWe, PwmH6];
        Pc19: [NandOE, PwmH5];
        Pc20: [NandWE, PwmH4];
        Pc21: [A0Nbs0, PwmL4];
        Pc22: [A1, PwmL5];
        Pc23: [A2, PwmL6];
        Pc24: [A3, PwmL7];
        Pc25: [A4, TioA6];
        Pc26: [A5, TioB6];
        Pc27: [A6, TClk6];
        Pc28: [A7, TioA7];
        Pc29: [A8, TioB7];
        Pc30: [A9, TClk7];
    }
}
