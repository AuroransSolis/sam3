#[doc = "Register `ECC_CTRL` writer"]
pub type W = crate::W<ECC_CTRL_SPEC>;
#[doc = "Field `RST` writer - Reset ECC"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset ECC"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<ECC_CTRL_SPEC> {
        RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<ECC_CTRL_SPEC> {
        SWRST_W::new(self, 1)
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
#[doc = "SMC ECC Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_CTRL_SPEC;
impl crate::RegisterSpec for ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for ECC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0"]
impl crate::Resettable for ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
