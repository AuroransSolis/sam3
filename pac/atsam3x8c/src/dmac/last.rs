#[doc = "Register `LAST` reader"]
pub struct R(crate::R<LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAST` writer"]
pub struct W(crate::W<LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAST_SPEC>;
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
impl From<crate::W<LAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAST0` reader - Source Last"]
pub struct SLAST0_R(crate::FieldReader<bool, bool>);
impl SLAST0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST0` writer - Source Last"]
pub struct SLAST0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST0_W<'a> {
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
#[doc = "Field `DLAST0` reader - Destination Last"]
pub struct DLAST0_R(crate::FieldReader<bool, bool>);
impl DLAST0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAST0` writer - Destination Last"]
pub struct DLAST0_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST0_W<'a> {
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
#[doc = "Field `SLAST1` reader - Source Last"]
pub struct SLAST1_R(crate::FieldReader<bool, bool>);
impl SLAST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST1` writer - Source Last"]
pub struct SLAST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST1_W<'a> {
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
#[doc = "Field `DLAST1` reader - Destination Last"]
pub struct DLAST1_R(crate::FieldReader<bool, bool>);
impl DLAST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAST1` writer - Destination Last"]
pub struct DLAST1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST1_W<'a> {
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
#[doc = "Field `SLAST2` reader - Source Last"]
pub struct SLAST2_R(crate::FieldReader<bool, bool>);
impl SLAST2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST2` writer - Source Last"]
pub struct SLAST2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST2_W<'a> {
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
#[doc = "Field `DLAST2` reader - Destination Last"]
pub struct DLAST2_R(crate::FieldReader<bool, bool>);
impl DLAST2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAST2` writer - Destination Last"]
pub struct DLAST2_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST2_W<'a> {
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
#[doc = "Field `SLAST3` reader - Source Last"]
pub struct SLAST3_R(crate::FieldReader<bool, bool>);
impl SLAST3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST3` writer - Source Last"]
pub struct SLAST3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST3_W<'a> {
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
#[doc = "Field `DLAST3` reader - Destination Last"]
pub struct DLAST3_R(crate::FieldReader<bool, bool>);
impl DLAST3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAST3` writer - Destination Last"]
pub struct DLAST3_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST3_W<'a> {
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
#[doc = "Field `SLAST4` reader - Source Last"]
pub struct SLAST4_R(crate::FieldReader<bool, bool>);
impl SLAST4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST4` writer - Source Last"]
pub struct SLAST4_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST4_W<'a> {
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
#[doc = "Field `DLAST4` reader - Destination Last"]
pub struct DLAST4_R(crate::FieldReader<bool, bool>);
impl DLAST4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAST4` writer - Destination Last"]
pub struct DLAST4_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST4_W<'a> {
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
#[doc = "Field `SLAST5` reader - Source Last"]
pub struct SLAST5_R(crate::FieldReader<bool, bool>);
impl SLAST5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST5` writer - Source Last"]
pub struct SLAST5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST5_W<'a> {
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
#[doc = "Field `DLAST5` reader - Destination Last"]
pub struct DLAST5_R(crate::FieldReader<bool, bool>);
impl DLAST5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAST5` writer - Destination Last"]
pub struct DLAST5_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST5_W<'a> {
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
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&self) -> SLAST0_R {
        SLAST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&self) -> DLAST0_R {
        DLAST0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&self) -> SLAST1_R {
        SLAST1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&self) -> DLAST1_R {
        DLAST1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&self) -> SLAST2_R {
        SLAST2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&self) -> DLAST2_R {
        DLAST2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&self) -> SLAST3_R {
        SLAST3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&self) -> DLAST3_R {
        DLAST3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&self) -> SLAST4_R {
        SLAST4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&self) -> DLAST4_R {
        DLAST4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&self) -> SLAST5_R {
        SLAST5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&self) -> DLAST5_R {
        DLAST5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&mut self) -> SLAST0_W {
        SLAST0_W { w: self }
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&mut self) -> DLAST0_W {
        DLAST0_W { w: self }
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&mut self) -> SLAST1_W {
        SLAST1_W { w: self }
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&mut self) -> DLAST1_W {
        DLAST1_W { w: self }
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&mut self) -> SLAST2_W {
        SLAST2_W { w: self }
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&mut self) -> DLAST2_W {
        DLAST2_W { w: self }
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&mut self) -> SLAST3_W {
        SLAST3_W { w: self }
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&mut self) -> DLAST3_W {
        DLAST3_W { w: self }
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&mut self) -> SLAST4_W {
        SLAST4_W { w: self }
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&mut self) -> DLAST4_W {
        DLAST4_W { w: self }
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&mut self) -> SLAST5_W {
        SLAST5_W { w: self }
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&mut self) -> DLAST5_W {
        DLAST5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Software Last Transfer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [last](index.html) module"]
pub struct LAST_SPEC;
impl crate::RegisterSpec for LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [last::R](R) reader structure"]
impl crate::Readable for LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [last::W](W) writer structure"]
impl crate::Writable for LAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAST to value 0"]
impl crate::Resettable for LAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
