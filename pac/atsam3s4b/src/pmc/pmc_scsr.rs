#[doc = "Register `PMC_SCSR` reader"]
pub struct R(crate::R<PMC_SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UDP` reader - USB Device Port Clock Status"]
pub struct UDP_R(crate::FieldReader<bool, bool>);
impl UDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub struct PCK0_R(crate::FieldReader<bool, bool>);
impl PCK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub struct PCK1_R(crate::FieldReader<bool, bool>);
impl PCK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK2` reader - Programmable Clock 2 Output Status"]
pub struct PCK2_R(crate::FieldReader<bool, bool>);
impl PCK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - USB Device Port Clock Status"]
    #[inline(always)]
    pub fn udp(&self) -> UDP_R {
        UDP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> PCK2_R {
        PCK2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "System Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scsr](index.html) module"]
pub struct PMC_SCSR_SPEC;
impl crate::RegisterSpec for PMC_SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_scsr::R](R) reader structure"]
impl crate::Readable for PMC_SCSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_SCSR to value 0x01"]
impl crate::Resettable for PMC_SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
