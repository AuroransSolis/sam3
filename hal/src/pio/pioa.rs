use crate::{
    pac::{pioa, PIOA},
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
        PioA(PIOA) => {
            #(Pa: N,)*
        }
    }
}}

// Type aliases to peripherals.

type CanTx0<Outw, Outp, Pupr, Mdvr, Odta> = Pin<
    PioA,
    Pa0,
    PeripheralControlled,
    Outw,
    Outp,
    Pupr,
    InterruptDisabled,
    Mdvr,
    PeripheralA,
    Odta,
    InputFilterDisabled,
    SystemClockGlitchFilter,
    AdditionalInterruptModesDisabled,
    DetectLevels,
    DetectRisingEdgeHighLevel,
>;
type PwmL3<Outw, Outp, Pupr, Mdvr, Odta> = Pin<
    PioA,
    Pa0,
    PeripheralControlled,
    Outw,
    Outp,
    Pupr,
    InterruptDisabled,
    Mdvr,
    PeripheralB,
    Odta,
    InputFilterDisabled,
    SystemClockGlitchFilter,
    AdditionalInterruptModesDisabled,
    DetectLevels,
    DetectRisingEdgeHighLevel,
>;
