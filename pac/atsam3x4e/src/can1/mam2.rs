#[doc = "Register `MAM2` reader"]
pub struct R(crate::R<MAM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAM2` writer"]
pub struct W(crate::W<MAM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAM2_SPEC>;
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
impl From<crate::W<MAM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIDvB` reader - Complementary bits for identifier in extended frame mode"]
pub struct MIDVB_R(crate::FieldReader<u32, u32>);
impl MIDVB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MIDVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIDVB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIDvB` writer - Complementary bits for identifier in extended frame mode"]
pub struct MIDVB_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
#[doc = "Field `MIDvA` reader - Identifier for standard frame mode"]
pub struct MIDVA_R(crate::FieldReader<u16, u16>);
impl MIDVA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MIDVA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIDVA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIDvA` writer - Identifier for standard frame mode"]
pub struct MIDVA_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDVA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | ((value as u32 & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Field `MIDE` reader - Identifier Version"]
pub struct MIDE_R(crate::FieldReader<bool, bool>);
impl MIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIDE` writer - Identifier Version"]
pub struct MIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MIDVB_R {
        MIDVB_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MIDVA_R {
        MIDVA_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&mut self) -> MIDVB_W {
        MIDVB_W { w: self }
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&mut self) -> MIDVA_W {
        MIDVA_W { w: self }
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&mut self) -> MIDE_W {
        MIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Acceptance Mask Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mam2](index.html) module"]
pub struct MAM2_SPEC;
impl crate::RegisterSpec for MAM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mam2::R](R) reader structure"]
impl crate::Readable for MAM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mam2::W](W) writer structure"]
impl crate::Writable for MAM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAM2 to value 0"]
impl crate::Resettable for MAM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
