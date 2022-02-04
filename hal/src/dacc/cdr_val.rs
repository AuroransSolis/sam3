use crate::pac::dacc::mr::USER_SEL_A;

#[derive(Clone, Copy)]
pub struct HalfCdrVal {
    data: u16,
}

const DATA_MASK: u16 = 0x0fff;
const TAG_MASK: u16 = 0x2000;

impl HalfCdrVal {
    pub fn new() -> Self {
        Self {
            data: 0,
        }
    }

    /// Set the lower 12 bits (analog output value) of the CDR value.
    pub fn dac_val(mut self, dac_val: u16) -> Self {
        self.data &= !DATA_MASK;
        self.data |= dac_val & DATA_MASK;
        self
    }

    pub fn tag(mut self, tag: USER_SEL_A) -> Self {
        self.data &= !TAG_MASK;
        self.data |= (tag as u16) << 12;
        self
    }

    pub fn bits(self) -> u32 {
        self.data as u32
    }
}

#[derive(Clone, Copy)]
pub struct CdrVal {
    lower: HalfCdrVal,
    upper: HalfCdrVal,
}

impl CdrVal {
    pub fn new() -> Self {
        Self {
            lower: HalfCdrVal::new(),
            upper: HalfCdrVal::new(),
        }
    }

    pub fn lower(mut self, lower: HalfCdrVal) -> Self {
        self.lower = lower;
        self
    }

    pub fn upper(mut self, upper: HalfCdrVal) -> Self {
        self.upper = upper;
        self
    }

    pub fn bits(self) -> u32 {
        self.lower.bits() + (self.upper.bits() << 16)
    }
}
