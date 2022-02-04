#[doc = "Register `PMC_SR` reader"]
pub struct R(crate::R<PMC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MOSCXTS` reader - Main XTAL Oscillator Status"]
pub struct MOSCXTS_R(crate::FieldReader<bool, bool>);
impl MOSCXTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCXTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCXTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKA` reader - PLLA Lock Status"]
pub struct LOCKA_R(crate::FieldReader<bool, bool>);
impl LOCKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKRDY` reader - Master Clock Status"]
pub struct MCKRDY_R(crate::FieldReader<bool, bool>);
impl MCKRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCSELS` reader - Slow Clock Oscillator Selection"]
pub struct OSCSELS_R(crate::FieldReader<bool, bool>);
impl OSCSELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSCSELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCSELS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready Status"]
pub struct PCKRDY0_R(crate::FieldReader<bool, bool>);
impl PCKRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready Status"]
pub struct PCKRDY1_R(crate::FieldReader<bool, bool>);
impl PCKRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready Status"]
pub struct PCKRDY2_R(crate::FieldReader<bool, bool>);
impl PCKRDY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCSELS` reader - Main Oscillator Selection Status"]
pub struct MOSCSELS_R(crate::FieldReader<bool, bool>);
impl MOSCSELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCSELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCSELS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCRCS` reader - Main On-Chip RC Oscillator Status"]
pub struct MOSCRCS_R(crate::FieldReader<bool, bool>);
impl MOSCRCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCRCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCRCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event"]
pub struct CFDEV_R(crate::FieldReader<bool, bool>);
impl CFDEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDS` reader - Clock Failure Detector Status"]
pub struct CFDS_R(crate::FieldReader<bool, bool>);
impl CFDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOS` reader - Clock Failure Detector Fault Output Status"]
pub struct FOS_R(crate::FieldReader<bool, bool>);
impl FOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Main XTAL Oscillator Status"]
    #[inline(always)]
    pub fn moscxts(&self) -> MOSCXTS_R {
        MOSCXTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Status"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Clock Status"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MCKRDY_R {
        MCKRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Slow Clock Oscillator Selection"]
    #[inline(always)]
    pub fn oscsels(&self) -> OSCSELS_R {
        OSCSELS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> PCKRDY0_R {
        PCKRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> PCKRDY1_R {
        PCKRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> PCKRDY2_R {
        PCKRDY2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status"]
    #[inline(always)]
    pub fn moscsels(&self) -> MOSCSELS_R {
        MOSCSELS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main On-Chip RC Oscillator Status"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MOSCRCS_R {
        MOSCRCS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event"]
    #[inline(always)]
    pub fn cfdev(&self) -> CFDEV_R {
        CFDEV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clock Failure Detector Status"]
    #[inline(always)]
    pub fn cfds(&self) -> CFDS_R {
        CFDS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clock Failure Detector Fault Output Status"]
    #[inline(always)]
    pub fn fos(&self) -> FOS_R {
        FOS_R::new(((self.bits >> 20) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0008
    }
}
