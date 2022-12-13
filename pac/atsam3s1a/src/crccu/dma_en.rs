#[doc = "Register `DMA_EN` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<DMA_EN_SPEC>);
#[doc = "Field `DMAEN` writer - DMA Enable Register"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_EN_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DMA Enable Register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCU DMA Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_en](index.html) module"]
pub struct DMA_EN_SPEC;
impl crate::RegisterSpec for DMA_EN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_en::W](W) writer structure"]
impl crate::Writable for DMA_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_EN to value 0"]
impl crate::Resettable for DMA_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
