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
#[doc = "Field `OOVH4` reader - Output Override Value for PWMH output of the channel 4"]
pub type Oovh4R = crate::BitReader;
#[doc = "Field `OOVH4` writer - Output Override Value for PWMH output of the channel 4"]
pub type Oovh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVH5` reader - Output Override Value for PWMH output of the channel 5"]
pub type Oovh5R = crate::BitReader;
#[doc = "Field `OOVH5` writer - Output Override Value for PWMH output of the channel 5"]
pub type Oovh5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVH6` reader - Output Override Value for PWMH output of the channel 6"]
pub type Oovh6R = crate::BitReader;
#[doc = "Field `OOVH6` writer - Output Override Value for PWMH output of the channel 6"]
pub type Oovh6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVH7` reader - Output Override Value for PWMH output of the channel 7"]
pub type Oovh7R = crate::BitReader;
#[doc = "Field `OOVH7` writer - Output Override Value for PWMH output of the channel 7"]
pub type Oovh7W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `OOVL4` reader - Output Override Value for PWML output of the channel 4"]
pub type Oovl4R = crate::BitReader;
#[doc = "Field `OOVL4` writer - Output Override Value for PWML output of the channel 4"]
pub type Oovl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL5` reader - Output Override Value for PWML output of the channel 5"]
pub type Oovl5R = crate::BitReader;
#[doc = "Field `OOVL5` writer - Output Override Value for PWML output of the channel 5"]
pub type Oovl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL6` reader - Output Override Value for PWML output of the channel 6"]
pub type Oovl6R = crate::BitReader;
#[doc = "Field `OOVL6` writer - Output Override Value for PWML output of the channel 6"]
pub type Oovl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOVL7` reader - Output Override Value for PWML output of the channel 7"]
pub type Oovl7R = crate::BitReader;
#[doc = "Field `OOVL7` writer - Output Override Value for PWML output of the channel 7"]
pub type Oovl7W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - Output Override Value for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn oovh4(&self) -> Oovh4R {
        Oovh4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Override Value for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn oovh5(&self) -> Oovh5R {
        Oovh5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Override Value for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn oovh6(&self) -> Oovh6R {
        Oovh6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Override Value for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn oovh7(&self) -> Oovh7R {
        Oovh7R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 20 - Output Override Value for PWML output of the channel 4"]
    #[inline(always)]
    pub fn oovl4(&self) -> Oovl4R {
        Oovl4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Override Value for PWML output of the channel 5"]
    #[inline(always)]
    pub fn oovl5(&self) -> Oovl5R {
        Oovl5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output Override Value for PWML output of the channel 6"]
    #[inline(always)]
    pub fn oovl6(&self) -> Oovl6R {
        Oovl6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output Override Value for PWML output of the channel 7"]
    #[inline(always)]
    pub fn oovl7(&self) -> Oovl7R {
        Oovl7R::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 4 - Output Override Value for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oovh4(&mut self) -> Oovh4W<OovSpec> {
        Oovh4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Override Value for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oovh5(&mut self) -> Oovh5W<OovSpec> {
        Oovh5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Override Value for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oovh6(&mut self) -> Oovh6W<OovSpec> {
        Oovh6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Override Value for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oovh7(&mut self) -> Oovh7W<OovSpec> {
        Oovh7W::new(self, 7)
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
    #[doc = "Bit 20 - Output Override Value for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oovl4(&mut self) -> Oovl4W<OovSpec> {
        Oovl4W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Override Value for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oovl5(&mut self) -> Oovl5W<OovSpec> {
        Oovl5W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Override Value for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oovl6(&mut self) -> Oovl6W<OovSpec> {
        Oovl6W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Override Value for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oovl7(&mut self) -> Oovl7W<OovSpec> {
        Oovl7W::new(self, 23)
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
