#[doc = "Register `SEQR2` reader"]
pub struct R(crate::R<SEQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQR2` writer"]
pub struct W(crate::W<SEQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQR2_SPEC>;
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
impl From<crate::W<SEQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub struct USCH9_R(crate::FieldReader<u8, u8>);
impl USCH9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub struct USCH9_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub struct USCH10_R(crate::FieldReader<u8, u8>);
impl USCH10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub struct USCH10_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub struct USCH11_R(crate::FieldReader<u8, u8>);
impl USCH11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub struct USCH11_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `USCH12` reader - User Sequence Number 12"]
pub struct USCH12_R(crate::FieldReader<u8, u8>);
impl USCH12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH12` writer - User Sequence Number 12"]
pub struct USCH12_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `USCH13` reader - User Sequence Number 13"]
pub struct USCH13_R(crate::FieldReader<u8, u8>);
impl USCH13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH13` writer - User Sequence Number 13"]
pub struct USCH13_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `USCH14` reader - User Sequence Number 14"]
pub struct USCH14_R(crate::FieldReader<u8, u8>);
impl USCH14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH14` writer - User Sequence Number 14"]
pub struct USCH14_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `USCH15` reader - User Sequence Number 15"]
pub struct USCH15_R(crate::FieldReader<u8, u8>);
impl USCH15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USCH15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USCH15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USCH15` writer - User Sequence Number 15"]
pub struct USCH15_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 12"]
    #[inline(always)]
    pub fn usch12(&self) -> USCH12_R {
        USCH12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 13"]
    #[inline(always)]
    pub fn usch13(&self) -> USCH13_R {
        USCH13_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 14"]
    #[inline(always)]
    pub fn usch14(&self) -> USCH14_R {
        USCH14_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 15"]
    #[inline(always)]
    pub fn usch15(&self) -> USCH15_R {
        USCH15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&mut self) -> USCH9_W {
        USCH9_W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&mut self) -> USCH10_W {
        USCH10_W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&mut self) -> USCH11_W {
        USCH11_W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 12"]
    #[inline(always)]
    pub fn usch12(&mut self) -> USCH12_W {
        USCH12_W { w: self }
    }
    #[doc = "Bits 16:19 - User Sequence Number 13"]
    #[inline(always)]
    pub fn usch13(&mut self) -> USCH13_W {
        USCH13_W { w: self }
    }
    #[doc = "Bits 20:23 - User Sequence Number 14"]
    #[inline(always)]
    pub fn usch14(&mut self) -> USCH14_W {
        USCH14_W { w: self }
    }
    #[doc = "Bits 24:27 - User Sequence Number 15"]
    #[inline(always)]
    pub fn usch15(&mut self) -> USCH15_W {
        USCH15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Sequence Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr2](index.html) module"]
pub struct SEQR2_SPEC;
impl crate::RegisterSpec for SEQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqr2::R](R) reader structure"]
impl crate::Readable for SEQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqr2::W](W) writer structure"]
impl crate::Writable for SEQR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQR2 to value 0"]
impl crate::Resettable for SEQR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
