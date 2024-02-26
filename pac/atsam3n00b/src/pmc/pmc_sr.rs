#[doc = "Register `PMC_SR` reader"]
pub type R = crate::R<PmcSrSpec>;
#[doc = "Field `MOSCXTS` reader - Main XTAL Oscillator Status"]
pub type MoscxtsR = crate::BitReader;
#[doc = "Field `LOCKA` reader - PLLA Lock Status"]
pub type LockaR = crate::BitReader;
#[doc = "Field `MCKRDY` reader - Master Clock Status"]
pub type MckrdyR = crate::BitReader;
#[doc = "Field `OSCSELS` reader - Slow Clock Oscillator Selection"]
pub type OscselsR = crate::BitReader;
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready Status"]
pub type Pckrdy0R = crate::BitReader;
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready Status"]
pub type Pckrdy1R = crate::BitReader;
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready Status"]
pub type Pckrdy2R = crate::BitReader;
#[doc = "Field `MOSCSELS` reader - Main Oscillator Selection Status"]
pub type MoscselsR = crate::BitReader;
#[doc = "Field `MOSCRCS` reader - Main On-Chip RC Oscillator Status"]
pub type MoscrcsR = crate::BitReader;
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event"]
pub type CfdevR = crate::BitReader;
#[doc = "Field `CFDS` reader - Clock Failure Detector Status"]
pub type CfdsR = crate::BitReader;
#[doc = "Field `FOS` reader - Clock Failure Detector Fault Output Status"]
pub type FosR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main XTAL Oscillator Status"]
    #[inline(always)]
    pub fn moscxts(&self) -> MoscxtsR {
        MoscxtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Status"]
    #[inline(always)]
    pub fn locka(&self) -> LockaR {
        LockaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Clock Status"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MckrdyR {
        MckrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Slow Clock Oscillator Selection"]
    #[inline(always)]
    pub fn oscsels(&self) -> OscselsR {
        OscselsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> Pckrdy0R {
        Pckrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> Pckrdy1R {
        Pckrdy1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> Pckrdy2R {
        Pckrdy2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status"]
    #[inline(always)]
    pub fn moscsels(&self) -> MoscselsR {
        MoscselsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main On-Chip RC Oscillator Status"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MoscrcsR {
        MoscrcsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event"]
    #[inline(always)]
    pub fn cfdev(&self) -> CfdevR {
        CfdevR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock Failure Detector Status"]
    #[inline(always)]
    pub fn cfds(&self) -> CfdsR {
        CfdsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Failure Detector Fault Output Status"]
    #[inline(always)]
    pub fn fos(&self) -> FosR {
        FosR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcSrSpec;
impl crate::RegisterSpec for PmcSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_sr::R`](R) reader structure"]
impl crate::Readable for PmcSrSpec {}
#[doc = "`reset()` method sets PMC_SR to value 0x0001_0008"]
impl crate::Resettable for PmcSrSpec {
    const RESET_VALUE: u32 = 0x0001_0008;
}
