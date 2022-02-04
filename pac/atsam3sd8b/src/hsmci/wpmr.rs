#[doc = "Register `WPMR` reader"]
pub struct R(crate::R<WPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPMR` writer"]
pub struct W(crate::W<WPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPMR_SPEC>;
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
impl From<crate::W<WPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP_EN` reader - Write Protection Enable"]
pub struct WP_EN_R(crate::FieldReader<bool, bool>);
impl WP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP_EN` writer - Write Protection Enable"]
pub struct WP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_EN_W<'a> {
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
#[doc = "Field `WP_KEY` reader - Write Protection Key password"]
pub struct WP_KEY_R(crate::FieldReader<u32, u32>);
impl WP_KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WP_KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_KEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP_KEY` writer - Write Protection Key password"]
pub struct WP_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&self) -> WP_EN_R {
        WP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key password"]
    #[inline(always)]
    pub fn wp_key(&self) -> WP_KEY_R {
        WP_KEY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&mut self) -> WP_EN_W {
        WP_EN_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection Key password"]
    #[inline(always)]
    pub fn wp_key(&mut self) -> WP_KEY_W {
        WP_KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](index.html) module"]
pub struct WPMR_SPEC;
impl crate::RegisterSpec for WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpmr::R](R) reader structure"]
impl crate::Readable for WPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpmr::W](W) writer structure"]
impl crate::Writable for WPMR_SPEC {
    type Writer = W;
}
