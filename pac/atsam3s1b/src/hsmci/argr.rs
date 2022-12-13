#[doc = "Register `ARGR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ARGR_SPEC>);
#[doc = "Register `ARGR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ARGR_SPEC>);
#[doc = "Field `ARG` reader - Command Argument"]
pub type ARG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARG` writer - Command Argument"]
pub type ARG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARGR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<0> {
        ARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argr](index.html) module"]
pub struct ARGR_SPEC;
impl crate::RegisterSpec for ARGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [argr::R](R) reader structure"]
impl crate::Readable for ARGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [argr::W](W) writer structure"]
impl crate::Writable for ARGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARGR to value 0"]
impl crate::Resettable for ARGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
