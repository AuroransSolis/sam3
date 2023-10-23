#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRCCU Descriptor Base Register"]
    pub dscr: DSCR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - CRCCU DMA Enable Register"]
    pub dma_en: DMA_EN,
    #[doc = "0x0c - CRCCU DMA Disable Register"]
    pub dma_dis: DMA_DIS,
    #[doc = "0x10 - CRCCU DMA Status Register"]
    pub dma_sr: DMA_SR,
    #[doc = "0x14 - CRCCU DMA Interrupt Enable Register"]
    pub dma_ier: DMA_IER,
    #[doc = "0x18 - CRCCU DMA Interrupt Disable Register"]
    pub dma_idr: DMA_IDR,
    #[doc = "0x1c - CRCCU DMA Interrupt Mask Register"]
    pub dma_imr: DMA_IMR,
    #[doc = "0x20 - CRCCU DMA Interrupt Status Register"]
    pub dma_isr: DMA_ISR,
    _reserved8: [u8; 0x10],
    #[doc = "0x34 - CRCCU Control Register"]
    pub cr: CR,
    #[doc = "0x38 - CRCCU Mode Register"]
    pub mr: MR,
    #[doc = "0x3c - CRCCU Status Register"]
    pub sr: SR,
    #[doc = "0x40 - CRCCU Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - CRCCU Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - CRCCU Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - CRCCU Interrupt Status Register"]
    pub isr: ISR,
}
#[doc = "DSCR (rw) register accessor: CRCCU Descriptor Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`]
module"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "CRCCU Descriptor Base Register"]
pub mod dscr;
#[doc = "DMA_EN (w) register accessor: CRCCU DMA Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_en::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_en`]
module"]
pub type DMA_EN = crate::Reg<dma_en::DMA_EN_SPEC>;
#[doc = "CRCCU DMA Enable Register"]
pub mod dma_en;
#[doc = "DMA_DIS (w) register accessor: CRCCU DMA Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dis`]
module"]
pub type DMA_DIS = crate::Reg<dma_dis::DMA_DIS_SPEC>;
#[doc = "CRCCU DMA Disable Register"]
pub mod dma_dis;
#[doc = "DMA_SR (r) register accessor: CRCCU DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sr`]
module"]
pub type DMA_SR = crate::Reg<dma_sr::DMA_SR_SPEC>;
#[doc = "CRCCU DMA Status Register"]
pub mod dma_sr;
#[doc = "DMA_IER (w) register accessor: CRCCU DMA Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ier`]
module"]
pub type DMA_IER = crate::Reg<dma_ier::DMA_IER_SPEC>;
#[doc = "CRCCU DMA Interrupt Enable Register"]
pub mod dma_ier;
#[doc = "DMA_IDR (w) register accessor: CRCCU DMA Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_idr`]
module"]
pub type DMA_IDR = crate::Reg<dma_idr::DMA_IDR_SPEC>;
#[doc = "CRCCU DMA Interrupt Disable Register"]
pub mod dma_idr;
#[doc = "DMA_IMR (r) register accessor: CRCCU DMA Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_imr`]
module"]
pub type DMA_IMR = crate::Reg<dma_imr::DMA_IMR_SPEC>;
#[doc = "CRCCU DMA Interrupt Mask Register"]
pub mod dma_imr;
#[doc = "DMA_ISR (r) register accessor: CRCCU DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_isr`]
module"]
pub type DMA_ISR = crate::Reg<dma_isr::DMA_ISR_SPEC>;
#[doc = "CRCCU DMA Interrupt Status Register"]
pub mod dma_isr;
#[doc = "CR (w) register accessor: CRCCU Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CRCCU Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: CRCCU Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "CRCCU Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: CRCCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "CRCCU Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: CRCCU Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "CRCCU Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: CRCCU Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "CRCCU Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: CRCCU Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "CRCCU Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: CRCCU Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "CRCCU Interrupt Status Register"]
pub mod isr;
