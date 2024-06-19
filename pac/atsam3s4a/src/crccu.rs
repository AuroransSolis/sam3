#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dscr: Dscr,
    _reserved1: [u8; 0x04],
    dma_en: DmaEn,
    dma_dis: DmaDis,
    dma_sr: DmaSr,
    dma_ier: DmaIer,
    dma_idr: DmaIdr,
    dma_imr: DmaImr,
    dma_isr: DmaIsr,
    _reserved8: [u8; 0x10],
    cr: Cr,
    mr: Mr,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
}
impl RegisterBlock {
    #[doc = "0x00 - CRCCU Descriptor Base Register"]
    #[inline(always)]
    pub const fn dscr(&self) -> &Dscr {
        &self.dscr
    }
    #[doc = "0x08 - CRCCU DMA Enable Register"]
    #[inline(always)]
    pub const fn dma_en(&self) -> &DmaEn {
        &self.dma_en
    }
    #[doc = "0x0c - CRCCU DMA Disable Register"]
    #[inline(always)]
    pub const fn dma_dis(&self) -> &DmaDis {
        &self.dma_dis
    }
    #[doc = "0x10 - CRCCU DMA Status Register"]
    #[inline(always)]
    pub const fn dma_sr(&self) -> &DmaSr {
        &self.dma_sr
    }
    #[doc = "0x14 - CRCCU DMA Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dma_ier(&self) -> &DmaIer {
        &self.dma_ier
    }
    #[doc = "0x18 - CRCCU DMA Interrupt Disable Register"]
    #[inline(always)]
    pub const fn dma_idr(&self) -> &DmaIdr {
        &self.dma_idr
    }
    #[doc = "0x1c - CRCCU DMA Interrupt Mask Register"]
    #[inline(always)]
    pub const fn dma_imr(&self) -> &DmaImr {
        &self.dma_imr
    }
    #[doc = "0x20 - CRCCU DMA Interrupt Status Register"]
    #[inline(always)]
    pub const fn dma_isr(&self) -> &DmaIsr {
        &self.dma_isr
    }
    #[doc = "0x34 - CRCCU Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x38 - CRCCU Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x3c - CRCCU Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x40 - CRCCU Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x44 - CRCCU Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x48 - CRCCU Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x4c - CRCCU Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
}
#[doc = "DSCR (rw) register accessor: CRCCU Descriptor Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`]
module"]
#[doc(alias = "DSCR")]
pub type Dscr = crate::Reg<dscr::DscrSpec>;
#[doc = "CRCCU Descriptor Base Register"]
pub mod dscr;
#[doc = "DMA_EN (w) register accessor: CRCCU DMA Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_en::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_en`]
module"]
#[doc(alias = "DMA_EN")]
pub type DmaEn = crate::Reg<dma_en::DmaEnSpec>;
#[doc = "CRCCU DMA Enable Register"]
pub mod dma_en;
#[doc = "DMA_DIS (w) register accessor: CRCCU DMA Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dis`]
module"]
#[doc(alias = "DMA_DIS")]
pub type DmaDis = crate::Reg<dma_dis::DmaDisSpec>;
#[doc = "CRCCU DMA Disable Register"]
pub mod dma_dis;
#[doc = "DMA_SR (r) register accessor: CRCCU DMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sr`]
module"]
#[doc(alias = "DMA_SR")]
pub type DmaSr = crate::Reg<dma_sr::DmaSrSpec>;
#[doc = "CRCCU DMA Status Register"]
pub mod dma_sr;
#[doc = "DMA_IER (w) register accessor: CRCCU DMA Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ier`]
module"]
#[doc(alias = "DMA_IER")]
pub type DmaIer = crate::Reg<dma_ier::DmaIerSpec>;
#[doc = "CRCCU DMA Interrupt Enable Register"]
pub mod dma_ier;
#[doc = "DMA_IDR (w) register accessor: CRCCU DMA Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_idr`]
module"]
#[doc(alias = "DMA_IDR")]
pub type DmaIdr = crate::Reg<dma_idr::DmaIdrSpec>;
#[doc = "CRCCU DMA Interrupt Disable Register"]
pub mod dma_idr;
#[doc = "DMA_IMR (r) register accessor: CRCCU DMA Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_imr`]
module"]
#[doc(alias = "DMA_IMR")]
pub type DmaImr = crate::Reg<dma_imr::DmaImrSpec>;
#[doc = "CRCCU DMA Interrupt Mask Register"]
pub mod dma_imr;
#[doc = "DMA_ISR (r) register accessor: CRCCU DMA Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_isr`]
module"]
#[doc(alias = "DMA_ISR")]
pub type DmaIsr = crate::Reg<dma_isr::DmaIsrSpec>;
#[doc = "CRCCU DMA Interrupt Status Register"]
pub mod dma_isr;
#[doc = "CR (w) register accessor: CRCCU Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CRCCU Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: CRCCU Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "CRCCU Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: CRCCU Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "CRCCU Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: CRCCU Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "CRCCU Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: CRCCU Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "CRCCU Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: CRCCU Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "CRCCU Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: CRCCU Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "CRCCU Interrupt Status Register"]
pub mod isr;
