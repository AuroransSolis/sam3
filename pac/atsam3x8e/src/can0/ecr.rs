#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TecR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}
