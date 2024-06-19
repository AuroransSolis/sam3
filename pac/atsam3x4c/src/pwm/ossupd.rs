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
#[doc = "Field `OSSUPH4` writer - Output Selection Set for PWMH output of the channel 4"]
pub type Ossuph4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH5` writer - Output Selection Set for PWMH output of the channel 5"]
pub type Ossuph5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH6` writer - Output Selection Set for PWMH output of the channel 6"]
pub type Ossuph6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH7` writer - Output Selection Set for PWMH output of the channel 7"]
pub type Ossuph7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type Ossupl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type Ossupl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type Ossupl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type Ossupl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL4` writer - Output Selection Set for PWML output of the channel 4"]
pub type Ossupl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL5` writer - Output Selection Set for PWML output of the channel 5"]
pub type Ossupl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL6` writer - Output Selection Set for PWML output of the channel 6"]
pub type Ossupl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL7` writer - Output Selection Set for PWML output of the channel 7"]
pub type Ossupl7W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph4(&mut self) -> Ossuph4W<OssupdSpec> {
        Ossuph4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph5(&mut self) -> Ossuph5W<OssupdSpec> {
        Ossuph5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph6(&mut self) -> Ossuph6W<OssupdSpec> {
        Ossuph6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph7(&mut self) -> Ossuph7W<OssupdSpec> {
        Ossuph7W::new(self, 7)
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
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl4(&mut self) -> Ossupl4W<OssupdSpec> {
        Ossupl4W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl5(&mut self) -> Ossupl5W<OssupdSpec> {
        Ossupl5W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl6(&mut self) -> Ossupl6W<OssupdSpec> {
        Ossupl6W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl7(&mut self) -> Ossupl7W<OssupdSpec> {
        Ossupl7W::new(self, 23)
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
