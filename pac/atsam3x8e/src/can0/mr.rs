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
#[doc = "Field `CANEN` reader - CAN Controller Enable"]
pub struct CANEN_R(crate::FieldReader<bool, bool>);
impl CANEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CANEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CANEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CANEN` writer - CAN Controller Enable"]
pub struct CANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CANEN_W<'a> {
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
#[doc = "Field `LPM` reader - Disable/Enable Low Power Mode"]
pub struct LPM_R(crate::FieldReader<bool, bool>);
impl LPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM` writer - Disable/Enable Low Power Mode"]
pub struct LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_W<'a> {
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
#[doc = "Field `ABM` reader - Disable/Enable Autobaud/Listen mode"]
pub struct ABM_R(crate::FieldReader<bool, bool>);
impl ABM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABM` writer - Disable/Enable Autobaud/Listen mode"]
pub struct ABM_W<'a> {
    w: &'a mut W,
}
impl<'a> ABM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OVL` reader - Disable/Enable Overload Frame"]
pub struct OVL_R(crate::FieldReader<bool, bool>);
impl OVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVL` writer - Disable/Enable Overload Frame"]
pub struct OVL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TEOF` reader - Timestamp messages at each end of Frame"]
pub struct TEOF_R(crate::FieldReader<bool, bool>);
impl TEOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEOF` writer - Timestamp messages at each end of Frame"]
pub struct TEOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TTM` reader - Disable/Enable Time Triggered Mode"]
pub struct TTM_R(crate::FieldReader<bool, bool>);
impl TTM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTM` writer - Disable/Enable Time Triggered Mode"]
pub struct TTM_W<'a> {
    w: &'a mut W,
}
impl<'a> TTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TIMFRZ` reader - Enable Timer Freeze"]
pub struct TIMFRZ_R(crate::FieldReader<bool, bool>);
impl TIMFRZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMFRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMFRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMFRZ` writer - Enable Timer Freeze"]
pub struct TIMFRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMFRZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DRPT` reader - Disable Repeat"]
pub struct DRPT_R(crate::FieldReader<bool, bool>);
impl DRPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRPT` writer - Disable Repeat"]
pub struct DRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Reception Synchronization Stage (not readable)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXSYNC_A {
    #[doc = "0: Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DOUBLE_PP = 0,
    #[doc = "1: Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DOUBLE_PN = 1,
    #[doc = "2: Rx Signal with Single Synchro Stage (Positive Edge)"]
    SINGLE_P = 2,
    #[doc = "3: Rx Signal with No Synchro Stage"]
    NONE = 3,
}
impl From<RXSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXSYNC` reader - Reception Synchronization Stage (not readable)"]
pub struct RXSYNC_R(crate::FieldReader<u8, RXSYNC_A>);
impl RXSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXSYNC_A> {
        match self.bits {
            0 => Some(RXSYNC_A::DOUBLE_PP),
            1 => Some(RXSYNC_A::DOUBLE_PN),
            2 => Some(RXSYNC_A::SINGLE_P),
            3 => Some(RXSYNC_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DOUBLE_PP`"]
    #[inline(always)]
    pub fn is_double_pp(&self) -> bool {
        **self == RXSYNC_A::DOUBLE_PP
    }
    #[doc = "Checks if the value of the field is `DOUBLE_PN`"]
    #[inline(always)]
    pub fn is_double_pn(&self) -> bool {
        **self == RXSYNC_A::DOUBLE_PN
    }
    #[doc = "Checks if the value of the field is `SINGLE_P`"]
    #[inline(always)]
    pub fn is_single_p(&self) -> bool {
        **self == RXSYNC_A::SINGLE_P
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == RXSYNC_A::NONE
    }
}
impl core::ops::Deref for RXSYNC_R {
    type Target = crate::FieldReader<u8, RXSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSYNC` writer - Reception Synchronization Stage (not readable)"]
pub struct RXSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSYNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn double_pp(self) -> &'a mut W {
        self.variant(RXSYNC_A::DOUBLE_PP)
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn double_pn(self) -> &'a mut W {
        self.variant(RXSYNC_A::DOUBLE_PN)
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn single_p(self) -> &'a mut W {
        self.variant(RXSYNC_A::SINGLE_P)
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RXSYNC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&self) -> ABM_R {
        ABM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&self) -> OVL_R {
        OVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&self) -> TEOF_R {
        TEOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&self) -> TTM_R {
        TTM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&self) -> TIMFRZ_R {
        TIMFRZ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&self) -> DRPT_R {
        DRPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&self) -> RXSYNC_R {
        RXSYNC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&mut self) -> CANEN_W {
        CANEN_W { w: self }
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W {
        LPM_W { w: self }
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&mut self) -> ABM_W {
        ABM_W { w: self }
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&mut self) -> OVL_W {
        OVL_W { w: self }
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&mut self) -> TEOF_W {
        TEOF_W { w: self }
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&mut self) -> TTM_W {
        TTM_W { w: self }
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&mut self) -> TIMFRZ_W {
        TIMFRZ_W { w: self }
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&mut self) -> DRPT_W {
        DRPT_W { w: self }
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&mut self) -> RXSYNC_W {
        RXSYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
