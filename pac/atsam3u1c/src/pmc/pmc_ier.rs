#[doc = "Register `PMC_IER` writer"]
pub type W = crate::W<PmcIerSpec>;
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Enable"]
pub type MoscxtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Enable"]
pub type LockaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Enable"]
pub type MckrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKU` writer - UTMI PLL Lock Interrupt Enable"]
pub type LockuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Enable"]
pub type Pckrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Enable"]
pub type Pckrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Enable"]
pub type Pckrdy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCSELS` writer - Main Oscillator Selection Status Interrupt Enable"]
pub type MoscselsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCS` writer - Main On-Chip RC Status Interrupt Enable"]
pub type MoscrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Enable"]
pub type CfdevW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MoscxtsW<PmcIerSpec> {
        MoscxtsW::new(self, 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LockaW<PmcIerSpec> {
        LockaW::new(self, 1)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MckrdyW<PmcIerSpec> {
        MckrdyW::new(self, 3)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locku(&mut self) -> LockuW<PmcIerSpec> {
        LockuW::new(self, 6)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> Pckrdy0W<PmcIerSpec> {
        Pckrdy0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> Pckrdy1W<PmcIerSpec> {
        Pckrdy1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> Pckrdy2W<PmcIerSpec> {
        Pckrdy2W::new(self, 10)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MoscselsW<PmcIerSpec> {
        MoscselsW::new(self, 16)
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MoscrcsW<PmcIerSpec> {
        MoscrcsW::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CfdevW<PmcIerSpec> {
        CfdevW::new(self, 18)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcIerSpec;
impl crate::RegisterSpec for PmcIerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_ier::W`](W) writer structure"]
impl crate::Writable for PmcIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
