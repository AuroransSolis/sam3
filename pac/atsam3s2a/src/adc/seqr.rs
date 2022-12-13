#[doc = "Register `SEQR[%s]` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SEQR_SPEC>);
#[doc = "Register `SEQR[%s]` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<SEQR_SPEC>);
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Sequence Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr](index.html) module"]
pub struct SEQR_SPEC;
impl crate::RegisterSpec for SEQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqr::R](R) reader structure"]
impl crate::Readable for SEQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqr::W](W) writer structure"]
impl crate::Writable for SEQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
