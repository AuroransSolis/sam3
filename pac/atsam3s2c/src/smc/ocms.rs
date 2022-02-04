#[doc = "Register `OCMS` reader"]
pub struct R(crate::R<OCMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCMS` writer"]
pub struct W(crate::W<OCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMS_SPEC>;
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
impl From<crate::W<OCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub struct SMSE_R(crate::FieldReader<bool, bool>);
impl SMSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub struct SMSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSE_W<'a> {
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
#[doc = "Field `CS0SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS0SE_R(crate::FieldReader<bool, bool>);
impl CS0SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS0SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS0SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0SE_W<'a> {
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
#[doc = "Field `CS1SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS1SE_R(crate::FieldReader<bool, bool>);
impl CS1SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS1SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS1SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1SE_W<'a> {
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
#[doc = "Field `CS2SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS2SE_R(crate::FieldReader<bool, bool>);
impl CS2SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS2SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS2SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS2SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS2SE_W<'a> {
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
#[doc = "Field `CS3SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS3SE_R(crate::FieldReader<bool, bool>);
impl CS3SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS3SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS3SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS3SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub struct CS3SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3SE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&self) -> CS0SE_R {
        CS0SE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&self) -> CS1SE_R {
        CS1SE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&self) -> CS2SE_R {
        CS2SE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&self) -> CS3SE_R {
        CS3SE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&mut self) -> SMSE_W {
        SMSE_W { w: self }
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&mut self) -> CS0SE_W {
        CS0SE_W { w: self }
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&mut self) -> CS1SE_W {
        CS1SE_W { w: self }
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&mut self) -> CS2SE_W {
        CS2SE_W { w: self }
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&mut self) -> CS3SE_W {
        CS3SE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC OCMS MODE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](index.html) module"]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocms::R](R) reader structure"]
impl crate::Readable for OCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocms::W](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
