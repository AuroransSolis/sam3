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
    #[doc = "0: Master clock"]
    MCK = 0,
    #[doc = "1: Master clock/2"]
    MCK_DIV_2 = 1,
    #[doc = "2: Master clock/4"]
    MCK_DIV_4 = 2,
    #[doc = "3: Master clock/8"]
    MCK_DIV_8 = 3,
    #[doc = "4: Master clock/16"]
    MCK_DIV_16 = 4,
    #[doc = "5: Master clock/32"]
    MCK_DIV_32 = 5,
    #[doc = "6: Master clock/64"]
    MCK_DIV_64 = 6,
    #[doc = "7: Master clock/128"]
    MCK_DIV_128 = 7,
    #[doc = "8: Master clock/256"]
    MCK_DIV_256 = 8,
    #[doc = "9: Master clock/512"]
    MCK_DIV_512 = 9,
    #[doc = "10: Master clock/1024"]
    MCK_DIV_1024 = 10,
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
            1 => Some(CPRE_A::MCK_DIV_2),
            2 => Some(CPRE_A::MCK_DIV_4),
            3 => Some(CPRE_A::MCK_DIV_8),
            4 => Some(CPRE_A::MCK_DIV_16),
            5 => Some(CPRE_A::MCK_DIV_32),
            6 => Some(CPRE_A::MCK_DIV_64),
            7 => Some(CPRE_A::MCK_DIV_128),
            8 => Some(CPRE_A::MCK_DIV_256),
            9 => Some(CPRE_A::MCK_DIV_512),
            10 => Some(CPRE_A::MCK_DIV_1024),
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
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        **self == CPRE_A::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        **self == CPRE_A::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        **self == CPRE_A::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        **self == CPRE_A::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        **self == CPRE_A::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        **self == CPRE_A::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        **self == CPRE_A::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        **self == CPRE_A::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        **self == CPRE_A::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        **self == CPRE_A::MCK_DIV_1024
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
    #[doc = "Master clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPRE_A::MCK)
    }
    #[doc = "Master clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_2)
    }
    #[doc = "Master clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_4)
    }
    #[doc = "Master clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_8)
    }
    #[doc = "Master clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_16)
    }
    #[doc = "Master clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_32)
    }
    #[doc = "Master clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_64)
    }
    #[doc = "Master clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_128)
    }
    #[doc = "Master clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_256)
    }
    #[doc = "Master clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_512)
    }
    #[doc = "Master clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_1024)
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
#[doc = "Field `CES` reader - Counter Event Selection"]
pub struct CES_R(crate::FieldReader<bool, bool>);
impl CES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CES` writer - Counter Event Selection"]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
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
#[doc = "Field `DTE` reader - Dead-Time Generator Enable"]
pub struct DTE_R(crate::FieldReader<bool, bool>);
impl DTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTE` writer - Dead-Time Generator Enable"]
pub struct DTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DTHI` reader - Dead-Time PWMHx Output Inverted"]
pub struct DTHI_R(crate::FieldReader<bool, bool>);
impl DTHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTHI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTHI` writer - Dead-Time PWMHx Output Inverted"]
pub struct DTHI_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DTLI` reader - Dead-Time PWMLx Output Inverted"]
pub struct DTLI_R(crate::FieldReader<bool, bool>);
impl DTLI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTLI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTLI` writer - Dead-Time PWMLx Output Inverted"]
pub struct DTLI_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
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
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DTHI_R {
        DTHI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DTLI_R {
        DTLI_R::new(((self.bits >> 18) & 0x01) != 0)
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
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DTE_W {
        DTE_W { w: self }
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&mut self) -> DTHI_W {
        DTHI_W { w: self }
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&mut self) -> DTLI_W {
        DTLI_W { w: self }
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
