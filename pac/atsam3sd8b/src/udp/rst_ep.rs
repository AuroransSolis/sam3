#[doc = "Register `RST_EP` reader"]
pub struct R(crate::R<RST_EP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EP` writer"]
pub struct W(crate::W<RST_EP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EP_SPEC>;
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
impl From<crate::W<RST_EP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0` reader - Reset Endpoint 0"]
pub struct EP0_R(crate::FieldReader<bool, bool>);
impl EP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0` writer - Reset Endpoint 0"]
pub struct EP0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_W<'a> {
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
#[doc = "Field `EP1` reader - Reset Endpoint 1"]
pub struct EP1_R(crate::FieldReader<bool, bool>);
impl EP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1` writer - Reset Endpoint 1"]
pub struct EP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_W<'a> {
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
#[doc = "Field `EP2` reader - Reset Endpoint 2"]
pub struct EP2_R(crate::FieldReader<bool, bool>);
impl EP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2` writer - Reset Endpoint 2"]
pub struct EP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_W<'a> {
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
#[doc = "Field `EP3` reader - Reset Endpoint 3"]
pub struct EP3_R(crate::FieldReader<bool, bool>);
impl EP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3` writer - Reset Endpoint 3"]
pub struct EP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_W<'a> {
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
#[doc = "Field `EP4` reader - Reset Endpoint 4"]
pub struct EP4_R(crate::FieldReader<bool, bool>);
impl EP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4` writer - Reset Endpoint 4"]
pub struct EP4_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_W<'a> {
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
#[doc = "Field `EP5` reader - Reset Endpoint 5"]
pub struct EP5_R(crate::FieldReader<bool, bool>);
impl EP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5` writer - Reset Endpoint 5"]
pub struct EP5_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_W<'a> {
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
#[doc = "Field `EP6` reader - Reset Endpoint 6"]
pub struct EP6_R(crate::FieldReader<bool, bool>);
impl EP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6` writer - Reset Endpoint 6"]
pub struct EP6_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_W<'a> {
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
#[doc = "Field `EP7` reader - Reset Endpoint 7"]
pub struct EP7_R(crate::FieldReader<bool, bool>);
impl EP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7` writer - Reset Endpoint 7"]
pub struct EP7_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reset Endpoint 0"]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset Endpoint 1"]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset Endpoint 2"]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Endpoint 3"]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset Endpoint 4"]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset Endpoint 5"]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset Endpoint 6"]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset Endpoint 7"]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Endpoint 0"]
    #[inline(always)]
    pub fn ep0(&mut self) -> EP0_W {
        EP0_W { w: self }
    }
    #[doc = "Bit 1 - Reset Endpoint 1"]
    #[inline(always)]
    pub fn ep1(&mut self) -> EP1_W {
        EP1_W { w: self }
    }
    #[doc = "Bit 2 - Reset Endpoint 2"]
    #[inline(always)]
    pub fn ep2(&mut self) -> EP2_W {
        EP2_W { w: self }
    }
    #[doc = "Bit 3 - Reset Endpoint 3"]
    #[inline(always)]
    pub fn ep3(&mut self) -> EP3_W {
        EP3_W { w: self }
    }
    #[doc = "Bit 4 - Reset Endpoint 4"]
    #[inline(always)]
    pub fn ep4(&mut self) -> EP4_W {
        EP4_W { w: self }
    }
    #[doc = "Bit 5 - Reset Endpoint 5"]
    #[inline(always)]
    pub fn ep5(&mut self) -> EP5_W {
        EP5_W { w: self }
    }
    #[doc = "Bit 6 - Reset Endpoint 6"]
    #[inline(always)]
    pub fn ep6(&mut self) -> EP6_W {
        EP6_W { w: self }
    }
    #[doc = "Bit 7 - Reset Endpoint 7"]
    #[inline(always)]
    pub fn ep7(&mut self) -> EP7_W {
        EP7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_ep](index.html) module"]
pub struct RST_EP_SPEC;
impl crate::RegisterSpec for RST_EP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_ep::R](R) reader structure"]
impl crate::Readable for RST_EP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_ep::W](W) writer structure"]
impl crate::Writable for RST_EP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EP to value 0"]
impl crate::Resettable for RST_EP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
