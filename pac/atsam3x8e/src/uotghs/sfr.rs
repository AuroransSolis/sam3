#[doc = "Register `SFR` writer"]
pub type W = crate::W<SfrSpec>;
#[doc = "Field `IDTIS` writer - ID Transition Interrupt Set"]
pub type IdtisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSTIS` writer - VBus Transition Interrupt Set"]
pub type VbustisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPIS` writer - SRP Interrupt Set"]
pub type SrpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBERRIS` writer - VBus Error Interrupt Set"]
pub type VberrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCERRIS` writer - B-Connection Error Interrupt Set"]
pub type BcerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROLEEXIS` writer - Role Exchange Interrupt Set"]
pub type RoleexisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPERRIS` writer - HNP Error Interrupt Set"]
pub type HnperrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOIS` writer - Suspend Time-Out Interrupt Set"]
pub type StoisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSRQS` writer - VBus Request Set"]
pub type VbusrqsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn idtis(&mut self) -> IdtisW<SfrSpec> {
        IdtisW::new(self, 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbustis(&mut self) -> VbustisW<SfrSpec> {
        VbustisW::new(self, 1)
    }
    #[doc = "Bit 2 - SRP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn srpis(&mut self) -> SrpisW<SfrSpec> {
        SrpisW::new(self, 2)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn vberris(&mut self) -> VberrisW<SfrSpec> {
        VberrisW::new(self, 3)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn bcerris(&mut self) -> BcerrisW<SfrSpec> {
        BcerrisW::new(self, 4)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn roleexis(&mut self) -> RoleexisW<SfrSpec> {
        RoleexisW::new(self, 5)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hnperris(&mut self) -> HnperrisW<SfrSpec> {
        HnperrisW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn stois(&mut self) -> StoisW<SfrSpec> {
        StoisW::new(self, 7)
    }
    #[doc = "Bit 9 - VBus Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqs(&mut self) -> VbusrqsW<SfrSpec> {
        VbusrqsW::new(self, 9)
    }
}
#[doc = "General Status Set Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSpec;
impl crate::RegisterSpec for SfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
