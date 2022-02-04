#[doc = "Register `TPR` reader"]
pub struct R(crate::R<TPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR` writer"]
pub struct W(crate::W<TPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR_SPEC>;
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
impl From<crate::W<TPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPTR` reader - Transmit Counter Register"]
pub struct TXPTR_R(crate::FieldReader<u32, u32>);
impl TXPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TXPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPTR` writer - Transmit Counter Register"]
pub struct TXPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txptr(&self) -> TXPTR_R {
        TXPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txptr(&mut self) -> TXPTR_W {
        TXPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr](index.html) module"]
pub struct TPR_SPEC;
impl crate::RegisterSpec for TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr::R](R) reader structure"]
impl crate::Readable for TPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr::W](W) writer structure"]
impl crate::Writable for TPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
