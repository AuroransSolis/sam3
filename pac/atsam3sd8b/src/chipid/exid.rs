#[doc = "Register `EXID` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<EXID_SPEC>);
#[doc = "Field `EXID` reader - Chip ID Extension"]
pub type EXID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID Extension"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(self.bits)
    }
}
#[doc = "Chip ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exid](index.html) module"]
pub struct EXID_SPEC;
impl crate::RegisterSpec for EXID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exid::R](R) reader structure"]
impl crate::Readable for EXID_SPEC {
    type Reader = R;
}
