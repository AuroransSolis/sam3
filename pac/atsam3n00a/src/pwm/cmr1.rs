#[doc = "Register `CMR1` reader"]
pub struct R(crate::R<CMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR1` writer"]
pub struct W(crate::W<CMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR1_SPEC>;
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
impl From<crate::W<CMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPRE_A {
    #[doc = "0: Master Clock"]
    MCK = 0,
    #[doc = "1: Master Clock divided by 2"]
    MCKDIV2 = 1,
    #[doc = "2: Master Clock divided by 4"]
    MCKDIV4 = 2,
    #[doc = "3: Master Clock divided by 8"]
    MCKDIV8 = 3,
    #[doc = "4: Master Clock divided by 16"]
    MCKDIV16 = 4,
    #[doc = "5: Master Clock divided by 32"]
    MCKDIV32 = 5,
    #[doc = "6: Master Clock divided by 64"]
    MCKDIV64 = 6,
    #[doc = "7: Master Clock divided by 128"]
    MCKDIV128 = 7,
    #[doc = "8: Master Clock divided by 256"]
    MCKDIV256 = 8,
    #[doc = "9: Master Clock divided by 512"]
    MCKDIV512 = 9,
    #[doc = "10: Master Clock divided by 1024"]
    MCKDIV1024 = 10,
    #[doc = "11: Clock A"]
    CLKA = 11,
    #[doc = "12: Clock B"]
    CLKB = 12,
}
impl From<CPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub struct CPRE_R(crate::FieldReader<u8, CPRE_A>);
impl CPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPRE_A> {
        match self.bits {
            0 => Some(CPRE_A::MCK),
            1 => Some(CPRE_A::MCKDIV2),
            2 => Some(CPRE_A::MCKDIV4),
            3 => Some(CPRE_A::MCKDIV8),
            4 => Some(CPRE_A::MCKDIV16),
            5 => Some(CPRE_A::MCKDIV32),
            6 => Some(CPRE_A::MCKDIV64),
            7 => Some(CPRE_A::MCKDIV128),
            8 => Some(CPRE_A::MCKDIV256),
            9 => Some(CPRE_A::MCKDIV512),
            10 => Some(CPRE_A::MCKDIV1024),
            11 => Some(CPRE_A::CLKA),
            12 => Some(CPRE_A::CLKB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == CPRE_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        **self == CPRE_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        **self == CPRE_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV8`"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        **self == CPRE_A::MCKDIV8
    }
    #[doc = "Checks if the value of the field is `MCKDIV16`"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        **self == CPRE_A::MCKDIV16
    }
    #[doc = "Checks if the value of the field is `MCKDIV32`"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        **self == CPRE_A::MCKDIV32
    }
    #[doc = "Checks if the value of the field is `MCKDIV64`"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        **self == CPRE_A::MCKDIV64
    }
    #[doc = "Checks if the value of the field is `MCKDIV128`"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        **self == CPRE_A::MCKDIV128
    }
    #[doc = "Checks if the value of the field is `MCKDIV256`"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        **self == CPRE_A::MCKDIV256
    }
    #[doc = "Checks if the value of the field is `MCKDIV512`"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        **self == CPRE_A::MCKDIV512
    }
    #[doc = "Checks if the value of the field is `MCKDIV1024`"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        **self == CPRE_A::MCKDIV1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        **self == CPRE_A::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        **self == CPRE_A::CLKB
    }
}
impl core::ops::Deref for CPRE_R {
    type Target = crate::FieldReader<u8, CPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub struct CPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPRE_A::MCK)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut W {
        self.variant(CPRE_A::MCKDIV1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPRE_A::CLKA)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPRE_A::CLKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub struct CALG_R(crate::FieldReader<bool, bool>);
impl CALG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALG` writer - Channel Alignment"]
pub struct CALG_W<'a> {
    w: &'a mut W,
}
impl<'a> CALG_W<'a> {
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
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, bool>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CPD` reader - Channel Update Period"]
pub struct CPD_R(crate::FieldReader<bool, bool>);
impl CPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPD` writer - Channel Update Period"]
pub struct CPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel Update Period"]
    #[inline(always)]
    pub fn cpd(&self) -> CPD_R {
        CPD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&mut self) -> CPRE_W {
        CPRE_W { w: self }
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&mut self) -> CALG_W {
        CALG_W { w: self }
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 10 - Channel Update Period"]
    #[inline(always)]
    pub fn cpd(&mut self) -> CPD_W {
        CPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr1](index.html) module"]
pub struct CMR1_SPEC;
impl crate::RegisterSpec for CMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr1::R](R) reader structure"]
impl crate::Readable for CMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr1::W](W) writer structure"]
impl crate::Writable for CMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMR1 to value 0"]
impl crate::Resettable for CMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
