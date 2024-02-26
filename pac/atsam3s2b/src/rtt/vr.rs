#[doc = "Register `VR` reader"]
pub type R = crate::R<VrSpec>;
#[doc = "Field `CRTV` reader - Current Real-time Value"]
pub type CrtvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current Real-time Value"]
    #[inline(always)]
    pub fn crtv(&self) -> CrtvR {
        CrtvR::new(self.bits)
    }
}
#[doc = "Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrSpec;
impl crate::RegisterSpec for VrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vr::R`](R) reader structure"]
impl crate::Readable for VrSpec {}
#[doc = "`reset()` method sets VR to value 0"]
impl crate::Resettable for VrSpec {
    const RESET_VALUE: u32 = 0;
}
