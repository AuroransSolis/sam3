#[doc = "Register `DMA_SR` reader"]
pub type R = crate::R<DmaSrSpec>;
#[doc = "Field `DMASR` reader - DMA Status Register"]
pub type DmasrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Status Register"]
    #[inline(always)]
    pub fn dmasr(&self) -> DmasrR {
        DmasrR::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSrSpec;
impl crate::RegisterSpec for DmaSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sr::R`](R) reader structure"]
impl crate::Readable for DmaSrSpec {}
#[doc = "`reset()` method sets DMA_SR to value 0"]
impl crate::Resettable for DmaSrSpec {
    const RESET_VALUE: u32 = 0;
}
