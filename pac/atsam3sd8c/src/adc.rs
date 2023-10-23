#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Channel Sequence Register 1"]
    pub seqr1: SEQR1,
    #[doc = "0x0c - Channel Sequence Register 2"]
    pub seqr2: SEQR2,
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: CHSR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Last Converted Data Register"]
    pub lcdr: LCDR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    _reserved12: [u8; 0x08],
    #[doc = "0x3c - Overrun Status Register"]
    pub over: OVER,
    #[doc = "0x40 - Extended Mode Register"]
    pub emr: EMR,
    #[doc = "0x44 - Compare Window Register"]
    pub cwr: CWR,
    #[doc = "0x48 - Channel Gain Register"]
    pub cgr: CGR,
    #[doc = "0x4c - Channel Offset Register"]
    pub cor: COR,
    #[doc = "0x50 - Channel Data Register 0"]
    pub cdr0: CDR0,
    #[doc = "0x54 - Channel Data Register 1"]
    pub cdr1: CDR1,
    #[doc = "0x58 - Channel Data Register 2"]
    pub cdr2: CDR2,
    #[doc = "0x5c - Channel Data Register 3"]
    pub cdr3: CDR3,
    #[doc = "0x60 - Channel Data Register 4"]
    pub cdr4: CDR4,
    #[doc = "0x64 - Channel Data Register 5"]
    pub cdr5: CDR5,
    #[doc = "0x68 - Channel Data Register 6"]
    pub cdr6: CDR6,
    #[doc = "0x6c - Channel Data Register 7"]
    pub cdr7: CDR7,
    #[doc = "0x70 - Channel Data Register 8"]
    pub cdr8: CDR8,
    #[doc = "0x74 - Channel Data Register 9"]
    pub cdr9: CDR9,
    #[doc = "0x78 - Channel Data Register 10"]
    pub cdr10: CDR10,
    #[doc = "0x7c - Channel Data Register 11"]
    pub cdr11: CDR11,
    #[doc = "0x80 - Channel Data Register 12"]
    pub cdr12: CDR12,
    #[doc = "0x84 - Channel Data Register 13"]
    pub cdr13: CDR13,
    #[doc = "0x88 - Channel Data Register 14"]
    pub cdr14: CDR14,
    _reserved32: [u8; 0x08],
    #[doc = "0x94 - Analog Control Register"]
    pub acr: ACR,
    _reserved33: [u8; 0x4c],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved35: [u8; 0x14],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    _reserved37: [u8; 0x08],
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
    _reserved39: [u8; 0x08],
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
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
#[doc = "SEQR1 (rw) register accessor: Channel Sequence Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqr1`]
module"]
pub type SEQR1 = crate::Reg<seqr1::SEQR1_SPEC>;
#[doc = "Channel Sequence Register 1"]
pub mod seqr1;
#[doc = "SEQR2 (rw) register accessor: Channel Sequence Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqr2`]
module"]
pub type SEQR2 = crate::Reg<seqr2::SEQR2_SPEC>;
#[doc = "Channel Sequence Register 2"]
pub mod seqr2;
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
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "OVER (r) register accessor: Overrun Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`over::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@over`]
module"]
pub type OVER = crate::Reg<over::OVER_SPEC>;
#[doc = "Overrun Status Register"]
pub mod over;
#[doc = "EMR (rw) register accessor: Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`]
module"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "CWR (rw) register accessor: Compare Window Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwr`]
module"]
pub type CWR = crate::Reg<cwr::CWR_SPEC>;
#[doc = "Compare Window Register"]
pub mod cwr;
#[doc = "CGR (rw) register accessor: Channel Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgr`]
module"]
pub type CGR = crate::Reg<cgr::CGR_SPEC>;
#[doc = "Channel Gain Register"]
pub mod cgr;
#[doc = "COR (rw) register accessor: Channel Offset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cor`]
module"]
pub type COR = crate::Reg<cor::COR_SPEC>;
#[doc = "Channel Offset Register"]
pub mod cor;
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
#[doc = "CDR8 (r) register accessor: Channel Data Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr8`]
module"]
pub type CDR8 = crate::Reg<cdr8::CDR8_SPEC>;
#[doc = "Channel Data Register 8"]
pub mod cdr8;
#[doc = "CDR9 (r) register accessor: Channel Data Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr9`]
module"]
pub type CDR9 = crate::Reg<cdr9::CDR9_SPEC>;
#[doc = "Channel Data Register 9"]
pub mod cdr9;
#[doc = "CDR10 (r) register accessor: Channel Data Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr10`]
module"]
pub type CDR10 = crate::Reg<cdr10::CDR10_SPEC>;
#[doc = "Channel Data Register 10"]
pub mod cdr10;
#[doc = "CDR11 (r) register accessor: Channel Data Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr11`]
module"]
pub type CDR11 = crate::Reg<cdr11::CDR11_SPEC>;
#[doc = "Channel Data Register 11"]
pub mod cdr11;
#[doc = "CDR12 (r) register accessor: Channel Data Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr12`]
module"]
pub type CDR12 = crate::Reg<cdr12::CDR12_SPEC>;
#[doc = "Channel Data Register 12"]
pub mod cdr12;
#[doc = "CDR13 (r) register accessor: Channel Data Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr13`]
module"]
pub type CDR13 = crate::Reg<cdr13::CDR13_SPEC>;
#[doc = "Channel Data Register 13"]
pub mod cdr13;
#[doc = "CDR14 (r) register accessor: Channel Data Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr14`]
module"]
pub type CDR14 = crate::Reg<cdr14::CDR14_SPEC>;
#[doc = "Channel Data Register 14"]
pub mod cdr14;
#[doc = "ACR (rw) register accessor: Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Analog Control Register"]
pub mod acr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
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
