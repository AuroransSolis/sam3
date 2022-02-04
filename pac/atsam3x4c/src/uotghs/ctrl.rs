#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDTE` reader - ID Transition Interrupt Enable"]
pub struct IDTE_R(crate::FieldReader<bool, bool>);
impl IDTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDTE` writer - ID Transition Interrupt Enable"]
pub struct IDTE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTE_W<'a> {
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
#[doc = "Field `VBUSTE` reader - VBus Transition Interrupt Enable"]
pub struct VBUSTE_R(crate::FieldReader<bool, bool>);
impl VBUSTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSTE` writer - VBus Transition Interrupt Enable"]
pub struct VBUSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSTE_W<'a> {
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
#[doc = "Field `SRPE` reader - SRP Interrupt Enable"]
pub struct SRPE_R(crate::FieldReader<bool, bool>);
impl SRPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPE` writer - SRP Interrupt Enable"]
pub struct SRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPE_W<'a> {
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
#[doc = "Field `VBERRE` reader - VBus Error Interrupt Enable"]
pub struct VBERRE_R(crate::FieldReader<bool, bool>);
impl VBERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBERRE` writer - VBus Error Interrupt Enable"]
pub struct VBERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBERRE_W<'a> {
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
#[doc = "Field `BCERRE` reader - B-Connection Error Interrupt Enable"]
pub struct BCERRE_R(crate::FieldReader<bool, bool>);
impl BCERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCERRE` writer - B-Connection Error Interrupt Enable"]
pub struct BCERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCERRE_W<'a> {
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
#[doc = "Field `ROLEEXE` reader - Role Exchange Interrupt Enable"]
pub struct ROLEEXE_R(crate::FieldReader<bool, bool>);
impl ROLEEXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROLEEXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROLEEXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROLEEXE` writer - Role Exchange Interrupt Enable"]
pub struct ROLEEXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLEEXE_W<'a> {
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
#[doc = "Field `HNPERRE` reader - HNP Error Interrupt Enable"]
pub struct HNPERRE_R(crate::FieldReader<bool, bool>);
impl HNPERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HNPERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNPERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPERRE` writer - HNP Error Interrupt Enable"]
pub struct HNPERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPERRE_W<'a> {
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
#[doc = "Field `STOE` reader - Suspend Time-Out Interrupt Enable"]
pub struct STOE_R(crate::FieldReader<bool, bool>);
impl STOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOE` writer - Suspend Time-Out Interrupt Enable"]
pub struct STOE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOE_W<'a> {
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
#[doc = "Field `VBUSHWC` reader - VBus Hardware Control"]
pub struct VBUSHWC_R(crate::FieldReader<bool, bool>);
impl VBUSHWC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSHWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSHWC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSHWC` writer - VBus Hardware Control"]
pub struct VBUSHWC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSHWC_W<'a> {
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
#[doc = "Field `SRPSEL` reader - SRP Selection"]
pub struct SRPSEL_R(crate::FieldReader<bool, bool>);
impl SRPSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPSEL` writer - SRP Selection"]
pub struct SRPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPSEL_W<'a> {
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
#[doc = "Field `SRPREQ` reader - SRP Request"]
pub struct SRPREQ_R(crate::FieldReader<bool, bool>);
impl SRPREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRPREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRPREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPREQ` writer - SRP Request"]
pub struct SRPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPREQ_W<'a> {
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
#[doc = "Field `HNPREQ` reader - HNP Request"]
pub struct HNPREQ_R(crate::FieldReader<bool, bool>);
impl HNPREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HNPREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNPREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPREQ` writer - HNP Request"]
pub struct HNPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `OTGPADE` reader - OTG Pad Enable"]
pub struct OTGPADE_R(crate::FieldReader<bool, bool>);
impl OTGPADE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTGPADE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGPADE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGPADE` writer - OTG Pad Enable"]
pub struct OTGPADE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGPADE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `VBUSPO` reader - VBus Polarity Off"]
pub struct VBUSPO_R(crate::FieldReader<bool, bool>);
impl VBUSPO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSPO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSPO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSPO` writer - VBus Polarity Off"]
pub struct VBUSPO_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSPO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub struct FRZCLK_R(crate::FieldReader<bool, bool>);
impl FRZCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub struct FRZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `USBE` reader - UOTGHS Enable"]
pub struct USBE_R(crate::FieldReader<bool, bool>);
impl USBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBE` writer - UOTGHS Enable"]
pub struct USBE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TIMVALUE` reader - Timer Value"]
pub struct TIMVALUE_R(crate::FieldReader<u8, u8>);
impl TIMVALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMVALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMVALUE` writer - Timer Value"]
pub struct TIMVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TIMPAGE` reader - Timer Page"]
pub struct TIMPAGE_R(crate::FieldReader<u8, u8>);
impl TIMPAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMPAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMPAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMPAGE` writer - Timer Page"]
pub struct TIMPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `UNLOCK` reader - Timer Access Unlock"]
pub struct UNLOCK_R(crate::FieldReader<bool, bool>);
impl UNLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLOCK` writer - Timer Access Unlock"]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "UOTGID Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIDE_A {
    #[doc = "0: The USB mode (device/host) is selected from the UIMOD bit."]
    UIMOD = 0,
    #[doc = "1: The USB mode (device/host) is selected from the UOTGID input pin."]
    UOTGID = 1,
}
impl From<UIDE_A> for bool {
    #[inline(always)]
    fn from(variant: UIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIDE` reader - UOTGID Pin Enable"]
pub struct UIDE_R(crate::FieldReader<bool, UIDE_A>);
impl UIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UIDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIDE_A {
        match self.bits {
            false => UIDE_A::UIMOD,
            true => UIDE_A::UOTGID,
        }
    }
    #[doc = "Checks if the value of the field is `UIMOD`"]
    #[inline(always)]
    pub fn is_uimod(&self) -> bool {
        **self == UIDE_A::UIMOD
    }
    #[doc = "Checks if the value of the field is `UOTGID`"]
    #[inline(always)]
    pub fn is_uotgid(&self) -> bool {
        **self == UIDE_A::UOTGID
    }
}
impl core::ops::Deref for UIDE_R {
    type Target = crate::FieldReader<bool, UIDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIDE` writer - UOTGID Pin Enable"]
pub struct UIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    #[inline(always)]
    pub fn uimod(self) -> &'a mut W {
        self.variant(UIDE_A::UIMOD)
    }
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    #[inline(always)]
    pub fn uotgid(self) -> &'a mut W {
        self.variant(UIDE_A::UOTGID)
    }
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
#[doc = "UOTGHS Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIMOD_A {
    #[doc = "0: The module is in USB host mode."]
    HOST = 0,
    #[doc = "1: The module is in USB device mode."]
    DEVICE = 1,
}
impl From<UIMOD_A> for bool {
    #[inline(always)]
    fn from(variant: UIMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIMOD` reader - UOTGHS Mode"]
pub struct UIMOD_R(crate::FieldReader<bool, UIMOD_A>);
impl UIMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UIMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIMOD_A {
        match self.bits {
            false => UIMOD_A::HOST,
            true => UIMOD_A::DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        **self == UIMOD_A::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        **self == UIMOD_A::DEVICE
    }
}
impl core::ops::Deref for UIMOD_R {
    type Target = crate::FieldReader<bool, UIMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIMOD` writer - UOTGHS Mode"]
pub struct UIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> UIMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The module is in USB host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMOD_A::HOST)
    }
    #[doc = "The module is in USB device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMOD_A::DEVICE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    pub fn idte(&self) -> IDTE_R {
        IDTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    pub fn vbuste(&self) -> VBUSTE_R {
        VBUSTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    pub fn srpe(&self) -> SRPE_R {
        SRPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vberre(&self) -> VBERRE_R {
        VBERRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn bcerre(&self) -> BCERRE_R {
        BCERRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    pub fn roleexe(&self) -> ROLEEXE_R {
        ROLEEXE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    pub fn hnperre(&self) -> HNPERRE_R {
        HNPERRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    pub fn stoe(&self) -> STOE_R {
        STOE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VBUSHWC_R {
        VBUSHWC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    pub fn srpsel(&self) -> SRPSEL_R {
        SRPSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    pub fn otgpade(&self) -> OTGPADE_R {
        OTGPADE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    pub fn vbuspo(&self) -> VBUSPO_R {
        VBUSPO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    pub fn timvalue(&self) -> TIMVALUE_R {
        TIMVALUE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    pub fn timpage(&self) -> TIMPAGE_R {
        TIMPAGE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    pub fn uide(&self) -> UIDE_R {
        UIDE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    pub fn idte(&mut self) -> IDTE_W {
        IDTE_W { w: self }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    pub fn vbuste(&mut self) -> VBUSTE_W {
        VBUSTE_W { w: self }
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    pub fn srpe(&mut self) -> SRPE_W {
        SRPE_W { w: self }
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vberre(&mut self) -> VBERRE_W {
        VBERRE_W { w: self }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn bcerre(&mut self) -> BCERRE_W {
        BCERRE_W { w: self }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    pub fn roleexe(&mut self) -> ROLEEXE_W {
        ROLEEXE_W { w: self }
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    pub fn hnperre(&mut self) -> HNPERRE_W {
        HNPERRE_W { w: self }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    pub fn stoe(&mut self) -> STOE_W {
        STOE_W { w: self }
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&mut self) -> VBUSHWC_W {
        VBUSHWC_W { w: self }
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    pub fn srpsel(&mut self) -> SRPSEL_W {
        SRPSEL_W { w: self }
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    pub fn srpreq(&mut self) -> SRPREQ_W {
        SRPREQ_W { w: self }
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HNPREQ_W {
        HNPREQ_W { w: self }
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    pub fn otgpade(&mut self) -> OTGPADE_W {
        OTGPADE_W { w: self }
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    pub fn vbuspo(&mut self) -> VBUSPO_W {
        VBUSPO_W { w: self }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> FRZCLK_W {
        FRZCLK_W { w: self }
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> USBE_W {
        USBE_W { w: self }
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    pub fn timvalue(&mut self) -> TIMVALUE_W {
        TIMVALUE_W { w: self }
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    pub fn timpage(&mut self) -> TIMPAGE_W {
        TIMPAGE_W { w: self }
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    pub fn uide(&mut self) -> UIDE_W {
        UIDE_W { w: self }
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> UIMOD_W {
        UIMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_4000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_4000
    }
}
