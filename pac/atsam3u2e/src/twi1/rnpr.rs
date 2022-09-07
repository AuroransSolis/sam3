#[doc = "Register `RNPR` reader"]
pub struct R(crate::R<RNPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNPR` writer"]
pub struct W(crate::W<RNPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RNPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNPTR` reader - Receive Next Pointer"]
pub type RXNPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXNPTR` writer - Receive Next Pointer"]
pub type RXNPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RNPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RXNPTR_R {
        RXNPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&mut self) -> RXNPTR_W<0> {
        RXNPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnpr](index.html) module"]
pub struct RNPR_SPEC;
impl crate::RegisterSpec for RNPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnpr::R](R) reader structure"]
impl crate::Readable for RNPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnpr::W](W) writer structure"]
impl crate::Writable for RNPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNPR to value 0"]
impl crate::Resettable for RNPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
