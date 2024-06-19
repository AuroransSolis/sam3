#[doc = "Register `DMA_IDR` writer"]
pub type W = crate::W<DmaIdrSpec>;
#[doc = "Field `DMAIDR` writer - Interrupt Disable register"]
pub type DmaidrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt Disable register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaidr(&mut self) -> DmaidrW<DmaIdrSpec> {
        DmaidrW::new(self, 0)
    }
}
#[doc = "CRCCU DMA Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIdrSpec;
impl crate::RegisterSpec for DmaIdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_idr::W`](W) writer structure"]
impl crate::Writable for DmaIdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_IDR to value 0"]
impl crate::Resettable for DmaIdrSpec {
    const RESET_VALUE: u32 = 0;
}
