#[doc = "Register `PMC_PCER0` writer"]
pub type W = crate::W<PmcPcer0Spec>;
#[doc = "Field `PID8` writer - Peripheral Clock 8 Enable"]
pub type Pid8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID9` writer - Peripheral Clock 9 Enable"]
pub type Pid9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID10` writer - Peripheral Clock 10 Enable"]
pub type Pid10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID11` writer - Peripheral Clock 11 Enable"]
pub type Pid11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID12` writer - Peripheral Clock 12 Enable"]
pub type Pid12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID13` writer - Peripheral Clock 13 Enable"]
pub type Pid13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID14` writer - Peripheral Clock 14 Enable"]
pub type Pid14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID15` writer - Peripheral Clock 15 Enable"]
pub type Pid15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID16` writer - Peripheral Clock 16 Enable"]
pub type Pid16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID17` writer - Peripheral Clock 17 Enable"]
pub type Pid17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID18` writer - Peripheral Clock 18 Enable"]
pub type Pid18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID19` writer - Peripheral Clock 19 Enable"]
pub type Pid19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID20` writer - Peripheral Clock 20 Enable"]
pub type Pid20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID21` writer - Peripheral Clock 21 Enable"]
pub type Pid21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID22` writer - Peripheral Clock 22 Enable"]
pub type Pid22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID23` writer - Peripheral Clock 23 Enable"]
pub type Pid23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID24` writer - Peripheral Clock 24 Enable"]
pub type Pid24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID25` writer - Peripheral Clock 25 Enable"]
pub type Pid25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID26` writer - Peripheral Clock 26 Enable"]
pub type Pid26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID27` writer - Peripheral Clock 27 Enable"]
pub type Pid27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID28` writer - Peripheral Clock 28 Enable"]
pub type Pid28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID29` writer - Peripheral Clock 29 Enable"]
pub type Pid29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID30` writer - Peripheral Clock 30 Enable"]
pub type Pid30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID31` writer - Peripheral Clock 31 Enable"]
pub type Pid31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Peripheral Clock 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid8(&mut self) -> Pid8W<PmcPcer0Spec> {
        Pid8W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid9(&mut self) -> Pid9W<PmcPcer0Spec> {
        Pid9W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid10(&mut self) -> Pid10W<PmcPcer0Spec> {
        Pid10W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid11(&mut self) -> Pid11W<PmcPcer0Spec> {
        Pid11W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 12 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid12(&mut self) -> Pid12W<PmcPcer0Spec> {
        Pid12W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid13(&mut self) -> Pid13W<PmcPcer0Spec> {
        Pid13W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid14(&mut self) -> Pid14W<PmcPcer0Spec> {
        Pid14W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid15(&mut self) -> Pid15W<PmcPcer0Spec> {
        Pid15W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid16(&mut self) -> Pid16W<PmcPcer0Spec> {
        Pid16W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral Clock 17 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid17(&mut self) -> Pid17W<PmcPcer0Spec> {
        Pid17W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid18(&mut self) -> Pid18W<PmcPcer0Spec> {
        Pid18W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid19(&mut self) -> Pid19W<PmcPcer0Spec> {
        Pid19W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid20(&mut self) -> Pid20W<PmcPcer0Spec> {
        Pid20W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid21(&mut self) -> Pid21W<PmcPcer0Spec> {
        Pid21W::new(self, 21)
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid22(&mut self) -> Pid22W<PmcPcer0Spec> {
        Pid22W::new(self, 22)
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid23(&mut self) -> Pid23W<PmcPcer0Spec> {
        Pid23W::new(self, 23)
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid24(&mut self) -> Pid24W<PmcPcer0Spec> {
        Pid24W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid25(&mut self) -> Pid25W<PmcPcer0Spec> {
        Pid25W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid26(&mut self) -> Pid26W<PmcPcer0Spec> {
        Pid26W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid27(&mut self) -> Pid27W<PmcPcer0Spec> {
        Pid27W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid28(&mut self) -> Pid28W<PmcPcer0Spec> {
        Pid28W::new(self, 28)
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid29(&mut self) -> Pid29W<PmcPcer0Spec> {
        Pid29W::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid30(&mut self) -> Pid30W<PmcPcer0Spec> {
        Pid30W::new(self, 30)
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid31(&mut self) -> Pid31W<PmcPcer0Spec> {
        Pid31W::new(self, 31)
    }
}
#[doc = "Peripheral Clock Enable Register 0\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pcer0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcer0Spec;
impl crate::RegisterSpec for PmcPcer0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcer0::W`](W) writer structure"]
impl crate::Writable for PmcPcer0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
