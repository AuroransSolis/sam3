#[doc = "Register `VER` reader"]
pub type R = crate::R<VerSpec>;
#[doc = "Field `NVTIM` reader - Non-valid Time"]
pub type NvtimR = crate::BitReader;
#[doc = "Field `NVCAL` reader - Non-valid Calendar"]
pub type NvcalR = crate::BitReader;
#[doc = "Field `NVTIMALR` reader - Non-valid Time Alarm"]
pub type NvtimalrR = crate::BitReader;
#[doc = "Field `NVCALALR` reader - Non-valid Calendar Alarm"]
pub type NvcalalrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Non-valid Time"]
    #[inline(always)]
    pub fn nvtim(&self) -> NvtimR {
        NvtimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-valid Calendar"]
    #[inline(always)]
    pub fn nvcal(&self) -> NvcalR {
        NvcalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-valid Time Alarm"]
    #[inline(always)]
    pub fn nvtimalr(&self) -> NvtimalrR {
        NvtimalrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-valid Calendar Alarm"]
    #[inline(always)]
    pub fn nvcalalr(&self) -> NvcalalrR {
        NvcalalrR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Valid Entry Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VerSpec;
impl crate::RegisterSpec for VerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VerSpec {}
#[doc = "`reset()` method sets VER to value 0"]
impl crate::Resettable for VerSpec {
    const RESET_VALUE: u32 = 0;
}
