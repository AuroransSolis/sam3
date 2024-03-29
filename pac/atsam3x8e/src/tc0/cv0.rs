#[doc = "Register `CV0` reader"]
pub type R = crate::R<Cv0Spec>;
#[doc = "Field `CV` reader - Counter Value"]
pub type CvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cv0Spec;
impl crate::RegisterSpec for Cv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv0::R`](R) reader structure"]
impl crate::Readable for Cv0Spec {}
#[doc = "`reset()` method sets CV0 to value 0"]
impl crate::Resettable for Cv0Spec {
    const RESET_VALUE: u32 = 0;
}
