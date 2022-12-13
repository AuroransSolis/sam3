#[doc = "Register `RSE` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RSE_SPEC>);
#[doc = "Register `RSE` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RSE_SPEC>);
#[doc = "Field `RSE` reader - Receive Symbol Errors"]
pub type RSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSE` writer - Receive Symbol Errors"]
pub type RSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<0> {
        RSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rse](index.html) module"]
pub struct RSE_SPEC;
impl crate::RegisterSpec for RSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rse::R](R) reader structure"]
impl crate::Readable for RSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rse::W](W) writer structure"]
impl crate::Writable for RSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSE to value 0"]
impl crate::Resettable for RSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
