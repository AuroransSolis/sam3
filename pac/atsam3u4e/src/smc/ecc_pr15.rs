#[doc = "Register `ECC_PR15` reader"]
pub type R = crate::R<EccPr15Spec>;
#[doc = "Field `BITADDR` reader - Corrupted Bit Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
pub type BitaddrR = crate::FieldReader;
#[doc = "Field `WORDADDR` reader - Corrupted Word Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
pub type WordaddrR = crate::FieldReader;
#[doc = "Field `NPARITY` reader - Parity N"]
pub type NparityR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - Corrupted Bit Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BitaddrR {
        BitaddrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:10 - Corrupted Word Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WordaddrR {
        WordaddrR::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 12:22 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NparityR {
        NparityR::new(((self.bits >> 12) & 0x07ff) as u16)
    }
}
#[doc = "SMC ECC parity 15 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPr15Spec;
impl crate::RegisterSpec for EccPr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pr15::R`](R) reader structure"]
impl crate::Readable for EccPr15Spec {}
#[doc = "`reset()` method sets ECC_PR15 to value 0"]
impl crate::Resettable for EccPr15Spec {
    const RESET_VALUE: u32 = 0;
}
