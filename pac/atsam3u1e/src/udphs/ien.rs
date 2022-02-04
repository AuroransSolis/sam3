#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DET_SUSPD` reader - Suspend Interrupt Enable"]
pub struct DET_SUSPD_R(crate::FieldReader<bool, bool>);
impl DET_SUSPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DET_SUSPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_SUSPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DET_SUSPD` writer - Suspend Interrupt Enable"]
pub struct DET_SUSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DET_SUSPD_W<'a> {
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
#[doc = "Field `MICRO_SOF` reader - Micro-SOF Interrupt Enable"]
pub struct MICRO_SOF_R(crate::FieldReader<bool, bool>);
impl MICRO_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MICRO_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MICRO_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MICRO_SOF` writer - Micro-SOF Interrupt Enable"]
pub struct MICRO_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> MICRO_SOF_W<'a> {
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
#[doc = "Field `INT_SOF` reader - SOF Interrupt Enable"]
pub struct INT_SOF_R(crate::FieldReader<bool, bool>);
impl INT_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_SOF` writer - SOF Interrupt Enable"]
pub struct INT_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SOF_W<'a> {
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
#[doc = "Field `ENDRESET` reader - End Of Reset Interrupt Enable"]
pub struct ENDRESET_R(crate::FieldReader<bool, bool>);
impl ENDRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRESET` writer - End Of Reset Interrupt Enable"]
pub struct ENDRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRESET_W<'a> {
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
#[doc = "Field `WAKE_UP` reader - Wake Up CPU Interrupt Enable"]
pub struct WAKE_UP_R(crate::FieldReader<bool, bool>);
impl WAKE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKE_UP` writer - Wake Up CPU Interrupt Enable"]
pub struct WAKE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_UP_W<'a> {
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
#[doc = "Field `ENDOFRSM` reader - End Of Resume Interrupt Enable"]
pub struct ENDOFRSM_R(crate::FieldReader<bool, bool>);
impl ENDOFRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDOFRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDOFRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDOFRSM` writer - End Of Resume Interrupt Enable"]
pub struct ENDOFRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDOFRSM_W<'a> {
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
#[doc = "Field `UPSTR_RES` reader - Upstream Resume Interrupt Enable"]
pub struct UPSTR_RES_R(crate::FieldReader<bool, bool>);
impl UPSTR_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPSTR_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPSTR_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPSTR_RES` writer - Upstream Resume Interrupt Enable"]
pub struct UPSTR_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> UPSTR_RES_W<'a> {
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
#[doc = "Field `EPT_0` reader - Endpoint 0 Interrupt Enable"]
pub struct EPT_0_R(crate::FieldReader<bool, bool>);
impl EPT_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_0` writer - Endpoint 0 Interrupt Enable"]
pub struct EPT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_0_W<'a> {
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
#[doc = "Field `EPT_1` reader - Endpoint 1 Interrupt Enable"]
pub struct EPT_1_R(crate::FieldReader<bool, bool>);
impl EPT_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_1` writer - Endpoint 1 Interrupt Enable"]
pub struct EPT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_1_W<'a> {
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
#[doc = "Field `EPT_2` reader - Endpoint 2 Interrupt Enable"]
pub struct EPT_2_R(crate::FieldReader<bool, bool>);
impl EPT_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_2` writer - Endpoint 2 Interrupt Enable"]
pub struct EPT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_2_W<'a> {
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
#[doc = "Field `EPT_3` reader - Endpoint 3 Interrupt Enable"]
pub struct EPT_3_R(crate::FieldReader<bool, bool>);
impl EPT_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_3` writer - Endpoint 3 Interrupt Enable"]
pub struct EPT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_3_W<'a> {
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
#[doc = "Field `EPT_4` reader - Endpoint 4 Interrupt Enable"]
pub struct EPT_4_R(crate::FieldReader<bool, bool>);
impl EPT_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_4` writer - Endpoint 4 Interrupt Enable"]
pub struct EPT_4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_4_W<'a> {
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
#[doc = "Field `EPT_5` reader - Endpoint 5 Interrupt Enable"]
pub struct EPT_5_R(crate::FieldReader<bool, bool>);
impl EPT_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_5` writer - Endpoint 5 Interrupt Enable"]
pub struct EPT_5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_5_W<'a> {
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
#[doc = "Field `EPT_6` reader - Endpoint 6 Interrupt Enable"]
pub struct EPT_6_R(crate::FieldReader<bool, bool>);
impl EPT_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_6` writer - Endpoint 6 Interrupt Enable"]
pub struct EPT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_6_W<'a> {
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
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt Enable"]
pub struct DMA_1_R(crate::FieldReader<bool, bool>);
impl DMA_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
pub struct DMA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_1_W<'a> {
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
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt Enable"]
pub struct DMA_2_R(crate::FieldReader<bool, bool>);
impl DMA_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
pub struct DMA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt Enable"]
pub struct DMA_3_R(crate::FieldReader<bool, bool>);
impl DMA_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
pub struct DMA_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt Enable"]
pub struct DMA_4_R(crate::FieldReader<bool, bool>);
impl DMA_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
pub struct DMA_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt Enable"]
pub struct DMA_5_R(crate::FieldReader<bool, bool>);
impl DMA_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
pub struct DMA_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_5_W<'a> {
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
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt Enable"]
pub struct DMA_6_R(crate::FieldReader<bool, bool>);
impl DMA_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
pub struct DMA_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn det_suspd(&self) -> DET_SUSPD_R {
        DET_SUSPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Micro-SOF Interrupt Enable"]
    #[inline(always)]
    pub fn micro_sof(&self) -> MICRO_SOF_R {
        MICRO_SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn int_sof(&self) -> INT_SOF_R {
        INT_SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn endreset(&self) -> ENDRESET_R {
        ENDRESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Enable"]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn endofrsm(&self) -> ENDOFRSM_R {
        ENDOFRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn upstr_res(&self) -> UPSTR_RES_R {
        UPSTR_RES_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_0(&self) -> EPT_0_R {
        EPT_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_1(&self) -> EPT_1_R {
        EPT_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_2(&self) -> EPT_2_R {
        EPT_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_3(&self) -> EPT_3_R {
        EPT_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_4(&self) -> EPT_4_R {
        EPT_4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_5(&self) -> EPT_5_R {
        EPT_5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_6(&self) -> EPT_6_R {
        EPT_6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn det_suspd(&mut self) -> DET_SUSPD_W {
        DET_SUSPD_W { w: self }
    }
    #[doc = "Bit 2 - Micro-SOF Interrupt Enable"]
    #[inline(always)]
    pub fn micro_sof(&mut self) -> MICRO_SOF_W {
        MICRO_SOF_W { w: self }
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn int_sof(&mut self) -> INT_SOF_W {
        INT_SOF_W { w: self }
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn endreset(&mut self) -> ENDRESET_W {
        ENDRESET_W { w: self }
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Enable"]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W {
        WAKE_UP_W { w: self }
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn endofrsm(&mut self) -> ENDOFRSM_W {
        ENDOFRSM_W { w: self }
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn upstr_res(&mut self) -> UPSTR_RES_W {
        UPSTR_RES_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_0(&mut self) -> EPT_0_W {
        EPT_0_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_1(&mut self) -> EPT_1_W {
        EPT_1_W { w: self }
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_2(&mut self) -> EPT_2_W {
        EPT_2_W { w: self }
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_3(&mut self) -> EPT_3_W {
        EPT_3_W { w: self }
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_4(&mut self) -> EPT_4_W {
        EPT_4_W { w: self }
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_5(&mut self) -> EPT_5_W {
        EPT_5_W { w: self }
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_6(&mut self) -> EPT_6_W {
        EPT_6_W { w: self }
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> DMA_1_W {
        DMA_1_W { w: self }
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> DMA_2_W {
        DMA_2_W { w: self }
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> DMA_3_W {
        DMA_3_W { w: self }
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> DMA_4_W {
        DMA_4_W { w: self }
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> DMA_5_W {
        DMA_5_W { w: self }
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> DMA_6_W {
        DMA_6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0x10"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
