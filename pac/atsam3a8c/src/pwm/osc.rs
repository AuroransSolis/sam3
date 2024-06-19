#[doc = "Register `OSC` writer"]
pub type W = crate::W<OscSpec>;
#[doc = "Field `OSCH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type Osch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type Osch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type Osch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type Osch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH4` writer - Output Selection Clear for PWMH output of the channel 4"]
pub type Osch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH5` writer - Output Selection Clear for PWMH output of the channel 5"]
pub type Osch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH6` writer - Output Selection Clear for PWMH output of the channel 6"]
pub type Osch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCH7` writer - Output Selection Clear for PWMH output of the channel 7"]
pub type Osch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type Oscl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type Oscl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type Oscl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type Oscl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL4` writer - Output Selection Clear for PWML output of the channel 4"]
pub type Oscl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL5` writer - Output Selection Clear for PWML output of the channel 5"]
pub type Oscl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL6` writer - Output Selection Clear for PWML output of the channel 6"]
pub type Oscl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCL7` writer - Output Selection Clear for PWML output of the channel 7"]
pub type Oscl7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osch0(&mut self) -> Osch0W<OscSpec> {
        Osch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osch1(&mut self) -> Osch1W<OscSpec> {
        Osch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osch2(&mut self) -> Osch2W<OscSpec> {
        Osch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osch3(&mut self) -> Osch3W<OscSpec> {
        Osch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Output Selection Clear for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn osch4(&mut self) -> Osch4W<OscSpec> {
        Osch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Selection Clear for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn osch5(&mut self) -> Osch5W<OscSpec> {
        Osch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Selection Clear for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn osch6(&mut self) -> Osch6W<OscSpec> {
        Osch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Selection Clear for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn osch7(&mut self) -> Osch7W<OscSpec> {
        Osch7W::new(self, 7)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscl0(&mut self) -> Oscl0W<OscSpec> {
        Oscl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscl1(&mut self) -> Oscl1W<OscSpec> {
        Oscl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscl2(&mut self) -> Oscl2W<OscSpec> {
        Oscl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscl3(&mut self) -> Oscl3W<OscSpec> {
        Oscl3W::new(self, 19)
    }
    #[doc = "Bit 20 - Output Selection Clear for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oscl4(&mut self) -> Oscl4W<OscSpec> {
        Oscl4W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Selection Clear for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oscl5(&mut self) -> Oscl5W<OscSpec> {
        Oscl5W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Selection Clear for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oscl6(&mut self) -> Oscl6W<OscSpec> {
        Oscl6W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Selection Clear for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oscl7(&mut self) -> Oscl7W<OscSpec> {
        Oscl7W::new(self, 23)
    }
}
#[doc = "PWM Output Selection Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscSpec;
impl crate::RegisterSpec for OscSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`osc::W`](W) writer structure"]
impl crate::Writable for OscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
