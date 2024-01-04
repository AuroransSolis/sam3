#[doc = "Register `PMC_PCER1` writer"]
pub type W = crate::W<PMC_PCER1_SPEC>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Enable"]
pub type PID32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Enable"]
pub type PID33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Enable"]
pub type PID34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Enable"]
pub type PID35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID36` writer - Peripheral Clock 36 Enable"]
pub type PID36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Enable"]
pub type PID37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID38` writer - Peripheral Clock 38 Enable"]
pub type PID38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Enable"]
pub type PID39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Enable"]
pub type PID40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Enable"]
pub type PID41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Enable"]
pub type PID42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Enable"]
pub type PID43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Enable"]
pub type PID44_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid32(&mut self) -> PID32_W<PMC_PCER1_SPEC> {
        PID32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid33(&mut self) -> PID33_W<PMC_PCER1_SPEC> {
        PID33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid34(&mut self) -> PID34_W<PMC_PCER1_SPEC> {
        PID34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid35(&mut self) -> PID35_W<PMC_PCER1_SPEC> {
        PID35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid36(&mut self) -> PID36_W<PMC_PCER1_SPEC> {
        PID36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid37(&mut self) -> PID37_W<PMC_PCER1_SPEC> {
        PID37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid38(&mut self) -> PID38_W<PMC_PCER1_SPEC> {
        PID38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid39(&mut self) -> PID39_W<PMC_PCER1_SPEC> {
        PID39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid40(&mut self) -> PID40_W<PMC_PCER1_SPEC> {
        PID40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid41(&mut self) -> PID41_W<PMC_PCER1_SPEC> {
        PID41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid42(&mut self) -> PID42_W<PMC_PCER1_SPEC> {
        PID42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid43(&mut self) -> PID43_W<PMC_PCER1_SPEC> {
        PID43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid44(&mut self) -> PID44_W<PMC_PCER1_SPEC> {
        PID44_W::new(self, 12)
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
#[doc = "Peripheral Clock Enable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcer1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCER1_SPEC;
impl crate::RegisterSpec for PMC_PCER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcer1::W`](W) writer structure"]
impl crate::Writable for PMC_PCER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
