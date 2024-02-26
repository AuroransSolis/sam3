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
