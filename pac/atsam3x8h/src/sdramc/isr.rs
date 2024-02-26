#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `RES` reader - Refresh Error Status"]
pub type ResR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Refresh Error Status"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
}
#[doc = "SDRAMC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
