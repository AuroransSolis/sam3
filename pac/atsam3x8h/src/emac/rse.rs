#[doc = "Register `RSE` reader"]
pub struct R(crate::R<RSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSE` writer"]
pub struct W(crate::W<RSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSE_SPEC>;
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
impl From<crate::W<RSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSE` reader - Receive Symbol Errors"]
pub struct RSE_R(crate::FieldReader<u8, u8>);
impl RSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSE` writer - Receive Symbol Errors"]
pub struct RSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W {
        RSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rse](index.html) module"]
pub struct RSE_SPEC;
impl crate::RegisterSpec for RSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rse::R](R) reader structure"]
impl crate::Readable for RSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rse::W](W) writer structure"]
impl crate::Writable for RSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSE to value 0"]
impl crate::Resettable for RSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
