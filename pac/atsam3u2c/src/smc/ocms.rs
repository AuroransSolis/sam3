#[doc = "Register `OCMS` reader"]
pub struct R(crate::R<OCMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCMS` writer"]
pub struct W(crate::W<OCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMS_SPEC>;
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
impl From<crate::W<OCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub struct SMSE_R(crate::FieldReader<bool, bool>);
impl SMSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub struct SMSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SRSE` reader - SRAM Scrambling Enable"]
pub struct SRSE_R(crate::FieldReader<bool, bool>);
impl SRSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRSE` writer - SRAM Scrambling Enable"]
pub struct SRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    pub fn srse(&self) -> SRSE_R {
        SRSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&mut self) -> SMSE_W {
        SMSE_W { w: self }
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    pub fn srse(&mut self) -> SRSE_W {
        SRSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](index.html) module"]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocms::R](R) reader structure"]
impl crate::Readable for OCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocms::W](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
