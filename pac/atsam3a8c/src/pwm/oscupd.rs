#[doc = "Register `OSCUPD` writer"]
pub type W = crate::W<OscupdSpec>;
#[doc = "Field `OSCUPH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type Oscuph0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type Oscuph1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type Oscuph2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type Oscuph3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH4` writer - Output Selection Clear for PWMH output of the channel 4"]
pub type Oscuph4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH5` writer - Output Selection Clear for PWMH output of the channel 5"]
pub type Oscuph5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH6` writer - Output Selection Clear for PWMH output of the channel 6"]
pub type Oscuph6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH7` writer - Output Selection Clear for PWMH output of the channel 7"]
pub type Oscuph7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type Oscupl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type Oscupl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type Oscupl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type Oscupl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL4` writer - Output Selection Clear for PWML output of the channel 4"]
pub type Oscupl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL5` writer - Output Selection Clear for PWML output of the channel 5"]
pub type Oscupl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL6` writer - Output Selection Clear for PWML output of the channel 6"]
pub type Oscupl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL7` writer - Output Selection Clear for PWML output of the channel 7"]
pub type Oscupl7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph0(&mut self) -> Oscuph0W<OscupdSpec> {
        Oscuph0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph1(&mut self) -> Oscuph1W<OscupdSpec> {
        Oscuph1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph2(&mut self) -> Oscuph2W<OscupdSpec> {
        Oscuph2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph3(&mut self) -> Oscuph3W<OscupdSpec> {
        Oscuph3W::new(self, 3)
    }
    #[doc = "Bit 4 - Output Selection Clear for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph4(&mut self) -> Oscuph4W<OscupdSpec> {
        Oscuph4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Selection Clear for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph5(&mut self) -> Oscuph5W<OscupdSpec> {
        Oscuph5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Selection Clear for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph6(&mut self) -> Oscuph6W<OscupdSpec> {
        Oscuph6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Selection Clear for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph7(&mut self) -> Oscuph7W<OscupdSpec> {
        Oscuph7W::new(self, 7)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl0(&mut self) -> Oscupl0W<OscupdSpec> {
        Oscupl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl1(&mut self) -> Oscupl1W<OscupdSpec> {
        Oscupl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl2(&mut self) -> Oscupl2W<OscupdSpec> {
        Oscupl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl3(&mut self) -> Oscupl3W<OscupdSpec> {
        Oscupl3W::new(self, 19)
    }
    #[doc = "Bit 20 - Output Selection Clear for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl4(&mut self) -> Oscupl4W<OscupdSpec> {
        Oscupl4W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Selection Clear for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl5(&mut self) -> Oscupl5W<OscupdSpec> {
        Oscupl5W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Selection Clear for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl6(&mut self) -> Oscupl6W<OscupdSpec> {
        Oscupl6W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Selection Clear for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl7(&mut self) -> Oscupl7W<OscupdSpec> {
        Oscupl7W::new(self, 23)
    }
}
#[doc = "PWM Output Selection Clear Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscupdSpec;
impl crate::RegisterSpec for OscupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oscupd::W`](W) writer structure"]
impl crate::Writable for OscupdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
