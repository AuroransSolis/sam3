#[doc = "Register `FADDR` reader"]
pub struct R(crate::R<FADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FADDR` writer"]
pub struct W(crate::W<FADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FADDR_SPEC>;
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
impl From<crate::W<FADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADD` reader - Function Address Value"]
pub struct FADD_R(crate::FieldReader<u8, u8>);
impl FADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADD` writer - Function Address Value"]
pub struct FADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `FEN` reader - Function Enable"]
pub struct FEN_R(crate::FieldReader<bool, bool>);
impl FEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEN` writer - Function Enable"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    pub fn fadd(&mut self) -> FADD_W {
        FADD_W { w: self }
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](index.html) module"]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faddr::R](R) reader structure"]
impl crate::Readable for FADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faddr::W](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FADDR to value 0x0100"]
impl crate::Resettable for FADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
