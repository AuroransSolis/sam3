#[doc = "Register `PMC_PCSR1` reader"]
pub type R = crate::R<PMC_PCSR1_SPEC>;
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub type PID32_R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub type PID33_R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub type PID34_R = crate::BitReader;
#[doc = "Field `PID35` reader - Peripheral Clock 35 Status"]
pub type PID35_R = crate::BitReader;
#[doc = "Field `PID36` reader - Peripheral Clock 36 Status"]
pub type PID36_R = crate::BitReader;
#[doc = "Field `PID37` reader - Peripheral Clock 37 Status"]
pub type PID37_R = crate::BitReader;
#[doc = "Field `PID38` reader - Peripheral Clock 38 Status"]
pub type PID38_R = crate::BitReader;
#[doc = "Field `PID39` reader - Peripheral Clock 39 Status"]
pub type PID39_R = crate::BitReader;
#[doc = "Field `PID40` reader - Peripheral Clock 40 Status"]
pub type PID40_R = crate::BitReader;
#[doc = "Field `PID41` reader - Peripheral Clock 41 Status"]
pub type PID41_R = crate::BitReader;
#[doc = "Field `PID42` reader - Peripheral Clock 42 Status"]
pub type PID42_R = crate::BitReader;
#[doc = "Field `PID43` reader - Peripheral Clock 43 Status"]
pub type PID43_R = crate::BitReader;
#[doc = "Field `PID44` reader - Peripheral Clock 44 Status"]
pub type PID44_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Status"]
    #[inline(always)]
    pub fn pid35(&self) -> PID35_R {
        PID35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Status"]
    #[inline(always)]
    pub fn pid36(&self) -> PID36_R {
        PID36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Status"]
    #[inline(always)]
    pub fn pid37(&self) -> PID37_R {
        PID37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Status"]
    #[inline(always)]
    pub fn pid38(&self) -> PID38_R {
        PID38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Status"]
    #[inline(always)]
    pub fn pid39(&self) -> PID39_R {
        PID39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Status"]
    #[inline(always)]
    pub fn pid40(&self) -> PID40_R {
        PID40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Status"]
    #[inline(always)]
    pub fn pid41(&self) -> PID41_R {
        PID41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Status"]
    #[inline(always)]
    pub fn pid42(&self) -> PID42_R {
        PID42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Status"]
    #[inline(always)]
    pub fn pid43(&self) -> PID43_R {
        PID43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Status"]
    #[inline(always)]
    pub fn pid44(&self) -> PID44_R {
        PID44_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pcsr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCSR1_SPEC;
impl crate::RegisterSpec for PMC_PCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pcsr1::R`](R) reader structure"]
impl crate::Readable for PMC_PCSR1_SPEC {}
#[doc = "`reset()` method sets PMC_PCSR1 to value 0"]
impl crate::Resettable for PMC_PCSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
