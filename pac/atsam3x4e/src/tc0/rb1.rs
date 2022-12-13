#[doc = "Register `RB1` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RB1_SPEC>);
#[doc = "Register `RB1` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RB1_SPEC>);
#[doc = "Field `RB` reader - Register B"]
pub type RB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RB` writer - Register B"]
pub type RB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RB1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RB_W<0> {
        RB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register B (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb1](index.html) module"]
pub struct RB1_SPEC;
impl crate::RegisterSpec for RB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rb1::R](R) reader structure"]
impl crate::Readable for RB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rb1::W](W) writer structure"]
impl crate::Writable for RB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RB1 to value 0"]
impl crate::Resettable for RB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
