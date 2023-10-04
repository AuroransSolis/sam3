use crate::{
    pac::PIOA,
    pio::{def_pioc, peripheral::impl_peripheral_fnsel},
};
use seq_macro::seq;

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
impl_peripheral_fnsel! {
    PioA {
        Pa0: ab,
        Pa1: ab,
        Pa2: ab,
        Pa3: ab,
        Pa4: ab,
        Pa5: ab,
        Pa6: ab,
        Pa7: ab,
        Pa8: ab,
        Pa10: ab,
        Pa11: ab,
        Pa12: ab,
        Pa13: ab,
        Pa14: ab,
        Pa15: ab,
        Pa16: ab,
        Pa17: ab,
        Pa18: ab,
        Pa19: ab,
        Pa20: ab,
        Pa21: ab,
        Pa22: ab,
        Pa23: ab,
        Pa24: ab,
        Pa25: ab,
        Pa26: ab,
        Pa27: ab,
        Pa28: ab,
        Pa29: ab,
    }
}

#[cfg(any(feature = "sam3a100", feature = "sam3x100", feature = "sam3x144"))]
impl_peripheral_fnsel! {
    PioA {
        Pa30: noab,
        Pa31: noab,
    }
}

#[cfg(feature = "sam3x217")]
impl_peripheral_fnsel! {
    PioA {
        Pa30: ab,
        Pa31: ab,
    }
}

// #[cfg(any(feature = "sam3a", feature = "sam3x"))]
// def_peripheral_multiplex! {
//     PioA {
//         Pa0: [a CanTx0, b PwmL3],
//         Pa1: [a CanRx0, b Pck0],
//         Pa2: [a TioA1, b NandRdy],
//         Pa3: [a TioB1, b PwmFi1],
//         Pa4: [a TClk1, b NWait],
//         Pa5: [a TioA2, b PwmFi0],
//         Pa6: [a TioB2, b Ncs0],
//         Pa7: [a TClk2, b Ncs1],
//         Pa8: [a URxD, b PwmH0],
//         Pa9: [a UTxD, b PwmH3],
//         Pa10: [a RxD0, b DaTrg],
//         Pa11: [a TxD0, b AdTrg],
//         Pa12: [a RxD1, b PwmL1],
//         Pa13: [a TxD1, b PwmH2],
//         Pa14: [a Rts1, b Tk],
//         Pa15: [a Cts1, b Tf],
//         Pa16: [a SpCk1, b Td],
//         Pa17: [a Twd0, b SpCk0],
//         Pa18: [a TwCk0, b Pa18A20],
//         Pa19: [a MCCk, b PwmH1],
//         Pa20: [a MCCdA, b PwmL2],
//         Pa21: [a MCDA0, b PwmL0],
//         Pa22: [a MCDA1, b TClk3],
//         Pa23: [a MCDA2, b TClk4],
//         Pa24: [a MCDA3, b Pa24PCk1],
//         Pa25: [a SPI0MiSo, b A18],
//         Pa26: [a SPI0MoSi, b A19],
//         Pa27: [a SPIOSPCk, b Pa27A20],
//         Pa28: [a SPI0NPCS0, b Pck2],
//         Pa29: [a SPI0NPCS1, b NRd],
//     }
// }

// #[cfg(feature = "sam3x8h")]
// def_peripheral_multiplex! {
//     PioA {
//         Pa30: [a SPI0NPCS2, b Pa30PCk1],
//         Pa31: [a SPI0NPCS3, b PCk2],
//     }
// }

#[cfg(feature = "sam3n48")]
seq! {N in 0..21 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3n64", feature = "sam3n100"))]
seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(feature = "sam3n")]
impl_peripheral_fnsel! {
    PioA {
        Pa0: [a, b],
        Pa1: [a, b],
        Pa2: [a, b, c],
        Pa3: [a, b],
        Pa4: [a, b],
        Pa5: [a, b],
        Pa6: [a, b],
        Pa7: [a, b],
        Pa8: [a, b],
        Pa9: [a, b],
        Pa10: [a, b],
        Pa11: [a, b],
        Pa12: [a, b],
        Pa13: [a, b],
        Pa14: [a, b],
        Pa15: [b],
        Pa16: [b],
        Pa17: [b],
        Pa18: [b],
        Pa19: [],
        Pa20: [],
    }
}

#[cfg(any(feature = "sam3n64", feature = "sam3n100"))]
impl_peripheral_fnsel! {
    PioA {
        Pa21: [a, b],
        Pa22: [a, b],
        Pa23: [a, b],
        Pa24: [a, b],
        Pa25: [a, b],
        Pa26: [b],
        Pa27: [b],
        Pa28: [b],
        Pa29: [b],
        Pa30: [b],
        Pa31: [a, b],
    }
}

