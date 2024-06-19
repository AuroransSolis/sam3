#[doc = "Register `PMC_PCER1` writer"]
pub type W = crate::W<PmcPcer1Spec>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Enable"]
pub type Pid32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Enable"]
pub type Pid33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Enable"]
pub type Pid34W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
#[doc = "Peripheral Clock Enable Register 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pcer1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
