#[doc = "Register `PMC_PCER1` writer"]
pub type W = crate::W<PmcPcer1Spec>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Enable"]
pub type Pid32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Enable"]
pub type Pid33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Enable"]
pub type Pid34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Enable"]
pub type Pid35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID36` writer - Peripheral Clock 36 Enable"]
pub type Pid36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Enable"]
pub type Pid37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID38` writer - Peripheral Clock 38 Enable"]
pub type Pid38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Enable"]
pub type Pid39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Enable"]
pub type Pid40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Enable"]
pub type Pid41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Enable"]
pub type Pid42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Enable"]
pub type Pid43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Enable"]
pub type Pid44W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid32(&mut self) -> Pid32W<PmcPcer1Spec> {
        Pid32W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid33(&mut self) -> Pid33W<PmcPcer1Spec> {
        Pid33W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid34(&mut self) -> Pid34W<PmcPcer1Spec> {
        Pid34W::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid35(&mut self) -> Pid35W<PmcPcer1Spec> {
        Pid35W::new(self, 3)
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid36(&mut self) -> Pid36W<PmcPcer1Spec> {
        Pid36W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid37(&mut self) -> Pid37W<PmcPcer1Spec> {
        Pid37W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid38(&mut self) -> Pid38W<PmcPcer1Spec> {
        Pid38W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid39(&mut self) -> Pid39W<PmcPcer1Spec> {
        Pid39W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid40(&mut self) -> Pid40W<PmcPcer1Spec> {
        Pid40W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid41(&mut self) -> Pid41W<PmcPcer1Spec> {
        Pid41W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid42(&mut self) -> Pid42W<PmcPcer1Spec> {
        Pid42W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid43(&mut self) -> Pid43W<PmcPcer1Spec> {
        Pid43W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid44(&mut self) -> Pid44W<PmcPcer1Spec> {
        Pid44W::new(self, 12)
    }
}
#[doc = "Peripheral Clock Enable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcer1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcer1Spec;
impl crate::RegisterSpec for PmcPcer1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcer1::W`](W) writer structure"]
impl crate::Writable for PmcPcer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
