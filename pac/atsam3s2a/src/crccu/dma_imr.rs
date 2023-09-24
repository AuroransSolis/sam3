#[doc = "Register `DMA_IMR` reader"]
pub type R = crate::R<DMA_IMR_SPEC>;
#[doc = "Field `DMAIMR` reader - Interrupt Mask Register"]
pub type DMAIMR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn dmaimr(&self) -> DMAIMR_R {
        DMAIMR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IMR_SPEC;
impl crate::RegisterSpec for DMA_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_imr::R`](R) reader structure"]
impl crate::Readable for DMA_IMR_SPEC {}
#[doc = "`reset()` method sets DMA_IMR to value 0"]
impl crate::Resettable for DMA_IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
