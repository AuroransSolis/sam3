#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    mr: MR,
    _reserved2: [u8; 0x08],
    cher: CHER,
    chdr: CHDR,
    chsr: CHSR,
    sr: SR,
    lcdr: LCDR,
    ier: IER,
    idr: IDR,
    imr: IMR,
    cdr0: CDR0,
    cdr1: CDR1,
    cdr2: CDR2,
    cdr3: CDR3,
    cdr4: CDR4,
    cdr5: CDR5,
    cdr6: CDR6,
    cdr7: CDR7,
    _reserved18: [u8; 0xb0],
    rpr: RPR,
    rcr: RCR,
    _reserved20: [u8; 0x08],
    rnpr: RNPR,
    rncr: RNCR,
    _reserved22: [u8; 0x08],
    ptcr: PTCR,
    ptsr: PTSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &MR {
        &self.mr
    }
    #[doc = "0x10 - Channel Enable Register"]
    #[inline(always)]
    pub const fn cher(&self) -> &CHER {
        &self.cher
    }
    #[doc = "0x14 - Channel Disable Register"]
    #[inline(always)]
    pub const fn chdr(&self) -> &CHDR {
        &self.chdr
    }
    #[doc = "0x18 - Channel Status Register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &CHSR {
        &self.chsr
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x20 - Last Converted Data Register"]
    #[inline(always)]
    pub const fn lcdr(&self) -> &LCDR {
        &self.lcdr
    }
    #[doc = "0x24 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x28 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x2c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x30 - Channel Data Register 0"]
    #[inline(always)]
    pub const fn cdr0(&self) -> &CDR0 {
        &self.cdr0
    }
    #[doc = "0x34 - Channel Data Register 1"]
    #[inline(always)]
    pub const fn cdr1(&self) -> &CDR1 {
        &self.cdr1
    }
    #[doc = "0x38 - Channel Data Register 2"]
    #[inline(always)]
    pub const fn cdr2(&self) -> &CDR2 {
        &self.cdr2
    }
    #[doc = "0x3c - Channel Data Register 3"]
    #[inline(always)]
    pub const fn cdr3(&self) -> &CDR3 {
        &self.cdr3
    }
    #[doc = "0x40 - Channel Data Register 4"]
    #[inline(always)]
    pub const fn cdr4(&self) -> &CDR4 {
        &self.cdr4
    }
    #[doc = "0x44 - Channel Data Register 5"]
    #[inline(always)]
    pub const fn cdr5(&self) -> &CDR5 {
        &self.cdr5
    }
    #[doc = "0x48 - Channel Data Register 6"]
    #[inline(always)]
    pub const fn cdr6(&self) -> &CDR6 {
        &self.cdr6
    }
    #[doc = "0x4c - Channel Data Register 7"]
    #[inline(always)]
    pub const fn cdr7(&self) -> &CDR7 {
        &self.cdr7
    }
    #[doc = "0x100 - Receive Pointer Register"]
    #[inline(always)]
    pub const fn rpr(&self) -> &RPR {
        &self.rpr
    }
    #[doc = "0x104 - Receive Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x110 - Receive Next Pointer Register"]
    #[inline(always)]
    pub const fn rnpr(&self) -> &RNPR {
        &self.rnpr
    }
    #[doc = "0x114 - Receive Next Counter Register"]
    #[inline(always)]
    pub const fn rncr(&self) -> &RNCR {
        &self.rncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &PTCR {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &PTSR {
        &self.ptsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "CHER (w) register accessor: Channel Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`]
module"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: Channel Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`]
module"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`]
module"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "LCDR (r) register accessor: Last Converted Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdr`]
module"]
pub type LCDR = crate::Reg<lcdr::LCDR_SPEC>;
#[doc = "Last Converted Data Register"]
pub mod lcdr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "CDR0 (r) register accessor: Channel Data Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr0`]
module"]
pub type CDR0 = crate::Reg<cdr0::CDR0_SPEC>;
#[doc = "Channel Data Register 0"]
pub mod cdr0;
#[doc = "CDR1 (r) register accessor: Channel Data Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr1`]
module"]
pub type CDR1 = crate::Reg<cdr1::CDR1_SPEC>;
#[doc = "Channel Data Register 1"]
pub mod cdr1;
#[doc = "CDR2 (r) register accessor: Channel Data Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr2`]
module"]
pub type CDR2 = crate::Reg<cdr2::CDR2_SPEC>;
#[doc = "Channel Data Register 2"]
pub mod cdr2;
#[doc = "CDR3 (r) register accessor: Channel Data Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr3`]
module"]
pub type CDR3 = crate::Reg<cdr3::CDR3_SPEC>;
#[doc = "Channel Data Register 3"]
pub mod cdr3;
#[doc = "CDR4 (r) register accessor: Channel Data Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr4`]
module"]
pub type CDR4 = crate::Reg<cdr4::CDR4_SPEC>;
#[doc = "Channel Data Register 4"]
pub mod cdr4;
#[doc = "CDR5 (r) register accessor: Channel Data Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr5`]
module"]
pub type CDR5 = crate::Reg<cdr5::CDR5_SPEC>;
#[doc = "Channel Data Register 5"]
pub mod cdr5;
#[doc = "CDR6 (r) register accessor: Channel Data Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr6`]
module"]
pub type CDR6 = crate::Reg<cdr6::CDR6_SPEC>;
#[doc = "Channel Data Register 6"]
pub mod cdr6;
#[doc = "CDR7 (r) register accessor: Channel Data Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr7`]
module"]
pub type CDR7 = crate::Reg<cdr7::CDR7_SPEC>;
#[doc = "Channel Data Register 7"]
pub mod cdr7;
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`]
module"]
pub type RPR = crate::Reg<rpr::RPR_SPEC>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`]
module"]
pub type RNPR = crate::Reg<rnpr::RNPR_SPEC>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`]
module"]
pub type RNCR = crate::Reg<rncr::RNCR_SPEC>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
