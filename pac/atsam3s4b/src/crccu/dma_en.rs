#[doc = "Register `DMA_EN` writer"]
pub type W = crate::W<DMA_EN_SPEC>;
#[doc = "Field `DMAEN` writer - DMA Enable Register"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - DMA Enable Register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DMA_EN_SPEC, 0> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRCCU DMA Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_en::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_EN_SPEC;
impl crate::RegisterSpec for DMA_EN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_en::W`](W) writer structure"]
impl crate::Writable for DMA_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_EN to value 0"]
impl crate::Resettable for DMA_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
