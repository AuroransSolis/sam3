#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Field `FRDY` reader - Flash Ready Status"]
pub type FrdyR = crate::BitReader;
#[doc = "Field `FCMDE` reader - Flash Command Error Status"]
pub type FcmdeR = crate::BitReader;
#[doc = "Field `FLOCKE` reader - Flash Lock Error Status"]
pub type FlockeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flash Ready Status"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status"]
    #[inline(always)]
    pub fn fcmde(&self) -> FcmdeR {
        FcmdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status"]
    #[inline(always)]
    pub fn flocke(&self) -> FlockeR {
        FlockeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "EEFC Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`reset()` method sets FSR to value 0x01"]
impl crate::Resettable for FsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
