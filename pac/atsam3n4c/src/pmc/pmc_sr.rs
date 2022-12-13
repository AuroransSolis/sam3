#[doc = "Register `PMC_SR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<PMC_SR_SPEC>);
#[doc = "Field `MOSCXTS` reader - Main XTAL Oscillator Status"]
pub type MOSCXTS_R = crate::BitReader<bool>;
#[doc = "Field `LOCKA` reader - PLLA Lock Status"]
pub type LOCKA_R = crate::BitReader<bool>;
#[doc = "Field `MCKRDY` reader - Master Clock Status"]
pub type MCKRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSCSELS` reader - Slow Clock Oscillator Selection"]
pub type OSCSELS_R = crate::BitReader<bool>;
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready Status"]
pub type PCKRDY0_R = crate::BitReader<bool>;
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready Status"]
pub type PCKRDY1_R = crate::BitReader<bool>;
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready Status"]
pub type PCKRDY2_R = crate::BitReader<bool>;
#[doc = "Field `MOSCSELS` reader - Main Oscillator Selection Status"]
pub type MOSCSELS_R = crate::BitReader<bool>;
#[doc = "Field `MOSCRCS` reader - Main On-Chip RC Oscillator Status"]
pub type MOSCRCS_R = crate::BitReader<bool>;
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event"]
pub type CFDEV_R = crate::BitReader<bool>;
#[doc = "Field `CFDS` reader - Clock Failure Detector Status"]
pub type CFDS_R = crate::BitReader<bool>;
#[doc = "Field `FOS` reader - Clock Failure Detector Fault Output Status"]
pub type FOS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Main XTAL Oscillator Status"]
    #[inline(always)]
    pub fn moscxts(&self) -> MOSCXTS_R {
        MOSCXTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Status"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Clock Status"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MCKRDY_R {
        MCKRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Slow Clock Oscillator Selection"]
    #[inline(always)]
    pub fn oscsels(&self) -> OSCSELS_R {
        OSCSELS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> PCKRDY0_R {
        PCKRDY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> PCKRDY1_R {
        PCKRDY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> PCKRDY2_R {
        PCKRDY2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status"]
    #[inline(always)]
    pub fn moscsels(&self) -> MOSCSELS_R {
        MOSCSELS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main On-Chip RC Oscillator Status"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MOSCRCS_R {
        MOSCRCS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event"]
    #[inline(always)]
    pub fn cfdev(&self) -> CFDEV_R {
        CFDEV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock Failure Detector Status"]
    #[inline(always)]
    pub fn cfds(&self) -> CFDS_R {
        CFDS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Failure Detector Fault Output Status"]
    #[inline(always)]
    pub fn fos(&self) -> FOS_R {
        FOS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_sr](index.html) module"]
pub struct PMC_SR_SPEC;
impl crate::RegisterSpec for PMC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_sr::R](R) reader structure"]
impl crate::Readable for PMC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_SR to value 0x0001_0008"]
impl crate::Resettable for PMC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0008;
}
