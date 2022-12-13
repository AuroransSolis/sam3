#[doc = "Register `PTSR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<PTSR_SPEC>);
#[doc = "Field `RXTEN` reader - Receiver Transfer Enable"]
pub type RXTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXTEN` reader - Transmitter Transfer Enable"]
pub type TXTEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 8) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
