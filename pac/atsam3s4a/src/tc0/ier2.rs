#[doc = "Register `IER2` writer"]
pub type W = crate::W<IER2_SPEC>;
#[doc = "Field `COVFS` writer - Counter Overflow"]
pub type COVFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOVRS` writer - Load Overrun"]
pub type LOVRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPAS` writer - RA Compare"]
pub type CPAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPBS` writer - RB Compare"]
pub type CPBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCS` writer - RC Compare"]
pub type CPCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDRAS` writer - RA Loading"]
pub type LDRAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDRBS` writer - RB Loading"]
pub type LDRBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRGS` writer - External Trigger"]
pub type ETRGS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn covfs(&mut self) -> COVFS_W<IER2_SPEC> {
        COVFS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn lovrs(&mut self) -> LOVRS_W<IER2_SPEC> {
        LOVRS_W::new(self, 1)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpas(&mut self) -> CPAS_W<IER2_SPEC> {
        CPAS_W::new(self, 2)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpbs(&mut self) -> CPBS_W<IER2_SPEC> {
        CPBS_W::new(self, 3)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcs(&mut self) -> CPCS_W<IER2_SPEC> {
        CPCS_W::new(self, 4)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldras(&mut self) -> LDRAS_W<IER2_SPEC> {
        LDRAS_W::new(self, 5)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldrbs(&mut self) -> LDRBS_W<IER2_SPEC> {
        LDRBS_W::new(self, 6)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn etrgs(&mut self) -> ETRGS_W<IER2_SPEC> {
        ETRGS_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER2_SPEC;
impl crate::RegisterSpec for IER2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier2::W`](W) writer structure"]
impl crate::Writable for IER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
