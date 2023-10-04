use crate::{
    pac::PIOB,
    pio::{def_pioc, peripheral::impl_peripheral_fnsel},
};
use seq_macro::seq;

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
seq! {N in 0..32 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3x")]
impl_peripheral_fnsel! {
    PioB {
        Pb0: a,
        Pb1: a,
        Pb2: a,
        Pb3: a,
        Pb4: a,
        Pb5: a,
        Pb6: a,
        Pb7: a,
        Pb8: a,
        Pb9: a,
    }
}

#[cfg(feature = "sam3a")]
impl_peripheral_fnsel! {
    PioB {
        Pb0: b,
        Pb1: b,
        Pb2: b,
        Pb3: b,
        Pb4: b,
        Pb5: b,
        Pb6: b,
        Pb7: b,
        Pb8: b,
        Pb9: b,
    }
}

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
impl_peripheral_fnsel! {
    PioB {
        Pb10: ab,
        Pb11: ab,
        Pb12: ab,
        Pb13: ab,
        Pb14: ab,
        Pb15: ab,
        Pb16: ab,
        Pb17: ab,
        Pb18: ab,
        Pb19: ab,
        Pb20: ab,
        Pb21: ab,
        Pb22: ab,
        Pb23: ab,
        Pb24: ab,
        Pb25: ab,
        Pb26: ab,
        Pb27: ab,
        Pb28: a,
        Pb29: a,
        Pb30: a,
        Pb31: a,
    }
}

// #[cfg(feature = "sam3a")]
// def_peripheral_multiplex! {
//     PioB {
//         Pb0: [b TioA3],
//         Pb1: [b TioB3],
//         Pb2: [b TioA4],
//         Pb3: [b TioB4],
//         Pb4: [b TioA5],
//         Pb5: [b TioB5],
//         Pb6: [b PwmL4],
//         Pb7: [b PwmL5],
//         Pb8: [b PwmL6],
//         Pb9: [b PwmL7],
//     }
// }

// #[cfg(feature = "sam3x")]
// def_peripheral_multiplex! {
//     PioB {
//         Pb0: [a ETxCkERefCk],
//         Pb1: [a ETxEn],
//         Pb2: [a ETx0],
//         Pb3: [a ETx1],
//         Pb4: [a ECrSDVERxDV],
//         Pb5: [a ERx0],
//         Pb6: [a ERx1],
//         Pb7: [a ERxEr],
//         Pb8: [a EMDC],
//         Pb9: [a EMDio],
//     }
// }

// #[cfg(any(feature = "sam3a", feature = "sam3x"))]
// def_peripheral_multiplex! {
//     PioB {
//         Pb10: [a UOtgVBOF, b A18],
//         Pb11: [a UOtgId, b A19],
//         Pb12: [a Twd1, b PwmH0],
//         Pb13: [a TwCk1, b PwmH1],
//         Pb14: [a CanTx1, b PwmH2],
//         Pb15: [a CanRx1, b PwmH3],
//         Pb16: [a TClk5, b PwmL0],
//         Pb17: [a Rf, b PwmL1],
//         Pb18: [a Rd, b PwmL2],
//         Pb19: [a Rk, b PwmL3],
//         Pb20: [a TxD2, b SPI0NPCS1],
//         Pb21: [a RxD2, b SPI0NPCS2],
//         Pb22: [a Rts2, b Pck0],
//         Pb23: [a Cts2, b SPI0NPCS3],
//         Pb24: [a SCk2, b Ncs2],
//         Pb25: [a Rts0, b TioA0],
//         Pb26: [a Cts0, b TClk0],
//         Pb27: [a Ncs3, b TioB0],
//         Pb28: [a TCkSwClk],
//         Pb29: [a Tdi],
//         Pb30: [a TdoTraceSwO],
//         Pb31: [a TwmSwDio],
//     }
// }

#[cfg(feature = "sam3n48")]
seq! {N in 0..13 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3n64", feature = "sam3n100"))]
seq! {N in 0..15 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3n")]
impl_peripheral_fnsel! {
    PioB {
        Pb0: [a],
        Pb1: [a],
        Pb2: [a, b],
        Pb3: [a, b],
        Pb4: [a, b],
        Pb5: [a],
        Pb6: [],
        Pb7: [],
        Pb8: [],
        Pb9: [],
        Pb10: [],
        Pb11: [],
        Pb12: [],
    }
}

#[cfg(any(feature = "sam3n64", feature = "sam3n100"))]
impl_peripheral_fnsel! {
    PioB {
        Pb13: [b],
        Pb14: [a, b],
    }
}

#[cfg(feature = "sam3s48")]
seq! {N in 0..13 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3s64", feature = "sam3s100"))]
seq! {N in 0..15 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3s")]
impl_peripheral_fnsel! {
    PioB {
        Pb0: [a],
        Pb1: [a],
        Pb2: [a, b],
        Pb3: [a, b],
        Pb4: [a, b],
        Pb5: [a],
        Pb6: [],
        Pb7: [],
        Pb8: [],
        Pb9: [],
        Pb10: [],
        Pb11: [],
        Pb12: [a],
    }
}

#[cfg(any(feature = "sam3s64", feature = "sam3s100"))]
impl_peripheral_fnsel! {
    PioB {
        Pb13: [a, b],
        Pb14: [a, b],
    }
}

#[cfg(feature = "sam3s8")]
seq! {N in 0..15 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3s8")]
impl_peripheral_fnsel! {
    PioB {
        Pb0: [a],
        Pb1: [a],
        Pb2: [a, b],
        Pb3: [a, b],
        Pb4: [a, b],
        Pb5: [a, b],
        Pb6: [],
        Pb7: [],
        Pb8: [],
        Pb9: [],
        Pb10: [],
        Pb11: [],
        Pb12: [a],
        Pb13: [a, b],
        Pb14: [a, b],
    }
}

#[cfg(feature = "sam3u100")]
seq! {N in 0..25 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3u144")]
seq! {N in 0..32 {
    def_pioc! {
        PioB(PIOB) => {
            #(Pb: N,)*
        }
    }
}}

#[cfg(feature = "sam3u")]
impl_peripheral_fnsel! {
    PioB {
        Pb0: ab,
        Pb1: ab,
        Pb2: ab,
        Pb3: ab,
        Pb4: ab,
        Pb5: ab,
        Pb6: ab,
        Pb7: ab,
        Pb8: ab,
        Pb9: ab,
        Pb10: ab,
        Pb11: ab,
        Pb12: ab,
        Pb13: ab,
        Pb14: ab,
        Pb15: ab,
        Pb16: ab,
        Pb17: ab,
        Pb18: ab,
        Pb19: ab,
        Pb20: ab,
        Pb21: ab,
        Pb22: ab,
        Pb23: ab,
        Pb24: ab,
    }
}

#[cfg(feature = "sam3u144")]
impl_peripheral_fnsel! {
    PioB {
        Pb25: ab,
        Pb26: ab,
        Pb27: ab,
        Pb28: ab,
        Pb29: a,
        Pb30: a,
        Pb31: a,
    }
}
