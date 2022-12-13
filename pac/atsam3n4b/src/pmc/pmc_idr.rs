#[doc = "Register `PMC_IDR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<PMC_IDR_SPEC>);
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Disable"]
pub type MOSCXTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Disable"]
pub type LOCKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Disable"]
pub type MCKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Disable"]
pub type PCKRDY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Disable"]
pub type PCKRDY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Disable"]
pub type PCKRDY2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `MOSCSELS` writer - Main Oscillator Selection Status Interrupt Disable"]
pub type MOSCSELS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `MOSCRCS` writer - Main On-Chip RC Status Interrupt Disable"]
pub type MOSCRCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Disable"]
pub type CFDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MOSCXTS_W<0> {
        MOSCXTS_W::new(self)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<1> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MCKRDY_W<3> {
        MCKRDY_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W<8> {
        PCKRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W<9> {
        PCKRDY1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W<10> {
        PCKRDY2_W::new(self)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MOSCSELS_W<16> {
        MOSCSELS_W::new(self)
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MOSCRCS_W<17> {
        MOSCRCS_W::new(self)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CFDEV_W<18> {
        CFDEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_idr](index.html) module"]
pub struct PMC_IDR_SPEC;
impl crate::RegisterSpec for PMC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_idr::W](W) writer structure"]
impl crate::Writable for PMC_IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
