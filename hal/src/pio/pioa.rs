use crate::{
    pac::{pioa, PIOA},
    pio::{def_pioc, pin::Pin},
};
use seq_macro::seq;

seq! {N in 0..32 {
    def_pioc! {
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

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
