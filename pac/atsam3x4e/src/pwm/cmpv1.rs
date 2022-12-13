#[doc = "Register `CMPV1` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CMPV1_SPEC>);
#[doc = "Register `CMPV1` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CMPV1_SPEC>);
#[doc = "Field `CV` reader - Comparison x Value"]
pub type CV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CV` writer - Comparison x Value"]
pub type CV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPV1_SPEC, u32, u32, 24, O>;
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub type CVM_R = crate::BitReader<bool>;
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub type CVM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPV1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CV_W<0> {
        CV_W::new(self)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cvm(&mut self) -> CVM_W<24> {
        CVM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 1 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv1](index.html) module"]
pub struct CMPV1_SPEC;
impl crate::RegisterSpec for CMPV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpv1::R](R) reader structure"]
impl crate::Readable for CMPV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpv1::W](W) writer structure"]
impl crate::Writable for CMPV1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPV1 to value 0"]
impl crate::Resettable for CMPV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
