use crate::{
    pac::PIOA,
    pio::{
        def_pioc,
        filter::InputFilterCfg,
        interrupt::InterruptCfg,
        peripheral::{
            impl_peripheral_absel, MultiDriverDisabled, MultiDriverEnabled, OutputSyncWriteCfg,
            PeripheralA, PeripheralB, PeripheralControlled,
        },
        pin::{Pin, PullupResistorCfg, def_peripheral_multiplex},
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

// type CanTx0<Pupr, Irpt, Filt> =
//     Pin<PioA, Pa0, MultiDriverDisabled<PeripheralControlled<PeripheralA>>, Pupr, Irpt, Filt>;
// type CanTx0MD<Sync, Pupr, Irpt, Filt> =
//     Pin<PioA, Pa0, MultiDriverEnabled<PeripheralControlled<PeripheralA>, Sync>, Pupr, Irpt, Filt>;

def_peripheral_multiplex! {
    PioA {
        Pa0: CanTx0, PwmL3;
        Pa1: CanRx0, Pck0;
        Pa2: TioA1, NandRdy;
        Pa3: TioB1, PwmFi1;
        Pa4: TClk1, NWait;
        Pa5: TioA2, PwmFi0;
        Pa6: TioB2, Ncs0;
        Pa7: TClk2, Ncs1;
        Pa8: URxD, PwmH0;
        Pa9: UTxD, PwmH3;
    }
}
