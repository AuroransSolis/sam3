#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `VERSION` reader - Hardware Module Version"]
pub type VersionR = crate::FieldReader<u16>;
#[doc = "Field `MFN` reader - Metal Fix Number"]
pub type MfnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Hardware Module Version"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Metal Fix Number"]
    #[inline(always)]
    pub fn mfn(&self) -> MfnR {
        MfnR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
