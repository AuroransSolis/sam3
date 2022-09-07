#[doc = "Register `ECC_CTRL` writer"]
pub struct W(crate::W<ECC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_CTRL_SPEC>;
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
impl From<crate::W<ECC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST` writer - Reset ECC"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECC_CTRL_SPEC, bool, O>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECC_CTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Reset ECC"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<0> {
        RST_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<1> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC ECC Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_ctrl](index.html) module"]
pub struct ECC_CTRL_SPEC;
impl crate::RegisterSpec for ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ecc_ctrl::W](W) writer structure"]
impl crate::Writable for ECC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0"]
impl crate::Resettable for ECC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
