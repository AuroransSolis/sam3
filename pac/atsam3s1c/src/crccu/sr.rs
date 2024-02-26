#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `CRC` reader - Cyclic Redundancy Check Value"]
pub type CrcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cyclic Redundancy Check Value"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(self.bits)
    }
}
#[doc = "CRCCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0xffff_ffff"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
