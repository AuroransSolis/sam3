#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gcfg: Gcfg,
    en: En,
    sreq: Sreq,
    creq: Creq,
    last: Last,
    _reserved5: [u8; 0x04],
    ebcier: Ebcier,
    ebcidr: Ebcidr,
    ebcimr: Ebcimr,
    ebcisr: Ebcisr,
    cher: Cher,
    chdr: Chdr,
    chsr: Chsr,
    _reserved12: [u8; 0x08],
    saddr0: Saddr0,
    daddr0: Daddr0,
    dscr0: Dscr0,
    ctrla0: Ctrla0,
    ctrlb0: Ctrlb0,
    cfg0: Cfg0,
    _reserved18: [u8; 0x10],
    saddr1: Saddr1,
    daddr1: Daddr1,
    dscr1: Dscr1,
    ctrla1: Ctrla1,
    ctrlb1: Ctrlb1,
    cfg1: Cfg1,
    _reserved24: [u8; 0x10],
    saddr2: Saddr2,
    daddr2: Daddr2,
    dscr2: Dscr2,
    ctrla2: Ctrla2,
    ctrlb2: Ctrlb2,
    cfg2: Cfg2,
    _reserved30: [u8; 0x10],
    saddr3: Saddr3,
    daddr3: Daddr3,
    dscr3: Dscr3,
    ctrla3: Ctrla3,
    ctrlb3: Ctrlb3,
    cfg3: Cfg3,
    _reserved36: [u8; 0x0118],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAC Global Configuration Register"]
    #[inline(always)]
    pub const fn gcfg(&self) -> &Gcfg {
        &self.gcfg
    }
    #[doc = "0x04 - DMAC Enable Register"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - DMAC Software Single Request Register"]
    #[inline(always)]
    pub const fn sreq(&self) -> &Sreq {
        &self.sreq
    }
    #[doc = "0x0c - DMAC Software Chunk Transfer Request Register"]
    #[inline(always)]
    pub const fn creq(&self) -> &Creq {
        &self.creq
    }
    #[doc = "0x10 - DMAC Software Last Transfer Flag Register"]
    #[inline(always)]
    pub const fn last(&self) -> &Last {
        &self.last
    }
    #[doc = "0x18 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
    #[inline(always)]
    pub const fn ebcier(&self) -> &Ebcier {
        &self.ebcier
    }
    #[doc = "0x1c - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
    #[inline(always)]
    pub const fn ebcidr(&self) -> &Ebcidr {
        &self.ebcidr
    }
    #[doc = "0x20 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
    #[inline(always)]
    pub const fn ebcimr(&self) -> &Ebcimr {
        &self.ebcimr
    }
    #[doc = "0x24 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
    #[inline(always)]
    pub const fn ebcisr(&self) -> &Ebcisr {
        &self.ebcisr
    }
    #[doc = "0x28 - DMAC Channel Handler Enable Register"]
    #[inline(always)]
    pub const fn cher(&self) -> &Cher {
        &self.cher
    }
    #[doc = "0x2c - DMAC Channel Handler Disable Register"]
    #[inline(always)]
    pub const fn chdr(&self) -> &Chdr {
        &self.chdr
    }
    #[doc = "0x30 - DMAC Channel Handler Status Register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &Chsr {
        &self.chsr
    }
    #[doc = "0x3c - DMAC Channel Source Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn saddr0(&self) -> &Saddr0 {
        &self.saddr0
    }
    #[doc = "0x40 - DMAC Channel Destination Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn daddr0(&self) -> &Daddr0 {
        &self.daddr0
    }
    #[doc = "0x44 - DMAC Channel Descriptor Address Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn dscr0(&self) -> &Dscr0 {
        &self.dscr0
    }
    #[doc = "0x48 - DMAC Channel Control A Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ctrla0(&self) -> &Ctrla0 {
        &self.ctrla0
    }
    #[doc = "0x4c - DMAC Channel Control B Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ctrlb0(&self) -> &Ctrlb0 {
        &self.ctrlb0
    }
    #[doc = "0x50 - DMAC Channel Configuration Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x64 - DMAC Channel Source Address Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn saddr1(&self) -> &Saddr1 {
        &self.saddr1
    }
    #[doc = "0x68 - DMAC Channel Destination Address Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn daddr1(&self) -> &Daddr1 {
        &self.daddr1
    }
    #[doc = "0x6c - DMAC Channel Descriptor Address Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn dscr1(&self) -> &Dscr1 {
        &self.dscr1
    }
    #[doc = "0x70 - DMAC Channel Control A Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ctrla1(&self) -> &Ctrla1 {
        &self.ctrla1
    }
    #[doc = "0x74 - DMAC Channel Control B Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ctrlb1(&self) -> &Ctrlb1 {
        &self.ctrlb1
    }
    #[doc = "0x78 - DMAC Channel Configuration Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x8c - DMAC Channel Source Address Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn saddr2(&self) -> &Saddr2 {
        &self.saddr2
    }
    #[doc = "0x90 - DMAC Channel Destination Address Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn daddr2(&self) -> &Daddr2 {
        &self.daddr2
    }
    #[doc = "0x94 - DMAC Channel Descriptor Address Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn dscr2(&self) -> &Dscr2 {
        &self.dscr2
    }
    #[doc = "0x98 - DMAC Channel Control A Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ctrla2(&self) -> &Ctrla2 {
        &self.ctrla2
    }
    #[doc = "0x9c - DMAC Channel Control B Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ctrlb2(&self) -> &Ctrlb2 {
        &self.ctrlb2
    }
    #[doc = "0xa0 - DMAC Channel Configuration Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0xb4 - DMAC Channel Source Address Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn saddr3(&self) -> &Saddr3 {
        &self.saddr3
    }
    #[doc = "0xb8 - DMAC Channel Destination Address Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn daddr3(&self) -> &Daddr3 {
        &self.daddr3
    }
    #[doc = "0xbc - DMAC Channel Descriptor Address Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn dscr3(&self) -> &Dscr3 {
        &self.dscr3
    }
    #[doc = "0xc0 - DMAC Channel Control A Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ctrla3(&self) -> &Ctrla3 {
        &self.ctrla3
    }
    #[doc = "0xc4 - DMAC Channel Control B Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ctrlb3(&self) -> &Ctrlb3 {
        &self.ctrlb3
    }
    #[doc = "0xc8 - DMAC Channel Configuration Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cfg3(&self) -> &Cfg3 {
        &self.cfg3
    }
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "GCFG (rw) register accessor: DMAC Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcfg`]
module"]
#[doc(alias = "GCFG")]
pub type Gcfg = crate::Reg<gcfg::GcfgSpec>;
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "EN (rw) register accessor: DMAC Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "SREQ (rw) register accessor: DMAC Software Single Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sreq`]
module"]
#[doc(alias = "SREQ")]
pub type Sreq = crate::Reg<sreq::SreqSpec>;
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "CREQ (rw) register accessor: DMAC Software Chunk Transfer Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`creq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`creq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@creq`]
module"]
#[doc(alias = "CREQ")]
pub type Creq = crate::Reg<creq::CreqSpec>;
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "LAST (rw) register accessor: DMAC Software Last Transfer Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@last`]
module"]
#[doc(alias = "LAST")]
pub type Last = crate::Reg<last::LastSpec>;
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "EBCIER (w) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register.\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebcier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcier`]
module"]
#[doc(alias = "EBCIER")]
pub type Ebcier = crate::Reg<ebcier::EbcierSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "EBCIDR (w) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebcidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcidr`]
module"]
#[doc(alias = "EBCIDR")]
pub type Ebcidr = crate::Reg<ebcidr::EbcidrSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "EBCIMR (r) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ebcimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcimr`]
module"]
#[doc(alias = "EBCIMR")]
pub type Ebcimr = crate::Reg<ebcimr::EbcimrSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "EBCISR (r) register accessor: DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ebcisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebcisr`]
module"]
#[doc(alias = "EBCISR")]
pub type Ebcisr = crate::Reg<ebcisr::EbcisrSpec>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "CHER (w) register accessor: DMAC Channel Handler Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`]
module"]
#[doc(alias = "CHER")]
pub type Cher = crate::Reg<cher::CherSpec>;
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: DMAC Channel Handler Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`]
module"]
#[doc(alias = "CHDR")]
pub type Chdr = crate::Reg<chdr::ChdrSpec>;
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: DMAC Channel Handler Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`]
module"]
#[doc(alias = "CHSR")]
pub type Chsr = crate::Reg<chsr::ChsrSpec>;
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "SADDR0 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr0`]
module"]
#[doc(alias = "SADDR0")]
pub type Saddr0 = crate::Reg<saddr0::Saddr0Spec>;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr0;
#[doc = "DADDR0 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr0`]
module"]
#[doc(alias = "DADDR0")]
pub type Daddr0 = crate::Reg<daddr0::Daddr0Spec>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr0;
#[doc = "DSCR0 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr0`]
module"]
#[doc(alias = "DSCR0")]
pub type Dscr0 = crate::Reg<dscr0::Dscr0Spec>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr0;
#[doc = "CTRLA0 (rw) register accessor: DMAC Channel Control A Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla0`]
module"]
#[doc(alias = "CTRLA0")]
pub type Ctrla0 = crate::Reg<ctrla0::Ctrla0Spec>;
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla0;
#[doc = "CTRLB0 (rw) register accessor: DMAC Channel Control B Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb0`]
module"]
#[doc(alias = "CTRLB0")]
pub type Ctrlb0 = crate::Reg<ctrlb0::Ctrlb0Spec>;
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb0;
#[doc = "CFG0 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg0;
#[doc = "SADDR1 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr1`]
module"]
#[doc(alias = "SADDR1")]
pub type Saddr1 = crate::Reg<saddr1::Saddr1Spec>;
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub mod saddr1;
#[doc = "DADDR1 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr1`]
module"]
#[doc(alias = "DADDR1")]
pub type Daddr1 = crate::Reg<daddr1::Daddr1Spec>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub mod daddr1;
#[doc = "DSCR1 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr1`]
module"]
#[doc(alias = "DSCR1")]
pub type Dscr1 = crate::Reg<dscr1::Dscr1Spec>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub mod dscr1;
#[doc = "CTRLA1 (rw) register accessor: DMAC Channel Control A Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla1`]
module"]
#[doc(alias = "CTRLA1")]
pub type Ctrla1 = crate::Reg<ctrla1::Ctrla1Spec>;
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub mod ctrla1;
#[doc = "CTRLB1 (rw) register accessor: DMAC Channel Control B Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb1`]
module"]
#[doc(alias = "CTRLB1")]
pub type Ctrlb1 = crate::Reg<ctrlb1::Ctrlb1Spec>;
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub mod ctrlb1;
#[doc = "CFG1 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub mod cfg1;
#[doc = "SADDR2 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr2`]
module"]
#[doc(alias = "SADDR2")]
pub type Saddr2 = crate::Reg<saddr2::Saddr2Spec>;
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub mod saddr2;
#[doc = "DADDR2 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr2`]
module"]
#[doc(alias = "DADDR2")]
pub type Daddr2 = crate::Reg<daddr2::Daddr2Spec>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub mod daddr2;
#[doc = "DSCR2 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr2`]
module"]
#[doc(alias = "DSCR2")]
pub type Dscr2 = crate::Reg<dscr2::Dscr2Spec>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub mod dscr2;
#[doc = "CTRLA2 (rw) register accessor: DMAC Channel Control A Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla2`]
module"]
#[doc(alias = "CTRLA2")]
pub type Ctrla2 = crate::Reg<ctrla2::Ctrla2Spec>;
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub mod ctrla2;
#[doc = "CTRLB2 (rw) register accessor: DMAC Channel Control B Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb2`]
module"]
#[doc(alias = "CTRLB2")]
pub type Ctrlb2 = crate::Reg<ctrlb2::Ctrlb2Spec>;
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub mod ctrlb2;
#[doc = "CFG2 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub mod cfg2;
#[doc = "SADDR3 (rw) register accessor: DMAC Channel Source Address Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr3`]
module"]
#[doc(alias = "SADDR3")]
pub type Saddr3 = crate::Reg<saddr3::Saddr3Spec>;
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub mod saddr3;
#[doc = "DADDR3 (rw) register accessor: DMAC Channel Destination Address Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr3`]
module"]
#[doc(alias = "DADDR3")]
pub type Daddr3 = crate::Reg<daddr3::Daddr3Spec>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub mod daddr3;
#[doc = "DSCR3 (rw) register accessor: DMAC Channel Descriptor Address Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr3`]
module"]
#[doc(alias = "DSCR3")]
pub type Dscr3 = crate::Reg<dscr3::Dscr3Spec>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub mod dscr3;
#[doc = "CTRLA3 (rw) register accessor: DMAC Channel Control A Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla3`]
module"]
#[doc(alias = "CTRLA3")]
pub type Ctrla3 = crate::Reg<ctrla3::Ctrla3Spec>;
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub mod ctrla3;
#[doc = "CTRLB3 (rw) register accessor: DMAC Channel Control B Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb3`]
module"]
#[doc(alias = "CTRLB3")]
pub type Ctrlb3 = crate::Reg<ctrlb3::Ctrlb3Spec>;
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub mod ctrlb3;
#[doc = "CFG3 (rw) register accessor: DMAC Channel Configuration Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg3`]
module"]
#[doc(alias = "CFG3")]
pub type Cfg3 = crate::Reg<cfg3::Cfg3Spec>;
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub mod cfg3;
#[doc = "WPMR (rw) register accessor: DMAC Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: DMAC Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
