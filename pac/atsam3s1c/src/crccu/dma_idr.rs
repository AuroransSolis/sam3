#[doc = "Register `DMA_IDR` writer"]
pub type W = crate::W<DMA_IDR_SPEC>;
#[doc = "Field `DMAIDR` writer - Interrupt Disable register"]
pub type DMAIDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Interrupt Disable register"]
    #[inline(always)]
    #[must_use]
    pub fn dmaidr(&mut self) -> DMAIDR_W<DMA_IDR_SPEC, 0> {
        DMAIDR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRCCU DMA Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IDR_SPEC;
impl crate::RegisterSpec for DMA_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_idr::W`](W) writer structure"]
impl crate::Writable for DMA_IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IDR to value 0"]
impl crate::Resettable for DMA_IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
