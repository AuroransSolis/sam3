#[doc = "Register `CCNT2` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CCNT2_SPEC>);
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt2](index.html) module"]
pub struct CCNT2_SPEC;
impl crate::RegisterSpec for CCNT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccnt2::R](R) reader structure"]
impl crate::Readable for CCNT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCNT2 to value 0"]
impl crate::Resettable for CCNT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
