#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    pub frm_num: crate::Reg<frm_num::FRM_NUM_SPEC>,
    #[doc = "0x04 - Global State Register"]
    pub glb_stat: crate::Reg<glb_stat::GLB_STAT_SPEC>,
    #[doc = "0x08 - Function Address Register"]
    pub faddr: crate::Reg<faddr::FADDR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x20 - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - Reset Endpoint Register"]
    pub rst_ep: crate::Reg<rst_ep::RST_EP_SPEC>,
    _reserved9: [u8; 0x04],
    _reserved_9_csr: [u8; 0x20],
    #[doc = "0x50..0x70 - Endpoint FIFO Data Register"]
    pub fdr: [crate::Reg<fdr::FDR_SPEC>; 8],
    _reserved11: [u8; 0x04],
    #[doc = "0x74 - Transceiver Control Register"]
    pub txvc: crate::Reg<txvc::TXVC_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn isoendpt_csr0_isoendpt(
        &self,
    ) -> &crate::Reg<isoendpt_csr0_isoendpt::ISOENDPT_CSR0_ISOENDPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<isoendpt_csr0_isoendpt::ISOENDPT_CSR0_ISOENDPT_SPEC>)
        }
    }
    #[doc = "0x30..0x50 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &[crate::Reg<csr::CSR_SPEC>; 8] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const [crate::Reg<csr::CSR_SPEC>; 8])
        }
    }
}
#[doc = "FRM_NUM register accessor: an alias for `Reg<FRM_NUM_SPEC>`"]
pub type FRM_NUM = crate::Reg<frm_num::FRM_NUM_SPEC>;
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "GLB_STAT register accessor: an alias for `Reg<GLB_STAT_SPEC>`"]
pub type GLB_STAT = crate::Reg<glb_stat::GLB_STAT_SPEC>;
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "FADDR register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Function Address Register"]
pub mod faddr;
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
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RST_EP register accessor: an alias for `Reg<RST_EP_SPEC>`"]
pub type RST_EP = crate::Reg<rst_ep::RST_EP_SPEC>;
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod csr;
#[doc = "ISOENDPT_CSR0_ISOENDPT register accessor: an alias for `Reg<ISOENDPT_CSR0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_CSR0_ISOENDPT = crate::Reg<isoendpt_csr0_isoendpt::ISOENDPT_CSR0_ISOENDPT_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod isoendpt_csr0_isoendpt;
#[doc = "FDR register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Endpoint FIFO Data Register"]
pub mod fdr;
#[doc = "TXVC register accessor: an alias for `Reg<TXVC_SPEC>`"]
pub type TXVC = crate::Reg<txvc::TXVC_SPEC>;
#[doc = "Transceiver Control Register"]
pub mod txvc;
