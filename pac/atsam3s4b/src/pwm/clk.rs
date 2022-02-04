#[doc = "Register `CLK` reader"]
pub struct R(crate::R<CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK` writer"]
pub struct W(crate::W<CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SPEC>;
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
impl From<crate::W<CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVA` reader - CLKA, CLKB Divide Factor"]
pub struct DIVA_R(crate::FieldReader<u8, u8>);
impl DIVA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVA` writer - CLKA, CLKB Divide Factor"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PREA` reader - CLKA, CLKB Source Clock Selection"]
pub struct PREA_R(crate::FieldReader<u8, u8>);
impl PREA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREA` writer - CLKA, CLKB Source Clock Selection"]
pub struct PREA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DIVB` reader - CLKA, CLKB Divide Factor"]
pub struct DIVB_R(crate::FieldReader<u8, u8>);
impl DIVB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVB` writer - CLKA, CLKB Divide Factor"]
pub struct DIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PREB` reader - CLKA, CLKB Source Clock Selection"]
pub struct PREB_R(crate::FieldReader<u8, u8>);
impl PREB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREB` writer - CLKA, CLKB Source Clock Selection"]
pub struct PREB_W<'a> {
    w: &'a mut W,
}
impl<'a> PREB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&self) -> PREA_R {
        PREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&self) -> PREB_R {
        PREB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 8:11 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&mut self) -> PREA_W {
        PREA_W { w: self }
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W {
        DIVB_W { w: self }
    }
    #[doc = "Bits 24:27 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&mut self) -> PREB_W {
        PREB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
