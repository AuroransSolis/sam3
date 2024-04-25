//! Wrapper struct for [`PIOC`] and its pins.
#![allow(unused_imports)]

#[cfg(feature = "3fn")]
use crate::pio::peripheral::PeripheralC;
#[cfg(feature = "4fn")]
use crate::pio::peripheral::PeripheralD;
use crate::{
    pac::PIOC,
    pio::peripheral::{PeripheralA, PeripheralB},
};
use seq_macro::seq;

#[cfg(any(feature = "sam3a", feature = "sam3x"))]
seq! {N in 0..31 { // why is this only different for these?????????????????????
    crate::pio::def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(any(feature = "sam3x144", feature = "sam3x217"))]
crate::pio::pin_peripherals! {
    pio: PIOC,
    pinopts: [
        [pin: Pc2, peripherals: [A, B]],
        [pin: Pc3, peripherals: [A, B]],
        [pin: Pc4, peripherals: [A, B]],
        [pin: Pc5, peripherals: [A, B]],
        [pin: Pc6, peripherals: [A, B]],
        [pin: Pc7, peripherals: [A, B]],
        [pin: Pc8, peripherals: [A, B]],
        [pin: Pc9, peripherals: [A, B]],
        [pin: Pc10, peripherals: [A, B]],
        [pin: Pc11, peripherals: [A, B]],
        [pin: Pc12, peripherals: [A, B]],
        [pin: Pc13, peripherals: [A, B]],
        [pin: Pc14, peripherals: [A, B]],
        [pin: Pc15, peripherals: [A, B]],
        [pin: Pc16, peripherals: [A, B]],
        [pin: Pc17, peripherals: [A, B]],
        [pin: Pc18, peripherals: [A, B]],
        [pin: Pc19, peripherals: [A, B]],
        [pin: Pc20, peripherals: [A, B]],
        [pin: Pc21, peripherals: [A, B]],
        [pin: Pc22, peripherals: [A, B]],
        [pin: Pc23, peripherals: [A, B]],
        [pin: Pc24, peripherals: [A, B]],
        [pin: Pc25, peripherals: [A, B]],
        [pin: Pc26, peripherals: [A, B]],
        [pin: Pc27, peripherals: [A, B]],
        [pin: Pc28, peripherals: [A, B]],
        [pin: Pc29, peripherals: [A, B]],
        [pin: Pc30, peripherals: [A, B]],
    ],
}

#[cfg(any(
    feature = "sam3n100",
    feature = "sam3s100",
    feature = "sam3s8100",
    feature = "sam3u144"
))]
seq! {N in 0..32 {
    crate::pio::def_pioc! {
        PioC(PIOC) => {
            #(Pc: N,)*
        }
    }
}}

#[cfg(feature = "sam3n100")]
crate::pio::pin_peripherals! {
    pio: PIOC,
    pinopts: [
        [pin: Pc4, peripherals: [B]],
        [pin: Pc7, peripherals: [B]],
        [pin: Pc8, peripherals: [B]],
        [pin: Pc9, peripherals: [B]],
        [pin: Pc10, peripherals: [B]],
        [pin: Pc11, peripherals: [B]],
        [pin: Pc14, peripherals: [B]],
        [pin: Pc16, peripherals: [B]],
        [pin: Pc17, peripherals: [B]],
        [pin: Pc18, peripherals: [B]],
        [pin: Pc19, peripherals: [B]],
        [pin: Pc20, peripherals: [B]],
        [pin: Pc21, peripherals: [B]],
        [pin: Pc22, peripherals: [B]],
        [pin: Pc23, peripherals: [B]],
        [pin: Pc24, peripherals: [B]],
        [pin: Pc25, peripherals: [B]],
        [pin: Pc26, peripherals: [B]],
        [pin: Pc27, peripherals: [B]],
        [pin: Pc28, peripherals: [B]],
        [pin: Pc29, peripherals: [B]],
        [pin: Pc30, peripherals: [B]],
        [pin: Pc31, peripherals: [B]],
    ],
}

