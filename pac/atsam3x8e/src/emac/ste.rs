#[doc = "Register `STE` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<STE_SPEC>);
#[doc = "Register `STE` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<STE_SPEC>);
#[doc = "Field `SQER` reader - SQE test errors"]
pub type SQER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQER` writer - SQE test errors"]
pub type SQER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&self) -> SQER_R {
        SQER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    #[must_use]
    pub fn sqer(&mut self) -> SQER_W<0> {
        SQER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SQE Test Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ste](index.html) module"]
pub struct STE_SPEC;
impl crate::RegisterSpec for STE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ste::R](R) reader structure"]
impl crate::Readable for STE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ste::W](W) writer structure"]
impl crate::Writable for STE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STE to value 0"]
impl crate::Resettable for STE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
