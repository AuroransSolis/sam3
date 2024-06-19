#[doc = "Register `OSSUPD` writer"]
pub type W = crate::W<OssupdSpec>;
#[doc = "Field `OSSUPH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type Ossuph0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type Ossuph1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type Ossuph2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type Ossuph3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type Ossupl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type Ossupl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type Ossupl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type Ossupl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph0(&mut self) -> Ossuph0W<OssupdSpec> {
        Ossuph0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph1(&mut self) -> Ossuph1W<OssupdSpec> {
        Ossuph1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph2(&mut self) -> Ossuph2W<OssupdSpec> {
        Ossuph2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph3(&mut self) -> Ossuph3W<OssupdSpec> {
        Ossuph3W::new(self, 3)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl0(&mut self) -> Ossupl0W<OssupdSpec> {
        Ossupl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl1(&mut self) -> Ossupl1W<OssupdSpec> {
        Ossupl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl2(&mut self) -> Ossupl2W<OssupdSpec> {
        Ossupl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl3(&mut self) -> Ossupl3W<OssupdSpec> {
        Ossupl3W::new(self, 19)
    }
}
#[doc = "PWM Output Selection Set Update Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ossupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OssupdSpec;
impl crate::RegisterSpec for OssupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ossupd::W`](W) writer structure"]
impl crate::Writable for OssupdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
