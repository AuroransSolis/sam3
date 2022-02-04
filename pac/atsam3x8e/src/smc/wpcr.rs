#[doc = "Register `WPCR` writer"]
pub struct W(crate::W<WPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR_SPEC>;
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
impl From<crate::W<WPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Write Protection KEY Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WP_KEY_AW {
    #[doc = "5459267: Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    PASSWD = 5459267,
}
impl From<WP_KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: WP_KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WP_KEY` writer - Write Protection KEY Password"]
pub struct WP_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP_KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WP_KEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&mut self) -> WP_EN_W {
        WP_EN_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection KEY Password"]
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
#[doc = "Write Protection Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](index.html) module"]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wpcr::W](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR to value 0"]
impl crate::Resettable for WPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
