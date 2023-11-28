#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gcfg: GCFG,
    en: EN,
    sreq: SREQ,
    creq: CREQ,
    last: LAST,
    _reserved5: [u8; 0x04],
    ebcier: EBCIER,
    ebcidr: EBCIDR,
    ebcimr: EBCIMR,
    ebcisr: EBCISR,
    cher: CHER,
    chdr: CHDR,
    chsr: CHSR,
    _reserved12: [u8; 0x08],
    saddr0: SADDR0,
    daddr0: DADDR0,
    dscr0: DSCR0,
    ctrla0: CTRLA0,
    ctrlb0: CTRLB0,
    cfg0: CFG0,
    _reserved18: [u8; 0x10],
    saddr1: SADDR1,
    daddr1: DADDR1,
    dscr1: DSCR1,
    ctrla1: CTRLA1,
    ctrlb1: CTRLB1,
    cfg1: CFG1,
    _reserved24: [u8; 0x10],
    saddr2: SADDR2,
    daddr2: DADDR2,
    dscr2: DSCR2,
    ctrla2: CTRLA2,
    ctrlb2: CTRLB2,
    cfg2: CFG2,
    _reserved30: [u8; 0x10],
    saddr3: SADDR3,
    daddr3: DADDR3,
    dscr3: DSCR3,
    ctrla3: CTRLA3,
    ctrlb3: CTRLB3,
    cfg3: CFG3,
    _reserved36: [u8; 0x10],
    saddr4: SADDR4,
    daddr4: DADDR4,
    dscr4: DSCR4,
    ctrla4: CTRLA4,
    ctrlb4: CTRLB4,
    cfg4: CFG4,
    _reserved42: [u8; 0x10],
    saddr5: SADDR5,
    daddr5: DADDR5,
    dscr5: DSCR5,
    ctrla5: CTRLA5,
    ctrlb5: CTRLB5,
    cfg5: CFG5,
    _reserved48: [u8; 0xc8],
    wpmr: WPMR,
    wpsr: WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAC Global Configuration Register"]
    #[inline(always)]
    pub const fn gcfg(&self) -> &GCFG {
        &self.gcfg
    }
    #[doc = "0x04 - DMAC Enable Register"]
    #[inline(always)]
    pub const fn en(&self) -> &EN {
        &self.en
    }
    #[doc = "0x08 - DMAC Software Single Request Register"]
    #[inline(always)]
    pub const fn sreq(&self) -> &SREQ {
        &self.sreq
    }
    #[doc = "0x0c - DMAC Software Chunk Transfer Request Register"]
    #[inline(always)]
    pub const fn creq(&self) -> &CREQ {
        &self.creq
    }
    #[doc = "0x10 - DMAC Software Last Transfer Flag Register"]
    #[inline(always)]
    pub const fn last(&self) -> &LAST {
        &self.last
    }
    #[doc = "0x18 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
    #[inline(always)]
    pub const fn ebcier(&self) -> &EBCIER {
        &self.ebcier
    }
    #[doc = "0x1c - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
    #[inline(always)]
    pub const fn ebcidr(&self) -> &EBCIDR {
        &self.ebcidr
    }
    #[doc = "0x20 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
    #[inline(always)]
    pub const fn ebcimr(&self) -> &EBCIMR {
        &self.ebcimr
    }
    #[doc = "0x24 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
    #[inline(always)]
    pub const fn ebcisr(&self) -> &EBCISR {
        &self.ebcisr
    }
    #[doc = "0x28 - DMAC Channel Handler Enable Register"]
    #[inline(always)]
    pub const fn cher(&self) -> &CHER {
        &self.cher
    }
    #[doc = "0x2c - DMAC Channel Handler Disable Register"]
    #[inline(always)]
    pub const fn chdr(&self) -> &CHDR {
        &self.chdr
    }
    #[doc = "0x30 - DMAC Channel Handler Status Register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &CHSR {
        &self.chsr
    }
    #[doc = "0x3c - DMAC Channel Source Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn saddr0(&self) -> &SADDR0 {
        &self.saddr0
    }
    #[doc = "0x40 - DMAC Channel Destination Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn daddr0(&self) -> &DADDR0 {
        &self.daddr0
    }
    #[doc = "0x44 - DMAC Channel Descriptor Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn dscr0(&self) -> &DSCR0 {
        &self.dscr0
    }
    #[doc = "0x48 - DMAC Channel Control A Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ctrla0(&self) -> &CTRLA0 {
        &self.ctrla0
    }
    #[doc = "0x4c - DMAC Channel Control B Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ctrlb0(&self) -> &CTRLB0 {
        &self.ctrlb0
    }
    #[doc = "0x50 - DMAC Channel Configuration Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &CFG0 {
        &self.cfg0
    }
    #[doc = "0x64 - DMAC Channel Source Address Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn saddr1(&self) -> &SADDR1 {
        &self.saddr1
    }
    #[doc = "0x68 - DMAC Channel Destination Address Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn daddr1(&self) -> &DADDR1 {
        &self.daddr1
    }
    #[doc = "0x6c - DMAC Channel Descriptor Address Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn dscr1(&self) -> &DSCR1 {
        &self.dscr1
    }
    #[doc = "0x70 - DMAC Channel Control A Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ctrla1(&self) -> &CTRLA1 {
        &self.ctrla1
    }
    #[doc = "0x74 - DMAC Channel Control B Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ctrlb1(&self) -> &CTRLB1 {
        &self.ctrlb1
    }
    #[doc = "0x78 - DMAC Channel Configuration Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x8c - DMAC Channel Source Address Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn saddr2(&self) -> &SADDR2 {
        &self.saddr2
    }
    #[doc = "0x90 - DMAC Channel Destination Address Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn daddr2(&self) -> &DADDR2 {
        &self.daddr2
    }
    #[doc = "0x94 - DMAC Channel Descriptor Address Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn dscr2(&self) -> &DSCR2 {
        &self.dscr2
    }
    #[doc = "0x98 - DMAC Channel Control A Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ctrla2(&self) -> &CTRLA2 {
        &self.ctrla2
    }
    #[doc = "0x9c - DMAC Channel Control B Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ctrlb2(&self) -> &CTRLB2 {
        &self.ctrlb2
    }
    #[doc = "0xa0 - DMAC Channel Configuration Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    #[doc = "0xb4 - DMAC Channel Source Address Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn saddr3(&self) -> &SADDR3 {
        &self.saddr3
    }
    #[doc = "0xb8 - DMAC Channel Destination Address Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn daddr3(&self) -> &DADDR3 {
        &self.daddr3
    }
    #[doc = "0xbc - DMAC Channel Descriptor Address Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn dscr3(&self) -> &DSCR3 {
        &self.dscr3
    }
    #[doc = "0xc0 - DMAC Channel Control A Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ctrla3(&self) -> &CTRLA3 {
        &self.ctrla3
    }
    #[doc = "0xc4 - DMAC Channel Control B Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ctrlb3(&self) -> &CTRLB3 {
        &self.ctrlb3
    }
    #[doc = "0xc8 - DMAC Channel Configuration Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cfg3(&self) -> &CFG3 {
        &self.cfg3
    }
    #[doc = "0xdc - DMAC Channel Source Address Register (ch_num = 4)"]
    #[inline(always)]
    pub const fn saddr4(&self) -> &SADDR4 {
        &self.saddr4
    }
    #[doc = "0xe0 - DMAC Channel Destination Address Register (ch_num = 4)"]
    #[inline(always)]
    pub const fn daddr4(&self) -> &DADDR4 {
        &self.daddr4
    }
    #[doc = "0xe4 - DMAC Channel Descriptor Address Register (ch_num = 4)"]
    #[inline(always)]
    pub const fn dscr4(&self) -> &DSCR4 {
        &self.dscr4
    }
    #[doc = "0xe8 - DMAC Channel Control A Register (ch_num = 4)"]
    #[inline(always)]
    pub const fn ctrla4(&self) -> &CTRLA4 {
        &self.ctrla4
    }
    #[doc = "0xec - DMAC Channel Control B Register (ch_num = 4)"]
    #[inline(always)]
    pub const fn ctrlb4(&self) -> &CTRLB4 {
        &self.ctrlb4
    }
    #[doc = "0xf0 - DMAC Channel Configuration Register (ch_num = 4)"]
    #[inline(always)]
    pub const fn cfg4(&self) -> &CFG4 {
        &self.cfg4
    }
    #[doc = "0x104 - DMAC Channel Source Address Register (ch_num = 5)"]
    #[inline(always)]
    pub const fn saddr5(&self) -> &SADDR5 {
        &self.saddr5
    }
    #[doc = "0x108 - DMAC Channel Destination Address Register (ch_num = 5)"]
    #[inline(always)]
    pub const fn daddr5(&self) -> &DADDR5 {
        &self.daddr5
    }
    #[doc = "0x10c - DMAC Channel Descriptor Address Register (ch_num = 5)"]
    #[inline(always)]
    pub const fn dscr5(&self) -> &DSCR5 {
        &self.dscr5
    }
    #[doc = "0x110 - DMAC Channel Control A Register (ch_num = 5)"]
    #[inline(always)]
    pub const fn ctrla5(&self) -> &CTRLA5 {
        &self.ctrla5
    }
    #[doc = "0x114 - DMAC Channel Control B Register (ch_num = 5)"]
    #[inline(always)]
    pub const fn ctrlb5(&self) -> &CTRLB5 {
        &self.ctrlb5
    }
    #[doc = "0x118 - DMAC Channel Configuration Register (ch_num = 5)"]
    #[inline(always)]
    pub const fn cfg5(&self) -> &CFG5 {
        &self.cfg5
    }
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &WPMR {
        &self.wpmr
    }
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &WPSR {
        &self.wpsr
    }
}
#[doc = "GCFG (rw) register accessor: DMAC Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcfg`]
module"]
pub type GCFG = crate::Reg<gcfg::GCFG_SPEC>;
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "EN (rw) register accessor: DMAC Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "SREQ (rw) register accessor: DMAC Software Single Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sreq`]
module"]
pub type SREQ = crate::Reg<sreq::SREQ_SPEC>;
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "CREQ (rw) register accessor: DMAC Software Chunk Transfer Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`creq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`creq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@creq`]
module"]
pub type CREQ = crate::Reg<creq::CREQ_SPEC>;
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "LAST (rw) register accessor: DMAC Software Last Transfer Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`last::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`last::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@last`]
module"]
pub type LAST = crate::Reg<last::LAST_SPEC>;
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "EBCIER (w) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register.\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ebcier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcier`]
module"]
pub type EBCIER = crate::Reg<ebcier::EBCIER_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "EBCIDR (w) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ebcidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcidr`]
module"]
pub type EBCIDR = crate::Reg<ebcidr::EBCIDR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "EBCIMR (r) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ebcimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcimr`]
module"]
pub type EBCIMR = crate::Reg<ebcimr::EBCIMR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "EBCISR (r) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ebcisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcisr`]
module"]
pub type EBCISR = crate::Reg<ebcisr::EBCISR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "CHER (w) register accessor: DMAC Channel Handler Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`]
module"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: DMAC Channel Handler Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`]
module"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: DMAC Channel Handler Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`]
module"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "SADDR0 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr0`]
module"]
pub type SADDR0 = crate::Reg<saddr0::SADDR0_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr0;
#[doc = "DADDR0 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr0`]
module"]
pub type DADDR0 = crate::Reg<daddr0::DADDR0_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr0;
#[doc = "DSCR0 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr0`]
module"]
pub type DSCR0 = crate::Reg<dscr0::DSCR0_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr0;
#[doc = "CTRLA0 (rw) register accessor: DMAC Channel Control A Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla0`]
module"]
pub type CTRLA0 = crate::Reg<ctrla0::CTRLA0_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla0;
#[doc = "CTRLB0 (rw) register accessor: DMAC Channel Control B Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb0`]
module"]
pub type CTRLB0 = crate::Reg<ctrlb0::CTRLB0_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb0;
#[doc = "CFG0 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg0;
#[doc = "SADDR1 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr1`]
module"]
pub type SADDR1 = crate::Reg<saddr1::SADDR1_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub mod saddr1;
#[doc = "DADDR1 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr1`]
module"]
pub type DADDR1 = crate::Reg<daddr1::DADDR1_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub mod daddr1;
#[doc = "DSCR1 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr1`]
module"]
pub type DSCR1 = crate::Reg<dscr1::DSCR1_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub mod dscr1;
#[doc = "CTRLA1 (rw) register accessor: DMAC Channel Control A Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla1`]
module"]
pub type CTRLA1 = crate::Reg<ctrla1::CTRLA1_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub mod ctrla1;
#[doc = "CTRLB1 (rw) register accessor: DMAC Channel Control B Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb1`]
module"]
pub type CTRLB1 = crate::Reg<ctrlb1::CTRLB1_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub mod ctrlb1;
#[doc = "CFG1 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub mod cfg1;
#[doc = "SADDR2 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr2`]
module"]
pub type SADDR2 = crate::Reg<saddr2::SADDR2_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub mod saddr2;
#[doc = "DADDR2 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr2`]
module"]
pub type DADDR2 = crate::Reg<daddr2::DADDR2_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub mod daddr2;
#[doc = "DSCR2 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr2`]
module"]
pub type DSCR2 = crate::Reg<dscr2::DSCR2_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub mod dscr2;
#[doc = "CTRLA2 (rw) register accessor: DMAC Channel Control A Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla2`]
module"]
pub type CTRLA2 = crate::Reg<ctrla2::CTRLA2_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub mod ctrla2;
#[doc = "CTRLB2 (rw) register accessor: DMAC Channel Control B Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb2`]
module"]
pub type CTRLB2 = crate::Reg<ctrlb2::CTRLB2_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub mod ctrlb2;
#[doc = "CFG2 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub mod cfg2;
#[doc = "SADDR3 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr3`]
module"]
pub type SADDR3 = crate::Reg<saddr3::SADDR3_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub mod saddr3;
#[doc = "DADDR3 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr3`]
module"]
pub type DADDR3 = crate::Reg<daddr3::DADDR3_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub mod daddr3;
#[doc = "DSCR3 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr3`]
module"]
pub type DSCR3 = crate::Reg<dscr3::DSCR3_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub mod dscr3;
#[doc = "CTRLA3 (rw) register accessor: DMAC Channel Control A Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla3`]
module"]
pub type CTRLA3 = crate::Reg<ctrla3::CTRLA3_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub mod ctrla3;
#[doc = "CTRLB3 (rw) register accessor: DMAC Channel Control B Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb3`]
module"]
pub type CTRLB3 = crate::Reg<ctrlb3::CTRLB3_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub mod ctrlb3;
#[doc = "CFG3 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg3`]
module"]
pub type CFG3 = crate::Reg<cfg3::CFG3_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub mod cfg3;
#[doc = "SADDR4 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr4`]
module"]
pub type SADDR4 = crate::Reg<saddr4::SADDR4_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 4)"]
pub mod saddr4;
#[doc = "DADDR4 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr4`]
module"]
pub type DADDR4 = crate::Reg<daddr4::DADDR4_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)"]
pub mod daddr4;
#[doc = "DSCR4 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr4`]
module"]
pub type DSCR4 = crate::Reg<dscr4::DSCR4_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)"]
pub mod dscr4;
#[doc = "CTRLA4 (rw) register accessor: DMAC Channel Control A Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla4`]
module"]
pub type CTRLA4 = crate::Reg<ctrla4::CTRLA4_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 4)"]
pub mod ctrla4;
#[doc = "CTRLB4 (rw) register accessor: DMAC Channel Control B Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb4`]
module"]
pub type CTRLB4 = crate::Reg<ctrlb4::CTRLB4_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 4)"]
pub mod ctrlb4;
#[doc = "CFG4 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg4`]
module"]
pub type CFG4 = crate::Reg<cfg4::CFG4_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 4)"]
pub mod cfg4;
#[doc = "SADDR5 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr5`]
module"]
pub type SADDR5 = crate::Reg<saddr5::SADDR5_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 5)"]
pub mod saddr5;
#[doc = "DADDR5 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr5`]
module"]
pub type DADDR5 = crate::Reg<daddr5::DADDR5_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 5)"]
pub mod daddr5;
#[doc = "DSCR5 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr5`]
module"]
pub type DSCR5 = crate::Reg<dscr5::DSCR5_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)"]
pub mod dscr5;
#[doc = "CTRLA5 (rw) register accessor: DMAC Channel Control A Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla5`]
module"]
pub type CTRLA5 = crate::Reg<ctrla5::CTRLA5_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 5)"]
pub mod ctrla5;
#[doc = "CTRLB5 (rw) register accessor: DMAC Channel Control B Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb5`]
module"]
pub type CTRLB5 = crate::Reg<ctrlb5::CTRLB5_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 5)"]
pub mod ctrlb5;
#[doc = "CFG5 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg5`]
module"]
pub type CFG5 = crate::Reg<cfg5::CFG5_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 5)"]
pub mod cfg5;
#[doc = "WPMR (rw) register accessor: DMAC Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: DMAC Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
