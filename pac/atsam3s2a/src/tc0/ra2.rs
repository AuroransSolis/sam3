#[doc = "Register `RA2` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RA2_SPEC>);
#[doc = "Register `RA2` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RA2_SPEC>);
#[doc = "Field `RA` reader - Register A"]
pub type RA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RA` writer - Register A"]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RA2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<0> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register A (channel = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra2](index.html) module"]
pub struct RA2_SPEC;
impl crate::RegisterSpec for RA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ra2::R](R) reader structure"]
impl crate::Readable for RA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra2::W](W) writer structure"]
impl crate::Writable for RA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RA2 to value 0"]
impl crate::Resettable for RA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
