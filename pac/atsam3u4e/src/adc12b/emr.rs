#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFMODES` reader - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
pub struct OFFMODES_R(crate::FieldReader<bool, bool>);
impl OFFMODES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFMODES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFMODES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFMODES` writer - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
pub struct OFFMODES_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFMODES_W<'a> {
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
#[doc = "Field `OFF_MODE_STARTUP_TIME` reader - Startup Time"]
pub struct OFF_MODE_STARTUP_TIME_R(crate::FieldReader<u8, u8>);
impl OFF_MODE_STARTUP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OFF_MODE_STARTUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF_MODE_STARTUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF_MODE_STARTUP_TIME` writer - Startup Time"]
pub struct OFF_MODE_STARTUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_MODE_STARTUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
    #[inline(always)]
    pub fn offmodes(&self) -> OFFMODES_R {
        OFFMODES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Startup Time"]
    #[inline(always)]
    pub fn off_mode_startup_time(&self) -> OFF_MODE_STARTUP_TIME_R {
        OFF_MODE_STARTUP_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
    #[inline(always)]
    pub fn offmodes(&mut self) -> OFFMODES_W {
        OFFMODES_W { w: self }
    }
    #[doc = "Bits 16:23 - Startup Time"]
    #[inline(always)]
    pub fn off_mode_startup_time(&mut self) -> OFF_MODE_STARTUP_TIME_W {
        OFF_MODE_STARTUP_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
