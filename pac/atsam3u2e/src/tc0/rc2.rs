#[doc = "Register `RC2` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RC2_SPEC>);
#[doc = "Register `RC2` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RC2_SPEC>);
#[doc = "Field `RC` reader - Register C"]
pub type RC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RC` writer - Register C"]
pub type RC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<0> {
        RC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register C (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc2](index.html) module"]
pub struct RC2_SPEC;
impl crate::RegisterSpec for RC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc2::R](R) reader structure"]
impl crate::Readable for RC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc2::W](W) writer structure"]
impl crate::Writable for RC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC2 to value 0"]
impl crate::Resettable for RC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
