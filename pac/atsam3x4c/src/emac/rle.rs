#[doc = "Register `RLE` reader"]
pub struct R(crate::R<RLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLE` writer"]
pub struct W(crate::W<RLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLE_SPEC>;
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
impl From<crate::W<RLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLFM` reader - Receive Length Field Mismatch"]
pub struct RLFM_R(crate::FieldReader<u8, u8>);
impl RLFM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RLFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLFM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLFM` writer - Receive Length Field Mismatch"]
pub struct RLFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RLFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&self) -> RLFM_R {
        RLFM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&mut self) -> RLFM_W {
        RLFM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Received Length Field Mismatch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rle](index.html) module"]
pub struct RLE_SPEC;
impl crate::RegisterSpec for RLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rle::R](R) reader structure"]
impl crate::Readable for RLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rle::W](W) writer structure"]
impl crate::Writable for RLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RLE to value 0"]
impl crate::Resettable for RLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