// #[cfg(feature = "sam3n")]
// def_peripheral_multiplex! {
//     PioA {
//         Pa0: [a Pa0Pwm0, b TioA0],
//         Pa1: [a Pa1Pwm1, b TioB0],
//         Pa2: [a Pa2Pwm2, b SCk0, c DatRG],
//         Pa3: [a Twd0, b Pa3NPCS3],
//         Pa4: [a TWCk0, b TClk0],
//         Pa5: [a RxD0, b Pa5NPCS3],
//         Pa6: [a TxD0, b PCk0],
//         Pa7: [a Rts0, b Pwm3],
//         Pa8: [a Cts0, b AdTrg],
//         Pa9: [a URxD0, b NPCS1],
//         Pa10: [a UTxD0, b NPCS2],
//         Pa11: [a NPCS0, b Pa11Pwm0],
//         Pa12: [a MiSo, b Pa12Pwm1],
//         Pa13: [a MoSi, b Pa13Pwm2],
//         Pa14: [a ]
//     }
// }

#[cfg(feature = "sam3s48")]
seq! {N in 0..21 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3s64", feature = "sam3s100"))]
seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(feature = "sam3s")]
impl_peripheral_fnsel! {
    PioA {
        Pa0: [a, b, c],
        Pa1: [a, b, c],
        Pa2: [a, b, c],
        Pa3: [a, b],
        Pa4: [a, b],
        Pa5: [a, b],
        Pa6: [a, b],
        Pa7: [a, b],
        Pa8: [a, b],
        Pa9: [a, b, c],
        Pa10: [a, b],
        Pa11: [a, b],
        Pa12: [a, b],
        Pa13: [a, b],
        Pa14: [a, b],
        Pa15: [a, b, c],
        Pa16: [a, b, c],
        Pa17: [a, b, c],
        Pa18: [a, b, c],
        Pa19: [a, b, c],
        Pa20: [a, b, c],
    }
}

#[cfg(feature = "sam3s48")]
impl_peripheral_fnsel! {
    PioA {
        Pa21: [],
        Pa22: [],
        Pa23: [],
        Pa24: [],
        Pa25: [],
        Pa26: [],
        Pa27: [],
        Pa28: [],
        Pa29: [],
        Pa30: [],
        Pa31: [],
    }
}

#[cfg(any(feature = "sam3s64", feature = "sam3s100"))]
impl_peripheral_fnsel! {
    PioA {
        Pa21: [a, b],
        Pa22: [a, b, c],
        Pa23: [a, b, c],
        Pa24: [a, b, c],
        Pa25: [a, b, c],
        Pa26: [a, b, c],
        Pa27: [a, b, c],
        Pa28: [a, b, c],
        Pa29: [a, b, c],
        Pa30: [a, b, c],
        Pa31: [a, b, c],
    }
}

#[cfg(feature = "sam3s8")]
seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(feature = "sam3s8")]
impl_peripheral_fnsel! {
    PioA {
        Pa0: [a, b, c],
        Pa1: [a, b, c],
        Pa2: [a, b, c],
        Pa3: [a, b],
        Pa4: [a, b],
        Pa5: [a, b],
        Pa6: [a, b],
        Pa7: [a, b],
        Pa8: [a, b],
        Pa9: [a, b, c],
        Pa10: [a, b],
        Pa11: [a, b],
        Pa12: [a, b],
        Pa13: [a, b],
        Pa14: [a, b],
        Pa15: [a, b, c, d],
        Pa16: [a, b, c, d],
        Pa17: [a, b, c],
        Pa18: [a, b, c],
        Pa19: [a, b, c],
        Pa20: [a, b, c],
        Pa21: [a, b],
        Pa22: [a, b, c],
        Pa23: [a, b, c, d],
        Pa24: [a, b, c, d],
        Pa25: [a, b, c, d],
        Pa26: [a, b, c, d],
        Pa27: [a, b, c, d],
        Pa28: [a, b, c, d],
        Pa29: [a, b, c, d],
        Pa30: [a, b, c, d],
        Pa31: [a, b, c, d],
    }
}

#[cfg(feature = "sam3u")]
seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

#[cfg(feature = "sam3u")]
impl_peripheral_fnsel! {
    PioA {
        Pa0: ab,
        Pa1: ab,
        Pa2: ab,
        Pa3: ab,
        Pa4: ab,
        Pa5: ab,
        Pa6: ab,
        Pa7: ab,
        Pa8: ab,
        Pa9: ab,
        Pa10: ab,
        Pa11: ab,
        Pa12: ab,
        Pa13: a,
        Pa14: a,
        Pa15: ab,
        Pa16: ab,
        Pa17: ab,
        Pa18: ab,
        Pa19: ab,
        Pa20: ab,
        Pa21: ab,
        Pa22: ab,
        Pa23: ab,
        Pa26: ab,
        Pa27: ab,
        Pa28: ab,
        Pa29: ab,
        Pa30: ab,
        Pa31: ab,
    }
}

#[cfg(feature = "sam3u100")]
impl_peripheral_fnsel! {
    PioA {
        Pa24: b,
        Pa25: b,
    }
}

#[cfg(feature = "sam3u144")]
impl_peripheral_fnsel! {
    PioA {
        Pa24: ab,
        Pa25: ab,
    }
}

// #[cfg(feature = "sam3u")]
// def_peripheral_multiplex! {
//     PioA {
//         Pa0: [a TioB0, b Np],
//     }
// }
