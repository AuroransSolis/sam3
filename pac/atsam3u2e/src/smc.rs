#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC NFC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - SMC NFC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SMC NFC Status Register"]
    pub sr: SR,
    #[doc = "0x0c - SMC NFC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x10 - SMC NFC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x14 - SMC NFC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x18 - SMC NFC Address Cycle Zero Register"]
    pub addr: ADDR,
    #[doc = "0x1c - SMC Bank Address Register"]
    pub bank: BANK,
    #[doc = "0x20 - SMC ECC Control Register"]
    pub ecc_ctrl: ECC_CTRL,
    #[doc = "0x24 - SMC ECC Mode Register"]
    pub ecc_md: ECC_MD,
    #[doc = "0x28 - SMC ECC Status 1 Register"]
    pub ecc_sr1: ECC_SR1,
    _reserved_11_ecc_pr0: [u8; 0x04],
    _reserved_12_ecc_pr1: [u8; 0x04],
    #[doc = "0x34 - SMC ECC status 2 Register"]
    pub ecc_sr2: ECC_SR2,
    _reserved_14_ecc_pr2: [u8; 0x04],
    _reserved_15_ecc_pr3: [u8; 0x04],
    _reserved_16_ecc_pr4: [u8; 0x04],
    _reserved_17_ecc_pr5: [u8; 0x04],
    _reserved_18_ecc_pr6: [u8; 0x04],
    _reserved_19_ecc_pr7: [u8; 0x04],
    #[doc = "0x50 - SMC ECC parity 8 Register"]
    pub ecc_pr8: ECC_PR8,
    #[doc = "0x54 - SMC ECC parity 9 Register"]
    pub ecc_pr9: ECC_PR9,
    #[doc = "0x58 - SMC ECC parity 10 Register"]
    pub ecc_pr10: ECC_PR10,
    #[doc = "0x5c - SMC ECC parity 11 Register"]
    pub ecc_pr11: ECC_PR11,
    #[doc = "0x60 - SMC ECC parity 12 Register"]
    pub ecc_pr12: ECC_PR12,
    #[doc = "0x64 - SMC ECC parity 13 Register"]
    pub ecc_pr13: ECC_PR13,
    #[doc = "0x68 - SMC ECC parity 14 Register"]
    pub ecc_pr14: ECC_PR14,
    #[doc = "0x6c - SMC ECC parity 15 Register"]
    pub ecc_pr15: ECC_PR15,
    #[doc = "0x70 - SMC Setup Register (CS_number = 0)"]
    pub setup0: SETUP0,
    #[doc = "0x74 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: PULSE0,
    #[doc = "0x78 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: CYCLE0,
    #[doc = "0x7c - SMC Timings Register (CS_number = 0)"]
    pub timings0: TIMINGS0,
    #[doc = "0x80 - SMC Mode Register (CS_number = 0)"]
    pub mode0: MODE0,
    #[doc = "0x84 - SMC Setup Register (CS_number = 1)"]
    pub setup1: SETUP1,
    #[doc = "0x88 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: PULSE1,
    #[doc = "0x8c - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: CYCLE1,
    #[doc = "0x90 - SMC Timings Register (CS_number = 1)"]
    pub timings1: TIMINGS1,
    #[doc = "0x94 - SMC Mode Register (CS_number = 1)"]
    pub mode1: MODE1,
    #[doc = "0x98 - SMC Setup Register (CS_number = 2)"]
    pub setup2: SETUP2,
    #[doc = "0x9c - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: PULSE2,
    #[doc = "0xa0 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: CYCLE2,
    #[doc = "0xa4 - SMC Timings Register (CS_number = 2)"]
    pub timings2: TIMINGS2,
    #[doc = "0xa8 - SMC Mode Register (CS_number = 2)"]
    pub mode2: MODE2,
    #[doc = "0xac - SMC Setup Register (CS_number = 3)"]
    pub setup3: SETUP3,
    #[doc = "0xb0 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: PULSE3,
    #[doc = "0xb4 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: CYCLE3,
    #[doc = "0xb8 - SMC Timings Register (CS_number = 3)"]
    pub timings3: TIMINGS3,
    #[doc = "0xbc - SMC Mode Register (CS_number = 3)"]
    pub mode3: MODE3,
    _reserved48: [u8; 0x50],
    #[doc = "0x110 - SMC OCMS Register"]
    pub ocms: OCMS,
    #[doc = "0x114 - SMC OCMS KEY1 Register"]
    pub key1: KEY1,
    #[doc = "0x118 - SMC OCMS KEY2 Register"]
    pub key2: KEY2,
    _reserved51: [u8; 0xc8],
    #[doc = "0x1e4 - Write Protection Control Register"]
    pub wpcr: WPCR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
