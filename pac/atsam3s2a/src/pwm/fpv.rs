#[doc = "Register `FPV` reader"]
pub type R = crate::R<FpvSpec>;
#[doc = "Register `FPV` writer"]
pub type W = crate::W<FpvSpec>;
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub type Fpvh0R = crate::BitReader;
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub type Fpvh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub type Fpvh1R = crate::BitReader;
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub type Fpvh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub type Fpvh2R = crate::BitReader;
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub type Fpvh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub type Fpvh3R = crate::BitReader;
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub type Fpvh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub type Fpvl0R = crate::BitReader;
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub type Fpvl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub type Fpvl1R = crate::BitReader;
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub type Fpvl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub type Fpvl2R = crate::BitReader;
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub type Fpvl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub type Fpvl3R = crate::BitReader;
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub type Fpvl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> Fpvh0R {
        Fpvh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> Fpvh1R {
        Fpvh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> Fpvh2R {
        Fpvh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> Fpvh3R {
        Fpvh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> Fpvl0R {
        Fpvl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> Fpvl1R {
        Fpvl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> Fpvl2R {
        Fpvl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> Fpvl3R {
        Fpvl3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh0(&mut self) -> Fpvh0W<FpvSpec> {
        Fpvh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh1(&mut self) -> Fpvh1W<FpvSpec> {
        Fpvh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh2(&mut self) -> Fpvh2W<FpvSpec> {
        Fpvh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh3(&mut self) -> Fpvh3W<FpvSpec> {
        Fpvh3W::new(self, 3)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl0(&mut self) -> Fpvl0W<FpvSpec> {
        Fpvl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl1(&mut self) -> Fpvl1W<FpvSpec> {
        Fpvl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl2(&mut self) -> Fpvl2W<FpvSpec> {
        Fpvl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl3(&mut self) -> Fpvl3W<FpvSpec> {
        Fpvl3W::new(self, 19)
    }
}
#[doc = "PWM Fault Protection Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpvSpec;
impl crate::RegisterSpec for FpvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpv::R`](R) reader structure"]
impl crate::Readable for FpvSpec {}
#[doc = "`write(|w| ..)` method takes [`fpv::W`](W) writer structure"]
impl crate::Writable for FpvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPV to value 0"]
impl crate::Resettable for FpvSpec {
    const RESET_VALUE: u32 = 0;
}
