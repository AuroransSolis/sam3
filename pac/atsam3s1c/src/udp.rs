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
#[doc = "FRM_NUM (r) register accessor: Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`frm_num`]
module"]
pub type FRM_NUM = crate::Reg<frm_num::FRM_NUM_SPEC>;
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "GLB_STAT (rw) register accessor: Global State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`glb_stat`]
module"]
pub type GLB_STAT = crate::Reg<glb_stat::GLB_STAT_SPEC>;
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "FADDR (rw) register accessor: Function Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`faddr`]
module"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Function Address Register"]
pub mod faddr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RST_EP (rw) register accessor: Reset Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rst_ep`]
module"]
pub type RST_EP = crate::Reg<rst_ep::RST_EP_SPEC>;
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "CSR (rw) register accessor: Endpoint Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod csr;
#[doc = "ISOENDPT_CSR0_ISOENDPT (rw) register accessor: Endpoint Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_csr0_isoendpt::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_csr0_isoendpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoendpt_csr0_isoendpt`]
module"]
pub type ISOENDPT_CSR0_ISOENDPT = crate::Reg<isoendpt_csr0_isoendpt::ISOENDPT_CSR0_ISOENDPT_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod isoendpt_csr0_isoendpt;
#[doc = "FDR (rw) register accessor: Endpoint FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fdr`]
module"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Endpoint FIFO Data Register"]
pub mod fdr;
#[doc = "TXVC (rw) register accessor: Transceiver Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txvc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txvc`]
module"]
pub type TXVC = crate::Reg<txvc::TXVC_SPEC>;
#[doc = "Transceiver Control Register"]
pub mod txvc;