impl RegisterBlock {
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr0_w8bit(&self) -> &W8BIT_ECC_PR0_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub const fn w9bit_ecc_pr0_w9bit(&self) -> &W9BIT_ECC_PR0_W9BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub const fn ecc_pr0(&self) -> &ECC_PR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr1_w8bit(&self) -> &W8BIT_ECC_PR1_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub const fn w9bit_ecc_pr1_w9bit(&self) -> &W9BIT_ECC_PR1_W9BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub const fn ecc_pr1(&self) -> &ECC_PR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr2_w8bit(&self) -> &W8BIT_ECC_PR2_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub const fn ecc_pr2(&self) -> &ECC_PR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr3_w8bit(&self) -> &W8BIT_ECC_PR3_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub const fn ecc_pr3(&self) -> &ECC_PR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60usize).cast() }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr4_w8bit(&self) -> &W8BIT_ECC_PR4_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub const fn ecc_pr4(&self) -> &ECC_PR4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr5_w8bit(&self) -> &W8BIT_ECC_PR5_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub const fn ecc_pr5(&self) -> &ECC_PR5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr6_w8bit(&self) -> &W8BIT_ECC_PR6_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub const fn ecc_pr6(&self) -> &ECC_PR6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr7_w8bit(&self) -> &W8BIT_ECC_PR7_W8BIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub const fn ecc_pr7(&self) -> &ECC_PR7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "SMC NFC Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SMC NFC Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SMC NFC Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SMC NFC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "SMC NFC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "SMC NFC Interrupt Mask Register"]
pub mod imr;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SMC NFC Address Cycle Zero Register"]
pub mod addr;
#[doc = "BANK (rw) register accessor: an alias for `Reg<BANK_SPEC>`"]
pub type BANK = crate::Reg<bank::BANK_SPEC>;
#[doc = "SMC Bank Address Register"]
pub mod bank;
#[doc = "ECC_CTRL (w) register accessor: an alias for `Reg<ECC_CTRL_SPEC>`"]
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
#[doc = "SMC ECC Control Register"]
pub mod ecc_ctrl;
#[doc = "ECC_MD (rw) register accessor: an alias for `Reg<ECC_MD_SPEC>`"]
pub type ECC_MD = crate::Reg<ecc_md::ECC_MD_SPEC>;
#[doc = "SMC ECC Mode Register"]
pub mod ecc_md;
#[doc = "ECC_SR1 (r) register accessor: an alias for `Reg<ECC_SR1_SPEC>`"]
pub type ECC_SR1 = crate::Reg<ecc_sr1::ECC_SR1_SPEC>;
#[doc = "SMC ECC Status 1 Register"]
pub mod ecc_sr1;
#[doc = "ECC_PR0 (r) register accessor: an alias for `Reg<ECC_PR0_SPEC>`"]
pub type ECC_PR0 = crate::Reg<ecc_pr0::ECC_PR0_SPEC>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0;
#[doc = "W9BIT_ECC_PR0_W9BIT (r) register accessor: an alias for `Reg<W9BIT_ECC_PR0_W9BIT_SPEC>`"]
pub type W9BIT_ECC_PR0_W9BIT = crate::Reg<w9bit_ecc_pr0_w9bit::W9BIT_ECC_PR0_W9BIT_SPEC>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod w9bit_ecc_pr0_w9bit;
#[doc = "W8BIT_ECC_PR0_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR0_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR0_W8BIT = crate::Reg<w8bit_ecc_pr0_w8bit::W8BIT_ECC_PR0_W8BIT_SPEC>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod w8bit_ecc_pr0_w8bit;
#[doc = "ECC_PR1 (r) register accessor: an alias for `Reg<ECC_PR1_SPEC>`"]
pub type ECC_PR1 = crate::Reg<ecc_pr1::ECC_PR1_SPEC>;
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1;
#[doc = "W9BIT_ECC_PR1_W9BIT (r) register accessor: an alias for `Reg<W9BIT_ECC_PR1_W9BIT_SPEC>`"]
pub type W9BIT_ECC_PR1_W9BIT = crate::Reg<w9bit_ecc_pr1_w9bit::W9BIT_ECC_PR1_W9BIT_SPEC>;
#[doc = "SMC ECC parity 1 Register"]
pub mod w9bit_ecc_pr1_w9bit;
#[doc = "W8BIT_ECC_PR1_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR1_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR1_W8BIT = crate::Reg<w8bit_ecc_pr1_w8bit::W8BIT_ECC_PR1_W8BIT_SPEC>;
#[doc = "SMC ECC parity 1 Register"]
pub mod w8bit_ecc_pr1_w8bit;
#[doc = "ECC_SR2 (r) register accessor: an alias for `Reg<ECC_SR2_SPEC>`"]
pub type ECC_SR2 = crate::Reg<ecc_sr2::ECC_SR2_SPEC>;
#[doc = "SMC ECC status 2 Register"]
pub mod ecc_sr2;
#[doc = "ECC_PR2 (r) register accessor: an alias for `Reg<ECC_PR2_SPEC>`"]
pub type ECC_PR2 = crate::Reg<ecc_pr2::ECC_PR2_SPEC>;
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2;
#[doc = "W8BIT_ECC_PR2_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR2_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR2_W8BIT = crate::Reg<w8bit_ecc_pr2_w8bit::W8BIT_ECC_PR2_W8BIT_SPEC>;
#[doc = "SMC ECC parity 2 Register"]
pub mod w8bit_ecc_pr2_w8bit;
#[doc = "ECC_PR3 (r) register accessor: an alias for `Reg<ECC_PR3_SPEC>`"]
pub type ECC_PR3 = crate::Reg<ecc_pr3::ECC_PR3_SPEC>;
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3;
#[doc = "W8BIT_ECC_PR3_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR3_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR3_W8BIT = crate::Reg<w8bit_ecc_pr3_w8bit::W8BIT_ECC_PR3_W8BIT_SPEC>;
#[doc = "SMC ECC parity 3 Register"]
pub mod w8bit_ecc_pr3_w8bit;
#[doc = "ECC_PR4 (r) register accessor: an alias for `Reg<ECC_PR4_SPEC>`"]
pub type ECC_PR4 = crate::Reg<ecc_pr4::ECC_PR4_SPEC>;
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4;
#[doc = "W8BIT_ECC_PR4_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR4_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR4_W8BIT = crate::Reg<w8bit_ecc_pr4_w8bit::W8BIT_ECC_PR4_W8BIT_SPEC>;
#[doc = "SMC ECC parity 4 Register"]
pub mod w8bit_ecc_pr4_w8bit;
#[doc = "ECC_PR5 (r) register accessor: an alias for `Reg<ECC_PR5_SPEC>`"]
pub type ECC_PR5 = crate::Reg<ecc_pr5::ECC_PR5_SPEC>;
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5;
#[doc = "W8BIT_ECC_PR5_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR5_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR5_W8BIT = crate::Reg<w8bit_ecc_pr5_w8bit::W8BIT_ECC_PR5_W8BIT_SPEC>;
#[doc = "SMC ECC parity 5 Register"]
pub mod w8bit_ecc_pr5_w8bit;
#[doc = "ECC_PR6 (r) register accessor: an alias for `Reg<ECC_PR6_SPEC>`"]
pub type ECC_PR6 = crate::Reg<ecc_pr6::ECC_PR6_SPEC>;
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6;
#[doc = "W8BIT_ECC_PR6_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR6_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR6_W8BIT = crate::Reg<w8bit_ecc_pr6_w8bit::W8BIT_ECC_PR6_W8BIT_SPEC>;
#[doc = "SMC ECC parity 6 Register"]
pub mod w8bit_ecc_pr6_w8bit;
#[doc = "ECC_PR7 (r) register accessor: an alias for `Reg<ECC_PR7_SPEC>`"]
pub type ECC_PR7 = crate::Reg<ecc_pr7::ECC_PR7_SPEC>;
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7;
#[doc = "W8BIT_ECC_PR7_W8BIT (r) register accessor: an alias for `Reg<W8BIT_ECC_PR7_W8BIT_SPEC>`"]
pub type W8BIT_ECC_PR7_W8BIT = crate::Reg<w8bit_ecc_pr7_w8bit::W8BIT_ECC_PR7_W8BIT_SPEC>;
#[doc = "SMC ECC parity 7 Register"]
pub mod w8bit_ecc_pr7_w8bit;
#[doc = "ECC_PR8 (r) register accessor: an alias for `Reg<ECC_PR8_SPEC>`"]
pub type ECC_PR8 = crate::Reg<ecc_pr8::ECC_PR8_SPEC>;
#[doc = "SMC ECC parity 8 Register"]
pub mod ecc_pr8;
#[doc = "ECC_PR9 (r) register accessor: an alias for `Reg<ECC_PR9_SPEC>`"]
pub type ECC_PR9 = crate::Reg<ecc_pr9::ECC_PR9_SPEC>;
#[doc = "SMC ECC parity 9 Register"]
pub mod ecc_pr9;
#[doc = "ECC_PR10 (r) register accessor: an alias for `Reg<ECC_PR10_SPEC>`"]
pub type ECC_PR10 = crate::Reg<ecc_pr10::ECC_PR10_SPEC>;
#[doc = "SMC ECC parity 10 Register"]
pub mod ecc_pr10;
#[doc = "ECC_PR11 (r) register accessor: an alias for `Reg<ECC_PR11_SPEC>`"]
pub type ECC_PR11 = crate::Reg<ecc_pr11::ECC_PR11_SPEC>;
#[doc = "SMC ECC parity 11 Register"]
pub mod ecc_pr11;
#[doc = "ECC_PR12 (r) register accessor: an alias for `Reg<ECC_PR12_SPEC>`"]
pub type ECC_PR12 = crate::Reg<ecc_pr12::ECC_PR12_SPEC>;
#[doc = "SMC ECC parity 12 Register"]
pub mod ecc_pr12;
#[doc = "ECC_PR13 (r) register accessor: an alias for `Reg<ECC_PR13_SPEC>`"]
pub type ECC_PR13 = crate::Reg<ecc_pr13::ECC_PR13_SPEC>;
#[doc = "SMC ECC parity 13 Register"]
pub mod ecc_pr13;
#[doc = "ECC_PR14 (r) register accessor: an alias for `Reg<ECC_PR14_SPEC>`"]
pub type ECC_PR14 = crate::Reg<ecc_pr14::ECC_PR14_SPEC>;
#[doc = "SMC ECC parity 14 Register"]
pub mod ecc_pr14;
#[doc = "ECC_PR15 (r) register accessor: an alias for `Reg<ECC_PR15_SPEC>`"]
pub type ECC_PR15 = crate::Reg<ecc_pr15::ECC_PR15_SPEC>;
#[doc = "SMC ECC parity 15 Register"]
pub mod ecc_pr15;
#[doc = "SETUP0 (rw) register accessor: an alias for `Reg<SETUP0_SPEC>`"]
pub type SETUP0 = crate::Reg<setup0::SETUP0_SPEC>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 (rw) register accessor: an alias for `Reg<PULSE0_SPEC>`"]
pub type PULSE0 = crate::Reg<pulse0::PULSE0_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 (rw) register accessor: an alias for `Reg<CYCLE0_SPEC>`"]
pub type CYCLE0 = crate::Reg<cycle0::CYCLE0_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "TIMINGS0 (rw) register accessor: an alias for `Reg<TIMINGS0_SPEC>`"]
pub type TIMINGS0 = crate::Reg<timings0::TIMINGS0_SPEC>;
#[doc = "SMC Timings Register (CS_number = 0)"]
pub mod timings0;
#[doc = "MODE0 (rw) register accessor: an alias for `Reg<MODE0_SPEC>`"]
pub type MODE0 = crate::Reg<mode0::MODE0_SPEC>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 (rw) register accessor: an alias for `Reg<SETUP1_SPEC>`"]
pub type SETUP1 = crate::Reg<setup1::SETUP1_SPEC>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 (rw) register accessor: an alias for `Reg<PULSE1_SPEC>`"]
pub type PULSE1 = crate::Reg<pulse1::PULSE1_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 (rw) register accessor: an alias for `Reg<CYCLE1_SPEC>`"]
pub type CYCLE1 = crate::Reg<cycle1::CYCLE1_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "TIMINGS1 (rw) register accessor: an alias for `Reg<TIMINGS1_SPEC>`"]
pub type TIMINGS1 = crate::Reg<timings1::TIMINGS1_SPEC>;
#[doc = "SMC Timings Register (CS_number = 1)"]
pub mod timings1;
#[doc = "MODE1 (rw) register accessor: an alias for `Reg<MODE1_SPEC>`"]
pub type MODE1 = crate::Reg<mode1::MODE1_SPEC>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 (rw) register accessor: an alias for `Reg<SETUP2_SPEC>`"]
pub type SETUP2 = crate::Reg<setup2::SETUP2_SPEC>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 (rw) register accessor: an alias for `Reg<PULSE2_SPEC>`"]
pub type PULSE2 = crate::Reg<pulse2::PULSE2_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 (rw) register accessor: an alias for `Reg<CYCLE2_SPEC>`"]
pub type CYCLE2 = crate::Reg<cycle2::CYCLE2_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "TIMINGS2 (rw) register accessor: an alias for `Reg<TIMINGS2_SPEC>`"]
pub type TIMINGS2 = crate::Reg<timings2::TIMINGS2_SPEC>;
#[doc = "SMC Timings Register (CS_number = 2)"]
pub mod timings2;
#[doc = "MODE2 (rw) register accessor: an alias for `Reg<MODE2_SPEC>`"]
pub type MODE2 = crate::Reg<mode2::MODE2_SPEC>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 (rw) register accessor: an alias for `Reg<SETUP3_SPEC>`"]
pub type SETUP3 = crate::Reg<setup3::SETUP3_SPEC>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 (rw) register accessor: an alias for `Reg<PULSE3_SPEC>`"]
pub type PULSE3 = crate::Reg<pulse3::PULSE3_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 (rw) register accessor: an alias for `Reg<CYCLE3_SPEC>`"]
pub type CYCLE3 = crate::Reg<cycle3::CYCLE3_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "TIMINGS3 (rw) register accessor: an alias for `Reg<TIMINGS3_SPEC>`"]
pub type TIMINGS3 = crate::Reg<timings3::TIMINGS3_SPEC>;
#[doc = "SMC Timings Register (CS_number = 3)"]
pub mod timings3;
#[doc = "MODE3 (rw) register accessor: an alias for `Reg<MODE3_SPEC>`"]
pub type MODE3 = crate::Reg<mode3::MODE3_SPEC>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "OCMS (rw) register accessor: an alias for `Reg<OCMS_SPEC>`"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC OCMS Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPCR (w) register accessor: an alias for `Reg<WPCR_SPEC>`"]
pub type WPCR = crate::Reg<wpcr::WPCR_SPEC>;
#[doc = "Write Protection Control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
