#[doc = "Register `IMR1` reader"]
pub type R = crate::R<Imr1Spec>;
#[doc = "Field `COVFS` reader - Counter Overflow"]
pub type CovfsR = crate::BitReader;
#[doc = "Field `LOVRS` reader - Load Overrun"]
pub type LovrsR = crate::BitReader;
#[doc = "Field `CPAS` reader - RA Compare"]
pub type CpasR = crate::BitReader;
#[doc = "Field `CPBS` reader - RB Compare"]
pub type CpbsR = crate::BitReader;
#[doc = "Field `CPCS` reader - RC Compare"]
pub type CpcsR = crate::BitReader;
#[doc = "Field `LDRAS` reader - RA Loading"]
pub type LdrasR = crate::BitReader;
#[doc = "Field `LDRBS` reader - RB Loading"]
pub type LdrbsR = crate::BitReader;
#[doc = "Field `ETRGS` reader - External Trigger"]
pub type EtrgsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> CovfsR {
        CovfsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LovrsR {
        LovrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CpasR {
        CpasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CpbsR {
        CpbsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CpcsR {
        CpcsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LdrasR {
        LdrasR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LdrbsR {
        LdrbsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> EtrgsR {
        EtrgsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`imr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr1Spec;
impl crate::RegisterSpec for Imr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr1::R`](R) reader structure"]
impl crate::Readable for Imr1Spec {}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for Imr1Spec {
    const RESET_VALUE: u32 = 0;
}
