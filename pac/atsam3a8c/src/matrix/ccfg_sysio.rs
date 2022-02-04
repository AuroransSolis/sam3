#[doc = "Register `CCFG_SYSIO` reader"]
pub struct R(crate::R<CCFG_SYSIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SYSIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SYSIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SYSIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_SYSIO` writer"]
pub struct W(crate::W<CCFG_SYSIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_SYSIO_SPEC>;
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
impl From<crate::W<CCFG_SYSIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_SYSIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSIO12` reader - PC0 or ERASE Assignment"]
pub struct SYSIO12_R(crate::FieldReader<bool, bool>);
impl SYSIO12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO12` writer - PC0 or ERASE Assignment"]
pub struct SYSIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&mut self) -> SYSIO12_W {
        SYSIO12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System I/O Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](index.html) module"]
pub struct CCFG_SYSIO_SPEC;
impl crate::RegisterSpec for CCFG_SYSIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_sysio::R](R) reader structure"]
impl crate::Readable for CCFG_SYSIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](W) writer structure"]
impl crate::Writable for CCFG_SYSIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0"]
impl crate::Resettable for CCFG_SYSIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
