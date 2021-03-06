#[doc = "Register `FCSE` reader"]
pub struct R(crate::R<FCSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCSE` writer"]
pub struct W(crate::W<FCSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCSE_SPEC>;
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
impl From<crate::W<FCSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCSE` reader - Frame Check Sequence Errors"]
pub struct FCSE_R(crate::FieldReader<u8, u8>);
impl FCSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCSE` writer - Frame Check Sequence Errors"]
pub struct FCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fcse(&self) -> FCSE_R {
        FCSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fcse(&mut self) -> FCSE_W {
        FCSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcse](index.html) module"]
pub struct FCSE_SPEC;
impl crate::RegisterSpec for FCSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcse::R](R) reader structure"]
impl crate::Readable for FCSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcse::W](W) writer structure"]
impl crate::Writable for FCSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCSE to value 0"]
impl crate::Resettable for FCSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
