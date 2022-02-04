#[doc = "Register `FPV` reader"]
pub struct R(crate::R<FPV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPV` writer"]
pub struct W(crate::W<FPV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPV_SPEC>;
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
impl From<crate::W<FPV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub struct FPVH0_R(crate::FieldReader<bool, bool>);
impl FPVH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub struct FPVH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH0_W<'a> {
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
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub struct FPVH1_R(crate::FieldReader<bool, bool>);
impl FPVH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub struct FPVH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH1_W<'a> {
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
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub struct FPVH2_R(crate::FieldReader<bool, bool>);
impl FPVH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub struct FPVH2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH2_W<'a> {
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
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub struct FPVH3_R(crate::FieldReader<bool, bool>);
impl FPVH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub struct FPVH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH3_W<'a> {
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
#[doc = "Field `FPVH4` reader - Fault Protection Value for PWMH output on channel 4"]
pub struct FPVH4_R(crate::FieldReader<bool, bool>);
impl FPVH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH4` writer - Fault Protection Value for PWMH output on channel 4"]
pub struct FPVH4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH4_W<'a> {
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
#[doc = "Field `FPVH5` reader - Fault Protection Value for PWMH output on channel 5"]
pub struct FPVH5_R(crate::FieldReader<bool, bool>);
impl FPVH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH5` writer - Fault Protection Value for PWMH output on channel 5"]
pub struct FPVH5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH5_W<'a> {
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
#[doc = "Field `FPVH6` reader - Fault Protection Value for PWMH output on channel 6"]
pub struct FPVH6_R(crate::FieldReader<bool, bool>);
impl FPVH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH6` writer - Fault Protection Value for PWMH output on channel 6"]
pub struct FPVH6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH6_W<'a> {
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
#[doc = "Field `FPVH7` reader - Fault Protection Value for PWMH output on channel 7"]
pub struct FPVH7_R(crate::FieldReader<bool, bool>);
impl FPVH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH7` writer - Fault Protection Value for PWMH output on channel 7"]
pub struct FPVH7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH7_W<'a> {
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
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub struct FPVL0_R(crate::FieldReader<bool, bool>);
impl FPVL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub struct FPVL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL0_W<'a> {
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
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub struct FPVL1_R(crate::FieldReader<bool, bool>);
impl FPVL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub struct FPVL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL1_W<'a> {
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
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub struct FPVL2_R(crate::FieldReader<bool, bool>);
impl FPVL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub struct FPVL2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL2_W<'a> {
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
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub struct FPVL3_R(crate::FieldReader<bool, bool>);
impl FPVL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub struct FPVL3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL3_W<'a> {
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
#[doc = "Field `FPVL4` reader - Fault Protection Value for PWML output on channel 4"]
pub struct FPVL4_R(crate::FieldReader<bool, bool>);
impl FPVL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL4` writer - Fault Protection Value for PWML output on channel 4"]
pub struct FPVL4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL4_W<'a> {
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
#[doc = "Field `FPVL5` reader - Fault Protection Value for PWML output on channel 5"]
pub struct FPVL5_R(crate::FieldReader<bool, bool>);
impl FPVL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL5` writer - Fault Protection Value for PWML output on channel 5"]
pub struct FPVL5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL5_W<'a> {
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
#[doc = "Field `FPVL6` reader - Fault Protection Value for PWML output on channel 6"]
pub struct FPVL6_R(crate::FieldReader<bool, bool>);
impl FPVL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL6` writer - Fault Protection Value for PWML output on channel 6"]
pub struct FPVL6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL6_W<'a> {
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
#[doc = "Field `FPVL7` reader - Fault Protection Value for PWML output on channel 7"]
pub struct FPVL7_R(crate::FieldReader<bool, bool>);
impl FPVL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL7` writer - Fault Protection Value for PWML output on channel 7"]
pub struct FPVL7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> FPVH0_R {
        FPVH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> FPVH1_R {
        FPVH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> FPVH2_R {
        FPVH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> FPVH3_R {
        FPVH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    pub fn fpvh4(&self) -> FPVH4_R {
        FPVH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    pub fn fpvh5(&self) -> FPVH5_R {
        FPVH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    pub fn fpvh6(&self) -> FPVH6_R {
        FPVH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    pub fn fpvh7(&self) -> FPVH7_R {
        FPVH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> FPVL0_R {
        FPVL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> FPVL1_R {
        FPVL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> FPVL2_R {
        FPVL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> FPVL3_R {
        FPVL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    pub fn fpvl4(&self) -> FPVL4_R {
        FPVL4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    pub fn fpvl5(&self) -> FPVL5_R {
        FPVL5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    pub fn fpvl6(&self) -> FPVL6_R {
        FPVL6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    pub fn fpvl7(&self) -> FPVL7_R {
        FPVL7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&mut self) -> FPVH0_W {
        FPVH0_W { w: self }
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&mut self) -> FPVH1_W {
        FPVH1_W { w: self }
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&mut self) -> FPVH2_W {
        FPVH2_W { w: self }
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&mut self) -> FPVH3_W {
        FPVH3_W { w: self }
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    pub fn fpvh4(&mut self) -> FPVH4_W {
        FPVH4_W { w: self }
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    pub fn fpvh5(&mut self) -> FPVH5_W {
        FPVH5_W { w: self }
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    pub fn fpvh6(&mut self) -> FPVH6_W {
        FPVH6_W { w: self }
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    pub fn fpvh7(&mut self) -> FPVH7_W {
        FPVH7_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&mut self) -> FPVL0_W {
        FPVL0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&mut self) -> FPVL1_W {
        FPVL1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&mut self) -> FPVL2_W {
        FPVL2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&mut self) -> FPVL3_W {
        FPVL3_W { w: self }
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    pub fn fpvl4(&mut self) -> FPVL4_W {
        FPVL4_W { w: self }
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    pub fn fpvl5(&mut self) -> FPVL5_W {
        FPVL5_W { w: self }
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    pub fn fpvl6(&mut self) -> FPVL6_W {
        FPVL6_W { w: self }
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    pub fn fpvl7(&mut self) -> FPVL7_W {
        FPVL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpv](index.html) module"]
pub struct FPV_SPEC;
impl crate::RegisterSpec for FPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpv::R](R) reader structure"]
impl crate::Readable for FPV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpv::W](W) writer structure"]
impl crate::Writable for FPV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPV to value 0"]
impl crate::Resettable for FPV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
