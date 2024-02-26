#[doc = "Register `OSS` writer"]
pub type W = crate::W<OssSpec>;
#[doc = "Field `OSSH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type Ossh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type Ossh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type Ossh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type Ossh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH4` writer - Output Selection Set for PWMH output of the channel 4"]
pub type Ossh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH5` writer - Output Selection Set for PWMH output of the channel 5"]
pub type Ossh5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH6` writer - Output Selection Set for PWMH output of the channel 6"]
pub type Ossh6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSH7` writer - Output Selection Set for PWMH output of the channel 7"]
pub type Ossh7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type Ossl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type Ossl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type Ossl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type Ossl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL4` writer - Output Selection Set for PWML output of the channel 4"]
pub type Ossl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL5` writer - Output Selection Set for PWML output of the channel 5"]
pub type Ossl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL6` writer - Output Selection Set for PWML output of the channel 6"]
pub type Ossl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL7` writer - Output Selection Set for PWML output of the channel 7"]
pub type Ossl7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossh0(&mut self) -> Ossh0W<OssSpec> {
        Ossh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossh1(&mut self) -> Ossh1W<OssSpec> {
        Ossh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossh2(&mut self) -> Ossh2W<OssSpec> {
        Ossh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossh3(&mut self) -> Ossh3W<OssSpec> {
        Ossh3W::new(self, 3)
    }
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossh4(&mut self) -> Ossh4W<OssSpec> {
        Ossh4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossh5(&mut self) -> Ossh5W<OssSpec> {
        Ossh5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossh6(&mut self) -> Ossh6W<OssSpec> {
        Ossh6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossh7(&mut self) -> Ossh7W<OssSpec> {
        Ossh7W::new(self, 7)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossl0(&mut self) -> Ossl0W<OssSpec> {
        Ossl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossl1(&mut self) -> Ossl1W<OssSpec> {
        Ossl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossl2(&mut self) -> Ossl2W<OssSpec> {
        Ossl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossl3(&mut self) -> Ossl3W<OssSpec> {
        Ossl3W::new(self, 19)
    }
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossl4(&mut self) -> Ossl4W<OssSpec> {
        Ossl4W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossl5(&mut self) -> Ossl5W<OssSpec> {
        Ossl5W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossl6(&mut self) -> Ossl6W<OssSpec> {
        Ossl6W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossl7(&mut self) -> Ossl7W<OssSpec> {
        Ossl7W::new(self, 23)
    }
}
#[doc = "PWM Output Selection Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oss::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OssSpec;
impl crate::RegisterSpec for OssSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oss::W`](W) writer structure"]
impl crate::Writable for OssSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
