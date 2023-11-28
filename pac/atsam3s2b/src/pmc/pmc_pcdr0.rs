#[doc = "Register `PMC_PCDR0` writer"]
pub type W = crate::W<PMC_PCDR0_SPEC>;
#[doc = "Field `PID8` writer - Peripheral Clock 8 Disable"]
pub type PID8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID9` writer - Peripheral Clock 9 Disable"]
pub type PID9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID10` writer - Peripheral Clock 10 Disable"]
pub type PID10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID11` writer - Peripheral Clock 11 Disable"]
pub type PID11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID12` writer - Peripheral Clock 12 Disable"]
pub type PID12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID13` writer - Peripheral Clock 13 Disable"]
pub type PID13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID14` writer - Peripheral Clock 14 Disable"]
pub type PID14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID15` writer - Peripheral Clock 15 Disable"]
pub type PID15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID18` writer - Peripheral Clock 18 Disable"]
pub type PID18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID19` writer - Peripheral Clock 19 Disable"]
pub type PID19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID20` writer - Peripheral Clock 20 Disable"]
pub type PID20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID21` writer - Peripheral Clock 21 Disable"]
pub type PID21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID22` writer - Peripheral Clock 22 Disable"]
pub type PID22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID23` writer - Peripheral Clock 23 Disable"]
pub type PID23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID24` writer - Peripheral Clock 24 Disable"]
pub type PID24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID25` writer - Peripheral Clock 25 Disable"]
pub type PID25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID26` writer - Peripheral Clock 26 Disable"]
pub type PID26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID27` writer - Peripheral Clock 27 Disable"]
pub type PID27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID28` writer - Peripheral Clock 28 Disable"]
pub type PID28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID29` writer - Peripheral Clock 29 Disable"]
pub type PID29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID30` writer - Peripheral Clock 30 Disable"]
pub type PID30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID31` writer - Peripheral Clock 31 Disable"]
pub type PID31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Peripheral Clock 8 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid8(&mut self) -> PID8_W<PMC_PCDR0_SPEC> {
        PID8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 9 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid9(&mut self) -> PID9_W<PMC_PCDR0_SPEC> {
        PID9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid10(&mut self) -> PID10_W<PMC_PCDR0_SPEC> {
        PID10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid11(&mut self) -> PID11_W<PMC_PCDR0_SPEC> {
        PID11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 12 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid12(&mut self) -> PID12_W<PMC_PCDR0_SPEC> {
        PID12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid13(&mut self) -> PID13_W<PMC_PCDR0_SPEC> {
        PID13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid14(&mut self) -> PID14_W<PMC_PCDR0_SPEC> {
        PID14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid15(&mut self) -> PID15_W<PMC_PCDR0_SPEC> {
        PID15_W::new(self, 15)
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid18(&mut self) -> PID18_W<PMC_PCDR0_SPEC> {
        PID18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid19(&mut self) -> PID19_W<PMC_PCDR0_SPEC> {
        PID19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid20(&mut self) -> PID20_W<PMC_PCDR0_SPEC> {
        PID20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid21(&mut self) -> PID21_W<PMC_PCDR0_SPEC> {
        PID21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid22(&mut self) -> PID22_W<PMC_PCDR0_SPEC> {
        PID22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid23(&mut self) -> PID23_W<PMC_PCDR0_SPEC> {
        PID23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid24(&mut self) -> PID24_W<PMC_PCDR0_SPEC> {
        PID24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid25(&mut self) -> PID25_W<PMC_PCDR0_SPEC> {
        PID25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid26(&mut self) -> PID26_W<PMC_PCDR0_SPEC> {
        PID26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid27(&mut self) -> PID27_W<PMC_PCDR0_SPEC> {
        PID27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid28(&mut self) -> PID28_W<PMC_PCDR0_SPEC> {
        PID28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid29(&mut self) -> PID29_W<PMC_PCDR0_SPEC> {
        PID29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid30(&mut self) -> PID30_W<PMC_PCDR0_SPEC> {
        PID30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid31(&mut self) -> PID31_W<PMC_PCDR0_SPEC> {
        PID31_W::new(self, 31)
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
#[doc = "Peripheral Clock Disable Register 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcdr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCDR0_SPEC;
impl crate::RegisterSpec for PMC_PCDR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcdr0::W`](W) writer structure"]
impl crate::Writable for PMC_PCDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
