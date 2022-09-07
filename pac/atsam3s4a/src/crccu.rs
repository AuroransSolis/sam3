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
#[doc = "DSCR (rw) register accessor: an alias for `Reg<DSCR_SPEC>`"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "CRCCU Descriptor Base Register"]
pub mod dscr;
#[doc = "DMA_EN (w) register accessor: an alias for `Reg<DMA_EN_SPEC>`"]
pub type DMA_EN = crate::Reg<dma_en::DMA_EN_SPEC>;
#[doc = "CRCCU DMA Enable Register"]
pub mod dma_en;
#[doc = "DMA_DIS (w) register accessor: an alias for `Reg<DMA_DIS_SPEC>`"]
pub type DMA_DIS = crate::Reg<dma_dis::DMA_DIS_SPEC>;
#[doc = "CRCCU DMA Disable Register"]
pub mod dma_dis;
#[doc = "DMA_SR (r) register accessor: an alias for `Reg<DMA_SR_SPEC>`"]
pub type DMA_SR = crate::Reg<dma_sr::DMA_SR_SPEC>;
#[doc = "CRCCU DMA Status Register"]
pub mod dma_sr;
#[doc = "DMA_IER (w) register accessor: an alias for `Reg<DMA_IER_SPEC>`"]
pub type DMA_IER = crate::Reg<dma_ier::DMA_IER_SPEC>;
#[doc = "CRCCU DMA Interrupt Enable Register"]
pub mod dma_ier;
#[doc = "DMA_IDR (w) register accessor: an alias for `Reg<DMA_IDR_SPEC>`"]
pub type DMA_IDR = crate::Reg<dma_idr::DMA_IDR_SPEC>;
#[doc = "CRCCU DMA Interrupt Disable Register"]
pub mod dma_idr;
#[doc = "DMA_IMR (r) register accessor: an alias for `Reg<DMA_IMR_SPEC>`"]
pub type DMA_IMR = crate::Reg<dma_imr::DMA_IMR_SPEC>;
#[doc = "CRCCU DMA Interrupt Mask Register"]
pub mod dma_imr;
#[doc = "DMA_ISR (r) register accessor: an alias for `Reg<DMA_ISR_SPEC>`"]
pub type DMA_ISR = crate::Reg<dma_isr::DMA_ISR_SPEC>;
#[doc = "CRCCU DMA Interrupt Status Register"]
pub mod dma_isr;
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CRCCU Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "CRCCU Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "CRCCU Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "CRCCU Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "CRCCU Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "CRCCU Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "CRCCU Interrupt Status Register"]
pub mod isr;
