use crate::{
    pac::{pioa, PIOA},
    pio::Pioc,
};

pub struct PioA {
    pioa: PIOA,
}

impl Pioc<PIOA> {
    pub fn new(pio: PIOA) -> Self {
        Self { pio }
    }
}
