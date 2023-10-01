use crate::{
    pac::PIOE,
    pio::{def_pioc, pin::def_peripheral_multiplex},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioE(PIOE) => {
            #(Pe: N,)*
        }
    }
}}

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
