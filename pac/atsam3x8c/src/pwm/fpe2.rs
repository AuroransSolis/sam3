#[doc = "Register `FPE2` reader"]
pub struct R(crate::R<FPE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPE2` writer"]
pub struct W(crate::W<FPE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPE2_SPEC>;
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
impl From<crate::W<FPE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPE4` reader - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub struct FPE4_R(crate::FieldReader<u8, u8>);
impl FPE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE4` writer - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub struct FPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `FPE5` reader - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub struct FPE5_R(crate::FieldReader<u8, u8>);
impl FPE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE5` writer - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub struct FPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FPE6` reader - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub struct FPE6_R(crate::FieldReader<u8, u8>);
impl FPE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE6` writer - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub struct FPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `FPE7` reader - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub struct FPE7_R(crate::FieldReader<u8, u8>);
impl FPE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE7` writer - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub struct FPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe4(&self) -> FPE4_R {
        FPE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&self) -> FPE5_R {
        FPE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&self) -> FPE6_R {
        FPE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&self) -> FPE7_R {
        FPE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE4_W {
        FPE4_W { w: self }
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&mut self) -> FPE5_W {
        FPE5_W { w: self }
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&mut self) -> FPE6_W {
        FPE6_W { w: self }
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&mut self) -> FPE7_W {
        FPE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpe2](index.html) module"]
pub struct FPE2_SPEC;
impl crate::RegisterSpec for FPE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpe2::R](R) reader structure"]
impl crate::Readable for FPE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpe2::W](W) writer structure"]
impl crate::Writable for FPE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPE2 to value 0"]
impl crate::Resettable for FPE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
