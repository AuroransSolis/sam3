#[doc = "Register `PMC_PCSR1` reader"]
pub struct R(crate::R<PMC_PCSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub struct PID32_R(crate::FieldReader<bool, bool>);
impl PID32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub struct PID33_R(crate::FieldReader<bool, bool>);
impl PID33_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub struct PID34_R(crate::FieldReader<bool, bool>);
impl PID34_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcsr1](index.html) module"]
pub struct PMC_PCSR1_SPEC;
impl crate::RegisterSpec for PMC_PCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pcsr1::R](R) reader structure"]
impl crate::Readable for PMC_PCSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_PCSR1 to value 0"]
impl crate::Resettable for PMC_PCSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
