#[doc = "Register `PMC_FOCR` writer"]
pub type W = crate::W<PmcFocrSpec>;
#[doc = "Field `FOCLR` writer - Fault Output Clear"]
pub type FoclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline(always)]
    #[must_use]
    pub fn foclr(&mut self) -> FoclrW<PmcFocrSpec> {
        FoclrW::new(self, 0)
    }
}
#[doc = "Fault Output Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_focr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcFocrSpec;
impl crate::RegisterSpec for PmcFocrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_focr::W`](W) writer structure"]
impl crate::Writable for PmcFocrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
