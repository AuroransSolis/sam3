#[doc = "Register `DMA_IMR` reader"]
pub struct R(crate::R<DMA_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMAIMR` reader - Interrupt Mask Register"]
pub type DMAIMR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn dmaimr(&self) -> DMAIMR_R {
        DMAIMR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_imr](index.html) module"]
pub struct DMA_IMR_SPEC;
impl crate::RegisterSpec for DMA_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_imr::R](R) reader structure"]
impl crate::Readable for DMA_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IMR to value 0"]
impl crate::Resettable for DMA_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
