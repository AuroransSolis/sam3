#[doc = "Register `ECC_PR0` reader"]
pub struct R(crate::R<ECC_PR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_PR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_PR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_PR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BITADDR` reader - Bit Address"]
pub type BITADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WORDADDR` reader - Word Address"]
pub type WORDADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Bit Address"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BITADDR_R {
        BITADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Word Address"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WORDADDR_R {
        WORDADDR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[doc = "SMC ECC Parity 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr0](index.html) module"]
pub struct ECC_PR0_SPEC;
impl crate::RegisterSpec for ECC_PR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_pr0::R](R) reader structure"]
impl crate::Readable for ECC_PR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_PR0 to value 0"]
impl crate::Resettable for ECC_PR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
