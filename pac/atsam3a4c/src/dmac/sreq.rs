#[doc = "Register `SREQ` reader"]
pub struct R(crate::R<SREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREQ` writer"]
pub struct W(crate::W<SREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SREQ_SPEC>;
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
impl From<crate::W<SREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSREQ0` reader - Source Request"]
pub struct SSREQ0_R(crate::FieldReader<bool, bool>);
impl SSREQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSREQ0` writer - Source Request"]
pub struct SSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ0_W<'a> {
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
#[doc = "Field `DSREQ0` reader - Destination Request"]
pub struct DSREQ0_R(crate::FieldReader<bool, bool>);
impl DSREQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSREQ0` writer - Destination Request"]
pub struct DSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ0_W<'a> {
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
#[doc = "Field `SSREQ1` reader - Source Request"]
pub struct SSREQ1_R(crate::FieldReader<bool, bool>);
impl SSREQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSREQ1` writer - Source Request"]
pub struct SSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ1_W<'a> {
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
#[doc = "Field `DSREQ1` reader - Destination Request"]
pub struct DSREQ1_R(crate::FieldReader<bool, bool>);
impl DSREQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSREQ1` writer - Destination Request"]
pub struct DSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ1_W<'a> {
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
#[doc = "Field `SSREQ2` reader - Source Request"]
pub struct SSREQ2_R(crate::FieldReader<bool, bool>);
impl SSREQ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSREQ2` writer - Source Request"]
pub struct SSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ2_W<'a> {
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
#[doc = "Field `DSREQ2` reader - Destination Request"]
pub struct DSREQ2_R(crate::FieldReader<bool, bool>);
impl DSREQ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSREQ2` writer - Destination Request"]
pub struct DSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ2_W<'a> {
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
#[doc = "Field `SSREQ3` reader - Source Request"]
pub struct SSREQ3_R(crate::FieldReader<bool, bool>);
impl SSREQ3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSREQ3` writer - Source Request"]
pub struct SSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ3_W<'a> {
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
#[doc = "Field `DSREQ3` reader - Destination Request"]
pub struct DSREQ3_R(crate::FieldReader<bool, bool>);
impl DSREQ3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSREQ3` writer - Destination Request"]
pub struct DSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ3_W<'a> {
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
#[doc = "Field `SSREQ4` reader - Source Request"]
pub struct SSREQ4_R(crate::FieldReader<bool, bool>);
impl SSREQ4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSREQ4` writer - Source Request"]
pub struct SSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ4_W<'a> {
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
#[doc = "Field `DSREQ4` reader - Destination Request"]
pub struct DSREQ4_R(crate::FieldReader<bool, bool>);
impl DSREQ4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSREQ4` writer - Destination Request"]
pub struct DSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ4_W<'a> {
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
#[doc = "Field `SSREQ5` reader - Source Request"]
pub struct SSREQ5_R(crate::FieldReader<bool, bool>);
impl SSREQ5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSREQ5` writer - Source Request"]
pub struct SSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ5_W<'a> {
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
#[doc = "Field `DSREQ5` reader - Destination Request"]
pub struct DSREQ5_R(crate::FieldReader<bool, bool>);
impl DSREQ5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSREQ5` writer - Destination Request"]
pub struct DSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ5_W<'a> {
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
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&self) -> SSREQ0_R {
        SSREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&self) -> DSREQ0_R {
        DSREQ0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&self) -> SSREQ1_R {
        SSREQ1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&self) -> DSREQ1_R {
        DSREQ1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&self) -> SSREQ2_R {
        SSREQ2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&self) -> DSREQ2_R {
        DSREQ2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&self) -> SSREQ3_R {
        SSREQ3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&self) -> DSREQ3_R {
        DSREQ3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&self) -> SSREQ4_R {
        SSREQ4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&self) -> DSREQ4_R {
        DSREQ4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&self) -> SSREQ5_R {
        SSREQ5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&self) -> DSREQ5_R {
        DSREQ5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&mut self) -> SSREQ0_W {
        SSREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&mut self) -> DSREQ0_W {
        DSREQ0_W { w: self }
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&mut self) -> SSREQ1_W {
        SSREQ1_W { w: self }
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&mut self) -> DSREQ1_W {
        DSREQ1_W { w: self }
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&mut self) -> SSREQ2_W {
        SSREQ2_W { w: self }
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&mut self) -> DSREQ2_W {
        DSREQ2_W { w: self }
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&mut self) -> SSREQ3_W {
        SSREQ3_W { w: self }
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&mut self) -> DSREQ3_W {
        DSREQ3_W { w: self }
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&mut self) -> SSREQ4_W {
        SSREQ4_W { w: self }
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&mut self) -> DSREQ4_W {
        DSREQ4_W { w: self }
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&mut self) -> SSREQ5_W {
        SSREQ5_W { w: self }
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&mut self) -> DSREQ5_W {
        DSREQ5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Software Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sreq](index.html) module"]
pub struct SREQ_SPEC;
impl crate::RegisterSpec for SREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sreq::R](R) reader structure"]
impl crate::Readable for SREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sreq::W](W) writer structure"]
impl crate::Writable for SREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SREQ to value 0"]
impl crate::Resettable for SREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
