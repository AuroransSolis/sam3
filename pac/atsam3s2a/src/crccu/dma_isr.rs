#[doc = "Register `DMA_ISR` reader"]
pub type R = crate::R<DmaIsrSpec>;
#[doc = "Field `DMAISR` reader - Interrupt Status register"]
pub type DmaisrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status register"]
    #[inline(always)]
    pub fn dmaisr(&self) -> DmaisrR {
        DmaisrR::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIsrSpec;
impl crate::RegisterSpec for DmaIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_isr::R`](R) reader structure"]
impl crate::Readable for DmaIsrSpec {}
#[doc = "`reset()` method sets DMA_ISR to value 0"]
impl crate::Resettable for DmaIsrSpec {
    const RESET_VALUE: u32 = 0;
}
