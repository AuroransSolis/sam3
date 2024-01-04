#[doc = "Register `CKGR_MCFR` reader"]
pub type R = crate::R<CKGR_MCFR_SPEC>;
#[doc = "Register `CKGR_MCFR` writer"]
pub type W = crate::W<CKGR_MCFR_SPEC>;
#[doc = "Field `MAINF` reader - Main Clock Frequency"]
pub type MAINF_R = crate::FieldReader<u16>;
#[doc = "Field `MAINF` writer - Main Clock Frequency"]
pub type MAINF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAINFRDY` reader - Main Clock Ready"]
pub type MAINFRDY_R = crate::BitReader;
#[doc = "Field `MAINFRDY` writer - Main Clock Ready"]
pub type MAINFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCMEAS` reader - RC Oscillator Frequency Measure (write-only)"]
pub type RCMEAS_R = crate::BitReader;
#[doc = "Field `RCMEAS` writer - RC Oscillator Frequency Measure (write-only)"]
pub type RCMEAS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&self) -> MAINF_R {
        MAINF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Main Clock Ready"]
    #[inline(always)]
    pub fn mainfrdy(&self) -> MAINFRDY_R {
        MAINFRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    pub fn rcmeas(&self) -> RCMEAS_R {
        RCMEAS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn mainf(&mut self) -> MAINF_W<CKGR_MCFR_SPEC> {
        MAINF_W::new(self, 0)
    }
    #[doc = "Bit 16 - Main Clock Ready"]
    #[inline(always)]
    #[must_use]
    pub fn mainfrdy(&mut self) -> MAINFRDY_W<CKGR_MCFR_SPEC> {
        MAINFRDY_W::new(self, 16)
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rcmeas(&mut self) -> RCMEAS_W<CKGR_MCFR_SPEC> {
        RCMEAS_W::new(self, 20)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Main Clock Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mcfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mcfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGR_MCFR_SPEC;
impl crate::RegisterSpec for CKGR_MCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_mcfr::R`](R) reader structure"]
impl crate::Readable for CKGR_MCFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckgr_mcfr::W`](W) writer structure"]
impl crate::Writable for CKGR_MCFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGR_MCFR to value 0"]
impl crate::Resettable for CKGR_MCFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
