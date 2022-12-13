#[doc = "Register `FCSE` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<FCSE_SPEC>);
#[doc = "Register `FCSE` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<FCSE_SPEC>);
#[doc = "Field `FCSE` reader - Frame Check Sequence Errors"]
pub type FCSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCSE` writer - Frame Check Sequence Errors"]
pub type FCSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCSE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fcse(&self) -> FCSE_R {
        FCSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    #[must_use]
    pub fn fcse(&mut self) -> FCSE_W<0> {
        FCSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcse](index.html) module"]
pub struct FCSE_SPEC;
impl crate::RegisterSpec for FCSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcse::R](R) reader structure"]
impl crate::Readable for FCSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcse::W](W) writer structure"]
impl crate::Writable for FCSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCSE to value 0"]
impl crate::Resettable for FCSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
