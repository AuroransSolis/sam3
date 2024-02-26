#[doc = "Register `PMC_PCSR0` reader"]
pub type R = crate::R<PmcPcsr0Spec>;
#[doc = "Field `PID8` reader - Peripheral Clock 8 Status"]
pub type Pid8R = crate::BitReader;
#[doc = "Field `PID9` reader - Peripheral Clock 9 Status"]
pub type Pid9R = crate::BitReader;
#[doc = "Field `PID10` reader - Peripheral Clock 10 Status"]
pub type Pid10R = crate::BitReader;
#[doc = "Field `PID11` reader - Peripheral Clock 11 Status"]
pub type Pid11R = crate::BitReader;
#[doc = "Field `PID12` reader - Peripheral Clock 12 Status"]
pub type Pid12R = crate::BitReader;
#[doc = "Field `PID13` reader - Peripheral Clock 13 Status"]
pub type Pid13R = crate::BitReader;
#[doc = "Field `PID14` reader - Peripheral Clock 14 Status"]
pub type Pid14R = crate::BitReader;
#[doc = "Field `PID15` reader - Peripheral Clock 15 Status"]
pub type Pid15R = crate::BitReader;
#[doc = "Field `PID16` reader - Peripheral Clock 16 Status"]
pub type Pid16R = crate::BitReader;
#[doc = "Field `PID17` reader - Peripheral Clock 17 Status"]
pub type Pid17R = crate::BitReader;
#[doc = "Field `PID18` reader - Peripheral Clock 18 Status"]
pub type Pid18R = crate::BitReader;
#[doc = "Field `PID19` reader - Peripheral Clock 19 Status"]
pub type Pid19R = crate::BitReader;
#[doc = "Field `PID20` reader - Peripheral Clock 20 Status"]
pub type Pid20R = crate::BitReader;
#[doc = "Field `PID21` reader - Peripheral Clock 21 Status"]
pub type Pid21R = crate::BitReader;
#[doc = "Field `PID22` reader - Peripheral Clock 22 Status"]
pub type Pid22R = crate::BitReader;
#[doc = "Field `PID23` reader - Peripheral Clock 23 Status"]
pub type Pid23R = crate::BitReader;
#[doc = "Field `PID24` reader - Peripheral Clock 24 Status"]
pub type Pid24R = crate::BitReader;
#[doc = "Field `PID25` reader - Peripheral Clock 25 Status"]
pub type Pid25R = crate::BitReader;
#[doc = "Field `PID26` reader - Peripheral Clock 26 Status"]
pub type Pid26R = crate::BitReader;
#[doc = "Field `PID27` reader - Peripheral Clock 27 Status"]
pub type Pid27R = crate::BitReader;
#[doc = "Field `PID28` reader - Peripheral Clock 28 Status"]
pub type Pid28R = crate::BitReader;
#[doc = "Field `PID29` reader - Peripheral Clock 29 Status"]
pub type Pid29R = crate::BitReader;
#[doc = "Field `PID30` reader - Peripheral Clock 30 Status"]
pub type Pid30R = crate::BitReader;
#[doc = "Field `PID31` reader - Peripheral Clock 31 Status"]
pub type Pid31R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Peripheral Clock 8 Status"]
    #[inline(always)]
    pub fn pid8(&self) -> Pid8R {
        Pid8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 9 Status"]
    #[inline(always)]
    pub fn pid9(&self) -> Pid9R {
        Pid9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Status"]
    #[inline(always)]
    pub fn pid10(&self) -> Pid10R {
        Pid10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Status"]
    #[inline(always)]
    pub fn pid11(&self) -> Pid11R {
        Pid11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 12 Status"]
    #[inline(always)]
    pub fn pid12(&self) -> Pid12R {
        Pid12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Status"]
    #[inline(always)]
    pub fn pid13(&self) -> Pid13R {
        Pid13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Status"]
    #[inline(always)]
    pub fn pid14(&self) -> Pid14R {
        Pid14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Status"]
    #[inline(always)]
    pub fn pid15(&self) -> Pid15R {
        Pid15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Status"]
    #[inline(always)]
    pub fn pid16(&self) -> Pid16R {
        Pid16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 17 Status"]
    #[inline(always)]
    pub fn pid17(&self) -> Pid17R {
        Pid17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Status"]
    #[inline(always)]
    pub fn pid18(&self) -> Pid18R {
        Pid18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Status"]
    #[inline(always)]
    pub fn pid19(&self) -> Pid19R {
        Pid19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Status"]
    #[inline(always)]
    pub fn pid20(&self) -> Pid20R {
        Pid20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Status"]
    #[inline(always)]
    pub fn pid21(&self) -> Pid21R {
        Pid21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Status"]
    #[inline(always)]
    pub fn pid22(&self) -> Pid22R {
        Pid22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Status"]
    #[inline(always)]
    pub fn pid23(&self) -> Pid23R {
        Pid23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Status"]
    #[inline(always)]
    pub fn pid24(&self) -> Pid24R {
        Pid24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Status"]
    #[inline(always)]
    pub fn pid25(&self) -> Pid25R {
        Pid25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Status"]
    #[inline(always)]
    pub fn pid26(&self) -> Pid26R {
        Pid26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Status"]
    #[inline(always)]
    pub fn pid27(&self) -> Pid27R {
        Pid27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Status"]
    #[inline(always)]
    pub fn pid28(&self) -> Pid28R {
        Pid28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Status"]
    #[inline(always)]
    pub fn pid29(&self) -> Pid29R {
        Pid29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Status"]
    #[inline(always)]
    pub fn pid30(&self) -> Pid30R {
        Pid30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Status"]
    #[inline(always)]
    pub fn pid31(&self) -> Pid31R {
        Pid31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pcsr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcsr0Spec;
impl crate::RegisterSpec for PmcPcsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pcsr0::R`](R) reader structure"]
impl crate::Readable for PmcPcsr0Spec {}
#[doc = "`reset()` method sets PMC_PCSR0 to value 0"]
impl crate::Resettable for PmcPcsr0Spec {
    const RESET_VALUE: u32 = 0;
}
