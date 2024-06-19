#[doc = "Register `CV2` reader"]
pub type R = crate::R<Cv2Spec>;
#[doc = "Field `CV` reader - Counter Value"]
pub type CvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`cv2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cv2Spec;
impl crate::RegisterSpec for Cv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv2::R`](R) reader structure"]
impl crate::Readable for Cv2Spec {}
#[doc = "`reset()` method sets CV2 to value 0"]
impl crate::Resettable for Cv2Spec {
    const RESET_VALUE: u32 = 0;
}
