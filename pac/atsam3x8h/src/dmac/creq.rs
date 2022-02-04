#[doc = "Register `CREQ` reader"]
pub struct R(crate::R<CREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CREQ` writer"]
pub struct W(crate::W<CREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CREQ_SPEC>;
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
impl From<crate::W<CREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCREQ0` reader - Source Chunk Request"]
pub struct SCREQ0_R(crate::FieldReader<bool, bool>);
impl SCREQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREQ0` writer - Source Chunk Request"]
pub struct SCREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ0_W<'a> {
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
#[doc = "Field `DCREQ0` reader - Destination Chunk Request"]
pub struct DCREQ0_R(crate::FieldReader<bool, bool>);
impl DCREQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCREQ0` writer - Destination Chunk Request"]
pub struct DCREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ0_W<'a> {
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
#[doc = "Field `SCREQ1` reader - Source Chunk Request"]
pub struct SCREQ1_R(crate::FieldReader<bool, bool>);
impl SCREQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREQ1` writer - Source Chunk Request"]
pub struct SCREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ1_W<'a> {
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
#[doc = "Field `DCREQ1` reader - Destination Chunk Request"]
pub struct DCREQ1_R(crate::FieldReader<bool, bool>);
impl DCREQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCREQ1` writer - Destination Chunk Request"]
pub struct DCREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ1_W<'a> {
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
#[doc = "Field `SCREQ2` reader - Source Chunk Request"]
pub struct SCREQ2_R(crate::FieldReader<bool, bool>);
impl SCREQ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREQ2` writer - Source Chunk Request"]
pub struct SCREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ2_W<'a> {
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
#[doc = "Field `DCREQ2` reader - Destination Chunk Request"]
pub struct DCREQ2_R(crate::FieldReader<bool, bool>);
impl DCREQ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCREQ2` writer - Destination Chunk Request"]
pub struct DCREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ2_W<'a> {
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
#[doc = "Field `SCREQ3` reader - Source Chunk Request"]
pub struct SCREQ3_R(crate::FieldReader<bool, bool>);
impl SCREQ3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREQ3` writer - Source Chunk Request"]
pub struct SCREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ3_W<'a> {
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
#[doc = "Field `DCREQ3` reader - Destination Chunk Request"]
pub struct DCREQ3_R(crate::FieldReader<bool, bool>);
impl DCREQ3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCREQ3` writer - Destination Chunk Request"]
pub struct DCREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ3_W<'a> {
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
#[doc = "Field `SCREQ4` reader - Source Chunk Request"]
pub struct SCREQ4_R(crate::FieldReader<bool, bool>);
impl SCREQ4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREQ4` writer - Source Chunk Request"]
pub struct SCREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ4_W<'a> {
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
#[doc = "Field `DCREQ4` reader - Destination Chunk Request"]
pub struct DCREQ4_R(crate::FieldReader<bool, bool>);
impl DCREQ4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCREQ4` writer - Destination Chunk Request"]
pub struct DCREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ4_W<'a> {
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
#[doc = "Field `SCREQ5` reader - Source Chunk Request"]
pub struct SCREQ5_R(crate::FieldReader<bool, bool>);
impl SCREQ5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCREQ5` writer - Source Chunk Request"]
pub struct SCREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ5_W<'a> {
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
#[doc = "Field `DCREQ5` reader - Destination Chunk Request"]
pub struct DCREQ5_R(crate::FieldReader<bool, bool>);
impl DCREQ5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCREQ5` writer - Destination Chunk Request"]
pub struct DCREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ5_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&self) -> SCREQ0_R {
        SCREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&self) -> DCREQ0_R {
        DCREQ0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&self) -> SCREQ1_R {
        SCREQ1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&self) -> DCREQ1_R {
        DCREQ1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&self) -> SCREQ2_R {
        SCREQ2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&self) -> DCREQ2_R {
        DCREQ2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&self) -> SCREQ3_R {
        SCREQ3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&self) -> DCREQ3_R {
        DCREQ3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&self) -> SCREQ4_R {
        SCREQ4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&self) -> DCREQ4_R {
        DCREQ4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&self) -> SCREQ5_R {
        SCREQ5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&self) -> DCREQ5_R {
        DCREQ5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&mut self) -> SCREQ0_W {
        SCREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&mut self) -> DCREQ0_W {
        DCREQ0_W { w: self }
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&mut self) -> SCREQ1_W {
        SCREQ1_W { w: self }
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&mut self) -> DCREQ1_W {
        DCREQ1_W { w: self }
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&mut self) -> SCREQ2_W {
        SCREQ2_W { w: self }
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&mut self) -> DCREQ2_W {
        DCREQ2_W { w: self }
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&mut self) -> SCREQ3_W {
        SCREQ3_W { w: self }
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&mut self) -> DCREQ3_W {
        DCREQ3_W { w: self }
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&mut self) -> SCREQ4_W {
        SCREQ4_W { w: self }
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&mut self) -> DCREQ4_W {
        DCREQ4_W { w: self }
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&mut self) -> SCREQ5_W {
        SCREQ5_W { w: self }
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&mut self) -> DCREQ5_W {
        DCREQ5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Software Chunk Transfer Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [creq](index.html) module"]
pub struct CREQ_SPEC;
impl crate::RegisterSpec for CREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [creq::R](R) reader structure"]
impl crate::Readable for CREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [creq::W](W) writer structure"]
impl crate::Writable for CREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CREQ to value 0"]
impl crate::Resettable for CREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
