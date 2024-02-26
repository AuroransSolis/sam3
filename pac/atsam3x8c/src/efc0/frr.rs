#[doc = "Register `FRR` reader"]
pub type R = crate::R<FrrSpec>;
#[doc = "Field `FVALUE` reader - Flash Result Value"]
pub type FvalueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Result Value"]
    #[inline(always)]
    pub fn fvalue(&self) -> FvalueR {
        FvalueR::new(self.bits)
    }
}
#[doc = "EEFC Flash Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrrSpec;
impl crate::RegisterSpec for FrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frr::R`](R) reader structure"]
impl crate::Readable for FrrSpec {}
#[doc = "`reset()` method sets FRR to value 0"]
impl crate::Resettable for FrrSpec {
    const RESET_VALUE: u32 = 0;
}
