#[doc = "Register `LCDR` reader"]
pub struct R(crate::R<LCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LDATA` reader - Last Data Converted"]
pub struct LDATA_R(crate::FieldReader<u16, u16>);
impl LDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Last Data Converted"]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdr](index.html) module"]
pub struct LCDR_SPEC;
impl crate::RegisterSpec for LCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdr::R](R) reader structure"]
impl crate::Readable for LCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCDR to value 0"]
impl crate::Resettable for LCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
