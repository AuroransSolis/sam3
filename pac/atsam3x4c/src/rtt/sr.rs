#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ALMS` reader - Real-time Alarm Status"]
pub type AlmsR = crate::BitReader;
#[doc = "Field `RTTINC` reader - Real-time Timer Increment"]
pub type RttincR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Real-time Alarm Status"]
    #[inline(always)]
    pub fn alms(&self) -> AlmsR {
        AlmsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time Timer Increment"]
    #[inline(always)]
    pub fn rttinc(&self) -> RttincR {
        RttincR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
