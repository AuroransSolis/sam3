#[doc = "Register `PFR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<PFR_SPEC>);
#[doc = "Register `PFR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<PFR_SPEC>);
#[doc = "Field `FROK` reader - Pause Frames received OK"]
pub type FROK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FROK` writer - Pause Frames received OK"]
pub type FROK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    #[must_use]
    pub fn frok(&mut self) -> FROK_W<0> {
        FROK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pause Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfr](index.html) module"]
pub struct PFR_SPEC;
impl crate::RegisterSpec for PFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfr::R](R) reader structure"]
impl crate::Readable for PFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfr::W](W) writer structure"]
impl crate::Writable for PFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFR to value 0"]
impl crate::Resettable for PFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
