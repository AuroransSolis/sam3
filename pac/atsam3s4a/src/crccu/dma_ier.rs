#[doc = "Register `DMA_IER` writer"]
pub type W = crate::W<DMA_IER_SPEC>;
#[doc = "Field `DMAIER` writer - Interrupt Enable register"]
pub type DMAIER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Interrupt Enable register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaier(&mut self) -> DMAIER_W<DMA_IER_SPEC, 0> {
        DMAIER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRCCU DMA Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IER_SPEC;
impl crate::RegisterSpec for DMA_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_ier::W`](W) writer structure"]
impl crate::Writable for DMA_IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IER to value 0"]
impl crate::Resettable for DMA_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
