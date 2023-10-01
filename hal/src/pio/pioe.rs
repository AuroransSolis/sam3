use crate::{
    pac::PIOE,
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::def_peripheral_multiplex},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioE(PIOE) => {
            #(Pe: N,)*
        }
    }
}}

impl_peripheral_absel! {
    PioE {
        Pe0: asel,
        Pe1: asel,
        Pe2: asel,
        Pe3: asel,
        Pe4: asel,
        Pe5: asel,
        Pe6: asel,
        Pe9: asel,
        Pe10: asel,
        Pe11: asel,
        Pe12: asel,
        Pe13: asel,
        Pe14: asel,
        Pe15: asel,
        Pe16: absel,
        Pe17: asel,
        Pe18: absel,
        Pe19: asel,
        Pe20: absel,
        Pe21: asel,
        Pe22: absel,
        Pe23: asel,
        Pe24: absel,
        Pe25: asel,
        Pe26: absel,
        Pe27: absel,
        Pe28: asel,
        Pe29: asel,
        Pe30: asel,
        Pe31: asel,
    }
}

def_peripheral_multiplex! {
    PioE {
        Pe0: [asel A19];
        Pe1: [asel A20];
        Pe2: [asel A21NandALE];
        Pe3: [asel A22NandCLE];
        Pe4: [asel A23];
        Pe5: [asel NCS4];
        Pe6: [asel NCS5];
        Pe9: [asel TioA3];
        Pe10: [asel TioB3];
        Pe11: [asel TioA4];
        Pe12: [asel TioB4];
        Pe13: [asel TioA5];
        Pe14: [asel TioB5];
        Pe15: [asel PwmH0];
        Pe16: [PwmH1, SCk3];
        Pe17: [asel PwmL2];
        Pe18: [PwmL0, NCS6];
        Pe19: [asel PwmL4];
        Pe20: [PwmH4, MCCdB];
        Pe21: [asel PwmL5];
        Pe22: [PwmH5, MCDb0];
        Pe23: [asel PwmL6];
        Pe24: [PwmH6, MCDb1];
        Pe25: [asel PwmL7];
        Pe26: [PwmH7, MCDb2];
        Pe27: [NCS7, MCDb3];
        Pe28: [asel SPI1MISO];
        Pe29: [asel SPI1MOSI];
        Pe30: [asel SPI1SPCk];
        Pe31: [asel SPI1NPCS0];
    }
}
