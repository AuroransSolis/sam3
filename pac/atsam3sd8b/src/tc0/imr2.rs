#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2_SPEC>;
#[doc = "Field `COVFS` reader - Counter Overflow"]
pub type COVFS_R = crate::BitReader;
#[doc = "Field `LOVRS` reader - Load Overrun"]
pub type LOVRS_R = crate::BitReader;
#[doc = "Field `CPAS` reader - RA Compare"]
pub type CPAS_R = crate::BitReader;
#[doc = "Field `CPBS` reader - RB Compare"]
pub type CPBS_R = crate::BitReader;
#[doc = "Field `CPCS` reader - RC Compare"]
pub type CPCS_R = crate::BitReader;
#[doc = "Field `LDRAS` reader - RA Loading"]
pub type LDRAS_R = crate::BitReader;
#[doc = "Field `LDRBS` reader - RB Loading"]
pub type LDRBS_R = crate::BitReader;
#[doc = "Field `ETRGS` reader - External Trigger"]
pub type ETRGS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2_SPEC {}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
