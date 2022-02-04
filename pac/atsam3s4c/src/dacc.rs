#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: crate::Reg<cher::CHER_SPEC>,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: crate::Reg<chdr::CHDR_SPEC>,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: crate::Reg<chsr::CHSR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - Conversion Data Register"]
    pub cdr: crate::Reg<cdr::CDR_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    _reserved10: [u8; 0x60],
    #[doc = "0x94 - Analog Current Register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved11: [u8; 0x4c],
    #[doc = "0xe4 - Write Protect Mode register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved13: [u8; 0x1c],
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: crate::Reg<tpr::TPR_SPEC>,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: crate::Reg<tnpr::TNPR_SPEC>,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: crate::Reg<tncr::TNCR_SPEC>,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: crate::Reg<ptcr::PTCR_SPEC>,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: crate::Reg<ptsr::PTSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "CHER register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHDR register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "CDR register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Conversion Data Register"]
pub mod cdr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Analog Current Register"]
pub mod acr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status register"]
pub mod wpsr;
#[doc = "TPR register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "TNPR register accessor: an alias for `Reg<TNPR_SPEC>`"]
pub type TNPR = crate::Reg<tnpr::TNPR_SPEC>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR register accessor: an alias for `Reg<TNCR_SPEC>`"]
pub type TNCR = crate::Reg<tncr::TNCR_SPEC>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR register accessor: an alias for `Reg<PTCR_SPEC>`"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR register accessor: an alias for `Reg<PTSR_SPEC>`"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
