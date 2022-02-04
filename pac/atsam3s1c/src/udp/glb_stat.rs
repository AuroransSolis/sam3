#[doc = "Register `GLB_STAT` reader"]
pub struct R(crate::R<GLB_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLB_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLB_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLB_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLB_STAT` writer"]
pub struct W(crate::W<GLB_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLB_STAT_SPEC>;
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
impl From<crate::W<GLB_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLB_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADDEN` reader - Function Address Enable"]
pub struct FADDEN_R(crate::FieldReader<bool, bool>);
impl FADDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FADDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDEN` writer - Function Address Enable"]
pub struct FADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDEN_W<'a> {
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
#[doc = "Field `CONFG` reader - Configured"]
pub struct CONFG_R(crate::FieldReader<bool, bool>);
impl CONFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFG` writer - Configured"]
pub struct CONFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFG_W<'a> {
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
#[doc = "Field `ESR` reader - Enable Send Resume"]
pub struct ESR_R(crate::FieldReader<bool, bool>);
impl ESR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESR` writer - Enable Send Resume"]
pub struct ESR_W<'a> {
    w: &'a mut W,
}
impl<'a> ESR_W<'a> {
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
#[doc = "Field `RSMINPR` reader - "]
pub struct RSMINPR_R(crate::FieldReader<bool, bool>);
impl RSMINPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSMINPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSMINPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSMINPR` writer - "]
pub struct RSMINPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMINPR_W<'a> {
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
#[doc = "Field `RMWUPE` reader - Remote Wake Up Enable"]
pub struct RMWUPE_R(crate::FieldReader<bool, bool>);
impl RMWUPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMWUPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMWUPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMWUPE` writer - Remote Wake Up Enable"]
pub struct RMWUPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMWUPE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    pub fn fadden(&self) -> FADDEN_R {
        FADDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    pub fn confg(&self) -> CONFG_R {
        CONFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    pub fn esr(&self) -> ESR_R {
        ESR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsminpr(&self) -> RSMINPR_R {
        RSMINPR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    pub fn rmwupe(&self) -> RMWUPE_R {
        RMWUPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    pub fn fadden(&mut self) -> FADDEN_W {
        FADDEN_W { w: self }
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    pub fn confg(&mut self) -> CONFG_W {
        CONFG_W { w: self }
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    pub fn esr(&mut self) -> ESR_W {
        ESR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsminpr(&mut self) -> RSMINPR_W {
        RSMINPR_W { w: self }
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    pub fn rmwupe(&mut self) -> RMWUPE_W {
        RMWUPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_stat](index.html) module"]
pub struct GLB_STAT_SPEC;
impl crate::RegisterSpec for GLB_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glb_stat::R](R) reader structure"]
impl crate::Readable for GLB_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glb_stat::W](W) writer structure"]
impl crate::Writable for GLB_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLB_STAT to value 0x10"]
impl crate::Resettable for GLB_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
