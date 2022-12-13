#[doc = "Register `SCDR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SCDR_SPEC>);
#[doc = "Register `SCDR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<SCDR_SPEC>);
#[doc = "Field `DIV` reader - "]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - "]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCDR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow Clock Divider Debouncing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scdr](index.html) module"]
pub struct SCDR_SPEC;
impl crate::RegisterSpec for SCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scdr::R](R) reader structure"]
impl crate::Readable for SCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scdr::W](W) writer structure"]
impl crate::Writable for SCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCDR to value 0"]
impl crate::Resettable for SCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
