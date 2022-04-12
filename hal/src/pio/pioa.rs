use crate::{
    pac::{pioa, PIOA},
    pio::{
        def_pioc,
        pin::{
            AdditionalInterruptModesDisabled, DetectLevels, DetectRisingEdgeHighLevel,
            InputFilterDisabled, InterruptDisabled, PeripheralA, PeripheralControlled, Pin,
            SystemClockGlitchFilter,
        },
    },
};

def_pioc! {
    PioA(PIOA) => {
        Pa0,
        Pa1,
        Pa2,
        Pa3,
        Pa4,
        Pa5,
        Pa6,
        Pa7,
        Pa8,
        Pa9,
        Pa10,
        Pa11,
        Pa12,
        Pa13,
        Pa14,
        Pa15,
        Pa16,
        Pa17,
        Pa18,
        Pa19,
        Pa20,
        Pa21,
        Pa22,
        Pa23,
        Pa24,
        Pa25,
        Pa26,
        Pa27,
        Pa28,
        Pa29,
        Pa30,
        Pa31,
    }
}

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
