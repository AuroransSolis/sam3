#[doc = "Register `PMC_PCSR1` reader"]
pub type R = crate::R<PmcPcsr1Spec>;
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub type Pid32R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub type Pid33R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub type Pid34R = crate::BitReader;
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
}
#[doc = "Peripheral Clock Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pcsr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
