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
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type Ossl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type Ossl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type Ossl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type Ossl3W<'a, REG> = crate::BitWriter<'a, REG>;
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
