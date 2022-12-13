#[doc = "Register `CV1` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CV1_SPEC>);
#[doc = "Field `CV` reader - Counter Value"]
pub type CV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv1](index.html) module"]
pub struct CV1_SPEC;
impl crate::RegisterSpec for CV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cv1::R](R) reader structure"]
impl crate::Readable for CV1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CV1 to value 0"]
impl crate::Resettable for CV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
