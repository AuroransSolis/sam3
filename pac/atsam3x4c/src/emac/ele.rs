#[doc = "Register `ELE` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ELE_SPEC>);
#[doc = "Register `ELE` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ELE_SPEC>);
#[doc = "Field `EXL` reader - Excessive Length Errors"]
pub type EXL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXL` writer - Excessive Length Errors"]
pub type EXL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ELE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    pub fn exl(&self) -> EXL_R {
        EXL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    #[must_use]
    pub fn exl(&mut self) -> EXL_W<0> {
        EXL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Excessive Length Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ele](index.html) module"]
pub struct ELE_SPEC;
impl crate::RegisterSpec for ELE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ele::R](R) reader structure"]
impl crate::Readable for ELE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ele::W](W) writer structure"]
impl crate::Writable for ELE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELE to value 0"]
impl crate::Resettable for ELE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
