#[doc = "Register `DMA_ISR` reader"]
pub type R = crate::R<DMA_ISR_SPEC>;
#[doc = "Field `DMAISR` reader - Interrupt Status register"]
pub type DMAISR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status register"]
    #[inline(always)]
    pub fn dmaisr(&self) -> DMAISR_R {
        DMAISR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_ISR_SPEC;
impl crate::RegisterSpec for DMA_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_isr::R`](R) reader structure"]
impl crate::Readable for DMA_ISR_SPEC {}
#[doc = "`reset()` method sets DMA_ISR to value 0"]
impl crate::Resettable for DMA_ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
