#[doc = "Register `MODE0` reader"]
pub struct R(crate::R<MODE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE0` writer"]
pub struct W(crate::W<MODE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE0_SPEC>;
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
impl From<crate::W<MODE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_MODE` reader - "]
pub struct READ_MODE_R(crate::FieldReader<bool, bool>);
impl READ_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_MODE` writer - "]
pub struct READ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_MODE_W<'a> {
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
#[doc = "Field `WRITE_MODE` reader - "]
pub struct WRITE_MODE_R(crate::FieldReader<bool, bool>);
impl WRITE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_MODE` writer - "]
pub struct WRITE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_MODE_W<'a> {
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
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Frozen Mode"]
    FROZEN = 2,
    #[doc = "3: Ready Mode"]
    READY = 3,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub struct EXNW_MODE_R(crate::FieldReader<u8, EXNW_MODE_A>);
impl EXNW_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXNW_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXNW_MODE_A> {
        match self.bits {
            0 => Some(EXNW_MODE_A::DISABLED),
            2 => Some(EXNW_MODE_A::FROZEN),
            3 => Some(EXNW_MODE_A::READY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXNW_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == EXNW_MODE_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == EXNW_MODE_A::READY
    }
}
impl core::ops::Deref for EXNW_MODE_R {
    type Target = crate::FieldReader<u8, EXNW_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub struct EXNW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXNW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXNW_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::READY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub struct TDF_CYCLES_R(crate::FieldReader<u8, u8>);
impl TDF_CYCLES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDF_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDF_CYCLES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub struct TDF_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> TDF_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub struct TDF_MODE_R(crate::FieldReader<bool, bool>);
impl TDF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub struct TDF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub struct PMEN_R(crate::FieldReader<bool, bool>);
impl PMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub struct PMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Page Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: 4-byte page"]
    _4_BYTE = 0,
    #[doc = "1: 8-byte page"]
    _8_BYTE = 1,
    #[doc = "2: 16-byte page"]
    _16_BYTE = 2,
    #[doc = "3: 32-byte page"]
    _32_BYTE = 3,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PS` reader - Page Size"]
pub struct PS_R(crate::FieldReader<u8, PS_A>);
impl PS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_4_BYTE,
            1 => PS_A::_8_BYTE,
            2 => PS_A::_16_BYTE,
            3 => PS_A::_32_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        **self == PS_A::_4_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == PS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == PS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == PS_A::_32_BYTE
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u8, PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Page Size"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(PS_A::_4_BYTE)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PS_A::_8_BYTE)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PS_A::_16_BYTE)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PS_A::_32_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> READ_MODE_W {
        READ_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WRITE_MODE_W {
        WRITE_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W {
        EXNW_MODE_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W {
        TDF_CYCLES_W { w: self }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W {
        TDF_MODE_W { w: self }
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&mut self) -> PMEN_W {
        PMEN_W { w: self }
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Mode Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode0](index.html) module"]
pub struct MODE0_SPEC;
impl crate::RegisterSpec for MODE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode0::R](R) reader structure"]
impl crate::Readable for MODE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode0::W](W) writer structure"]
impl crate::Writable for MODE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE0 to value 0x1000_0003"]
impl crate::Resettable for MODE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0003
    }
}
