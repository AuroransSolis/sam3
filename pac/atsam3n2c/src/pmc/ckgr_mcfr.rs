#[doc = "Register `CKGR_MCFR` reader"]
pub type R = crate::R<CkgrMcfrSpec>;
#[doc = "Field `MAINF` reader - Main Clock Frequency"]
pub type MainfR = crate::FieldReader<u16>;
#[doc = "Field `MAINFRDY` reader - Main Clock Ready"]
pub type MainfrdyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&self) -> MainfR {
        MainfR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Main Clock Ready"]
    #[inline(always)]
    pub fn mainfrdy(&self) -> MainfrdyR {
        MainfrdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Main Clock Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mcfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrMcfrSpec;
impl crate::RegisterSpec for CkgrMcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_mcfr::R`](R) reader structure"]
impl crate::Readable for CkgrMcfrSpec {}
#[doc = "`reset()` method sets CKGR_MCFR to value 0"]
impl crate::Resettable for CkgrMcfrSpec {
    const RESET_VALUE: u32 = 0;
}
