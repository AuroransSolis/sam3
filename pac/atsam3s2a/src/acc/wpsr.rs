#[doc = "Register `WPSR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<WPSR_SPEC>);
#[doc = "Field `WPROTERR` reader - Write PROTection ERRor"]
pub type WPROTERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Write PROTection ERRor"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
