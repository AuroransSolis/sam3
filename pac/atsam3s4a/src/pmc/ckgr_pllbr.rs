#[doc = "Register `CKGR_PLLBR` reader"]
pub struct R(crate::R<CKGR_PLLBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_PLLBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_PLLBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_PLLBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_PLLBR` writer"]
pub struct W(crate::W<CKGR_PLLBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_PLLBR_SPEC>;
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
impl From<crate::W<CKGR_PLLBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_PLLBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVB` reader - Divider"]
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
#[doc = "Field `DIVB` writer - Divider"]
pub struct DIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PLLBCOUNT` reader - PLLB Counter"]
pub struct PLLBCOUNT_R(crate::FieldReader<u8, u8>);
impl PLLBCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLLBCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLBCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLBCOUNT` writer - PLLB Counter"]
pub struct PLLBCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLBCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `MULB` reader - PLLB Multiplier"]
pub struct MULB_R(crate::FieldReader<u16, u16>);
impl MULB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MULB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULB` writer - PLLB Multiplier"]
pub struct MULB_W<'a> {
    w: &'a mut W,
}
impl<'a> MULB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PLLBCOUNT_R {
        PLLBCOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MULB_R {
        MULB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W {
        DIVB_W { w: self }
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&mut self) -> PLLBCOUNT_W {
        PLLBCOUNT_W { w: self }
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&mut self) -> MULB_W {
        MULB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_pllbr](index.html) module"]
pub struct CKGR_PLLBR_SPEC;
impl crate::RegisterSpec for CKGR_PLLBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_pllbr::R](R) reader structure"]
impl crate::Readable for CKGR_PLLBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_pllbr::W](W) writer structure"]
impl crate::Writable for CKGR_PLLBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_PLLBR to value 0x3f00"]
impl crate::Resettable for CKGR_PLLBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f00
    }
}
