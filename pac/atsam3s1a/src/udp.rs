#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    frm_num: FRM_NUM,
    glb_stat: GLB_STAT,
    faddr: FADDR,
    _reserved3: [u8; 0x04],
    ier: IER,
    idr: IDR,
    imr: IMR,
    isr: ISR,
    icr: ICR,
    _reserved8: [u8; 0x04],
    rst_ep: RST_EP,
    _reserved9: [u8; 0x04],
    _reserved_9_csr0: [u8; 0x04],
    csr1: CSR1,
    csr2: CSR2,
    csr3: CSR3,
    csr4: CSR4,
    csr5: CSR5,
    csr6: CSR6,
    csr7: CSR7,
    fdr0: FDR0,
    fdr1: FDR1,
    fdr2: FDR2,
    fdr3: FDR3,
    fdr4: FDR4,
    fdr5: FDR5,
    fdr6: FDR6,
    fdr7: FDR7,
    _reserved25: [u8; 0x04],
    txvc: TXVC,
}
impl RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    #[inline(always)]
    pub const fn frm_num(&self) -> &FRM_NUM {
        &self.frm_num
    }
    #[doc = "0x04 - Global State Register"]
    #[inline(always)]
    pub const fn glb_stat(&self) -> &GLB_STAT {
        &self.glb_stat
    }
    #[doc = "0x08 - Function Address Register"]
    #[inline(always)]
    pub const fn faddr(&self) -> &FADDR {
        &self.faddr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x14 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x18 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x1c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x20 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x28 - Reset Endpoint Register"]
    #[inline(always)]
    pub const fn rst_ep(&self) -> &RST_EP {
        &self.rst_ep
    }
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub const fn isoendpt_csr0_isoendpt(&self) -> &ISOENDPT_CSR0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Endpoint Control and Status Register 0"]
    #[inline(always)]
    pub const fn csr0(&self) -> &CSR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - Endpoint Control and Status Register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    #[doc = "0x38 - Endpoint Control and Status Register 2"]
    #[inline(always)]
    pub const fn csr2(&self) -> &CSR2 {
        &self.csr2
    }
    #[doc = "0x3c - Endpoint Control and Status Register 3"]
    #[inline(always)]
    pub const fn csr3(&self) -> &CSR3 {
        &self.csr3
    }
    #[doc = "0x40 - Endpoint Control and Status Register 4"]
    #[inline(always)]
    pub const fn csr4(&self) -> &CSR4 {
        &self.csr4
    }
    #[doc = "0x44 - Endpoint Control and Status Register 5"]
    #[inline(always)]
    pub const fn csr5(&self) -> &CSR5 {
        &self.csr5
    }
    #[doc = "0x48 - Endpoint Control and Status Register 6"]
    #[inline(always)]
    pub const fn csr6(&self) -> &CSR6 {
        &self.csr6
    }
    #[doc = "0x4c - Endpoint Control and Status Register 7"]
    #[inline(always)]
    pub const fn csr7(&self) -> &CSR7 {
        &self.csr7
    }
    #[doc = "0x50 - Endpoint FIFO Data Register 0"]
    #[inline(always)]
    pub const fn fdr0(&self) -> &FDR0 {
        &self.fdr0
    }
    #[doc = "0x54 - Endpoint FIFO Data Register 1"]
    #[inline(always)]
    pub const fn fdr1(&self) -> &FDR1 {
        &self.fdr1
    }
    #[doc = "0x58 - Endpoint FIFO Data Register 2"]
    #[inline(always)]
    pub const fn fdr2(&self) -> &FDR2 {
        &self.fdr2
    }
    #[doc = "0x5c - Endpoint FIFO Data Register 3"]
    #[inline(always)]
    pub const fn fdr3(&self) -> &FDR3 {
        &self.fdr3
    }
    #[doc = "0x60 - Endpoint FIFO Data Register 4"]
    #[inline(always)]
    pub const fn fdr4(&self) -> &FDR4 {
        &self.fdr4
    }
    #[doc = "0x64 - Endpoint FIFO Data Register 5"]
    #[inline(always)]
    pub const fn fdr5(&self) -> &FDR5 {
        &self.fdr5
    }
    #[doc = "0x68 - Endpoint FIFO Data Register 6"]
    #[inline(always)]
    pub const fn fdr6(&self) -> &FDR6 {
        &self.fdr6
    }
    #[doc = "0x6c - Endpoint FIFO Data Register 7"]
    #[inline(always)]
    pub const fn fdr7(&self) -> &FDR7 {
        &self.fdr7
    }
    #[doc = "0x74 - Transceiver Control Register"]
    #[inline(always)]
    pub const fn txvc(&self) -> &TXVC {
        &self.txvc
    }
}
#[doc = "FRM_NUM (r) register accessor: Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_num`]
module"]
pub type FRM_NUM = crate::Reg<frm_num::FRM_NUM_SPEC>;
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "GLB_STAT (rw) register accessor: Global State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_stat`]
module"]
pub type GLB_STAT = crate::Reg<glb_stat::GLB_STAT_SPEC>;
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "FADDR (rw) register accessor: Function Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`]
module"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Function Address Register"]
pub mod faddr;
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
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RST_EP (rw) register accessor: Reset Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_ep`]
module"]
pub type RST_EP = crate::Reg<rst_ep::RST_EP_SPEC>;
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "CSR0 (rw) register accessor: Endpoint Control and Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`]
module"]
pub type CSR0 = crate::Reg<csr0::CSR0_SPEC>;
#[doc = "Endpoint Control and Status Register 0"]
pub mod csr0;
#[doc = "CSR1 (rw) register accessor: Endpoint Control and Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "Endpoint Control and Status Register 1"]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: Endpoint Control and Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "Endpoint Control and Status Register 2"]
pub mod csr2;
#[doc = "CSR3 (rw) register accessor: Endpoint Control and Status Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`]
module"]
pub type CSR3 = crate::Reg<csr3::CSR3_SPEC>;
#[doc = "Endpoint Control and Status Register 3"]
pub mod csr3;
#[doc = "CSR4 (rw) register accessor: Endpoint Control and Status Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr4`]
module"]
pub type CSR4 = crate::Reg<csr4::CSR4_SPEC>;
#[doc = "Endpoint Control and Status Register 4"]
pub mod csr4;
#[doc = "CSR5 (rw) register accessor: Endpoint Control and Status Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr5`]
module"]
pub type CSR5 = crate::Reg<csr5::CSR5_SPEC>;
#[doc = "Endpoint Control and Status Register 5"]
pub mod csr5;
#[doc = "CSR6 (rw) register accessor: Endpoint Control and Status Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr6`]
module"]
pub type CSR6 = crate::Reg<csr6::CSR6_SPEC>;
#[doc = "Endpoint Control and Status Register 6"]
pub mod csr6;
#[doc = "CSR7 (rw) register accessor: Endpoint Control and Status Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr7`]
module"]
pub type CSR7 = crate::Reg<csr7::CSR7_SPEC>;
#[doc = "Endpoint Control and Status Register 7"]
pub mod csr7;
#[doc = "ISOENDPT_CSR0_ISOENDPT (rw) register accessor: Endpoint Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_csr0_isoendpt::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_csr0_isoendpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_csr0_isoendpt`]
module"]
pub type ISOENDPT_CSR0_ISOENDPT = crate::Reg<isoendpt_csr0_isoendpt::ISOENDPT_CSR0_ISOENDPT_SPEC>;
#[doc = "Endpoint Control and Status Register"]
pub mod isoendpt_csr0_isoendpt;
#[doc = "FDR0 (rw) register accessor: Endpoint FIFO Data Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr0`]
module"]
pub type FDR0 = crate::Reg<fdr0::FDR0_SPEC>;
#[doc = "Endpoint FIFO Data Register 0"]
pub mod fdr0;
#[doc = "FDR1 (rw) register accessor: Endpoint FIFO Data Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr1`]
module"]
pub type FDR1 = crate::Reg<fdr1::FDR1_SPEC>;
#[doc = "Endpoint FIFO Data Register 1"]
pub mod fdr1;
#[doc = "FDR2 (rw) register accessor: Endpoint FIFO Data Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr2`]
module"]
pub type FDR2 = crate::Reg<fdr2::FDR2_SPEC>;
#[doc = "Endpoint FIFO Data Register 2"]
pub mod fdr2;
#[doc = "FDR3 (rw) register accessor: Endpoint FIFO Data Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr3`]
module"]
pub type FDR3 = crate::Reg<fdr3::FDR3_SPEC>;
#[doc = "Endpoint FIFO Data Register 3"]
pub mod fdr3;
#[doc = "FDR4 (rw) register accessor: Endpoint FIFO Data Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr4`]
module"]
pub type FDR4 = crate::Reg<fdr4::FDR4_SPEC>;
#[doc = "Endpoint FIFO Data Register 4"]
pub mod fdr4;
#[doc = "FDR5 (rw) register accessor: Endpoint FIFO Data Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr5`]
module"]
pub type FDR5 = crate::Reg<fdr5::FDR5_SPEC>;
#[doc = "Endpoint FIFO Data Register 5"]
pub mod fdr5;
#[doc = "FDR6 (rw) register accessor: Endpoint FIFO Data Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr6`]
module"]
pub type FDR6 = crate::Reg<fdr6::FDR6_SPEC>;
#[doc = "Endpoint FIFO Data Register 6"]
pub mod fdr6;
#[doc = "FDR7 (rw) register accessor: Endpoint FIFO Data Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr7`]
module"]
pub type FDR7 = crate::Reg<fdr7::FDR7_SPEC>;
#[doc = "Endpoint FIFO Data Register 7"]
pub mod fdr7;
#[doc = "TXVC (rw) register accessor: Transceiver Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txvc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txvc`]
module"]
pub type TXVC = crate::Reg<txvc::TXVC_SPEC>;
#[doc = "Transceiver Control Register"]
pub mod txvc;
