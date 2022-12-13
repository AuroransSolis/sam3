#[doc = "Register `FRO` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<FRO_SPEC>);
#[doc = "Register `FRO` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<FRO_SPEC>);
#[doc = "Field `FROK` reader - Frames Received OK"]
pub type FROK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FROK` writer - Frames Received OK"]
pub type FROK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRO_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Received OK"]
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
#[doc = "Frames Received Ok Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro](index.html) module"]
pub struct FRO_SPEC;
impl crate::RegisterSpec for FRO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro::R](R) reader structure"]
impl crate::Readable for FRO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro::W](W) writer structure"]
impl crate::Writable for FRO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRO to value 0"]
impl crate::Resettable for FRO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
