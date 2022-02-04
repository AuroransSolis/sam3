#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    pub tr: crate::Reg<tr::TR_SPEC>,
    #[doc = "0x08 - SDRAMC Configuration Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - SDRAMC Low Power Register"]
    pub lpr: crate::Reg<lpr::LPR_SPEC>,
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    pub mdr: crate::Reg<mdr::MDR_SPEC>,
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x2c - SDRAMC OCMS Register 1"]
    pub ocms: crate::Reg<ocms::OCMS_SPEC>,
}
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "SDRAMC Mode Register"]
pub mod mr;
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "SDRAMC Refresh Timer Register"]
pub mod tr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "SDRAMC Configuration Register"]
pub mod cr;
#[doc = "LPR register accessor: an alias for `Reg<LPR_SPEC>`"]
pub type LPR = crate::Reg<lpr::LPR_SPEC>;
#[doc = "SDRAMC Low Power Register"]
pub mod lpr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "SDRAMC Interrupt Status Register"]
pub mod isr;
#[doc = "MDR register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "SDRAMC Memory Device Register"]
pub mod mdr;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "SDRAMC Configuration Register 1"]
pub mod cr1;
#[doc = "OCMS register accessor: an alias for `Reg<OCMS_SPEC>`"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SDRAMC OCMS Register 1"]
pub mod ocms;
