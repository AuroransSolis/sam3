use crate::{
    pac::PIOA,
    pio::{def_pioc, peripheral::impl_peripheral_absel, pin::Pin},
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

// def_peripherals! {
//     A {
//         0 {
//             A: CanTx0<
//                 ! PeripheralControlled,
//                 ? Outw,
//                 ? Otpt,
//                 ? Pupr,
//                 ! InterruptDisabled,
//                 ? Mdvr,
//                 ? Odta,
//                 ! InputFilterDisabled,
//                 ! SystemClockGlitchFilter,
//                 ? Frlh
//             >,
//             B: PwmL3<
//                 ! PeripheralControlled,
//                 ? Outw,
//                 ? Otpt,
//                 ? Pupr,
//                 ! InterruptDisabled,
//                 ? Mdvr,
//                 ? Odta,
//                 ! InputFilterDisabled,
//                 ! SystemClockGlitchFilter,
//                 ? Frlh
//             >,
//         },
//     };
// }

// Type aliases to peripherals.

// type CanTx0<Outw, Outp, Pupr, Mdvr, Odta> = Pin<
//     PioA,
//     Pa0,
//     PeripheralControlled,
//     Outw,
//     Outp,
//     Pupr,
//     InterruptDisabled,
//     Mdvr,
//     PeripheralA,
//     Odta,
//     InputFilterDisabled,
//     SystemClockGlitchFilter,
//     AdditionalInterruptModesDisabled,
//     DetectLevels,
//     DetectRisingEdgeHighLevel,
// >;
// type PwmL3<Outw, Outp, Pupr, Mdvr, Odta> = Pin<
//     PioA,
//     Pa0,
//     PeripheralControlled,
//     Outw,
//     Outp,
//     Pupr,
//     InterruptDisabled,
//     Mdvr,
//     PeripheralB,
//     Odta,
//     InputFilterDisabled,
//     SystemClockGlitchFilter,
//     AdditionalInterruptModesDisabled,
//     DetectLevels,
//     DetectRisingEdgeHighLevel,
// >;
