#[doc = "Register `ECC_PR1` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ECC_PR1_SPEC>);
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
    const RESET_VALUE: Self::Ux = 0;
}
