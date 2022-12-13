#[doc = "Register `RJA` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RJA_SPEC>);
#[doc = "Register `RJA` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RJA_SPEC>);
#[doc = "Field `RJB` reader - Receive Jabbers"]
pub type RJB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RJB` writer - Receive Jabbers"]
pub type RJB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RJA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&self) -> RJB_R {
        RJB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    #[must_use]
    pub fn rjb(&mut self) -> RJB_W<0> {
        RJB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Jabbers Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rja](index.html) module"]
pub struct RJA_SPEC;
impl crate::RegisterSpec for RJA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rja::R](R) reader structure"]
impl crate::Readable for RJA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rja::W](W) writer structure"]
impl crate::Writable for RJA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RJA to value 0"]
impl crate::Resettable for RJA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
