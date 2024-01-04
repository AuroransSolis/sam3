#[doc = "Register `WPCR` writer"]
pub type W = crate::W<WPCR_SPEC>;
#[doc = "Field `WP_EN` writer - Write Protection Enable"]
pub type WP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection KEY Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WP_KEY_AW {
    #[doc = "5459267: Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    Passwd = 5459267,
}
impl From<WP_KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: WP_KEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WP_KEY_AW {
    type Ux = u32;
}
#[doc = "Field `WP_KEY` writer - Write Protection KEY Password"]
pub type WP_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 24, WP_KEY_AW>;
impl<'a, REG> WP_KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(WP_KEY_AW::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wp_en(&mut self) -> WP_EN_W<WPCR_SPEC> {
        WP_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection KEY Password"]
    #[inline(always)]
    #[must_use]
    pub fn wp_key(&mut self) -> WP_KEY_W<WPCR_SPEC> {
        WP_KEY_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Write Protection Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpcr::W`](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR to value 0"]
impl crate::Resettable for WPCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
