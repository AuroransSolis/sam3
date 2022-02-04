#[doc = "Register `CKGR_MCFR` reader"]
pub struct R(crate::R<CKGR_MCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_MCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_MCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_MCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAINF` reader - Main Clock Frequency"]
pub struct MAINF_R(crate::FieldReader<u16, u16>);
impl MAINF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAINF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAINF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAINFRDY` reader - Main Clock Ready"]
pub struct MAINFRDY_R(crate::FieldReader<bool, bool>);
impl MAINFRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAINFRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAINFRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&self) -> MAINF_R {
        MAINF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Main Clock Ready"]
    #[inline(always)]
    pub fn mainfrdy(&self) -> MAINFRDY_R {
        MAINFRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Main Clock Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mcfr](index.html) module"]
pub struct CKGR_MCFR_SPEC;
impl crate::RegisterSpec for CKGR_MCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_mcfr::R](R) reader structure"]
impl crate::Readable for CKGR_MCFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CKGR_MCFR to value 0"]
impl crate::Resettable for CKGR_MCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
