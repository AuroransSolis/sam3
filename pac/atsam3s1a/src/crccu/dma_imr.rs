#[doc = "Register `DMA_IMR` reader"]
pub type R = crate::R<DmaImrSpec>;
#[doc = "Field `DMAIMR` reader - Interrupt Mask Register"]
pub type DmaimrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn dmaimr(&self) -> DmaimrR {
        DmaimrR::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaImrSpec;
impl crate::RegisterSpec for DmaImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_imr::R`](R) reader structure"]
impl crate::Readable for DmaImrSpec {}
#[doc = "`reset()` method sets DMA_IMR to value 0"]
impl crate::Resettable for DmaImrSpec {
    const RESET_VALUE: u32 = 0;
}
