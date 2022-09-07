#[doc = "Register `TNPR` reader"]
pub struct R(crate::R<TNPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TNPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TNPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TNPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TNPR` writer"]
pub struct W(crate::W<TNPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TNPR_SPEC>;
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
impl From<crate::W<TNPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TNPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXNPTR` reader - Transmit Next Pointer"]
pub type TXNPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXNPTR` writer - Transmit Next Pointer"]
pub type TXNPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TNPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnptr(&self) -> TXNPTR_R {
        TXNPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnptr(&mut self) -> TXNPTR_W<0> {
        TXNPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tnpr](index.html) module"]
pub struct TNPR_SPEC;
impl crate::RegisterSpec for TNPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tnpr::R](R) reader structure"]
impl crate::Readable for TNPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tnpr::W](W) writer structure"]
impl crate::Writable for TNPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TNPR to value 0"]
impl crate::Resettable for TNPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
