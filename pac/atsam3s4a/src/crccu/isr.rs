#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `ERRISR` reader - CRC Error Interrupt Status"]
pub type ErrisrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Status"]
    #[inline(always)]
    pub fn errisr(&self) -> ErrisrR {
        ErrisrR::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
