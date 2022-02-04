#[doc = "Register `PTSR` reader"]
pub struct R(crate::R<PTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTEN` reader - Receiver Transfer Enable"]
pub struct RXTEN_R(crate::FieldReader<bool, bool>);
impl RXTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTEN` reader - Transmitter Transfer Enable"]
pub struct TXTEN_R(crate::FieldReader<bool, bool>);
impl TXTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Transfer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr](index.html) module"]
pub struct PTSR_SPEC;
impl crate::RegisterSpec for PTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr::R](R) reader structure"]
impl crate::Readable for PTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTSR to value 0"]
impl crate::Resettable for PTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
