#[doc = "Register `PMC_WPMR` reader"]
pub type R = crate::R<PmcWpmrSpec>;
#[doc = "Register `PMC_WPMR` writer"]
pub type W = crate::W<PmcWpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protect Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` reader - Write Protect KEY"]
pub type WpkeyR = crate::FieldReader<u32>;
#[doc = "Field `WPKEY` writer - Write Protect KEY"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WpenW<PmcWpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WpkeyW<PmcWpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_wpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_wpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcWpmrSpec;
impl crate::RegisterSpec for PmcWpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_wpmr::R`](R) reader structure"]
impl crate::Readable for PmcWpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_wpmr::W`](W) writer structure"]
impl crate::Writable for PmcWpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_WPMR to value 0"]
impl crate::Resettable for PmcWpmrSpec {
    const RESET_VALUE: u32 = 0;
}
