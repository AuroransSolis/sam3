#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `ERRISR` reader - CRC Error Interrupt Status"]
pub type ERRISR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Status"]
    #[inline(always)]
    pub fn errisr(&self) -> ERRISR_R {
        ERRISR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
