#[doc = "Register `USRIO` reader"]
pub struct R(crate::R<USRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USRIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USRIO` writer"]
pub struct W(crate::W<USRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USRIO_SPEC>;
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
impl From<crate::W<USRIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMII` reader - Reduce MII"]
pub struct RMII_R(crate::FieldReader<bool, bool>);
impl RMII_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII` writer - Reduce MII"]
pub struct RMII_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_W<'a> {
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
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub struct CLKEN_R(crate::FieldReader<bool, bool>);
impl CLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    pub fn rmii(&mut self) -> RMII_W {
        RMII_W { w: self }
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrio](index.html) module"]
pub struct USRIO_SPEC;
impl crate::RegisterSpec for USRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usrio::R](R) reader structure"]
impl crate::Readable for USRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usrio::W](W) writer structure"]
impl crate::Writable for USRIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USRIO to value 0"]
impl crate::Resettable for USRIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
