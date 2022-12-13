#[doc = "Register `PCRHR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<PCRHR_SPEC>);
#[doc = "Field `RDATA` reader - Parallel Capture Mode Reception Data."]
pub type RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Parallel Capture Mode Reception Data."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "Parallel Capture Reception Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrhr](index.html) module"]
pub struct PCRHR_SPEC;
impl crate::RegisterSpec for PCRHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrhr::R](R) reader structure"]
impl crate::Readable for PCRHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCRHR to value 0"]
impl crate::Resettable for PCRHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
