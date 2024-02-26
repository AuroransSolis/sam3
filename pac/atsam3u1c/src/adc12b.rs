#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    _reserved2: [u8; 0x08],
    cher: Cher,
    chdr: Chdr,
    chsr: Chsr,
    sr: Sr,
    lcdr: Lcdr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    cdr0: Cdr0,
    cdr1: Cdr1,
    cdr2: Cdr2,
    cdr3: Cdr3,
    cdr4: Cdr4,
    cdr5: Cdr5,
    cdr6: Cdr6,
    cdr7: Cdr7,
    _reserved18: [u8; 0x14],
    acr: Acr,
    emr: Emr,
    _reserved20: [u8; 0x94],
    rpr: Rpr,
    rcr: Rcr,
    _reserved22: [u8; 0x08],
    rnpr: Rnpr,
    rncr: Rncr,
    _reserved24: [u8; 0x08],
    ptcr: Ptcr,
    ptsr: Ptsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x10 - Channel Enable Register"]
    #[inline(always)]
    pub const fn cher(&self) -> &Cher {
        &self.cher
    }
    #[doc = "0x14 - Channel Disable Register"]
    #[inline(always)]
    pub const fn chdr(&self) -> &Chdr {
        &self.chdr
    }
    #[doc = "0x18 - Channel Status Register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &Chsr {
        &self.chsr
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x20 - Last Converted Data Register"]
    #[inline(always)]
    pub const fn lcdr(&self) -> &Lcdr {
        &self.lcdr
    }
    #[doc = "0x24 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x28 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x2c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - Channel Data Register 0"]
    #[inline(always)]
    pub const fn cdr0(&self) -> &Cdr0 {
        &self.cdr0
    }
    #[doc = "0x34 - Channel Data Register 1"]
    #[inline(always)]
    pub const fn cdr1(&self) -> &Cdr1 {
        &self.cdr1
    }
    #[doc = "0x38 - Channel Data Register 2"]
    #[inline(always)]
    pub const fn cdr2(&self) -> &Cdr2 {
        &self.cdr2
    }
    #[doc = "0x3c - Channel Data Register 3"]
    #[inline(always)]
    pub const fn cdr3(&self) -> &Cdr3 {
        &self.cdr3
    }
    #[doc = "0x40 - Channel Data Register 4"]
    #[inline(always)]
    pub const fn cdr4(&self) -> &Cdr4 {
        &self.cdr4
    }
    #[doc = "0x44 - Channel Data Register 5"]
    #[inline(always)]
    pub const fn cdr5(&self) -> &Cdr5 {
        &self.cdr5
    }
    #[doc = "0x48 - Channel Data Register 6"]
    #[inline(always)]
    pub const fn cdr6(&self) -> &Cdr6 {
        &self.cdr6
    }
    #[doc = "0x4c - Channel Data Register 7"]
    #[inline(always)]
    pub const fn cdr7(&self) -> &Cdr7 {
        &self.cdr7
    }
    #[doc = "0x64 - Analog Control Register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x68 - Extended Mode Register"]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
    #[doc = "0x100 - Receive Pointer Register"]
    #[inline(always)]
    pub const fn rpr(&self) -> &Rpr {
        &self.rpr
    }
    #[doc = "0x104 - Receive Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x110 - Receive Next Pointer Register"]
    #[inline(always)]
    pub const fn rnpr(&self) -> &Rnpr {
        &self.rnpr
    }
    #[doc = "0x114 - Receive Next Counter Register"]
    #[inline(always)]
    pub const fn rncr(&self) -> &Rncr {
        &self.rncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &Ptcr {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &Ptsr {
        &self.ptsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "CHER (w) register accessor: Channel Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`]
module"]
#[doc(alias = "CHER")]
pub type Cher = crate::Reg<cher::CherSpec>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: Channel Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`]
module"]
#[doc(alias = "CHDR")]
pub type Chdr = crate::Reg<chdr::ChdrSpec>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`]
module"]
#[doc(alias = "CHSR")]
pub type Chsr = crate::Reg<chsr::ChsrSpec>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "LCDR (r) register accessor: Last Converted Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdr`]
module"]
#[doc(alias = "LCDR")]
pub type Lcdr = crate::Reg<lcdr::LcdrSpec>;
#[doc = "Last Converted Data Register"]
pub mod lcdr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "CDR0 (r) register accessor: Channel Data Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr0`]
module"]
#[doc(alias = "CDR0")]
pub type Cdr0 = crate::Reg<cdr0::Cdr0Spec>;
#[doc = "Channel Data Register 0"]
pub mod cdr0;
#[doc = "CDR1 (r) register accessor: Channel Data Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr1`]
module"]
#[doc(alias = "CDR1")]
pub type Cdr1 = crate::Reg<cdr1::Cdr1Spec>;
#[doc = "Channel Data Register 1"]
pub mod cdr1;
#[doc = "CDR2 (r) register accessor: Channel Data Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr2`]
module"]
#[doc(alias = "CDR2")]
pub type Cdr2 = crate::Reg<cdr2::Cdr2Spec>;
#[doc = "Channel Data Register 2"]
pub mod cdr2;
#[doc = "CDR3 (r) register accessor: Channel Data Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr3`]
module"]
#[doc(alias = "CDR3")]
pub type Cdr3 = crate::Reg<cdr3::Cdr3Spec>;
#[doc = "Channel Data Register 3"]
pub mod cdr3;
#[doc = "CDR4 (r) register accessor: Channel Data Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr4`]
module"]
#[doc(alias = "CDR4")]
pub type Cdr4 = crate::Reg<cdr4::Cdr4Spec>;
#[doc = "Channel Data Register 4"]
pub mod cdr4;
#[doc = "CDR5 (r) register accessor: Channel Data Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr5`]
module"]
#[doc(alias = "CDR5")]
pub type Cdr5 = crate::Reg<cdr5::Cdr5Spec>;
#[doc = "Channel Data Register 5"]
pub mod cdr5;
#[doc = "CDR6 (r) register accessor: Channel Data Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr6`]
module"]
#[doc(alias = "CDR6")]
pub type Cdr6 = crate::Reg<cdr6::Cdr6Spec>;
#[doc = "Channel Data Register 6"]
pub mod cdr6;
#[doc = "CDR7 (r) register accessor: Channel Data Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr7`]
module"]
#[doc(alias = "CDR7")]
pub type Cdr7 = crate::Reg<cdr7::Cdr7Spec>;
#[doc = "Channel Data Register 7"]
pub mod cdr7;
#[doc = "ACR (rw) register accessor: Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Analog Control Register"]
pub mod acr;
#[doc = "EMR (rw) register accessor: Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`]
module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`]
module"]
#[doc(alias = "RPR")]
pub type Rpr = crate::Reg<rpr::RprSpec>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`]
module"]
#[doc(alias = "RNPR")]
pub type Rnpr = crate::Reg<rnpr::RnprSpec>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`]
module"]
#[doc(alias = "RNCR")]
pub type Rncr = crate::Reg<rncr::RncrSpec>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
#[doc(alias = "PTCR")]
pub type Ptcr = crate::Reg<ptcr::PtcrSpec>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
#[doc(alias = "PTSR")]
pub type Ptsr = crate::Reg<ptsr::PtsrSpec>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