#[cfg(feature = "sam3s100")]
crate::pio::pin_peripherals! {
    pio: PIOC,
    pinopts: [
        [pin: Pc0, peripherals: [A, B]],
        [pin: Pc1, peripherals: [A, B]],
        [pin: Pc2, peripherals: [A, B]],
        [pin: Pc3, peripherals: [A, B]],
        [pin: Pc4, peripherals: [A, B]],
        [pin: Pc5, peripherals: [A]],
        [pin: Pc6, peripherals: [A]],
        [pin: Pc7, peripherals: [A]],
        [pin: Pc8, peripherals: [A]],
        [pin: Pc9, peripherals: [A]],
        [pin: Pc10, peripherals: [A]],
        [pin: Pc11, peripherals: [A]],
        [pin: Pc12, peripherals: [A]],
        [pin: Pc13, peripherals: [A, B]],
        [pin: Pc14, peripherals: [A]],
        [pin: Pc15, peripherals: [A, B]],
        [pin: Pc16, peripherals: [A]],
        [pin: Pc17, peripherals: [A]],
        [pin: Pc18, peripherals: [A, B]],
        [pin: Pc19, peripherals: [A, B]],
        [pin: Pc20, peripherals: [A, B]],
        [pin: Pc21, peripherals: [A, B]],
        [pin: Pc22, peripherals: [A, B]],
        [pin: Pc23, peripherals: [A, B]],
        [pin: Pc24, peripherals: [A, B]],
        [pin: Pc25, peripherals: [A, B]],
        [pin: Pc26, peripherals: [A, B]],
        [pin: Pc27, peripherals: [A, B]],
        [pin: Pc28, peripherals: [A, B]],
        [pin: Pc29, peripherals: [A, B]],
        [pin: Pc30, peripherals: [A, B]],
        [pin: Pc31, peripherals: [A, B]],
    ],
}

#[cfg(feature = "sam3s8100")]
crate::pio::pin_peripherals! {
    pio: PIOC,
    pinopts: [
        [pin: Pc0, peripherals: [A, B]],
        [pin: Pc1, peripherals: [A, B]],
        [pin: Pc2, peripherals: [A, B]],
        [pin: Pc3, peripherals: [A, B]],
        [pin: Pc4, peripherals: [A, B]],
        [pin: Pc5, peripherals: [A]],
        [pin: Pc6, peripherals: [A]],
        [pin: Pc7, peripherals: [A]],
        [pin: Pc8, peripherals: [A]],
        [pin: Pc9, peripherals: [A, B]],
        [pin: Pc10, peripherals: [A, B]],
        [pin: Pc11, peripherals: [A]],
        [pin: Pc12, peripherals: [A]],
        [pin: Pc13, peripherals: [A, B]],
        [pin: Pc14, peripherals: [A, B]],
        [pin: Pc15, peripherals: [A, B]],
        [pin: Pc16, peripherals: [A, B]],
        [pin: Pc17, peripherals: [A, B]],
        [pin: Pc18, peripherals: [A, B]],
        [pin: Pc19, peripherals: [A, B]],
        [pin: Pc20, peripherals: [A, B]],
        [pin: Pc21, peripherals: [A, B]],
        [pin: Pc22, peripherals: [A, B]],
        [pin: Pc23, peripherals: [A, B]],
        [pin: Pc24, peripherals: [A, B]],
        [pin: Pc25, peripherals: [A, B]],
        [pin: Pc26, peripherals: [A, B]],
        [pin: Pc27, peripherals: [A, B]],
        [pin: Pc28, peripherals: [A, B]],
        [pin: Pc29, peripherals: [A, B]],
        [pin: Pc30, peripherals: [A, B]],
        [pin: Pc31, peripherals: [A, B]],
    ],
}

#[cfg(feature = "sam3u144")]
crate::pio::pin_peripherals! {
    pio: PIOC,
    pinopts: [
        [pin: Pc0, peripherals: [A]],
        [pin: Pc1, peripherals: [A]],
        [pin: Pc2, peripherals: [A]],
        [pin: Pc3, peripherals: [A, B]],
        [pin: Pc4, peripherals: [A, B]],
        [pin: Pc5, peripherals: [A, B]],
        [pin: Pc6, peripherals: [A, B]],
        [pin: Pc7, peripherals: [A, B]],
        [pin: Pc8, peripherals: [A, B]],
        [pin: Pc9, peripherals: [A, B]],
        [pin: Pc10, peripherals: [A, B]],
        [pin: Pc11, peripherals: [A, B]],
        [pin: Pc12, peripherals: [A, B]],
        [pin: Pc13, peripherals: [A, B]],
        [pin: Pc14, peripherals: [A, B]],
        [pin: Pc15, peripherals: [A]],
        [pin: Pc16, peripherals: [A, B]],
        [pin: Pc17, peripherals: [A]],
        [pin: Pc18, peripherals: [A]],
        [pin: Pc19, peripherals: [A, B]],
        [pin: Pc20, peripherals: [A]],
        [pin: Pc21, peripherals: [A]],
        [pin: Pc22, peripherals: [A]],
        [pin: Pc23, peripherals: [A]],
        [pin: Pc24, peripherals: [A, B]],
        [pin: Pc25, peripherals: [A, B]],
        [pin: Pc26, peripherals: [A, B]],
        [pin: Pc27, peripherals: [A, B]],
        [pin: Pc28, peripherals: [B]],
        [pin: Pc29, peripherals: [A, B]],
        [pin: Pc30, peripherals: [A, B]],
        [pin: Pc31, peripherals: [A, B]],
    ],
}
