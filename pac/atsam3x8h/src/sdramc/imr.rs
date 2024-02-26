#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RES` reader - Refresh Error Status"]
pub type ResR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Refresh Error Status"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
}
#[doc = "SDRAMC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
