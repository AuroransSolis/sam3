#[doc = "Register `LCDR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<LCDR_SPEC>);
#[doc = "Field `LDATA` reader - Last Data Converted"]
pub type LDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Last Data Converted"]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdr](index.html) module"]
pub struct LCDR_SPEC;
impl crate::RegisterSpec for LCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdr::R](R) reader structure"]
impl crate::Readable for LCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCDR to value 0"]
impl crate::Resettable for LCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
