#[doc = "Register `RNPR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RNPR_SPEC>);
#[doc = "Register `RNPR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RNPR_SPEC>);
#[doc = "Field `RXNPTR` reader - Receive Next Pointer"]
pub type RXNPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXNPTR` writer - Receive Next Pointer"]
pub type RXNPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RNPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RXNPTR_R {
        RXNPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxnptr(&mut self) -> RXNPTR_W<0> {
        RXNPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnpr](index.html) module"]
pub struct RNPR_SPEC;
impl crate::RegisterSpec for RNPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnpr::R](R) reader structure"]
impl crate::Readable for RNPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnpr::W](W) writer structure"]
impl crate::Writable for RNPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNPR to value 0"]
impl crate::Resettable for RNPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
