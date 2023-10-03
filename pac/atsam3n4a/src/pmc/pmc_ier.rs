#[doc = "Register `PMC_IER` writer"]
pub type W = crate::W<PMC_IER_SPEC>;
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Enable"]
pub type MOSCXTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Enable"]
pub type LOCKA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Enable"]
pub type MCKRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Enable"]
pub type PCKRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Enable"]
pub type PCKRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Enable"]
pub type PCKRDY2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MOSCSELS` writer - Main Oscillator Selection Status Interrupt Enable"]
pub type MOSCSELS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MOSCRCS` writer - Main On-Chip RC Status Interrupt Enable"]
pub type MOSCRCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Enable"]
pub type CFDEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MOSCXTS_W<PMC_IER_SPEC, 0> {
        MOSCXTS_W::new(self)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<PMC_IER_SPEC, 1> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MCKRDY_W<PMC_IER_SPEC, 3> {
        MCKRDY_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W<PMC_IER_SPEC, 8> {
        PCKRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W<PMC_IER_SPEC, 9> {
        PCKRDY1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W<PMC_IER_SPEC, 10> {
        PCKRDY2_W::new(self)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MOSCSELS_W<PMC_IER_SPEC, 16> {
        MOSCSELS_W::new(self)
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MOSCRCS_W<PMC_IER_SPEC, 17> {
        MOSCRCS_W::new(self)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CFDEV_W<PMC_IER_SPEC, 18> {
        CFDEV_W::new(self)
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
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_IER_SPEC;
impl crate::RegisterSpec for PMC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_ier::W`](W) writer structure"]
impl crate::Writable for PMC_IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
