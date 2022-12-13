#[doc = "Register `RRE` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RRE_SPEC>);
#[doc = "Register `RRE` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RRE_SPEC>);
#[doc = "Field `RRE` reader - Receive Resource Errors"]
pub type RRE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RRE` writer - Receive Resource Errors"]
pub type RRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RRE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rre(&self) -> RRE_R {
        RRE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    #[must_use]
    pub fn rre(&mut self) -> RRE_W<0> {
        RRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rre](index.html) module"]
pub struct RRE_SPEC;
impl crate::RegisterSpec for RRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rre::R](R) reader structure"]
impl crate::Readable for RRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rre::W](W) writer structure"]
impl crate::Writable for RRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RRE to value 0"]
impl crate::Resettable for RRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
