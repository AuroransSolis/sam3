#[doc = "Register `DMA_IER` writer"]
pub type W = crate::W<DmaIerSpec>;
#[doc = "Field `DMAIER` writer - Interrupt Enable register"]
pub type DmaierW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt Enable register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaier(&mut self) -> DmaierW<DmaIerSpec> {
        DmaierW::new(self, 0)
    }
}
#[doc = "CRCCU DMA Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIerSpec;
impl crate::RegisterSpec for DmaIerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_ier::W`](W) writer structure"]
impl crate::Writable for DmaIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_IER to value 0"]
impl crate::Resettable for DmaIerSpec {
    const RESET_VALUE: u32 = 0;
}
