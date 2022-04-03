#[doc = "Register `CDR` writer"]
pub struct W(crate::W<CDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDR_SPEC>;
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
impl From<crate::W<CDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDR_HW0_DATA` writer - Data field of the lower CDR half-word"]
pub struct CDR_HW0_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CDR_HW0_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Channel select field of the lower CDR half-word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDR_HW0_CHSEL_AW {
    #[doc = "0: Channel 0"]
    CHANNEL0 = 0,
    #[doc = "1: Channel 1"]
    CHANNEL1 = 1,
}
impl From<CDR_HW0_CHSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: CDR_HW0_CHSEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CDR_HW0_CHSEL` writer - Channel select field of the lower CDR half-word"]
pub struct CDR_HW0_CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDR_HW0_CHSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDR_HW0_CHSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(CDR_HW0_CHSEL_AW::CHANNEL0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(CDR_HW0_CHSEL_AW::CHANNEL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CDR_HW1_DATA` writer - Data field of the upper CDR half-word"]
pub struct CDR_HW1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CDR_HW1_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Channel select field of the upper CDR half-word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDR_HW1_CHSEL_AW {
    #[doc = "0: Channel 0"]
    CHANNEL0 = 0,
    #[doc = "1: Channel 1"]
    CHANNEL1 = 1,
}
impl From<CDR_HW1_CHSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: CDR_HW1_CHSEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CDR_HW1_CHSEL` writer - Channel select field of the upper CDR half-word"]
pub struct CDR_HW1_CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDR_HW1_CHSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDR_HW1_CHSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(CDR_HW1_CHSEL_AW::CHANNEL0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(CDR_HW1_CHSEL_AW::CHANNEL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - Data field of the lower CDR half-word"]
    #[inline(always)]
    pub fn cdr_hw0_data(&mut self) -> CDR_HW0_DATA_W {
        CDR_HW0_DATA_W { w: self }
    }
    #[doc = "Bits 12:13 - Channel select field of the lower CDR half-word"]
    #[inline(always)]
    pub fn cdr_hw0_chsel(&mut self) -> CDR_HW0_CHSEL_W {
        CDR_HW0_CHSEL_W { w: self }
    }
    #[doc = "Bits 16:27 - Data field of the upper CDR half-word"]
    #[inline(always)]
    pub fn cdr_hw1_data(&mut self) -> CDR_HW1_DATA_W {
        CDR_HW1_DATA_W { w: self }
    }
    #[doc = "Bits 28:29 - Channel select field of the upper CDR half-word"]
    #[inline(always)]
    pub fn cdr_hw1_chsel(&mut self) -> CDR_HW1_CHSEL_W {
        CDR_HW1_CHSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conversion Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cdr::W](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
