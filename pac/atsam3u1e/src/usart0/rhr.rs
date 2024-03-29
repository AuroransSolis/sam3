#[doc = "Register `RHR` reader"]
pub type R = crate::R<RhrSpec>;
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RxchrR = crate::FieldReader<u16>;
#[doc = "Field `RXSYNH` reader - Received Sync"]
pub type RxsynhR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RxchrR {
        RxchrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RxsynhR {
        RxsynhR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receiver Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RhrSpec;
impl crate::RegisterSpec for RhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rhr::R`](R) reader structure"]
impl crate::Readable for RhrSpec {}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RhrSpec {
    const RESET_VALUE: u32 = 0;
}
