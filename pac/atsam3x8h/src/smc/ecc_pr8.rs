#[doc = "Register `ECC_PR8` reader"]
pub type R = crate::R<EccPr8Spec>;
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
#[doc = "SMC ECC parity 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPr8Spec;
impl crate::RegisterSpec for EccPr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pr8::R`](R) reader structure"]
impl crate::Readable for EccPr8Spec {}
#[doc = "`reset()` method sets ECC_PR8 to value 0"]
impl crate::Resettable for EccPr8Spec {
    const RESET_VALUE: u32 = 0;
}
