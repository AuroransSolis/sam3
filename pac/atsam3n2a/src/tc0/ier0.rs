#[doc = "Register `IER0` writer"]
pub type W = crate::W<Ier0Spec>;
#[doc = "Field `COVFS` writer - Counter Overflow"]
pub type CovfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOVRS` writer - Load Overrun"]
pub type LovrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPAS` writer - RA Compare"]
pub type CpasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPBS` writer - RB Compare"]
pub type CpbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCS` writer - RC Compare"]
pub type CpcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDRAS` writer - RA Loading"]
pub type LdrasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDRBS` writer - RB Loading"]
pub type LdrbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRGS` writer - External Trigger"]
pub type EtrgsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn covfs(&mut self) -> CovfsW<Ier0Spec> {
        CovfsW::new(self, 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn lovrs(&mut self) -> LovrsW<Ier0Spec> {
        LovrsW::new(self, 1)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpas(&mut self) -> CpasW<Ier0Spec> {
        CpasW::new(self, 2)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpbs(&mut self) -> CpbsW<Ier0Spec> {
        CpbsW::new(self, 3)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcs(&mut self) -> CpcsW<Ier0Spec> {
        CpcsW::new(self, 4)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldras(&mut self) -> LdrasW<Ier0Spec> {
        LdrasW::new(self, 5)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldrbs(&mut self) -> LdrbsW<Ier0Spec> {
        LdrbsW::new(self, 6)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn etrgs(&mut self) -> EtrgsW<Ier0Spec> {
        EtrgsW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier0Spec;
impl crate::RegisterSpec for Ier0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier0::W`](W) writer structure"]
impl crate::Writable for Ier0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
