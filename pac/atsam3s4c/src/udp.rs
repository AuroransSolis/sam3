#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frm_num: FrmNum,
    glb_stat: GlbStat,
    faddr: Faddr,
    _reserved3: [u8; 0x04],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    icr: Icr,
    _reserved8: [u8; 0x04],
    rst_ep: RstEp,
    _reserved9: [u8; 0x04],
    _reserved_9_csr0: [u8; 0x04],
    csr1: Csr1,
    csr2: Csr2,
    csr3: Csr3,
    csr4: Csr4,
    csr5: Csr5,
    csr6: Csr6,
    csr7: Csr7,
    fdr0: Fdr0,
    fdr1: Fdr1,
    fdr2: Fdr2,
    fdr3: Fdr3,
    fdr4: Fdr4,
    fdr5: Fdr5,
    fdr6: Fdr6,
    fdr7: Fdr7,
    _reserved25: [u8; 0x04],
    txvc: Txvc,
}
impl RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    #[inline(always)]
    pub const fn frm_num(&self) -> &FrmNum {
        &self.frm_num
    }
    #[doc = "0x04 - Global State Register"]
    #[inline(always)]
    pub const fn glb_stat(&self) -> &GlbStat {
        &self.glb_stat
    }
    #[doc = "0x08 - Function Address Register"]
    #[inline(always)]
    pub const fn faddr(&self) -> &Faddr {
        &self.faddr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x20 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x28 - Reset Endpoint Register"]
    #[inline(always)]
    pub const fn rst_ep(&self) -> &RstEp {
        &self.rst_ep
    }
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub const fn isoendpt_csr0_isoendpt(&self) -> &IsoendptCsr0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Endpoint Control and Status Register 0"]
    #[inline(always)]
    pub const fn csr0(&self) -> &Csr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - Endpoint Control and Status Register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x38 - Endpoint Control and Status Register 2"]
    #[inline(always)]
    pub const fn csr2(&self) -> &Csr2 {
        &self.csr2
    }
    #[doc = "0x3c - Endpoint Control and Status Register 3"]
    #[inline(always)]
    pub const fn csr3(&self) -> &Csr3 {
        &self.csr3
    }
    #[doc = "0x40 - Endpoint Control and Status Register 4"]
    #[inline(always)]
    pub const fn csr4(&self) -> &Csr4 {
        &self.csr4
    }
    #[doc = "0x44 - Endpoint Control and Status Register 5"]
    #[inline(always)]
    pub const fn csr5(&self) -> &Csr5 {
        &self.csr5
    }
    #[doc = "0x48 - Endpoint Control and Status Register 6"]
    #[inline(always)]
    pub const fn csr6(&self) -> &Csr6 {
        &self.csr6
    }
    #[doc = "0x4c - Endpoint Control and Status Register 7"]
    #[inline(always)]
    pub const fn csr7(&self) -> &Csr7 {
        &self.csr7
    }
    #[doc = "0x50 - Endpoint FIFO Data Register 0"]
    #[inline(always)]
    pub const fn fdr0(&self) -> &Fdr0 {
        &self.fdr0
    }
    #[doc = "0x54 - Endpoint FIFO Data Register 1"]
    #[inline(always)]
    pub const fn fdr1(&self) -> &Fdr1 {
        &self.fdr1
    }
    #[doc = "0x58 - Endpoint FIFO Data Register 2"]
    #[inline(always)]
    pub const fn fdr2(&self) -> &Fdr2 {
        &self.fdr2
    }
    #[doc = "0x5c - Endpoint FIFO Data Register 3"]
    #[inline(always)]
    pub const fn fdr3(&self) -> &Fdr3 {
        &self.fdr3
    }
    #[doc = "0x60 - Endpoint FIFO Data Register 4"]
    #[inline(always)]
    pub const fn fdr4(&self) -> &Fdr4 {
        &self.fdr4
    }
    #[doc = "0x64 - Endpoint FIFO Data Register 5"]
    #[inline(always)]
    pub const fn fdr5(&self) -> &Fdr5 {
        &self.fdr5
    }
    #[doc = "0x68 - Endpoint FIFO Data Register 6"]
    #[inline(always)]
    pub const fn fdr6(&self) -> &Fdr6 {
        &self.fdr6
    }
    #[doc = "0x6c - Endpoint FIFO Data Register 7"]
    #[inline(always)]
    pub const fn fdr7(&self) -> &Fdr7 {
        &self.fdr7
    }
    #[doc = "0x74 - Transceiver Control Register"]
    #[inline(always)]
    pub const fn txvc(&self) -> &Txvc {
        &self.txvc
    }
}
#[doc = "FRM_NUM (r) register accessor: Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_num`]
module"]
#[doc(alias = "FRM_NUM")]
pub type FrmNum = crate::Reg<frm_num::FrmNumSpec>;
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "GLB_STAT (rw) register accessor: Global State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_stat`]
module"]
#[doc(alias = "GLB_STAT")]
pub type GlbStat = crate::Reg<glb_stat::GlbStatSpec>;
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "FADDR (rw) register accessor: Function Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`]
module"]
#[doc(alias = "FADDR")]
pub type Faddr = crate::Reg<faddr::FaddrSpec>;
#[doc = "Function Address Register"]
pub mod faddr;
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
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RST_EP (rw) register accessor: Reset Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_ep`]
module"]
#[doc(alias = "RST_EP")]
pub type RstEp = crate::Reg<rst_ep::RstEpSpec>;
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "CSR0 (rw) register accessor: Endpoint Control and Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`]
module"]
#[doc(alias = "CSR0")]
pub type Csr0 = crate::Reg<csr0::Csr0Spec>;
#[doc = "Endpoint Control and Status Register 0"]
pub mod csr0;
#[doc = "CSR1 (rw) register accessor: Endpoint Control and Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = "Endpoint Control and Status Register 1"]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: Endpoint Control and Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
#[doc(alias = "CSR2")]
pub type Csr2 = crate::Reg<csr2::Csr2Spec>;
#[doc = "Endpoint Control and Status Register 2"]
pub mod csr2;
#[doc = "CSR3 (rw) register accessor: Endpoint Control and Status Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`]
module"]
#[doc(alias = "CSR3")]
pub type Csr3 = crate::Reg<csr3::Csr3Spec>;
#[doc = "Endpoint Control and Status Register 3"]
pub mod csr3;
#[doc = "CSR4 (rw) register accessor: Endpoint Control and Status Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr4`]
module"]
#[doc(alias = "CSR4")]
pub type Csr4 = crate::Reg<csr4::Csr4Spec>;
#[doc = "Endpoint Control and Status Register 4"]
pub mod csr4;
#[doc = "CSR5 (rw) register accessor: Endpoint Control and Status Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr5`]
module"]
#[doc(alias = "CSR5")]
pub type Csr5 = crate::Reg<csr5::Csr5Spec>;
#[doc = "Endpoint Control and Status Register 5"]
pub mod csr5;
#[doc = "CSR6 (rw) register accessor: Endpoint Control and Status Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr6`]
module"]
#[doc(alias = "CSR6")]
pub type Csr6 = crate::Reg<csr6::Csr6Spec>;
#[doc = "Endpoint Control and Status Register 6"]
pub mod csr6;
#[doc = "CSR7 (rw) register accessor: Endpoint Control and Status Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr7`]
module"]
#[doc(alias = "CSR7")]
pub type Csr7 = crate::Reg<csr7::Csr7Spec>;
#[doc = "Endpoint Control and Status Register 7"]
pub mod csr7;
#[doc = "ISOENDPT_CSR0_ISOENDPT (rw) register accessor: Endpoint Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_csr0_isoendpt::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_csr0_isoendpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_csr0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_CSR0_ISOENDPT")]
pub type IsoendptCsr0Isoendpt = crate::Reg<isoendpt_csr0_isoendpt::IsoendptCsr0IsoendptSpec>;
#[doc = "Endpoint Control and Status Register"]
pub mod isoendpt_csr0_isoendpt;
#[doc = "FDR0 (rw) register accessor: Endpoint FIFO Data Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr0`]
module"]
#[doc(alias = "FDR0")]
pub type Fdr0 = crate::Reg<fdr0::Fdr0Spec>;
#[doc = "Endpoint FIFO Data Register 0"]
pub mod fdr0;
#[doc = "FDR1 (rw) register accessor: Endpoint FIFO Data Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr1`]
module"]
#[doc(alias = "FDR1")]
pub type Fdr1 = crate::Reg<fdr1::Fdr1Spec>;
#[doc = "Endpoint FIFO Data Register 1"]
pub mod fdr1;
#[doc = "FDR2 (rw) register accessor: Endpoint FIFO Data Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr2`]
module"]
#[doc(alias = "FDR2")]
pub type Fdr2 = crate::Reg<fdr2::Fdr2Spec>;
#[doc = "Endpoint FIFO Data Register 2"]
pub mod fdr2;
#[doc = "FDR3 (rw) register accessor: Endpoint FIFO Data Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr3`]
module"]
#[doc(alias = "FDR3")]
pub type Fdr3 = crate::Reg<fdr3::Fdr3Spec>;
#[doc = "Endpoint FIFO Data Register 3"]
pub mod fdr3;
#[doc = "FDR4 (rw) register accessor: Endpoint FIFO Data Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr4`]
module"]
#[doc(alias = "FDR4")]
pub type Fdr4 = crate::Reg<fdr4::Fdr4Spec>;
#[doc = "Endpoint FIFO Data Register 4"]
pub mod fdr4;
#[doc = "FDR5 (rw) register accessor: Endpoint FIFO Data Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr5`]
module"]
#[doc(alias = "FDR5")]
pub type Fdr5 = crate::Reg<fdr5::Fdr5Spec>;
#[doc = "Endpoint FIFO Data Register 5"]
pub mod fdr5;
#[doc = "FDR6 (rw) register accessor: Endpoint FIFO Data Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr6`]
module"]
#[doc(alias = "FDR6")]
pub type Fdr6 = crate::Reg<fdr6::Fdr6Spec>;
#[doc = "Endpoint FIFO Data Register 6"]
pub mod fdr6;
#[doc = "FDR7 (rw) register accessor: Endpoint FIFO Data Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr7`]
module"]
#[doc(alias = "FDR7")]
pub type Fdr7 = crate::Reg<fdr7::Fdr7Spec>;
#[doc = "Endpoint FIFO Data Register 7"]
pub mod fdr7;
#[doc = "TXVC (rw) register accessor: Transceiver Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txvc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txvc`]
module"]
#[doc(alias = "TXVC")]
pub type Txvc = crate::Reg<txvc::TxvcSpec>;
#[doc = "Transceiver Control Register"]
pub mod txvc;
