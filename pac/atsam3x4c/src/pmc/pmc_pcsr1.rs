#[doc = "Register `PMC_PCSR1` reader"]
pub type R = crate::R<PmcPcsr1Spec>;
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub type Pid32R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub type Pid33R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub type Pid34R = crate::BitReader;
#[doc = "Field `PID35` reader - Peripheral Clock 35 Status"]
pub type Pid35R = crate::BitReader;
#[doc = "Field `PID36` reader - Peripheral Clock 36 Status"]
pub type Pid36R = crate::BitReader;
#[doc = "Field `PID37` reader - Peripheral Clock 37 Status"]
pub type Pid37R = crate::BitReader;
#[doc = "Field `PID38` reader - Peripheral Clock 38 Status"]
pub type Pid38R = crate::BitReader;
#[doc = "Field `PID39` reader - Peripheral Clock 39 Status"]
pub type Pid39R = crate::BitReader;
#[doc = "Field `PID40` reader - Peripheral Clock 40 Status"]
pub type Pid40R = crate::BitReader;
#[doc = "Field `PID41` reader - Peripheral Clock 41 Status"]
pub type Pid41R = crate::BitReader;
#[doc = "Field `PID42` reader - Peripheral Clock 42 Status"]
pub type Pid42R = crate::BitReader;
#[doc = "Field `PID43` reader - Peripheral Clock 43 Status"]
pub type Pid43R = crate::BitReader;
#[doc = "Field `PID44` reader - Peripheral Clock 44 Status"]
pub type Pid44R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> Pid32R {
        Pid32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> Pid33R {
        Pid33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> Pid34R {
        Pid34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Status"]
    #[inline(always)]
    pub fn pid35(&self) -> Pid35R {
        Pid35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Status"]
    #[inline(always)]
    pub fn pid36(&self) -> Pid36R {
        Pid36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Status"]
    #[inline(always)]
    pub fn pid37(&self) -> Pid37R {
        Pid37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Status"]
    #[inline(always)]
    pub fn pid38(&self) -> Pid38R {
        Pid38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Status"]
    #[inline(always)]
    pub fn pid39(&self) -> Pid39R {
        Pid39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Status"]
    #[inline(always)]
    pub fn pid40(&self) -> Pid40R {
        Pid40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Status"]
    #[inline(always)]
    pub fn pid41(&self) -> Pid41R {
        Pid41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Status"]
    #[inline(always)]
    pub fn pid42(&self) -> Pid42R {
        Pid42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Status"]
    #[inline(always)]
    pub fn pid43(&self) -> Pid43R {
        Pid43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Status"]
    #[inline(always)]
    pub fn pid44(&self) -> Pid44R {
        Pid44R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pcsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcsr1Spec;
impl crate::RegisterSpec for PmcPcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pcsr1::R`](R) reader structure"]
impl crate::Readable for PmcPcsr1Spec {}
#[doc = "`reset()` method sets PMC_PCSR1 to value 0"]
impl crate::Resettable for PmcPcsr1Spec {
    const RESET_VALUE: u32 = 0;
}
