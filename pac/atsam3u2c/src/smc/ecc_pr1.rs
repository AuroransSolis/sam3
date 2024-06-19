#[doc = "Register `ECC_PR1` reader"]
pub type R = crate::R<EccPr1Spec>;
#[doc = "Field `NPARITY` reader - Parity N"]
pub type NparityR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NparityR {
        NparityR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SMC ECC parity 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPr1Spec;
impl crate::RegisterSpec for EccPr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pr1::R`](R) reader structure"]
impl crate::Readable for EccPr1Spec {}
#[doc = "`reset()` method sets ECC_PR1 to value 0"]
impl crate::Resettable for EccPr1Spec {
    const RESET_VALUE: u32 = 0;
}
