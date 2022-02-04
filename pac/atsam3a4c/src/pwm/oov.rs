#[doc = "Register `OOV` reader"]
pub struct R(crate::R<OOV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OOV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OOV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OOV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OOV` writer"]
pub struct W(crate::W<OOV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OOV_SPEC>;
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
impl From<crate::W<OOV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OOV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OOVH0` reader - Output Override Value for PWMH output of the channel 0"]
pub struct OOVH0_R(crate::FieldReader<bool, bool>);
impl OOVH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH0` writer - Output Override Value for PWMH output of the channel 0"]
pub struct OOVH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH0_W<'a> {
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
#[doc = "Field `OOVH1` reader - Output Override Value for PWMH output of the channel 1"]
pub struct OOVH1_R(crate::FieldReader<bool, bool>);
impl OOVH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH1` writer - Output Override Value for PWMH output of the channel 1"]
pub struct OOVH1_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH1_W<'a> {
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
#[doc = "Field `OOVH2` reader - Output Override Value for PWMH output of the channel 2"]
pub struct OOVH2_R(crate::FieldReader<bool, bool>);
impl OOVH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH2` writer - Output Override Value for PWMH output of the channel 2"]
pub struct OOVH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH2_W<'a> {
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
#[doc = "Field `OOVH3` reader - Output Override Value for PWMH output of the channel 3"]
pub struct OOVH3_R(crate::FieldReader<bool, bool>);
impl OOVH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH3` writer - Output Override Value for PWMH output of the channel 3"]
pub struct OOVH3_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH3_W<'a> {
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
#[doc = "Field `OOVH4` reader - Output Override Value for PWMH output of the channel 4"]
pub struct OOVH4_R(crate::FieldReader<bool, bool>);
impl OOVH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH4` writer - Output Override Value for PWMH output of the channel 4"]
pub struct OOVH4_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH4_W<'a> {
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
#[doc = "Field `OOVH5` reader - Output Override Value for PWMH output of the channel 5"]
pub struct OOVH5_R(crate::FieldReader<bool, bool>);
impl OOVH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH5` writer - Output Override Value for PWMH output of the channel 5"]
pub struct OOVH5_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH5_W<'a> {
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
#[doc = "Field `OOVH6` reader - Output Override Value for PWMH output of the channel 6"]
pub struct OOVH6_R(crate::FieldReader<bool, bool>);
impl OOVH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH6` writer - Output Override Value for PWMH output of the channel 6"]
pub struct OOVH6_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH6_W<'a> {
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
#[doc = "Field `OOVH7` reader - Output Override Value for PWMH output of the channel 7"]
pub struct OOVH7_R(crate::FieldReader<bool, bool>);
impl OOVH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVH7` writer - Output Override Value for PWMH output of the channel 7"]
pub struct OOVH7_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVH7_W<'a> {
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
#[doc = "Field `OOVL0` reader - Output Override Value for PWML output of the channel 0"]
pub struct OOVL0_R(crate::FieldReader<bool, bool>);
impl OOVL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL0` writer - Output Override Value for PWML output of the channel 0"]
pub struct OOVL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL0_W<'a> {
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
#[doc = "Field `OOVL1` reader - Output Override Value for PWML output of the channel 1"]
pub struct OOVL1_R(crate::FieldReader<bool, bool>);
impl OOVL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL1` writer - Output Override Value for PWML output of the channel 1"]
pub struct OOVL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL1_W<'a> {
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
#[doc = "Field `OOVL2` reader - Output Override Value for PWML output of the channel 2"]
pub struct OOVL2_R(crate::FieldReader<bool, bool>);
impl OOVL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL2` writer - Output Override Value for PWML output of the channel 2"]
pub struct OOVL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL2_W<'a> {
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
#[doc = "Field `OOVL3` reader - Output Override Value for PWML output of the channel 3"]
pub struct OOVL3_R(crate::FieldReader<bool, bool>);
impl OOVL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL3` writer - Output Override Value for PWML output of the channel 3"]
pub struct OOVL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL3_W<'a> {
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
#[doc = "Field `OOVL4` reader - Output Override Value for PWML output of the channel 4"]
pub struct OOVL4_R(crate::FieldReader<bool, bool>);
impl OOVL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL4` writer - Output Override Value for PWML output of the channel 4"]
pub struct OOVL4_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL4_W<'a> {
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
#[doc = "Field `OOVL5` reader - Output Override Value for PWML output of the channel 5"]
pub struct OOVL5_R(crate::FieldReader<bool, bool>);
impl OOVL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL5` writer - Output Override Value for PWML output of the channel 5"]
pub struct OOVL5_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL5_W<'a> {
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
#[doc = "Field `OOVL6` reader - Output Override Value for PWML output of the channel 6"]
pub struct OOVL6_R(crate::FieldReader<bool, bool>);
impl OOVL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL6` writer - Output Override Value for PWML output of the channel 6"]
pub struct OOVL6_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL6_W<'a> {
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
#[doc = "Field `OOVL7` reader - Output Override Value for PWML output of the channel 7"]
pub struct OOVL7_R(crate::FieldReader<bool, bool>);
impl OOVL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOVL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOVL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOVL7` writer - Output Override Value for PWML output of the channel 7"]
pub struct OOVL7_W<'a> {
    w: &'a mut W,
}
impl<'a> OOVL7_W<'a> {
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
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oovh0(&self) -> OOVH0_R {
        OOVH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oovh1(&self) -> OOVH1_R {
        OOVH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oovh2(&self) -> OOVH2_R {
        OOVH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oovh3(&self) -> OOVH3_R {
        OOVH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output Override Value for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn oovh4(&self) -> OOVH4_R {
        OOVH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Output Override Value for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn oovh5(&self) -> OOVH5_R {
        OOVH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output Override Value for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn oovh6(&self) -> OOVH6_R {
        OOVH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output Override Value for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn oovh7(&self) -> OOVH7_R {
        OOVH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oovl0(&self) -> OOVL0_R {
        OOVL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oovl1(&self) -> OOVL1_R {
        OOVL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oovl2(&self) -> OOVL2_R {
        OOVL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oovl3(&self) -> OOVL3_R {
        OOVL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output Override Value for PWML output of the channel 4"]
    #[inline(always)]
    pub fn oovl4(&self) -> OOVL4_R {
        OOVL4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Output Override Value for PWML output of the channel 5"]
    #[inline(always)]
    pub fn oovl5(&self) -> OOVL5_R {
        OOVL5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output Override Value for PWML output of the channel 6"]
    #[inline(always)]
    pub fn oovl6(&self) -> OOVL6_R {
        OOVL6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Output Override Value for PWML output of the channel 7"]
    #[inline(always)]
    pub fn oovl7(&self) -> OOVL7_R {
        OOVL7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oovh0(&mut self) -> OOVH0_W {
        OOVH0_W { w: self }
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oovh1(&mut self) -> OOVH1_W {
        OOVH1_W { w: self }
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oovh2(&mut self) -> OOVH2_W {
        OOVH2_W { w: self }
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oovh3(&mut self) -> OOVH3_W {
        OOVH3_W { w: self }
    }
    #[doc = "Bit 4 - Output Override Value for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn oovh4(&mut self) -> OOVH4_W {
        OOVH4_W { w: self }
    }
    #[doc = "Bit 5 - Output Override Value for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn oovh5(&mut self) -> OOVH5_W {
        OOVH5_W { w: self }
    }
    #[doc = "Bit 6 - Output Override Value for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn oovh6(&mut self) -> OOVH6_W {
        OOVH6_W { w: self }
    }
    #[doc = "Bit 7 - Output Override Value for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn oovh7(&mut self) -> OOVH7_W {
        OOVH7_W { w: self }
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oovl0(&mut self) -> OOVL0_W {
        OOVL0_W { w: self }
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oovl1(&mut self) -> OOVL1_W {
        OOVL1_W { w: self }
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oovl2(&mut self) -> OOVL2_W {
        OOVL2_W { w: self }
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oovl3(&mut self) -> OOVL3_W {
        OOVL3_W { w: self }
    }
    #[doc = "Bit 20 - Output Override Value for PWML output of the channel 4"]
    #[inline(always)]
    pub fn oovl4(&mut self) -> OOVL4_W {
        OOVL4_W { w: self }
    }
    #[doc = "Bit 21 - Output Override Value for PWML output of the channel 5"]
    #[inline(always)]
    pub fn oovl5(&mut self) -> OOVL5_W {
        OOVL5_W { w: self }
    }
    #[doc = "Bit 22 - Output Override Value for PWML output of the channel 6"]
    #[inline(always)]
    pub fn oovl6(&mut self) -> OOVL6_W {
        OOVL6_W { w: self }
    }
    #[doc = "Bit 23 - Output Override Value for PWML output of the channel 7"]
    #[inline(always)]
    pub fn oovl7(&mut self) -> OOVL7_W {
        OOVL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Override Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oov](index.html) module"]
pub struct OOV_SPEC;
impl crate::RegisterSpec for OOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oov::R](R) reader structure"]
impl crate::Readable for OOV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oov::W](W) writer structure"]
impl crate::Writable for OOV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OOV to value 0"]
impl crate::Resettable for OOV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
