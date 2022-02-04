#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Status"]
pub struct FRDY_R(crate::FieldReader<bool, bool>);
impl FRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMDE` reader - Flash Command Error Status"]
pub struct FCMDE_R(crate::FieldReader<bool, bool>);
impl FCMDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCMDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLOCKE` reader - Flash Lock Error Status"]
pub struct FLOCKE_R(crate::FieldReader<bool, bool>);
impl FLOCKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLOCKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLOCKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Flash Ready Status"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status"]
    #[inline(always)]
    pub fn fcmde(&self) -> FCMDE_R {
        FCMDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status"]
    #[inline(always)]
    pub fn flocke(&self) -> FLOCKE_R {
        FLOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "EEFC Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSR to value 0x01"]
impl crate::Resettable for FSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
