#[doc = "Register `ECC_PR0` reader"]
pub type R = crate::R<EccPr0Spec>;
#[doc = "Field `BITADDR` reader - Bit Address"]
pub type BitaddrR = crate::FieldReader;
#[doc = "Field `WORDADDR` reader - Word Address"]
pub type WordaddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Bit Address"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BitaddrR {
        BitaddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Word Address"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WordaddrR {
        WordaddrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[doc = "SMC ECC Parity 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPr0Spec;
impl crate::RegisterSpec for EccPr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pr0::R`](R) reader structure"]
impl crate::Readable for EccPr0Spec {}
#[doc = "`reset()` method sets ECC_PR0 to value 0"]
impl crate::Resettable for EccPr0Spec {
    const RESET_VALUE: u32 = 0;
}
