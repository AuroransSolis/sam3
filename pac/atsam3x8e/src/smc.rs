#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    ctrl: Ctrl,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    addr: Addr,
    bank: Bank,
    ecc_ctrl: EccCtrl,
    ecc_md: EccMd,
    ecc_sr1: EccSr1,
    _reserved_11_ecc_pr0: [u8; 0x04],
    _reserved_12_ecc_pr1: [u8; 0x04],
    ecc_sr2: EccSr2,
    _reserved_14_ecc_pr2: [u8; 0x04],
    _reserved_15_ecc_pr3: [u8; 0x04],
    _reserved_16_ecc_pr4: [u8; 0x04],
    _reserved_17_ecc_pr5: [u8; 0x04],
    _reserved_18_ecc_pr6: [u8; 0x04],
    _reserved_19_ecc_pr7: [u8; 0x04],
    ecc_pr8: EccPr8,
    ecc_pr9: EccPr9,
    ecc_pr10: EccPr10,
    ecc_pr11: EccPr11,
    ecc_pr12: EccPr12,
    ecc_pr13: EccPr13,
    ecc_pr14: EccPr14,
    ecc_pr15: EccPr15,
    setup0: Setup0,
    pulse0: Pulse0,
    cycle0: Cycle0,
    timings0: Timings0,
    mode0: Mode0,
    setup1: Setup1,
    pulse1: Pulse1,
    cycle1: Cycle1,
    timings1: Timings1,
    mode1: Mode1,
    setup2: Setup2,
    pulse2: Pulse2,
    cycle2: Cycle2,
    timings2: Timings2,
    mode2: Mode2,
    setup3: Setup3,
    pulse3: Pulse3,
    cycle3: Cycle3,
    timings3: Timings3,
    mode3: Mode3,
    setup4: Setup4,
    pulse4: Pulse4,
    cycle4: Cycle4,
    timings4: Timings4,
    mode4: Mode4,
    setup5: Setup5,
    pulse5: Pulse5,
    cycle5: Cycle5,
    timings5: Timings5,
    mode5: Mode5,
    setup6: Setup6,
    pulse6: Pulse6,
    cycle6: Cycle6,
    timings6: Timings6,
    mode6: Mode6,
    setup7: Setup7,
    pulse7: Pulse7,
    cycle7: Cycle7,
    timings7: Timings7,
    mode7: Mode7,
    ocms: Ocms,
    key1: Key1,
    key2: Key2,
    _reserved71: [u8; 0xc8],
    wpcr: Wpcr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - SMC NFC Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - SMC NFC Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - SMC NFC Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - SMC NFC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x10 - SMC NFC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x14 - SMC NFC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x18 - SMC NFC Address Cycle Zero Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x1c - SMC Bank Address Register"]
    #[inline(always)]
    pub const fn bank(&self) -> &Bank {
        &self.bank
    }
    #[doc = "0x20 - SMC ECC Control Register"]
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &EccCtrl {
        &self.ecc_ctrl
    }
    #[doc = "0x24 - SMC ECC Mode Register"]
    #[inline(always)]
    pub const fn ecc_md(&self) -> &EccMd {
        &self.ecc_md
    }
    #[doc = "0x28 - SMC ECC Status 1 Register"]
    #[inline(always)]
    pub const fn ecc_sr1(&self) -> &EccSr1 {
        &self.ecc_sr1
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr0_w8bit(&self) -> &W8bitEccPr0W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub const fn w9bit_ecc_pr0_w9bit(&self) -> &W9bitEccPr0W9bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - SMC ECC Parity 0 Register"]
    #[inline(always)]
    pub const fn ecc_pr0(&self) -> &EccPr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr1_w8bit(&self) -> &W8bitEccPr1W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub const fn w9bit_ecc_pr1_w9bit(&self) -> &W9bitEccPr1W9bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - SMC ECC parity 1 Register"]
    #[inline(always)]
    pub const fn ecc_pr1(&self) -> &EccPr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - SMC ECC status 2 Register"]
    #[inline(always)]
    pub const fn ecc_sr2(&self) -> &EccSr2 {
        &self.ecc_sr2
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr2_w8bit(&self) -> &W8bitEccPr2W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - SMC ECC parity 2 Register"]
    #[inline(always)]
    pub const fn ecc_pr2(&self) -> &EccPr2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr3_w8bit(&self) -> &W8bitEccPr3W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - SMC ECC parity 3 Register"]
    #[inline(always)]
    pub const fn ecc_pr3(&self) -> &EccPr3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr4_w8bit(&self) -> &W8bitEccPr4W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - SMC ECC parity 4 Register"]
    #[inline(always)]
    pub const fn ecc_pr4(&self) -> &EccPr4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr5_w8bit(&self) -> &W8bitEccPr5W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - SMC ECC parity 5 Register"]
    #[inline(always)]
    pub const fn ecc_pr5(&self) -> &EccPr5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr6_w8bit(&self) -> &W8bitEccPr6W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - SMC ECC parity 6 Register"]
    #[inline(always)]
    pub const fn ecc_pr6(&self) -> &EccPr6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub const fn w8bit_ecc_pr7_w8bit(&self) -> &W8bitEccPr7W8bit {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - SMC ECC parity 7 Register"]
    #[inline(always)]
    pub const fn ecc_pr7(&self) -> &EccPr7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - SMC ECC parity 8 Register"]
    #[inline(always)]
    pub const fn ecc_pr8(&self) -> &EccPr8 {
        &self.ecc_pr8
    }
    #[doc = "0x54 - SMC ECC parity 9 Register"]
    #[inline(always)]
    pub const fn ecc_pr9(&self) -> &EccPr9 {
        &self.ecc_pr9
    }
    #[doc = "0x58 - SMC ECC parity 10 Register"]
    #[inline(always)]
    pub const fn ecc_pr10(&self) -> &EccPr10 {
        &self.ecc_pr10
    }
    #[doc = "0x5c - SMC ECC parity 11 Register"]
    #[inline(always)]
    pub const fn ecc_pr11(&self) -> &EccPr11 {
        &self.ecc_pr11
    }
    #[doc = "0x60 - SMC ECC parity 12 Register"]
    #[inline(always)]
    pub const fn ecc_pr12(&self) -> &EccPr12 {
        &self.ecc_pr12
    }
    #[doc = "0x64 - SMC ECC parity 13 Register"]
    #[inline(always)]
    pub const fn ecc_pr13(&self) -> &EccPr13 {
        &self.ecc_pr13
    }
    #[doc = "0x68 - SMC ECC parity 14 Register"]
    #[inline(always)]
    pub const fn ecc_pr14(&self) -> &EccPr14 {
        &self.ecc_pr14
    }
    #[doc = "0x6c - SMC ECC parity 15 Register"]
    #[inline(always)]
    pub const fn ecc_pr15(&self) -> &EccPr15 {
        &self.ecc_pr15
    }
    #[doc = "0x70 - SMC Setup Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn setup0(&self) -> &Setup0 {
        &self.setup0
    }
    #[doc = "0x74 - SMC Pulse Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn pulse0(&self) -> &Pulse0 {
        &self.pulse0
    }
    #[doc = "0x78 - SMC Cycle Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn cycle0(&self) -> &Cycle0 {
        &self.cycle0
    }
    #[doc = "0x7c - SMC Timings Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn timings0(&self) -> &Timings0 {
        &self.timings0
    }
    #[doc = "0x80 - SMC Mode Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn mode0(&self) -> &Mode0 {
        &self.mode0
    }
    #[doc = "0x84 - SMC Setup Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn setup1(&self) -> &Setup1 {
        &self.setup1
    }
    #[doc = "0x88 - SMC Pulse Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn pulse1(&self) -> &Pulse1 {
        &self.pulse1
    }
    #[doc = "0x8c - SMC Cycle Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn cycle1(&self) -> &Cycle1 {
        &self.cycle1
    }
    #[doc = "0x90 - SMC Timings Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn timings1(&self) -> &Timings1 {
        &self.timings1
    }
    #[doc = "0x94 - SMC Mode Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn mode1(&self) -> &Mode1 {
        &self.mode1
    }
    #[doc = "0x98 - SMC Setup Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn setup2(&self) -> &Setup2 {
        &self.setup2
    }
    #[doc = "0x9c - SMC Pulse Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn pulse2(&self) -> &Pulse2 {
        &self.pulse2
    }
    #[doc = "0xa0 - SMC Cycle Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn cycle2(&self) -> &Cycle2 {
        &self.cycle2
    }
    #[doc = "0xa4 - SMC Timings Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn timings2(&self) -> &Timings2 {
        &self.timings2
    }
    #[doc = "0xa8 - SMC Mode Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn mode2(&self) -> &Mode2 {
        &self.mode2
    }
    #[doc = "0xac - SMC Setup Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn setup3(&self) -> &Setup3 {
        &self.setup3
    }
    #[doc = "0xb0 - SMC Pulse Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn pulse3(&self) -> &Pulse3 {
        &self.pulse3
    }
    #[doc = "0xb4 - SMC Cycle Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn cycle3(&self) -> &Cycle3 {
        &self.cycle3
    }
    #[doc = "0xb8 - SMC Timings Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn timings3(&self) -> &Timings3 {
        &self.timings3
    }
    #[doc = "0xbc - SMC Mode Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn mode3(&self) -> &Mode3 {
        &self.mode3
    }
    #[doc = "0xc0 - SMC Setup Register (CS_number = 4)"]
    #[inline(always)]
    pub const fn setup4(&self) -> &Setup4 {
        &self.setup4
    }
    #[doc = "0xc4 - SMC Pulse Register (CS_number = 4)"]
    #[inline(always)]
    pub const fn pulse4(&self) -> &Pulse4 {
        &self.pulse4
    }
    #[doc = "0xc8 - SMC Cycle Register (CS_number = 4)"]
    #[inline(always)]
    pub const fn cycle4(&self) -> &Cycle4 {
        &self.cycle4
    }
    #[doc = "0xcc - SMC Timings Register (CS_number = 4)"]
    #[inline(always)]
    pub const fn timings4(&self) -> &Timings4 {
        &self.timings4
    }
    #[doc = "0xd0 - SMC Mode Register (CS_number = 4)"]
    #[inline(always)]
    pub const fn mode4(&self) -> &Mode4 {
        &self.mode4
    }
    #[doc = "0xd4 - SMC Setup Register (CS_number = 5)"]
    #[inline(always)]
    pub const fn setup5(&self) -> &Setup5 {
        &self.setup5
    }
    #[doc = "0xd8 - SMC Pulse Register (CS_number = 5)"]
    #[inline(always)]
    pub const fn pulse5(&self) -> &Pulse5 {
        &self.pulse5
    }
    #[doc = "0xdc - SMC Cycle Register (CS_number = 5)"]
    #[inline(always)]
    pub const fn cycle5(&self) -> &Cycle5 {
        &self.cycle5
    }
    #[doc = "0xe0 - SMC Timings Register (CS_number = 5)"]
    #[inline(always)]
    pub const fn timings5(&self) -> &Timings5 {
        &self.timings5
    }
    #[doc = "0xe4 - SMC Mode Register (CS_number = 5)"]
    #[inline(always)]
    pub const fn mode5(&self) -> &Mode5 {
        &self.mode5
    }
    #[doc = "0xe8 - SMC Setup Register (CS_number = 6)"]
    #[inline(always)]
    pub const fn setup6(&self) -> &Setup6 {
        &self.setup6
    }
    #[doc = "0xec - SMC Pulse Register (CS_number = 6)"]
    #[inline(always)]
    pub const fn pulse6(&self) -> &Pulse6 {
        &self.pulse6
    }
    #[doc = "0xf0 - SMC Cycle Register (CS_number = 6)"]
    #[inline(always)]
    pub const fn cycle6(&self) -> &Cycle6 {
        &self.cycle6
    }
    #[doc = "0xf4 - SMC Timings Register (CS_number = 6)"]
    #[inline(always)]
    pub const fn timings6(&self) -> &Timings6 {
        &self.timings6
    }
    #[doc = "0xf8 - SMC Mode Register (CS_number = 6)"]
    #[inline(always)]
    pub const fn mode6(&self) -> &Mode6 {
        &self.mode6
    }
    #[doc = "0xfc - SMC Setup Register (CS_number = 7)"]
    #[inline(always)]
    pub const fn setup7(&self) -> &Setup7 {
        &self.setup7
    }
    #[doc = "0x100 - SMC Pulse Register (CS_number = 7)"]
    #[inline(always)]
    pub const fn pulse7(&self) -> &Pulse7 {
        &self.pulse7
    }
    #[doc = "0x104 - SMC Cycle Register (CS_number = 7)"]
    #[inline(always)]
    pub const fn cycle7(&self) -> &Cycle7 {
        &self.cycle7
    }
    #[doc = "0x108 - SMC Timings Register (CS_number = 7)"]
    #[inline(always)]
    pub const fn timings7(&self) -> &Timings7 {
        &self.timings7
    }
    #[doc = "0x10c - SMC Mode Register (CS_number = 7)"]
    #[inline(always)]
    pub const fn mode7(&self) -> &Mode7 {
        &self.mode7
    }
    #[doc = "0x110 - SMC OCMS Register"]
    #[inline(always)]
    pub const fn ocms(&self) -> &Ocms {
        &self.ocms
    }
    #[doc = "0x114 - SMC OCMS KEY1 Register"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x118 - SMC OCMS KEY2 Register"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x1e4 - Write Protection Control Register"]
    #[inline(always)]
    pub const fn wpcr(&self) -> &Wpcr {
        &self.wpcr
    }
    #[doc = "0x1e8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "CFG (rw) register accessor: SMC NFC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "SMC NFC Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: SMC NFC Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SMC NFC Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: SMC NFC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SMC NFC Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: SMC NFC Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "SMC NFC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: SMC NFC Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "SMC NFC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: SMC NFC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "SMC NFC Interrupt Mask Register"]
