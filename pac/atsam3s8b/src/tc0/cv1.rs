#[doc = "Register `CV1` reader"]
pub type R = crate::R<Cv1Spec>;
#[doc = "Field `CV` reader - Counter Value"]
pub type CvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cv1Spec;
impl crate::RegisterSpec for Cv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv1::R`](R) reader structure"]
impl crate::Readable for Cv1Spec {}
#[doc = "`reset()` method sets CV1 to value 0"]
impl crate::Resettable for Cv1Spec {
    const RESET_VALUE: u32 = 0;
}
