#[doc = "Register `DMA_DIS` writer"]
pub type W = crate::W<DmaDisSpec>;
#[doc = "Field `DMADIS` writer - DMA Disable Register"]
pub type DmadisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DMA Disable Register"]
    #[inline(always)]
    #[must_use]
    pub fn dmadis(&mut self) -> DmadisW<DmaDisSpec> {
        DmadisW::new(self, 0)
    }
}
#[doc = "CRCCU DMA Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDisSpec;
impl crate::RegisterSpec for DmaDisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_dis::W`](W) writer structure"]
impl crate::Writable for DmaDisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_DIS to value 0"]
impl crate::Resettable for DmaDisSpec {
    const RESET_VALUE: u32 = 0;
}
