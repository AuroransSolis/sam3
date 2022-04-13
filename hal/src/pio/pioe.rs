use crate::{
    pac::{pioe, PIOE},
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
        PioE(PIOE) => {
            #(Pe: N,)*
        }
    }
}}
