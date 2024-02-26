#[doc = "Register `SR0` reader"]
pub type R = crate::R<Sr0Spec>;
#[doc = "Field `COVFS` reader - Counter Overflow Status"]
pub type CovfsR = crate::BitReader;
#[doc = "Field `LOVRS` reader - Load Overrun Status"]
pub type LovrsR = crate::BitReader;
#[doc = "Field `CPAS` reader - RA Compare Status"]
pub type CpasR = crate::BitReader;
#[doc = "Field `CPBS` reader - RB Compare Status"]
pub type CpbsR = crate::BitReader;
#[doc = "Field `CPCS` reader - RC Compare Status"]
pub type CpcsR = crate::BitReader;
#[doc = "Field `LDRAS` reader - RA Loading Status"]
pub type LdrasR = crate::BitReader;
#[doc = "Field `LDRBS` reader - RB Loading Status"]
pub type LdrbsR = crate::BitReader;
#[doc = "Field `ETRGS` reader - External Trigger Status"]
pub type EtrgsR = crate::BitReader;
#[doc = "Field `CLKSTA` reader - Clock Enabling Status"]
pub type ClkstaR = crate::BitReader;
#[doc = "Field `MTIOA` reader - TIOA Mirror"]
pub type MtioaR = crate::BitReader;
#[doc = "Field `MTIOB` reader - TIOB Mirror"]
pub type MtiobR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter Overflow Status"]
    #[inline(always)]
    pub fn covfs(&self) -> CovfsR {
        CovfsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status"]
    #[inline(always)]
    pub fn lovrs(&self) -> LovrsR {
        LovrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status"]
    #[inline(always)]
    pub fn cpas(&self) -> CpasR {
        CpasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status"]
    #[inline(always)]
    pub fn cpbs(&self) -> CpbsR {
        CpbsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status"]
    #[inline(always)]
    pub fn cpcs(&self) -> CpcsR {
        CpcsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status"]
    #[inline(always)]
    pub fn ldras(&self) -> LdrasR {
        LdrasR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LdrbsR {
        LdrbsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status"]
    #[inline(always)]
    pub fn etrgs(&self) -> EtrgsR {
        EtrgsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> ClkstaR {
        ClkstaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIOA Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MtioaR {
        MtioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIOB Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MtiobR {
        MtiobR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Status Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr0Spec;
impl crate::RegisterSpec for Sr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr0::R`](R) reader structure"]
impl crate::Readable for Sr0Spec {}
#[doc = "`reset()` method sets SR0 to value 0"]
impl crate::Resettable for Sr0Spec {
    const RESET_VALUE: u32 = 0;
}
