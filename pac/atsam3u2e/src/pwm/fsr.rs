#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Field `FIV` reader - Fault Input Value (fault input bit varies from 0 to 3)"]
pub type FivR = crate::FieldReader;
#[doc = "Field `FS` reader - Fault Status (fault input bit varies from 0 to 3)"]
pub type FsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Fault Input Value (fault input bit varies from 0 to 3)"]
    #[inline(always)]
    pub fn fiv(&self) -> FivR {
        FivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Status (fault input bit varies from 0 to 3)"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "PWM Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FsrSpec {
    const RESET_VALUE: u32 = 0;
}
