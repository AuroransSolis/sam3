#[doc = "Register `VR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<VR_SPEC>);
#[doc = "Field `CRTV` reader - Current Real-time Value"]
pub type CRTV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Real-time Value"]
    #[inline(always)]
    pub fn crtv(&self) -> CRTV_R {
        CRTV_R::new(self.bits)
    }
}
#[doc = "Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vr](index.html) module"]
pub struct VR_SPEC;
impl crate::RegisterSpec for VR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vr::R](R) reader structure"]
impl crate::Readable for VR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VR to value 0"]
impl crate::Resettable for VR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
