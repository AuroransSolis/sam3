#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - CRC Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - CRC Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `COMPARE` reader - CRC Compare"]
pub struct COMPARE_R(crate::FieldReader<bool, bool>);
impl COMPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE` writer - CRC Compare"]
pub struct COMPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_W<'a> {
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
#[doc = "Primitive Polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: Polynom 0x04C11DB7"]
    CCITT8023 = 0,
    #[doc = "1: Polynom 0x1EDC6F41"]
    CASTAGNOLI = 1,
    #[doc = "2: Polynom 0x1021"]
    CCITT16 = 2,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PTYPE` reader - Primitive Polynomial"]
pub struct PTYPE_R(crate::FieldReader<u8, PTYPE_A>);
impl PTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTYPE_A> {
        match self.bits {
            0 => Some(PTYPE_A::CCITT8023),
            1 => Some(PTYPE_A::CASTAGNOLI),
            2 => Some(PTYPE_A::CCITT16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CCITT8023`"]
    #[inline(always)]
    pub fn is_ccitt8023(&self) -> bool {
        **self == PTYPE_A::CCITT8023
    }
    #[doc = "Checks if the value of the field is `CASTAGNOLI`"]
    #[inline(always)]
    pub fn is_castagnoli(&self) -> bool {
        **self == PTYPE_A::CASTAGNOLI
    }
    #[doc = "Checks if the value of the field is `CCITT16`"]
    #[inline(always)]
    pub fn is_ccitt16(&self) -> bool {
        **self == PTYPE_A::CCITT16
    }
}
impl core::ops::Deref for PTYPE_R {
    type Target = crate::FieldReader<u8, PTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTYPE` writer - Primitive Polynomial"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn ccitt8023(self) -> &'a mut W {
        self.variant(PTYPE_A::CCITT8023)
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn castagnoli(self) -> &'a mut W {
        self.variant(PTYPE_A::CASTAGNOLI)
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn ccitt16(self) -> &'a mut W {
        self.variant(PTYPE_A::CCITT16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DIVIDER` reader - Request Divider"]
pub struct DIVIDER_R(crate::FieldReader<u8, u8>);
impl DIVIDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVIDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVIDER` writer - Request Divider"]
pub struct DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&mut self) -> COMPARE_W {
        COMPARE_W { w: self }
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W {
        DIVIDER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCU Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
