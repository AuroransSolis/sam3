#[doc = "Register `ODATA` reader"]
pub type R = crate::R<OdataSpec>;
#[doc = "Field `ODATA` reader - Output Data"]
pub type OdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> OdataR {
        OdataR::new(self.bits)
    }
}
#[doc = "Output Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdataSpec;
impl crate::RegisterSpec for OdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odata::R`](R) reader structure"]
impl crate::Readable for OdataSpec {}
#[doc = "`reset()` method sets ODATA to value 0"]
impl crate::Resettable for OdataSpec {
    const RESET_VALUE: u32 = 0;
}
