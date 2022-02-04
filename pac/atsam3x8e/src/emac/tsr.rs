#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSR` writer"]
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UBR` reader - Used Bit Read"]
pub struct UBR_R(crate::FieldReader<bool, bool>);
impl UBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBR` writer - Used Bit Read"]
pub struct UBR_W<'a> {
    w: &'a mut W,
}
impl<'a> UBR_W<'a> {
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
#[doc = "Field `COL` reader - Collision Occurred"]
pub struct COL_R(crate::FieldReader<bool, bool>);
impl COL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COL` writer - Collision Occurred"]
pub struct COL_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_W<'a> {
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
#[doc = "Field `RLES` reader - Retry Limit exceeded"]
pub struct RLES_R(crate::FieldReader<bool, bool>);
impl RLES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLES` writer - Retry Limit exceeded"]
pub struct RLES_W<'a> {
    w: &'a mut W,
}
impl<'a> RLES_W<'a> {
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
#[doc = "Field `TGO` reader - Transmit Go"]
pub struct TGO_R(crate::FieldReader<bool, bool>);
impl TGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TGO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGO` writer - Transmit Go"]
pub struct TGO_W<'a> {
    w: &'a mut W,
}
impl<'a> TGO_W<'a> {
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
#[doc = "Field `BEX` reader - Buffers exhausted mid frame"]
pub struct BEX_R(crate::FieldReader<bool, bool>);
impl BEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEX` writer - Buffers exhausted mid frame"]
pub struct BEX_W<'a> {
    w: &'a mut W,
}
impl<'a> BEX_W<'a> {
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
#[doc = "Field `COMP` reader - Transmit Complete"]
pub struct COMP_R(crate::FieldReader<bool, bool>);
impl COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - Transmit Complete"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
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
#[doc = "Field `UND` reader - Transmit Underrun"]
pub struct UND_R(crate::FieldReader<bool, bool>);
impl UND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UND` writer - Transmit Underrun"]
pub struct UND_W<'a> {
    w: &'a mut W,
}
impl<'a> UND_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UBR_R {
        UBR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    pub fn rles(&self) -> RLES_R {
        RLES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn tgo(&self) -> TGO_R {
        TGO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UND_R {
        UND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&mut self) -> UBR_W {
        UBR_W { w: self }
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W {
        COL_W { w: self }
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    pub fn rles(&mut self) -> RLES_W {
        RLES_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn tgo(&mut self) -> TGO_W {
        TGO_W { w: self }
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    pub fn bex(&mut self) -> BEX_W {
        BEX_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&mut self) -> UND_W {
        UND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsr::W](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
