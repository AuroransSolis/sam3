#[doc = "Register `PCRHR` reader"]
pub type R = crate::R<PcrhrSpec>;
#[doc = "Field `RDATA` reader - Parallel Capture Mode Reception Data."]
pub type RdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Parallel Capture Mode Reception Data."]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new(self.bits)
    }
}
#[doc = "Parallel Capture Reception Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrhrSpec;
impl crate::RegisterSpec for PcrhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrhr::R`](R) reader structure"]
impl crate::Readable for PcrhrSpec {}
#[doc = "`reset()` method sets PCRHR to value 0"]
impl crate::Resettable for PcrhrSpec {
    const RESET_VALUE: u32 = 0;
}
