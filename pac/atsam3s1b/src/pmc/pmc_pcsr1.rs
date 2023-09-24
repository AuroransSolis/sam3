#[doc = "Register `PMC_PCSR1` reader"]
pub type R = crate::R<PMC_PCSR1_SPEC>;
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub type PID32_R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub type PID33_R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub type PID34_R = crate::BitReader;
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
