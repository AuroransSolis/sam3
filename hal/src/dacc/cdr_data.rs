use crate::pac::dacc::cdr::{CdrHw0Chsel, CdrHw1Chsel};

pub trait CdrData {
    fn bits(self) -> u32;
}

#[derive(Clone, Copy)]
pub struct CdrHalfword {
    data: u16,
}

const DATA_MASK: u16 = 0x0fff;
const TAG_MASK: u16 = 0xf000;
#[allow(clippy::cast_possible_truncation)]
const TAG_OFFSET: u16 = DATA_MASK.trailing_ones() as u16;

impl CdrHalfword {
    #[must_use]
    pub fn new(data: u16) -> Self {
        Self {
            data: data & DATA_MASK,
        }
    }

    #[must_use]
    /// Set the lower 12 bits (analog output value) of the CDR value.
    pub fn data(mut self, data: u16) -> Self {
        self.data = (self.data & TAG_MASK) | (data & DATA_MASK);
        self
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    /// Select the channel to output this value on. The `TAG` field of the mode register must be
    /// enabled for this to affect the output channel.
    pub fn channel(mut self, channel: HalfCdrChannel) -> Self {
        self.data = (self.data & DATA_MASK) | ((channel as u8 as u16) << TAG_OFFSET);
        self
    }

    #[must_use]
    pub fn bits(self) -> u16 {
        self.data
    }
}

impl CdrData for CdrHalfword {
    #[allow(clippy::cast_lossless)]
    fn bits(self) -> u32 {
        self.bits() as u32
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum HalfCdrChannel {
    Channel0 = 0,
    Channel1 = 1,
}

impl From<HalfCdrChannel> for CdrHw0Chsel {
    fn from(value: HalfCdrChannel) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<HalfCdrChannel> for CdrHw1Chsel {
    fn from(value: HalfCdrChannel) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[derive(Clone, Copy)]
pub struct CdrWord {
    pub lower: CdrHalfword,
    pub upper: CdrHalfword,
}

impl CdrWord {
    #[must_use]
    pub fn new(lower: CdrHalfword, upper: CdrHalfword) -> Self {
        Self { lower, upper }
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    pub fn bits(self) -> u32 {
        self.lower.bits() as u32 | ((self.upper.bits() as u32) << 16)
    }
}

impl CdrData for CdrWord {
    fn bits(self) -> u32 {
        self.bits()
    }
}
