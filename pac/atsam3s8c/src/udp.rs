#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    pub frm_num: FRM_NUM,
    #[doc = "0x04 - Global State Register"]
    pub glb_stat: GLB_STAT,
    #[doc = "0x08 - Function Address Register"]
    pub faddr: FADDR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - Reset Endpoint Register"]
    pub rst_ep: RST_EP,
    _reserved9: [u8; 0x04],
    _reserved_9_csr: [u8; 0x20],
    #[doc = "0x50..0x70 - Endpoint FIFO Data Register"]
    pub fdr: [FDR; 8],
    _reserved11: [u8; 0x04],
    #[doc = "0x74 - Transceiver Control Register"]
    pub txvc: TXVC,
}
impl RegisterBlock {
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub const fn isoendpt_csr0_isoendpt(&self) -> &ISOENDPT_CSR0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30..0x50 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &[CSR; 8] {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
}
#[doc = "FRM_NUM (r) register accessor: an alias for `Reg<FRM_NUM_SPEC>`"]
pub type FRM_NUM = crate::Reg<frm_num::FRM_NUM_SPEC>;
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "GLB_STAT (rw) register accessor: an alias for `Reg<GLB_STAT_SPEC>`"]
pub type GLB_STAT = crate::Reg<glb_stat::GLB_STAT_SPEC>;
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "FADDR (rw) register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Function Address Register"]
pub mod faddr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RST_EP (rw) register accessor: an alias for `Reg<RST_EP_SPEC>`"]
pub type RST_EP = crate::Reg<rst_ep::RST_EP_SPEC>;
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod csr;
#[doc = "ISOENDPT_CSR0_ISOENDPT (rw) register accessor: an alias for `Reg<ISOENDPT_CSR0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_CSR0_ISOENDPT = crate::Reg<isoendpt_csr0_isoendpt::ISOENDPT_CSR0_ISOENDPT_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod isoendpt_csr0_isoendpt;
#[doc = "FDR (rw) register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Endpoint FIFO Data Register"]
pub mod fdr;
#[doc = "TXVC (rw) register accessor: an alias for `Reg<TXVC_SPEC>`"]
pub type TXVC = crate::Reg<txvc::TXVC_SPEC>;
#[doc = "Transceiver Control Register"]
pub mod txvc;
