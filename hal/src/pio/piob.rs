use crate::{
    pac::{piob, PIOB},
    pio::{
        def_pioc,
        pin::{
            AdditionalInterruptModesDisabled, DetectLevels, DetectRisingEdgeHighLevel,
            InputFilterDisabled, InterruptDisabled, PeripheralA, PeripheralB, PeripheralControlled,
            Pin, SystemClockGlitchFilter,
        },
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
