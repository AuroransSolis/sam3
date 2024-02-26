#[doc = "Register `CKGR_MCFR` reader"]
pub type R = crate::R<CkgrMcfrSpec>;
#[doc = "Register `CKGR_MCFR` writer"]
pub type W = crate::W<CkgrMcfrSpec>;
#[doc = "Field `MAINF` reader - Main Clock Frequency"]
pub type MainfR = crate::FieldReader<u16>;
#[doc = "Field `MAINF` writer - Main Clock Frequency"]
pub type MainfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAINFRDY` reader - Main Clock Ready"]
pub type MainfrdyR = crate::BitReader;
#[doc = "Field `MAINFRDY` writer - Main Clock Ready"]
pub type MainfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCMEAS` reader - RC Oscillator Frequency Measure (write-only)"]
pub type RcmeasR = crate::BitReader;
#[doc = "Field `RCMEAS` writer - RC Oscillator Frequency Measure (write-only)"]
pub type RcmeasW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    pub fn rcmeas(&self) -> RcmeasR {
        RcmeasR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn mainf(&mut self) -> MainfW<CkgrMcfrSpec> {
        MainfW::new(self, 0)
    }
    #[doc = "Bit 16 - Main Clock Ready"]
    #[inline(always)]
    #[must_use]
    pub fn mainfrdy(&mut self) -> MainfrdyW<CkgrMcfrSpec> {
        MainfrdyW::new(self, 16)
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rcmeas(&mut self) -> RcmeasW<CkgrMcfrSpec> {
        RcmeasW::new(self, 20)
    }
}
#[doc = "Main Clock Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mcfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mcfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrMcfrSpec;
impl crate::RegisterSpec for CkgrMcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_mcfr::R`](R) reader structure"]
impl crate::Readable for CkgrMcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_mcfr::W`](W) writer structure"]
impl crate::Writable for CkgrMcfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGR_MCFR to value 0"]
impl crate::Resettable for CkgrMcfrSpec {
    const RESET_VALUE: u32 = 0;
}
