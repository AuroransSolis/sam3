#[doc = "Register `ECC_PR10` reader"]
pub struct R(crate::R<ECC_PR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_PR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_PR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_PR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BITADDR` reader - Corrupted Bit Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
pub struct BITADDR_R(crate::FieldReader<u8, u8>);
impl BITADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BITADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BITADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORDADDR` reader - Corrupted Word Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
pub struct WORDADDR_R(crate::FieldReader<u8, u8>);
impl WORDADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WORDADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORDADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPARITY` reader - Parity N"]
pub struct NPARITY_R(crate::FieldReader<u16, u16>);
impl NPARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        NPARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPARITY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Corrupted Bit Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BITADDR_R {
        BITADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:10 - Corrupted Word Address in the Page between (i x 256) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WORDADDR_R {
        WORDADDR_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 12:22 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NPARITY_R {
        NPARITY_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
}
#[doc = "SMC ECC parity 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_pr10](index.html) module"]
pub struct ECC_PR10_SPEC;
impl crate::RegisterSpec for ECC_PR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_pr10::R](R) reader structure"]
impl crate::Readable for ECC_PR10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_PR10 to value 0"]
impl crate::Resettable for ECC_PR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
