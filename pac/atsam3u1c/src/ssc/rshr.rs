#[doc = "Register `RSHR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RSHR_SPEC>);
#[doc = "Field `RSDAT` reader - Receive Synchronization Data"]
pub type RSDAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Synchronization Data"]
    #[inline(always)]
    pub fn rsdat(&self) -> RSDAT_R {
        RSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rshr](index.html) module"]
pub struct RSHR_SPEC;
impl crate::RegisterSpec for RSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rshr::R](R) reader structure"]
impl crate::Readable for RSHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSHR to value 0"]
impl crate::Resettable for RSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
