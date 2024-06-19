#[doc = "Register `ECC_PR5` reader"]
pub type R = crate::R<EccPr5Spec>;
#[doc = "Field `BITADDR` reader - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
pub type BitaddrR = crate::FieldReader;
#[doc = "Field `WORDADDR` reader - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
pub type WordaddrR = crate::FieldReader<u16>;
#[doc = "Field `NPARITY` reader - Parity N"]
pub type NparityR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BitaddrR {
        BitaddrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:11 - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WordaddrR {
        WordaddrR::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:23 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NparityR {
        NparityR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[doc = "SMC ECC parity 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPr5Spec;
impl crate::RegisterSpec for EccPr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pr5::R`](R) reader structure"]
impl crate::Readable for EccPr5Spec {}
#[doc = "`reset()` method sets ECC_PR5 to value 0"]
impl crate::Resettable for EccPr5Spec {
    const RESET_VALUE: u32 = 0;
}
