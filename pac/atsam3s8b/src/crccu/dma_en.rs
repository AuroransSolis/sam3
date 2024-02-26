#[doc = "Register `DMA_EN` writer"]
pub type W = crate::W<DmaEnSpec>;
#[doc = "Field `DMAEN` writer - DMA Enable Register"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DMA Enable Register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<DmaEnSpec> {
        DmaenW::new(self, 0)
    }
}
#[doc = "CRCCU DMA Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_en::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnSpec;
impl crate::RegisterSpec for DmaEnSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_en::W`](W) writer structure"]
impl crate::Writable for DmaEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_EN to value 0"]
impl crate::Resettable for DmaEnSpec {
    const RESET_VALUE: u32 = 0;
}
