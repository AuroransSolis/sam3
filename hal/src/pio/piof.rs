use crate::{
    pac::{piof, PIOF},
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
        PioF(PIOF) => {
            #(Pf: N,)*
        }
    }
}}