pub mod imr;
#[doc = "ADDR (rw) register accessor: SMC NFC Address Cycle Zero Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "SMC NFC Address Cycle Zero Register"]
pub mod addr;
#[doc = "BANK (rw) register accessor: SMC Bank Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank`]
module"]
#[doc(alias = "BANK")]
pub type Bank = crate::Reg<bank::BankSpec>;
#[doc = "SMC Bank Address Register"]
pub mod bank;
#[doc = "ECC_CTRL (w) register accessor: SMC ECC Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctrl`]
module"]
#[doc(alias = "ECC_CTRL")]
pub type EccCtrl = crate::Reg<ecc_ctrl::EccCtrlSpec>;
#[doc = "SMC ECC Control Register"]
pub mod ecc_ctrl;
#[doc = "ECC_MD (rw) register accessor: SMC ECC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_md::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_md::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_md`]
module"]
#[doc(alias = "ECC_MD")]
pub type EccMd = crate::Reg<ecc_md::EccMdSpec>;
#[doc = "SMC ECC Mode Register"]
pub mod ecc_md;
#[doc = "ECC_SR1 (r) register accessor: SMC ECC Status 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_sr1`]
module"]
#[doc(alias = "ECC_SR1")]
pub type EccSr1 = crate::Reg<ecc_sr1::EccSr1Spec>;
#[doc = "SMC ECC Status 1 Register"]
pub mod ecc_sr1;
#[doc = "ECC_PR0 (r) register accessor: SMC ECC Parity 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr0`]
module"]
#[doc(alias = "ECC_PR0")]
pub type EccPr0 = crate::Reg<ecc_pr0::EccPr0Spec>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod ecc_pr0;
#[doc = "W9BIT_ECC_PR0_W9BIT (r) register accessor: SMC ECC Parity 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w9bit_ecc_pr0_w9bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w9bit_ecc_pr0_w9bit`]
module"]
#[doc(alias = "W9BIT_ECC_PR0_W9BIT")]
pub type W9bitEccPr0W9bit = crate::Reg<w9bit_ecc_pr0_w9bit::W9bitEccPr0W9bitSpec>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod w9bit_ecc_pr0_w9bit;
#[doc = "W8BIT_ECC_PR0_W8BIT (r) register accessor: SMC ECC Parity 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr0_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr0_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR0_W8BIT")]
pub type W8bitEccPr0W8bit = crate::Reg<w8bit_ecc_pr0_w8bit::W8bitEccPr0W8bitSpec>;
#[doc = "SMC ECC Parity 0 Register"]
pub mod w8bit_ecc_pr0_w8bit;
#[doc = "ECC_PR1 (r) register accessor: SMC ECC parity 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr1`]
module"]
#[doc(alias = "ECC_PR1")]
pub type EccPr1 = crate::Reg<ecc_pr1::EccPr1Spec>;
#[doc = "SMC ECC parity 1 Register"]
pub mod ecc_pr1;
#[doc = "W9BIT_ECC_PR1_W9BIT (r) register accessor: SMC ECC parity 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w9bit_ecc_pr1_w9bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w9bit_ecc_pr1_w9bit`]
module"]
#[doc(alias = "W9BIT_ECC_PR1_W9BIT")]
pub type W9bitEccPr1W9bit = crate::Reg<w9bit_ecc_pr1_w9bit::W9bitEccPr1W9bitSpec>;
#[doc = "SMC ECC parity 1 Register"]
pub mod w9bit_ecc_pr1_w9bit;
#[doc = "W8BIT_ECC_PR1_W8BIT (r) register accessor: SMC ECC parity 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr1_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr1_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR1_W8BIT")]
pub type W8bitEccPr1W8bit = crate::Reg<w8bit_ecc_pr1_w8bit::W8bitEccPr1W8bitSpec>;
#[doc = "SMC ECC parity 1 Register"]
pub mod w8bit_ecc_pr1_w8bit;
#[doc = "ECC_SR2 (r) register accessor: SMC ECC status 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_sr2`]
module"]
#[doc(alias = "ECC_SR2")]
pub type EccSr2 = crate::Reg<ecc_sr2::EccSr2Spec>;
#[doc = "SMC ECC status 2 Register"]
pub mod ecc_sr2;
#[doc = "ECC_PR2 (r) register accessor: SMC ECC parity 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr2`]
module"]
#[doc(alias = "ECC_PR2")]
pub type EccPr2 = crate::Reg<ecc_pr2::EccPr2Spec>;
#[doc = "SMC ECC parity 2 Register"]
pub mod ecc_pr2;
#[doc = "W8BIT_ECC_PR2_W8BIT (r) register accessor: SMC ECC parity 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr2_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr2_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR2_W8BIT")]
pub type W8bitEccPr2W8bit = crate::Reg<w8bit_ecc_pr2_w8bit::W8bitEccPr2W8bitSpec>;
#[doc = "SMC ECC parity 2 Register"]
pub mod w8bit_ecc_pr2_w8bit;
#[doc = "ECC_PR3 (r) register accessor: SMC ECC parity 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr3`]
module"]
#[doc(alias = "ECC_PR3")]
pub type EccPr3 = crate::Reg<ecc_pr3::EccPr3Spec>;
#[doc = "SMC ECC parity 3 Register"]
pub mod ecc_pr3;
#[doc = "W8BIT_ECC_PR3_W8BIT (r) register accessor: SMC ECC parity 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr3_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr3_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR3_W8BIT")]
pub type W8bitEccPr3W8bit = crate::Reg<w8bit_ecc_pr3_w8bit::W8bitEccPr3W8bitSpec>;
#[doc = "SMC ECC parity 3 Register"]
pub mod w8bit_ecc_pr3_w8bit;
#[doc = "ECC_PR4 (r) register accessor: SMC ECC parity 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr4`]
module"]
#[doc(alias = "ECC_PR4")]
pub type EccPr4 = crate::Reg<ecc_pr4::EccPr4Spec>;
#[doc = "SMC ECC parity 4 Register"]
pub mod ecc_pr4;
#[doc = "W8BIT_ECC_PR4_W8BIT (r) register accessor: SMC ECC parity 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr4_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr4_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR4_W8BIT")]
pub type W8bitEccPr4W8bit = crate::Reg<w8bit_ecc_pr4_w8bit::W8bitEccPr4W8bitSpec>;
#[doc = "SMC ECC parity 4 Register"]
pub mod w8bit_ecc_pr4_w8bit;
#[doc = "ECC_PR5 (r) register accessor: SMC ECC parity 5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr5`]
module"]
#[doc(alias = "ECC_PR5")]
pub type EccPr5 = crate::Reg<ecc_pr5::EccPr5Spec>;
#[doc = "SMC ECC parity 5 Register"]
pub mod ecc_pr5;
#[doc = "W8BIT_ECC_PR5_W8BIT (r) register accessor: SMC ECC parity 5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr5_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr5_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR5_W8BIT")]
pub type W8bitEccPr5W8bit = crate::Reg<w8bit_ecc_pr5_w8bit::W8bitEccPr5W8bitSpec>;
#[doc = "SMC ECC parity 5 Register"]
pub mod w8bit_ecc_pr5_w8bit;
#[doc = "ECC_PR6 (r) register accessor: SMC ECC parity 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr6`]
module"]
#[doc(alias = "ECC_PR6")]
pub type EccPr6 = crate::Reg<ecc_pr6::EccPr6Spec>;
#[doc = "SMC ECC parity 6 Register"]
pub mod ecc_pr6;
#[doc = "W8BIT_ECC_PR6_W8BIT (r) register accessor: SMC ECC parity 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr6_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr6_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR6_W8BIT")]
pub type W8bitEccPr6W8bit = crate::Reg<w8bit_ecc_pr6_w8bit::W8bitEccPr6W8bitSpec>;
#[doc = "SMC ECC parity 6 Register"]
pub mod w8bit_ecc_pr6_w8bit;
#[doc = "ECC_PR7 (r) register accessor: SMC ECC parity 7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr7`]
module"]
#[doc(alias = "ECC_PR7")]
pub type EccPr7 = crate::Reg<ecc_pr7::EccPr7Spec>;
#[doc = "SMC ECC parity 7 Register"]
pub mod ecc_pr7;
#[doc = "W8BIT_ECC_PR7_W8BIT (r) register accessor: SMC ECC parity 7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w8bit_ecc_pr7_w8bit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w8bit_ecc_pr7_w8bit`]
module"]
#[doc(alias = "W8BIT_ECC_PR7_W8BIT")]
pub type W8bitEccPr7W8bit = crate::Reg<w8bit_ecc_pr7_w8bit::W8bitEccPr7W8bitSpec>;
#[doc = "SMC ECC parity 7 Register"]
pub mod w8bit_ecc_pr7_w8bit;
#[doc = "ECC_PR8 (r) register accessor: SMC ECC parity 8 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr8`]
module"]
#[doc(alias = "ECC_PR8")]
pub type EccPr8 = crate::Reg<ecc_pr8::EccPr8Spec>;
#[doc = "SMC ECC parity 8 Register"]
pub mod ecc_pr8;
#[doc = "ECC_PR9 (r) register accessor: SMC ECC parity 9 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr9`]
module"]
#[doc(alias = "ECC_PR9")]
pub type EccPr9 = crate::Reg<ecc_pr9::EccPr9Spec>;
#[doc = "SMC ECC parity 9 Register"]
pub mod ecc_pr9;
#[doc = "ECC_PR10 (r) register accessor: SMC ECC parity 10 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr10`]
module"]
#[doc(alias = "ECC_PR10")]
pub type EccPr10 = crate::Reg<ecc_pr10::EccPr10Spec>;
#[doc = "SMC ECC parity 10 Register"]
pub mod ecc_pr10;
#[doc = "ECC_PR11 (r) register accessor: SMC ECC parity 11 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr11`]
module"]
#[doc(alias = "ECC_PR11")]
pub type EccPr11 = crate::Reg<ecc_pr11::EccPr11Spec>;
#[doc = "SMC ECC parity 11 Register"]
pub mod ecc_pr11;
#[doc = "ECC_PR12 (r) register accessor: SMC ECC parity 12 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr12`]
module"]
#[doc(alias = "ECC_PR12")]
pub type EccPr12 = crate::Reg<ecc_pr12::EccPr12Spec>;
#[doc = "SMC ECC parity 12 Register"]
pub mod ecc_pr12;
#[doc = "ECC_PR13 (r) register accessor: SMC ECC parity 13 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr13`]
module"]
#[doc(alias = "ECC_PR13")]
pub type EccPr13 = crate::Reg<ecc_pr13::EccPr13Spec>;
#[doc = "SMC ECC parity 13 Register"]
pub mod ecc_pr13;
#[doc = "ECC_PR14 (r) register accessor: SMC ECC parity 14 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr14`]
module"]
#[doc(alias = "ECC_PR14")]
pub type EccPr14 = crate::Reg<ecc_pr14::EccPr14Spec>;
#[doc = "SMC ECC parity 14 Register"]
pub mod ecc_pr14;
#[doc = "ECC_PR15 (r) register accessor: SMC ECC parity 15 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pr15`]
module"]
#[doc(alias = "ECC_PR15")]
pub type EccPr15 = crate::Reg<ecc_pr15::EccPr15Spec>;
#[doc = "SMC ECC parity 15 Register"]
pub mod ecc_pr15;
#[doc = "SETUP0 (rw) register accessor: SMC Setup Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup0`]
module"]
#[doc(alias = "SETUP0")]
pub type Setup0 = crate::Reg<setup0::Setup0Spec>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 (rw) register accessor: SMC Pulse Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse0`]
module"]
#[doc(alias = "PULSE0")]
pub type Pulse0 = crate::Reg<pulse0::Pulse0Spec>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 (rw) register accessor: SMC Cycle Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle0`]
module"]
#[doc(alias = "CYCLE0")]
pub type Cycle0 = crate::Reg<cycle0::Cycle0Spec>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "TIMINGS0 (rw) register accessor: SMC Timings Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings0`]
module"]
#[doc(alias = "TIMINGS0")]
pub type Timings0 = crate::Reg<timings0::Timings0Spec>;
#[doc = "SMC Timings Register (CS_number = 0)"]
pub mod timings0;
#[doc = "MODE0 (rw) register accessor: SMC Mode Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode0`]
module"]
#[doc(alias = "MODE0")]
pub type Mode0 = crate::Reg<mode0::Mode0Spec>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 (rw) register accessor: SMC Setup Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup1`]
module"]
#[doc(alias = "SETUP1")]
pub type Setup1 = crate::Reg<setup1::Setup1Spec>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 (rw) register accessor: SMC Pulse Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse1`]
module"]
#[doc(alias = "PULSE1")]
pub type Pulse1 = crate::Reg<pulse1::Pulse1Spec>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 (rw) register accessor: SMC Cycle Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle1`]
module"]
#[doc(alias = "CYCLE1")]
pub type Cycle1 = crate::Reg<cycle1::Cycle1Spec>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "TIMINGS1 (rw) register accessor: SMC Timings Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings1`]
module"]
#[doc(alias = "TIMINGS1")]
pub type Timings1 = crate::Reg<timings1::Timings1Spec>;
#[doc = "SMC Timings Register (CS_number = 1)"]
pub mod timings1;
#[doc = "MODE1 (rw) register accessor: SMC Mode Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode1`]
module"]
#[doc(alias = "MODE1")]
pub type Mode1 = crate::Reg<mode1::Mode1Spec>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 (rw) register accessor: SMC Setup Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup2`]
module"]
#[doc(alias = "SETUP2")]
pub type Setup2 = crate::Reg<setup2::Setup2Spec>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 (rw) register accessor: SMC Pulse Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse2`]
module"]
#[doc(alias = "PULSE2")]
pub type Pulse2 = crate::Reg<pulse2::Pulse2Spec>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 (rw) register accessor: SMC Cycle Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle2`]
module"]
#[doc(alias = "CYCLE2")]
pub type Cycle2 = crate::Reg<cycle2::Cycle2Spec>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "TIMINGS2 (rw) register accessor: SMC Timings Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings2`]
module"]
#[doc(alias = "TIMINGS2")]
pub type Timings2 = crate::Reg<timings2::Timings2Spec>;
#[doc = "SMC Timings Register (CS_number = 2)"]
pub mod timings2;
#[doc = "MODE2 (rw) register accessor: SMC Mode Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode2`]
module"]
#[doc(alias = "MODE2")]
pub type Mode2 = crate::Reg<mode2::Mode2Spec>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 (rw) register accessor: SMC Setup Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup3`]
module"]
#[doc(alias = "SETUP3")]
pub type Setup3 = crate::Reg<setup3::Setup3Spec>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 (rw) register accessor: SMC Pulse Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse3`]
module"]
#[doc(alias = "PULSE3")]
pub type Pulse3 = crate::Reg<pulse3::Pulse3Spec>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 (rw) register accessor: SMC Cycle Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle3`]
module"]
#[doc(alias = "CYCLE3")]
pub type Cycle3 = crate::Reg<cycle3::Cycle3Spec>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "TIMINGS3 (rw) register accessor: SMC Timings Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings3`]
module"]
#[doc(alias = "TIMINGS3")]
pub type Timings3 = crate::Reg<timings3::Timings3Spec>;
#[doc = "SMC Timings Register (CS_number = 3)"]
pub mod timings3;
#[doc = "MODE3 (rw) register accessor: SMC Mode Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode3`]
module"]
#[doc(alias = "MODE3")]
pub type Mode3 = crate::Reg<mode3::Mode3Spec>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "SETUP4 (rw) register accessor: SMC Setup Register (CS_number = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup4`]
module"]
#[doc(alias = "SETUP4")]
pub type Setup4 = crate::Reg<setup4::Setup4Spec>;
#[doc = "SMC Setup Register (CS_number = 4)"]
pub mod setup4;
#[doc = "PULSE4 (rw) register accessor: SMC Pulse Register (CS_number = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse4`]
module"]
#[doc(alias = "PULSE4")]
pub type Pulse4 = crate::Reg<pulse4::Pulse4Spec>;
#[doc = "SMC Pulse Register (CS_number = 4)"]
pub mod pulse4;
#[doc = "CYCLE4 (rw) register accessor: SMC Cycle Register (CS_number = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle4`]
module"]
#[doc(alias = "CYCLE4")]
pub type Cycle4 = crate::Reg<cycle4::Cycle4Spec>;
#[doc = "SMC Cycle Register (CS_number = 4)"]
pub mod cycle4;
#[doc = "TIMINGS4 (rw) register accessor: SMC Timings Register (CS_number = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings4`]
module"]
#[doc(alias = "TIMINGS4")]
pub type Timings4 = crate::Reg<timings4::Timings4Spec>;
#[doc = "SMC Timings Register (CS_number = 4)"]
pub mod timings4;
#[doc = "MODE4 (rw) register accessor: SMC Mode Register (CS_number = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode4`]
module"]
#[doc(alias = "MODE4")]
pub type Mode4 = crate::Reg<mode4::Mode4Spec>;
#[doc = "SMC Mode Register (CS_number = 4)"]
pub mod mode4;
#[doc = "SETUP5 (rw) register accessor: SMC Setup Register (CS_number = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup5`]
module"]
#[doc(alias = "SETUP5")]
pub type Setup5 = crate::Reg<setup5::Setup5Spec>;
#[doc = "SMC Setup Register (CS_number = 5)"]
pub mod setup5;
#[doc = "PULSE5 (rw) register accessor: SMC Pulse Register (CS_number = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse5`]
module"]
#[doc(alias = "PULSE5")]
pub type Pulse5 = crate::Reg<pulse5::Pulse5Spec>;
#[doc = "SMC Pulse Register (CS_number = 5)"]
pub mod pulse5;
#[doc = "CYCLE5 (rw) register accessor: SMC Cycle Register (CS_number = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle5`]
module"]
#[doc(alias = "CYCLE5")]
pub type Cycle5 = crate::Reg<cycle5::Cycle5Spec>;
#[doc = "SMC Cycle Register (CS_number = 5)"]
pub mod cycle5;
#[doc = "TIMINGS5 (rw) register accessor: SMC Timings Register (CS_number = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings5`]
module"]
#[doc(alias = "TIMINGS5")]
pub type Timings5 = crate::Reg<timings5::Timings5Spec>;
#[doc = "SMC Timings Register (CS_number = 5)"]
pub mod timings5;
#[doc = "MODE5 (rw) register accessor: SMC Mode Register (CS_number = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode5`]
module"]
#[doc(alias = "MODE5")]
pub type Mode5 = crate::Reg<mode5::Mode5Spec>;
#[doc = "SMC Mode Register (CS_number = 5)"]
pub mod mode5;
#[doc = "SETUP6 (rw) register accessor: SMC Setup Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup6`]
module"]
#[doc(alias = "SETUP6")]
pub type Setup6 = crate::Reg<setup6::Setup6Spec>;
#[doc = "SMC Setup Register (CS_number = 6)"]
pub mod setup6;
#[doc = "PULSE6 (rw) register accessor: SMC Pulse Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse6`]
module"]
#[doc(alias = "PULSE6")]
pub type Pulse6 = crate::Reg<pulse6::Pulse6Spec>;
#[doc = "SMC Pulse Register (CS_number = 6)"]
pub mod pulse6;
#[doc = "CYCLE6 (rw) register accessor: SMC Cycle Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle6`]
module"]
#[doc(alias = "CYCLE6")]
pub type Cycle6 = crate::Reg<cycle6::Cycle6Spec>;
#[doc = "SMC Cycle Register (CS_number = 6)"]
pub mod cycle6;
#[doc = "TIMINGS6 (rw) register accessor: SMC Timings Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings6`]
module"]
#[doc(alias = "TIMINGS6")]
pub type Timings6 = crate::Reg<timings6::Timings6Spec>;
#[doc = "SMC Timings Register (CS_number = 6)"]
pub mod timings6;
#[doc = "MODE6 (rw) register accessor: SMC Mode Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode6`]
module"]
#[doc(alias = "MODE6")]
pub type Mode6 = crate::Reg<mode6::Mode6Spec>;
#[doc = "SMC Mode Register (CS_number = 6)"]
pub mod mode6;
#[doc = "SETUP7 (rw) register accessor: SMC Setup Register (CS_number = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup7`]
module"]
#[doc(alias = "SETUP7")]
pub type Setup7 = crate::Reg<setup7::Setup7Spec>;
#[doc = "SMC Setup Register (CS_number = 7)"]
pub mod setup7;
#[doc = "PULSE7 (rw) register accessor: SMC Pulse Register (CS_number = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse7`]
module"]
#[doc(alias = "PULSE7")]
pub type Pulse7 = crate::Reg<pulse7::Pulse7Spec>;
#[doc = "SMC Pulse Register (CS_number = 7)"]
pub mod pulse7;
#[doc = "CYCLE7 (rw) register accessor: SMC Cycle Register (CS_number = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle7`]
module"]
#[doc(alias = "CYCLE7")]
pub type Cycle7 = crate::Reg<cycle7::Cycle7Spec>;
#[doc = "SMC Cycle Register (CS_number = 7)"]
pub mod cycle7;
#[doc = "TIMINGS7 (rw) register accessor: SMC Timings Register (CS_number = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timings7`]
module"]
#[doc(alias = "TIMINGS7")]
pub type Timings7 = crate::Reg<timings7::Timings7Spec>;
#[doc = "SMC Timings Register (CS_number = 7)"]
pub mod timings7;
#[doc = "MODE7 (rw) register accessor: SMC Mode Register (CS_number = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode7`]
module"]
#[doc(alias = "MODE7")]
pub type Mode7 = crate::Reg<mode7::Mode7Spec>;
#[doc = "SMC Mode Register (CS_number = 7)"]
pub mod mode7;
#[doc = "OCMS (rw) register accessor: SMC OCMS Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`]
module"]
#[doc(alias = "OCMS")]
pub type Ocms = crate::Reg<ocms::OcmsSpec>;
#[doc = "SMC OCMS Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: SMC OCMS KEY1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: SMC OCMS KEY2 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPCR (w) register accessor: Write Protection Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr`]
module"]
#[doc(alias = "WPCR")]
pub type Wpcr = crate::Reg<wpcr::WpcrSpec>;
#[doc = "Write Protection Control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
