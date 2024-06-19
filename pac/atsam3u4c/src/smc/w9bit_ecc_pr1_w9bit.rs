#[doc = "Register `ECC_PR1_W9BIT` reader"]
pub type R = crate::R<W9bitEccPr1W9bitSpec>;
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
#[doc = "SMC ECC parity 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`w9bit_ecc_pr1_w9bit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W9bitEccPr1W9bitSpec;
impl crate::RegisterSpec for W9bitEccPr1W9bitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w9bit_ecc_pr1_w9bit::R`](R) reader structure"]
impl crate::Readable for W9bitEccPr1W9bitSpec {}
#[doc = "`reset()` method sets ECC_PR1_W9BIT to value 0"]
impl crate::Resettable for W9bitEccPr1W9bitSpec {
    const RESET_VALUE: u32 = 0;
}
