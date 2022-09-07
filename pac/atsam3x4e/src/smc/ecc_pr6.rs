#[doc = "Register `ECC_PR6` reader"]
pub struct R(crate::R<ECC_PR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_PR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_PR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_PR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BITADDR` reader - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
pub type BITADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WORDADDR` reader - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
pub type WORDADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPARITY` reader - Parity N"]
pub type NPARITY_R = crate::FieldReader<u16, u16>;
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
#[doc = "SMC ECC parity 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr6](index.html) module"]
pub struct ECC_PR6_SPEC;
impl crate::RegisterSpec for ECC_PR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_pr6::R](R) reader structure"]
impl crate::Readable for ECC_PR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_PR6 to value 0"]
impl crate::Resettable for ECC_PR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
