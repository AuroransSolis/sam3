use crate::{
    pac::PIOC,
    pio::{def_pioc, peripheral::impl_peripheral_fnsel},
};
use seq_macro::seq;

// #[cfg(any(feature = "sam3a", feature = "sam3x"))]
// def_peripheral_multiplex! {
//     PioC {
//         Pc2: [a D0, b PwmL0],
//         Pc3: [a D1, b PwmH0],
//         Pc4: [a D2, b PwmL1],
//         Pc5: [a D3, b PwmH1],
//         Pc6: [a D4, b PwmL2],
//         Pc7: [a D5, b PwmH2],
//         Pc8: [a D6, b PwmL3],
//         Pc9: [a D7, b PwmH3],
//         Pc10: [a D8, b ECRS],
//         Pc11: [a D9, b ERx2],
//         Pc12: [a D10, b ERx3],
//         Pc13: [a D11, b ECol],
//         Pc14: [a D12, b ERxCk],
//         Pc15: [a D13, b ETx2],
//         Pc16: [a D14, b ETx3],
//         Pc17: [a D15, b ETxEr],
//         Pc18: [a NWr0NWe, b PwmH6],
//         Pc19: [a NandOE, b PwmH5],
//         Pc20: [a NandWE, b PwmH4],
//         Pc21: [a A0Nbs0, b PwmL4],
//         Pc22: [a A1, b PwmL5],
//         Pc23: [a A2, b PwmL6],
//         Pc24: [a A3, b PwmL7],
//         Pc25: [a A4, b TioA6],
//         Pc26: [a A5, b TioB6],
//         Pc27: [a A6, b TClk6],
//         Pc28: [a A7, b TioA7],
//         Pc29: [a A8, b TioB7],
//         Pc30: [a A9, b TClk7],
//     }
// }

#[cfg(feature = "sam3n100")]
seq! {N in 0..32 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(feature = "sam3n100")]
impl_peripheral_fnsel! {
    PioC {
        Pc0: [],
        Pc1: [],
        Pc2: [],
        Pc3: [],
        Pc4: [b],
        Pc5: [],
        Pc6: [],
        Pc7: [b],
        Pc8: [b],
        Pc9: [b],
        Pc10: [b],
        Pc11: [b],
        Pc12: [],
        Pc13: [],
        Pc14: [b],
        Pc15: [],
        Pc16: [b],
        Pc17: [b],
        Pc18: [b],
        Pc19: [b],
        Pc20: [b],
        Pc21: [b],
        Pc22: [b],
        Pc23: [b],
        Pc24: [b],
        Pc25: [b],
        Pc26: [b],
        Pc27: [b],
        Pc28: [b],
        Pc29: [b],
        Pc30: [b],
        Pc31: [b],
    }
}

#[cfg(feature = "sam3s100")]
seq! {N in 0..32 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(feature = "sam3s100")]
impl_peripheral_fnsel! {
    PioC {
        Pc0: [a, b],
        Pc1: [a, b],
        Pc2: [a, b],
        Pc3: [a, b],
        Pc4: [a, b],
        Pc5: [a],
        Pc6: [a],
        Pc7: [a],
        Pc8: [a],
        Pc9: [a],
        Pc10: [a],
        Pc11: [a],
        Pc12: [a],
        Pc13: [a, b],
        Pc14: [a],
        Pc15: [a, b],
        Pc16: [a],
        Pc17: [a],
        Pc18: [a, b],
        Pc19: [a, b],
        Pc20: [a, b],
        Pc21: [a, b],
        Pc22: [a, b],
        Pc23: [a, b],
        Pc24: [a, b],
        Pc25: [a, b],
        Pc26: [a, b],
        Pc27: [a, b],
        Pc28: [a, b],
        Pc29: [a, b],
        Pc30: [a, b],
        Pc31: [a, b],
    }
}

#[cfg(feature = "sam3s8100")]
seq! {N in 0..32 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(feature = "sam3s8100")]
impl_peripheral_fnsel! {
    PioC {
        Pc0: [a, b],
        Pc1: [a, b],
        Pc2: [a, b],
        Pc3: [a, b],
        Pc4: [a, b],
        Pc5: [a],
        Pc6: [a],
        Pc7: [a],
        Pc8: [a],
        Pc9: [a, b],
        Pc10: [a, b],
        Pc11: [a],
        Pc12: [a],
        Pc13: [a, b],
        Pc14: [a, b],
        Pc15: [a, b],
        Pc16: [a, b],
        Pc17: [a, b],
        Pc18: [a, b],
        Pc19: [a, b],
        Pc20: [a, b],
        Pc21: [a, b],
        Pc22: [a, b],
        Pc23: [a, b],
        Pc24: [a, b],
        Pc25: [a, b],
        Pc26: [a, b],
        Pc27: [a, b],
        Pc28: [a, b],
        Pc29: [a, b],
        Pc30: [a, b],
        Pc31: [a, b],
    }
}

#[cfg(feature = "sam3u144")]
seq! {N in 0..32 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(feature = "sam3u144")]
impl_peripheral_fnsel! {
    PioC {
        Pc0: a,
        Pc1: a,
        Pc2: a,
        Pc3: ab,
        Pc4: ab,
        Pc5: ab,
        Pc6: ab,
        Pc7: ab,
        Pc8: ab,
        Pc9: ab,
        Pc10: ab,
        Pc11: ab,
        Pc12: ab,
        Pc13: ab,
        Pc14: ab,
        Pc15: a,
        Pc16: ab,
        Pc17: a,
        Pc18: a,
        Pc19: ab,
        Pc20: a,
        Pc21: a,
        Pc22: a,
        Pc23: a,
        Pc24: ab,
        Pc25: ab,
        Pc26: ab,
        Pc27: ab,
        Pc28: b,
        Pc29: ab,
        Pc30: ab,
        Pc31: ab,
    }
}

#[cfg(any(feature = "sam3x144", feature = "sam3x217"))]
seq! {N in 0..31 {
    def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3x144", feature = "sam3x217"))]
impl_peripheral_fnsel! {
    PioC {
        Pc0: noab,
        Pc1: noab,
        Pc2: ab,
        Pc3: ab,
        Pc4: ab,
        Pc5: ab,
        Pc6: ab,
        Pc7: ab,
        Pc8: ab,
        Pc9: ab,
        Pc10: ab,
        Pc11: ab,
        Pc12: ab,
        Pc13: ab,
        Pc14: ab,
        Pc15: ab,
        Pc16: ab,
        Pc17: ab,
        Pc18: ab,
        Pc19: ab,
        Pc20: ab,
        Pc21: ab,
        Pc22: ab,
        Pc23: ab,
        Pc24: ab,
        Pc25: ab,
        Pc26: ab,
        Pc27: ab,
        Pc28: ab,
        Pc29: ab,
        Pc30: ab,
    }
}
