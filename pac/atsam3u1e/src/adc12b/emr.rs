#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Field `OFFMODES` reader - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
pub type OffmodesR = crate::BitReader;
#[doc = "Field `OFFMODES` writer - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
pub type OffmodesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF_MODE_STARTUP_TIME` reader - Startup Time"]
pub type OffModeStartupTimeR = crate::FieldReader;
#[doc = "Field `OFF_MODE_STARTUP_TIME` writer - Startup Time"]
pub type OffModeStartupTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
    #[inline(always)]
    pub fn offmodes(&self) -> OffmodesR {
        OffmodesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Startup Time"]
    #[inline(always)]
    pub fn off_mode_startup_time(&self) -> OffModeStartupTimeR {
        OffModeStartupTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
    #[inline(always)]
    #[must_use]
    pub fn offmodes(&mut self) -> OffmodesW<EmrSpec> {
        OffmodesW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Startup Time"]
    #[inline(always)]
    #[must_use]
    pub fn off_mode_startup_time(&mut self) -> OffModeStartupTimeW<EmrSpec> {
        OffModeStartupTimeW::new(self, 16)
    }
}
#[doc = "Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {
    const RESET_VALUE: u32 = 0;
}
