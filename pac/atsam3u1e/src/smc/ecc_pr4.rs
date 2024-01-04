#[doc = "Register `ECC_PR4` reader"]
pub type R = crate::R<ECC_PR4_SPEC>;
#[doc = "Field `BITADDR` reader - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
pub type BITADDR_R = crate::FieldReader;
#[doc = "Field `WORDADDR` reader - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
pub type WORDADDR_R = crate::FieldReader<u16>;
#[doc = "Field `NPARITY` reader - Parity N"]
pub type NPARITY_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BITADDR_R {
        BITADDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:11 - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WORDADDR_R {
        WORDADDR_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:23 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NPARITY_R {
        NPARITY_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[doc = "SMC ECC parity 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_PR4_SPEC;
impl crate::RegisterSpec for ECC_PR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pr4::R`](R) reader structure"]
impl crate::Readable for ECC_PR4_SPEC {}
#[doc = "`reset()` method sets ECC_PR4 to value 0"]
impl crate::Resettable for ECC_PR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
