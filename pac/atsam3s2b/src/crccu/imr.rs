#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `ERRIMR` reader - CRC Error Interrupt Mask"]
pub type ErrimrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn errimr(&self) -> ErrimrR {
        ErrimrR::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
