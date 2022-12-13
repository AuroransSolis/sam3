#[doc = "Register `ISR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ISR_SPEC>);
#[doc = "Field `ERRISR` reader - CRC Error Interrupt Status"]
pub type ERRISR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Status"]
    #[inline(always)]
    pub fn errisr(&self) -> ERRISR_R {
        ERRISR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
