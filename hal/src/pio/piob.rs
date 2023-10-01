#![allow(unused_imports)]

use crate::{
    pac::{piob, PIOB},
    pio::{
        def_pioc,
        peripheral::impl_peripheral_absel,
        pin::{def_peripheral_multiplex, Pin},
    },
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3x")]
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

#[cfg(feature = "sam3a")]
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

#[cfg(feature = "sam3x")]
def_peripheral_multiplex! {
    PioB {
        Pb0: [asel ETxCkERefCk];
        Pb1: [asel ETxEn];
        Pb2: [asel ETx0];
        Pb3: [asel ETx1];
        Pb4: [asel ECrSDVERxDV];
        Pb5: [asel ERx0];
        Pb6: [asel ERx1];
        Pb7: [asel ERxEr];
        Pb8: [asel EMDC];
        Pb9: [asel EMDio];
    }
}

#[cfg(feature = "sam3a")]
def_peripheral_multiplex! {
    PioB {
        Pb0: [bsel TioA3];
        Pb1: [bsel TioB3];
        Pb2: [bsel TioA4];
        Pb3: [bsel TioB4];
        Pb4: [bsel TioA5];
        Pb5: [bsel TioB5];
        Pb6: [bsel PwmL4];
        Pb7: [bsel PwmL5];
        Pb8: [bsel PwmL6];
        Pb9: [bsel PwmL7];
    }
}

def_peripheral_multiplex! {
    PioB {
        Pb10: [UOtgVBOF, A18];
        Pb11: [UOtgId, A19];
        Pb12: [Twd1, PwmH0];
        Pb13: [TwCk1, PwmH1];
        Pb14: [CanTx1, PwmH2];
        Pb15: [CanRx1, PwmH3];
        Pb16: [TClk5, PwmL0];
        Pb17: [Rf, PwmL1];
        Pb18: [Rd, PwmL2];
        Pb19: [Rk, PwmL3];
        Pb20: [TxD2, SPI0NPCS1];
        Pb21: [RxD2, SPI0NPCS2];
        Pb22: [Rts2, Pck0];
        Pb23: [Cts2, SPI0NPCS3];
        Pb24: [SCk2, Ncs2];
        Pb25: [Rts0, TioA0];
        Pb26: [Cts0, TClk0];
        Pb27: [Ncs3, TioB0];
        Pb28: [asel TCkSwClk];
        Pb29: [asel Tdi];
        Pb30: [asel TdoTraceSwO];
        Pb31: [asel TwmSwDio];
    }
}
