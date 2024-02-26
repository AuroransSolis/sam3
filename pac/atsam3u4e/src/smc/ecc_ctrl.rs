#[doc = "Register `ECC_CTRL` writer"]
pub type W = crate::W<EccCtrlSpec>;
#[doc = "Field `RST` writer - Reset ECC"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset ECC"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<EccCtrlSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<EccCtrlSpec> {
        SwrstW::new(self, 1)
    }
}
#[doc = "SMC ECC Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccCtrlSpec;
impl crate::RegisterSpec for EccCtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for EccCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0"]
impl crate::Resettable for EccCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
