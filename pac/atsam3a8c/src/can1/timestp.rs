#[doc = "Register `TIMESTP` reader"]
pub struct R(crate::R<TIMESTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MTIMESTAMP` reader - Timestamp"]
pub struct MTIMESTAMP_R(crate::FieldReader<u16, u16>);
impl MTIMESTAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MTIMESTAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIMESTAMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MTIMESTAMP_R {
        MTIMESTAMP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timestamp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestp](index.html) module"]
pub struct TIMESTP_SPEC;
impl crate::RegisterSpec for TIMESTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestp::R](R) reader structure"]
impl crate::Readable for TIMESTP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMESTP to value 0"]
impl crate::Resettable for TIMESTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
