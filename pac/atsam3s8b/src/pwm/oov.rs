#[doc = "Register `OOV` reader"]
pub type R = crate::R<OovSpec>;
#[doc = "Register `OOV` writer"]
pub type W = crate::W<OovSpec>;
#[doc = "Field `OOVH0` reader - Output Override Value for PWMH output of the channel 0"]
pub type Oovh0R = crate::BitReader;
#[doc = "Field `OOVH0` writer - Output Override Value for PWMH output of the channel 0"]
pub type Oovh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVH1` reader - Output Override Value for PWMH output of the channel 1"]
pub type Oovh1R = crate::BitReader;
#[doc = "Field `OOVH1` writer - Output Override Value for PWMH output of the channel 1"]
pub type Oovh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVH2` reader - Output Override Value for PWMH output of the channel 2"]
pub type Oovh2R = crate::BitReader;
#[doc = "Field `OOVH2` writer - Output Override Value for PWMH output of the channel 2"]
pub type Oovh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVH3` reader - Output Override Value for PWMH output of the channel 3"]
pub type Oovh3R = crate::BitReader;
#[doc = "Field `OOVH3` writer - Output Override Value for PWMH output of the channel 3"]
pub type Oovh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL0` reader - Output Override Value for PWML output of the channel 0"]
pub type Oovl0R = crate::BitReader;
#[doc = "Field `OOVL0` writer - Output Override Value for PWML output of the channel 0"]
pub type Oovl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL1` reader - Output Override Value for PWML output of the channel 1"]
pub type Oovl1R = crate::BitReader;
#[doc = "Field `OOVL1` writer - Output Override Value for PWML output of the channel 1"]
pub type Oovl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL2` reader - Output Override Value for PWML output of the channel 2"]
pub type Oovl2R = crate::BitReader;
#[doc = "Field `OOVL2` writer - Output Override Value for PWML output of the channel 2"]
pub type Oovl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL3` reader - Output Override Value for PWML output of the channel 3"]
pub type Oovl3R = crate::BitReader;
#[doc = "Field `OOVL3` writer - Output Override Value for PWML output of the channel 3"]
pub type Oovl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oovh0(&self) -> Oovh0R {
        Oovh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oovh1(&self) -> Oovh1R {
        Oovh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oovh2(&self) -> Oovh2R {
        Oovh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oovh3(&self) -> Oovh3R {
        Oovh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oovl0(&self) -> Oovl0R {
        Oovl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oovl1(&self) -> Oovl1R {
        Oovl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oovl2(&self) -> Oovl2R {
        Oovl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oovl3(&self) -> Oovl3R {
        Oovl3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oovh0(&mut self) -> Oovh0W<OovSpec> {
        Oovh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oovh1(&mut self) -> Oovh1W<OovSpec> {
        Oovh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oovh2(&mut self) -> Oovh2W<OovSpec> {
        Oovh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oovh3(&mut self) -> Oovh3W<OovSpec> {
        Oovh3W::new(self, 3)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oovl0(&mut self) -> Oovl0W<OovSpec> {
        Oovl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oovl1(&mut self) -> Oovl1W<OovSpec> {
        Oovl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oovl2(&mut self) -> Oovl2W<OovSpec> {
        Oovl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oovl3(&mut self) -> Oovl3W<OovSpec> {
        Oovl3W::new(self, 19)
    }
}
#[doc = "PWM Output Override Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oov::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oov::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OovSpec;
impl crate::RegisterSpec for OovSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oov::R`](R) reader structure"]
impl crate::Readable for OovSpec {}
#[doc = "`write(|w| ..)` method takes [`oov::W`](W) writer structure"]
impl crate::Writable for OovSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOV to value 0"]
impl crate::Resettable for OovSpec {
    const RESET_VALUE: u32 = 0;
}
