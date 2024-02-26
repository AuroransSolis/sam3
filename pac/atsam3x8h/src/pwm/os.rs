#[doc = "Register `OS` reader"]
pub type R = crate::R<OsSpec>;
#[doc = "Register `OS` writer"]
pub type W = crate::W<OsSpec>;
#[doc = "Field `OSH0` reader - Output Selection for PWMH output of the channel 0"]
pub type Osh0R = crate::BitReader;
#[doc = "Field `OSH0` writer - Output Selection for PWMH output of the channel 0"]
pub type Osh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH1` reader - Output Selection for PWMH output of the channel 1"]
pub type Osh1R = crate::BitReader;
#[doc = "Field `OSH1` writer - Output Selection for PWMH output of the channel 1"]
pub type Osh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH2` reader - Output Selection for PWMH output of the channel 2"]
pub type Osh2R = crate::BitReader;
#[doc = "Field `OSH2` writer - Output Selection for PWMH output of the channel 2"]
pub type Osh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH3` reader - Output Selection for PWMH output of the channel 3"]
pub type Osh3R = crate::BitReader;
#[doc = "Field `OSH3` writer - Output Selection for PWMH output of the channel 3"]
pub type Osh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH4` reader - Output Selection for PWMH output of the channel 4"]
pub type Osh4R = crate::BitReader;
#[doc = "Field `OSH4` writer - Output Selection for PWMH output of the channel 4"]
pub type Osh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH5` reader - Output Selection for PWMH output of the channel 5"]
pub type Osh5R = crate::BitReader;
#[doc = "Field `OSH5` writer - Output Selection for PWMH output of the channel 5"]
pub type Osh5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH6` reader - Output Selection for PWMH output of the channel 6"]
pub type Osh6R = crate::BitReader;
#[doc = "Field `OSH6` writer - Output Selection for PWMH output of the channel 6"]
pub type Osh6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSH7` reader - Output Selection for PWMH output of the channel 7"]
pub type Osh7R = crate::BitReader;
#[doc = "Field `OSH7` writer - Output Selection for PWMH output of the channel 7"]
pub type Osh7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL0` reader - Output Selection for PWML output of the channel 0"]
pub type Osl0R = crate::BitReader;
#[doc = "Field `OSL0` writer - Output Selection for PWML output of the channel 0"]
pub type Osl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL1` reader - Output Selection for PWML output of the channel 1"]
pub type Osl1R = crate::BitReader;
#[doc = "Field `OSL1` writer - Output Selection for PWML output of the channel 1"]
pub type Osl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL2` reader - Output Selection for PWML output of the channel 2"]
pub type Osl2R = crate::BitReader;
#[doc = "Field `OSL2` writer - Output Selection for PWML output of the channel 2"]
pub type Osl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL3` reader - Output Selection for PWML output of the channel 3"]
pub type Osl3R = crate::BitReader;
#[doc = "Field `OSL3` writer - Output Selection for PWML output of the channel 3"]
pub type Osl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL4` reader - Output Selection for PWML output of the channel 4"]
pub type Osl4R = crate::BitReader;
#[doc = "Field `OSL4` writer - Output Selection for PWML output of the channel 4"]
pub type Osl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL5` reader - Output Selection for PWML output of the channel 5"]
pub type Osl5R = crate::BitReader;
#[doc = "Field `OSL5` writer - Output Selection for PWML output of the channel 5"]
pub type Osl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL6` reader - Output Selection for PWML output of the channel 6"]
pub type Osl6R = crate::BitReader;
#[doc = "Field `OSL6` writer - Output Selection for PWML output of the channel 6"]
pub type Osl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSL7` reader - Output Selection for PWML output of the channel 7"]
pub type Osl7R = crate::BitReader;
#[doc = "Field `OSL7` writer - Output Selection for PWML output of the channel 7"]
pub type Osl7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&self) -> Osh0R {
        Osh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&self) -> Osh1R {
        Osh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&self) -> Osh2R {
        Osh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&self) -> Osh3R {
        Osh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Selection for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn osh4(&self) -> Osh4R {
        Osh4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Selection for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn osh5(&self) -> Osh5R {
        Osh5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Selection for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn osh6(&self) -> Osh6R {
        Osh6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Selection for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn osh7(&self) -> Osh7R {
        Osh7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&self) -> Osl0R {
        Osl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&self) -> Osl1R {
        Osl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&self) -> Osl2R {
        Osl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&self) -> Osl3R {
        Osl3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output Selection for PWML output of the channel 4"]
    #[inline(always)]
    pub fn osl4(&self) -> Osl4R {
        Osl4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Selection for PWML output of the channel 5"]
    #[inline(always)]
    pub fn osl5(&self) -> Osl5R {
        Osl5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output Selection for PWML output of the channel 6"]
    #[inline(always)]
    pub fn osl6(&self) -> Osl6R {
        Osl6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output Selection for PWML output of the channel 7"]
    #[inline(always)]
    pub fn osl7(&self) -> Osl7R {
        Osl7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osh0(&mut self) -> Osh0W<OsSpec> {
        Osh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osh1(&mut self) -> Osh1W<OsSpec> {
        Osh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osh2(&mut self) -> Osh2W<OsSpec> {
        Osh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osh3(&mut self) -> Osh3W<OsSpec> {
        Osh3W::new(self, 3)
    }
    #[doc = "Bit 4 - Output Selection for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn osh4(&mut self) -> Osh4W<OsSpec> {
        Osh4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Selection for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn osh5(&mut self) -> Osh5W<OsSpec> {
        Osh5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Selection for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn osh6(&mut self) -> Osh6W<OsSpec> {
        Osh6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Selection for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn osh7(&mut self) -> Osh7W<OsSpec> {
        Osh7W::new(self, 7)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osl0(&mut self) -> Osl0W<OsSpec> {
        Osl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osl1(&mut self) -> Osl1W<OsSpec> {
        Osl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osl2(&mut self) -> Osl2W<OsSpec> {
        Osl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osl3(&mut self) -> Osl3W<OsSpec> {
        Osl3W::new(self, 19)
    }
    #[doc = "Bit 20 - Output Selection for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn osl4(&mut self) -> Osl4W<OsSpec> {
        Osl4W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Selection for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn osl5(&mut self) -> Osl5W<OsSpec> {
        Osl5W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Selection for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn osl6(&mut self) -> Osl6W<OsSpec> {
        Osl6W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Selection for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn osl7(&mut self) -> Osl7W<OsSpec> {
        Osl7W::new(self, 23)
    }
}
#[doc = "PWM Output Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsSpec;
impl crate::RegisterSpec for OsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`os::R`](R) reader structure"]
impl crate::Readable for OsSpec {}
#[doc = "`write(|w| ..)` method takes [`os::W`](W) writer structure"]
impl crate::Writable for OsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OS to value 0"]
impl crate::Resettable for OsSpec {
    const RESET_VALUE: u32 = 0;
}
