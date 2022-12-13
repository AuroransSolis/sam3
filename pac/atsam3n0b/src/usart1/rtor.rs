#[doc = "Register `RTOR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RTOR_SPEC>);
#[doc = "Register `RTOR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RTOR_SPEC>);
#[doc = "Field `TO` reader - Time-out Value"]
pub type TO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TO` writer - Time-out Value"]
pub type TO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTOR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Time-out Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time-out Value"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](index.html) module"]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtor::R](R) reader structure"]
impl crate::Readable for RTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtor::W](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
