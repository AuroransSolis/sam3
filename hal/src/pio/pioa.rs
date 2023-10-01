use crate::{
    pac::PIOA,
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::def_peripheral_multiplex},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

impl_peripheral_absel! {
    PioA {
        Pa0: absel,
        Pa1: absel,
        Pa2: absel,
        Pa3: absel,
        Pa4: absel,
        Pa5: absel,
        Pa6: absel,
        Pa7: absel,
        Pa8: absel,
        Pa10: absel,
        Pa11: absel,
        Pa12: absel,
        Pa13: absel,
        Pa14: absel,
        Pa15: absel,
        Pa16: absel,
        Pa17: absel,
        Pa18: absel,
        Pa19: absel,
        Pa20: absel,
        Pa21: absel,
        Pa22: absel,
        Pa23: absel,
        Pa24: absel,
        Pa25: absel,
        Pa26: absel,
        Pa27: absel,
        Pa28: absel,
        Pa29: absel,
    }
}

#[cfg(not(feature = "sam3x8h"))]
impl_peripheral_absel! {
    PioA {
        Pa30: noab,
        Pa31: noab,
    }
}

#[cfg(feature = "sam3x8h")]
impl_peripheral_absel! {
    PioA {
        Pa30: absel,
        Pa31: absel,
    }
}

def_peripheral_multiplex! {
    PioA {
        Pa0: [CanTx0, PwmL3];
        Pa1: [CanRx0, Pck0];
        Pa2: [TioA1, NandRdy];
        Pa3: [TioB1, PwmFi1];
        Pa4: [TClk1, NWait];
        Pa5: [TioA2, PwmFi0];
        Pa6: [TioB2, Ncs0];
        Pa7: [TClk2, Ncs1];
        Pa8: [URxD, PwmH0];
        Pa9: [UTxD, PwmH3];
        Pa10: [RxD0, DaTrg];
        Pa11: [TxD0, AdTrg];
        Pa12: [RxD1, PwmL1];
        Pa13: [TxD1, PwmH2];
        Pa14: [Rts1, Tk];
        Pa15: [Cts1, Tf];
        Pa16: [SpCk1, Td];
        Pa17: [Twd0, SpCk0];
        Pa18: [TwCk0, Pa18A20];
        Pa19: [MCCk, PwmH1];
        Pa20: [MCCdA, PwmL2];
        Pa21: [MCDA0, PwmL0];
        Pa22: [MCDA1, TClk3];
        Pa23: [MCDA2, TClk4];
        Pa24: [MCDA3, Pa24PCk1];
        Pa25: [SPI0MISO, A18];
        Pa26: [SPI0MOSI, A19];
        Pa27: [SPIOSPCk, Pa27A20];
        Pa28: [SPI0NPCS0, Pck2];
        Pa29: [SPI0NPCS1, NRd];
    }
}

#[cfg(feature = "sam3x8h")]
def_peripheral_multiplex! {
    PioA {
        Pa30: [SPI0NPCS2, Pa30PCk1];
        Pa31: [SPI0NPCS3, PCk2];
    }
}
