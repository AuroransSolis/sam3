#[doc = "Register `DMA_IDR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<DMA_IDR_SPEC>);
#[doc = "Field `DMAIDR` writer - Interrupt Disable register"]
pub type DMAIDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Interrupt Disable register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaidr(&mut self) -> DMAIDR_W<0> {
        DMAIDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCU DMA Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_idr](index.html) module"]
pub struct DMA_IDR_SPEC;
impl crate::RegisterSpec for DMA_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_idr::W](W) writer structure"]
impl crate::Writable for DMA_IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IDR to value 0"]
impl crate::Resettable for DMA_IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
