#[doc = "Register `COR` reader"]
pub struct R(crate::R<COR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COR` writer"]
pub struct W(crate::W<COR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COR_SPEC>;
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
impl From<crate::W<COR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFF0` reader - Offset for channel 0"]
pub struct OFF0_R(crate::FieldReader<bool, bool>);
impl OFF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF0` writer - Offset for channel 0"]
pub struct OFF0_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF0_W<'a> {
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
#[doc = "Field `OFF1` reader - Offset for channel 1"]
pub struct OFF1_R(crate::FieldReader<bool, bool>);
impl OFF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF1` writer - Offset for channel 1"]
pub struct OFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF1_W<'a> {
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
#[doc = "Field `OFF2` reader - Offset for channel 2"]
pub struct OFF2_R(crate::FieldReader<bool, bool>);
impl OFF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF2` writer - Offset for channel 2"]
pub struct OFF2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF2_W<'a> {
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
#[doc = "Field `OFF3` reader - Offset for channel 3"]
pub struct OFF3_R(crate::FieldReader<bool, bool>);
impl OFF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF3` writer - Offset for channel 3"]
pub struct OFF3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF3_W<'a> {
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
#[doc = "Field `OFF4` reader - Offset for channel 4"]
pub struct OFF4_R(crate::FieldReader<bool, bool>);
impl OFF4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF4` writer - Offset for channel 4"]
pub struct OFF4_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF4_W<'a> {
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
#[doc = "Field `OFF5` reader - Offset for channel 5"]
pub struct OFF5_R(crate::FieldReader<bool, bool>);
impl OFF5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF5` writer - Offset for channel 5"]
pub struct OFF5_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF5_W<'a> {
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
#[doc = "Field `OFF6` reader - Offset for channel 6"]
pub struct OFF6_R(crate::FieldReader<bool, bool>);
impl OFF6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF6` writer - Offset for channel 6"]
pub struct OFF6_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF6_W<'a> {
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
#[doc = "Field `OFF7` reader - Offset for channel 7"]
pub struct OFF7_R(crate::FieldReader<bool, bool>);
impl OFF7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF7` writer - Offset for channel 7"]
pub struct OFF7_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF7_W<'a> {
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
#[doc = "Field `OFF8` reader - Offset for channel 8"]
pub struct OFF8_R(crate::FieldReader<bool, bool>);
impl OFF8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF8` writer - Offset for channel 8"]
pub struct OFF8_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF8_W<'a> {
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
#[doc = "Field `OFF9` reader - Offset for channel 9"]
pub struct OFF9_R(crate::FieldReader<bool, bool>);
impl OFF9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF9` writer - Offset for channel 9"]
pub struct OFF9_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF9_W<'a> {
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
#[doc = "Field `OFF10` reader - Offset for channel 10"]
pub struct OFF10_R(crate::FieldReader<bool, bool>);
impl OFF10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF10` writer - Offset for channel 10"]
pub struct OFF10_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF10_W<'a> {
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
#[doc = "Field `OFF11` reader - Offset for channel 11"]
pub struct OFF11_R(crate::FieldReader<bool, bool>);
impl OFF11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF11` writer - Offset for channel 11"]
pub struct OFF11_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF11_W<'a> {
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
#[doc = "Field `OFF12` reader - Offset for channel 12"]
pub struct OFF12_R(crate::FieldReader<bool, bool>);
impl OFF12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF12` writer - Offset for channel 12"]
pub struct OFF12_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF12_W<'a> {
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
#[doc = "Field `OFF13` reader - Offset for channel 13"]
pub struct OFF13_R(crate::FieldReader<bool, bool>);
impl OFF13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF13` writer - Offset for channel 13"]
pub struct OFF13_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF13_W<'a> {
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
#[doc = "Field `OFF14` reader - Offset for channel 14"]
pub struct OFF14_R(crate::FieldReader<bool, bool>);
impl OFF14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF14` writer - Offset for channel 14"]
pub struct OFF14_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF14_W<'a> {
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
#[doc = "Field `OFF15` reader - Offset for channel 15"]
pub struct OFF15_R(crate::FieldReader<bool, bool>);
impl OFF15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF15` writer - Offset for channel 15"]
pub struct OFF15_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF15_W<'a> {
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
#[doc = "Field `DIFF0` reader - Differential inputs for channel 0"]
pub struct DIFF0_R(crate::FieldReader<bool, bool>);
impl DIFF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF0` writer - Differential inputs for channel 0"]
pub struct DIFF0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF0_W<'a> {
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
#[doc = "Field `DIFF1` reader - Differential inputs for channel 1"]
pub struct DIFF1_R(crate::FieldReader<bool, bool>);
impl DIFF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF1` writer - Differential inputs for channel 1"]
pub struct DIFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF1_W<'a> {
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
#[doc = "Field `DIFF2` reader - Differential inputs for channel 2"]
pub struct DIFF2_R(crate::FieldReader<bool, bool>);
impl DIFF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF2` writer - Differential inputs for channel 2"]
pub struct DIFF2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF2_W<'a> {
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
#[doc = "Field `DIFF3` reader - Differential inputs for channel 3"]
pub struct DIFF3_R(crate::FieldReader<bool, bool>);
impl DIFF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF3` writer - Differential inputs for channel 3"]
pub struct DIFF3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DIFF4` reader - Differential inputs for channel 4"]
pub struct DIFF4_R(crate::FieldReader<bool, bool>);
impl DIFF4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF4` writer - Differential inputs for channel 4"]
pub struct DIFF4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF4_W<'a> {
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
#[doc = "Field `DIFF5` reader - Differential inputs for channel 5"]
pub struct DIFF5_R(crate::FieldReader<bool, bool>);
impl DIFF5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF5` writer - Differential inputs for channel 5"]
pub struct DIFF5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `DIFF6` reader - Differential inputs for channel 6"]
pub struct DIFF6_R(crate::FieldReader<bool, bool>);
impl DIFF6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF6` writer - Differential inputs for channel 6"]
pub struct DIFF6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF6_W<'a> {
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
#[doc = "Field `DIFF7` reader - Differential inputs for channel 7"]
pub struct DIFF7_R(crate::FieldReader<bool, bool>);
impl DIFF7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF7` writer - Differential inputs for channel 7"]
pub struct DIFF7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `DIFF8` reader - Differential inputs for channel 8"]
pub struct DIFF8_R(crate::FieldReader<bool, bool>);
impl DIFF8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF8` writer - Differential inputs for channel 8"]
pub struct DIFF8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF8_W<'a> {
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
#[doc = "Field `DIFF9` reader - Differential inputs for channel 9"]
pub struct DIFF9_R(crate::FieldReader<bool, bool>);
impl DIFF9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF9` writer - Differential inputs for channel 9"]
pub struct DIFF9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF9_W<'a> {
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
#[doc = "Field `DIFF10` reader - Differential inputs for channel 10"]
pub struct DIFF10_R(crate::FieldReader<bool, bool>);
impl DIFF10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF10` writer - Differential inputs for channel 10"]
pub struct DIFF10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF10_W<'a> {
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
#[doc = "Field `DIFF11` reader - Differential inputs for channel 11"]
pub struct DIFF11_R(crate::FieldReader<bool, bool>);
impl DIFF11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF11` writer - Differential inputs for channel 11"]
pub struct DIFF11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF11_W<'a> {
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
#[doc = "Field `DIFF12` reader - Differential inputs for channel 12"]
pub struct DIFF12_R(crate::FieldReader<bool, bool>);
impl DIFF12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF12` writer - Differential inputs for channel 12"]
pub struct DIFF12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF12_W<'a> {
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
#[doc = "Field `DIFF13` reader - Differential inputs for channel 13"]
pub struct DIFF13_R(crate::FieldReader<bool, bool>);
impl DIFF13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF13` writer - Differential inputs for channel 13"]
pub struct DIFF13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF13_W<'a> {
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
#[doc = "Field `DIFF14` reader - Differential inputs for channel 14"]
pub struct DIFF14_R(crate::FieldReader<bool, bool>);
impl DIFF14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF14` writer - Differential inputs for channel 14"]
pub struct DIFF14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF14_W<'a> {
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
#[doc = "Field `DIFF15` reader - Differential inputs for channel 15"]
pub struct DIFF15_R(crate::FieldReader<bool, bool>);
impl DIFF15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF15` writer - Differential inputs for channel 15"]
pub struct DIFF15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Offset for channel 0"]
    #[inline(always)]
    pub fn off0(&self) -> OFF0_R {
        OFF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Offset for channel 1"]
    #[inline(always)]
    pub fn off1(&self) -> OFF1_R {
        OFF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Offset for channel 2"]
    #[inline(always)]
    pub fn off2(&self) -> OFF2_R {
        OFF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Offset for channel 3"]
    #[inline(always)]
    pub fn off3(&self) -> OFF3_R {
        OFF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Offset for channel 4"]
    #[inline(always)]
    pub fn off4(&self) -> OFF4_R {
        OFF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Offset for channel 5"]
    #[inline(always)]
    pub fn off5(&self) -> OFF5_R {
        OFF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Offset for channel 6"]
    #[inline(always)]
    pub fn off6(&self) -> OFF6_R {
        OFF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Offset for channel 7"]
    #[inline(always)]
    pub fn off7(&self) -> OFF7_R {
        OFF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Offset for channel 8"]
    #[inline(always)]
    pub fn off8(&self) -> OFF8_R {
        OFF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Offset for channel 9"]
    #[inline(always)]
    pub fn off9(&self) -> OFF9_R {
        OFF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Offset for channel 10"]
    #[inline(always)]
    pub fn off10(&self) -> OFF10_R {
        OFF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Offset for channel 11"]
    #[inline(always)]
    pub fn off11(&self) -> OFF11_R {
        OFF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Offset for channel 12"]
    #[inline(always)]
    pub fn off12(&self) -> OFF12_R {
        OFF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Offset for channel 13"]
    #[inline(always)]
    pub fn off13(&self) -> OFF13_R {
        OFF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Offset for channel 14"]
    #[inline(always)]
    pub fn off14(&self) -> OFF14_R {
        OFF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Offset for channel 15"]
    #[inline(always)]
    pub fn off15(&self) -> OFF15_R {
        OFF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&self) -> DIFF0_R {
        DIFF0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&self) -> DIFF1_R {
        DIFF1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&self) -> DIFF2_R {
        DIFF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&self) -> DIFF3_R {
        DIFF3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&self) -> DIFF4_R {
        DIFF4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&self) -> DIFF5_R {
        DIFF5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&self) -> DIFF6_R {
        DIFF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&self) -> DIFF7_R {
        DIFF7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&self) -> DIFF8_R {
        DIFF8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&self) -> DIFF9_R {
        DIFF9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&self) -> DIFF10_R {
        DIFF10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&self) -> DIFF11_R {
        DIFF11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Differential inputs for channel 12"]
    #[inline(always)]
    pub fn diff12(&self) -> DIFF12_R {
        DIFF12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Differential inputs for channel 13"]
    #[inline(always)]
    pub fn diff13(&self) -> DIFF13_R {
        DIFF13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Differential inputs for channel 14"]
    #[inline(always)]
    pub fn diff14(&self) -> DIFF14_R {
        DIFF14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Differential inputs for channel 15"]
    #[inline(always)]
    pub fn diff15(&self) -> DIFF15_R {
        DIFF15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Offset for channel 0"]
    #[inline(always)]
    pub fn off0(&mut self) -> OFF0_W {
        OFF0_W { w: self }
    }
    #[doc = "Bit 1 - Offset for channel 1"]
    #[inline(always)]
    pub fn off1(&mut self) -> OFF1_W {
        OFF1_W { w: self }
    }
    #[doc = "Bit 2 - Offset for channel 2"]
    #[inline(always)]
    pub fn off2(&mut self) -> OFF2_W {
        OFF2_W { w: self }
    }
    #[doc = "Bit 3 - Offset for channel 3"]
    #[inline(always)]
    pub fn off3(&mut self) -> OFF3_W {
        OFF3_W { w: self }
    }
    #[doc = "Bit 4 - Offset for channel 4"]
    #[inline(always)]
    pub fn off4(&mut self) -> OFF4_W {
        OFF4_W { w: self }
    }
    #[doc = "Bit 5 - Offset for channel 5"]
    #[inline(always)]
    pub fn off5(&mut self) -> OFF5_W {
        OFF5_W { w: self }
    }
    #[doc = "Bit 6 - Offset for channel 6"]
    #[inline(always)]
    pub fn off6(&mut self) -> OFF6_W {
        OFF6_W { w: self }
    }
    #[doc = "Bit 7 - Offset for channel 7"]
    #[inline(always)]
    pub fn off7(&mut self) -> OFF7_W {
        OFF7_W { w: self }
    }
    #[doc = "Bit 8 - Offset for channel 8"]
    #[inline(always)]
    pub fn off8(&mut self) -> OFF8_W {
        OFF8_W { w: self }
    }
    #[doc = "Bit 9 - Offset for channel 9"]
    #[inline(always)]
    pub fn off9(&mut self) -> OFF9_W {
        OFF9_W { w: self }
    }
    #[doc = "Bit 10 - Offset for channel 10"]
    #[inline(always)]
    pub fn off10(&mut self) -> OFF10_W {
        OFF10_W { w: self }
    }
    #[doc = "Bit 11 - Offset for channel 11"]
    #[inline(always)]
    pub fn off11(&mut self) -> OFF11_W {
        OFF11_W { w: self }
    }
    #[doc = "Bit 12 - Offset for channel 12"]
    #[inline(always)]
    pub fn off12(&mut self) -> OFF12_W {
        OFF12_W { w: self }
    }
    #[doc = "Bit 13 - Offset for channel 13"]
    #[inline(always)]
    pub fn off13(&mut self) -> OFF13_W {
        OFF13_W { w: self }
    }
    #[doc = "Bit 14 - Offset for channel 14"]
    #[inline(always)]
    pub fn off14(&mut self) -> OFF14_W {
        OFF14_W { w: self }
    }
    #[doc = "Bit 15 - Offset for channel 15"]
    #[inline(always)]
    pub fn off15(&mut self) -> OFF15_W {
        OFF15_W { w: self }
    }
    #[doc = "Bit 16 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&mut self) -> DIFF0_W {
        DIFF0_W { w: self }
    }
    #[doc = "Bit 17 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&mut self) -> DIFF1_W {
        DIFF1_W { w: self }
    }
    #[doc = "Bit 18 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&mut self) -> DIFF2_W {
        DIFF2_W { w: self }
    }
    #[doc = "Bit 19 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&mut self) -> DIFF3_W {
        DIFF3_W { w: self }
    }
    #[doc = "Bit 20 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&mut self) -> DIFF4_W {
        DIFF4_W { w: self }
    }
    #[doc = "Bit 21 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&mut self) -> DIFF5_W {
        DIFF5_W { w: self }
    }
    #[doc = "Bit 22 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&mut self) -> DIFF6_W {
        DIFF6_W { w: self }
    }
    #[doc = "Bit 23 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&mut self) -> DIFF7_W {
        DIFF7_W { w: self }
    }
    #[doc = "Bit 24 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&mut self) -> DIFF8_W {
        DIFF8_W { w: self }
    }
    #[doc = "Bit 25 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&mut self) -> DIFF9_W {
        DIFF9_W { w: self }
    }
    #[doc = "Bit 26 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&mut self) -> DIFF10_W {
        DIFF10_W { w: self }
    }
    #[doc = "Bit 27 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&mut self) -> DIFF11_W {
        DIFF11_W { w: self }
    }
    #[doc = "Bit 28 - Differential inputs for channel 12"]
    #[inline(always)]
    pub fn diff12(&mut self) -> DIFF12_W {
        DIFF12_W { w: self }
    }
    #[doc = "Bit 29 - Differential inputs for channel 13"]
    #[inline(always)]
    pub fn diff13(&mut self) -> DIFF13_W {
        DIFF13_W { w: self }
    }
    #[doc = "Bit 30 - Differential inputs for channel 14"]
    #[inline(always)]
    pub fn diff14(&mut self) -> DIFF14_W {
        DIFF14_W { w: self }
    }
    #[doc = "Bit 31 - Differential inputs for channel 15"]
    #[inline(always)]
    pub fn diff15(&mut self) -> DIFF15_W {
        DIFF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cor](index.html) module"]
pub struct COR_SPEC;
impl crate::RegisterSpec for COR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cor::R](R) reader structure"]
impl crate::Readable for COR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cor::W](W) writer structure"]
impl crate::Writable for COR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COR to value 0"]
impl crate::Resettable for COR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
