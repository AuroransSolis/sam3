#[doc = "Register `RSPR[%s]` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RSPR_SPEC>);
#[doc = "Field `RSP` reader - Response"]
pub type RSP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new(self.bits)
    }
}
#[doc = "Response Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rspr](index.html) module"]
pub struct RSPR_SPEC;
impl crate::RegisterSpec for RSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rspr::R](R) reader structure"]
impl crate::Readable for RSPR_SPEC {
    type Reader = R;
}
