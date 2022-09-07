#[doc = "Register `ECC_PR1` reader"]
pub struct R(crate::R<ECC_PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPARITY` reader - Parity N"]
pub type NPARITY_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NPARITY_R {
        NPARITY_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SMC ECC parity 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr1](index.html) module"]
pub struct ECC_PR1_SPEC;
impl crate::RegisterSpec for ECC_PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_pr1::R](R) reader structure"]
impl crate::Readable for ECC_PR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_PR1 to value 0"]
impl crate::Resettable for ECC_PR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
