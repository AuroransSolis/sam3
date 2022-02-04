#[doc = "Register `TUND` reader"]
pub struct R(crate::R<TUND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TUND` writer"]
pub struct W(crate::W<TUND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUND_SPEC>;
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
impl From<crate::W<TUND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUND` reader - Transmit Underruns"]
pub struct TUND_R(crate::FieldReader<u8, u8>);
impl TUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUND` writer - Transmit Underruns"]
pub struct TUND_W<'a> {
    w: &'a mut W,
}
impl<'a> TUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    pub fn tund(&mut self) -> TUND_W {
        TUND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Underrun Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tund](index.html) module"]
pub struct TUND_SPEC;
impl crate::RegisterSpec for TUND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tund::R](R) reader structure"]
impl crate::Readable for TUND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tund::W](W) writer structure"]
impl crate::Writable for TUND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TUND to value 0"]
impl crate::Resettable for TUND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
