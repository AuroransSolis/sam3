#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `RES` reader - Refresh Error Status"]
pub type RES_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Refresh Error Status"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SDRAMC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
