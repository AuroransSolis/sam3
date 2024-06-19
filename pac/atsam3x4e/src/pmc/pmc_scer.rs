#[doc = "Register `PMC_SCER` writer"]
pub type W = crate::W<PmcScerSpec>;
#[doc = "Field `UOTGCLK` writer - Enable USB OTG Clock (48 MHz, USB_48M) for UTMI"]
pub type UotgclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub type Pck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub type Pck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Enable"]
pub type Pck2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Enable USB OTG Clock (48 MHz, USB_48M) for UTMI"]
    #[inline(always)]
    #[must_use]
    pub fn uotgclk(&mut self) -> UotgclkW<PmcScerSpec> {
        UotgclkW::new(self, 5)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck0(&mut self) -> Pck0W<PmcScerSpec> {
        Pck0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck1(&mut self) -> Pck1W<PmcScerSpec> {
        Pck1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck2(&mut self) -> Pck2W<PmcScerSpec> {
        Pck2W::new(self, 10)
    }
}
#[doc = "System Clock Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_scer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcScerSpec;
impl crate::RegisterSpec for PmcScerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_scer::W`](W) writer structure"]
impl crate::Writable for PmcScerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
