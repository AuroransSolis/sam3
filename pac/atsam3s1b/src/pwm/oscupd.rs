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
#[doc = "Field `OSCUPL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type Oscupl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type Oscupl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type Oscupl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type Oscupl3W<'a, REG> = crate::BitWriter<'a, REG>;
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
