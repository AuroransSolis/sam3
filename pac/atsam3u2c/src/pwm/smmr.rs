#[doc = "Register `SMMR` reader"]
pub type R = crate::R<SmmrSpec>;
#[doc = "Register `SMMR` writer"]
pub type W = crate::W<SmmrSpec>;
#[doc = "Field `GCEN0` reader - Gray Count ENable"]
pub type Gcen0R = crate::BitReader;
#[doc = "Field `GCEN0` writer - Gray Count ENable"]
pub type Gcen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCEN1` reader - Gray Count ENable"]
pub type Gcen1R = crate::BitReader;
#[doc = "Field `GCEN1` writer - Gray Count ENable"]
pub type Gcen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN0` reader - DOWN Count"]
pub type Down0R = crate::BitReader;
#[doc = "Field `DOWN0` writer - DOWN Count"]
pub type Down0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN1` reader - DOWN Count"]
pub type Down1R = crate::BitReader;
#[doc = "Field `DOWN1` writer - DOWN Count"]
pub type Down1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen0(&self) -> Gcen0R {
        Gcen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen1(&self) -> Gcen1R {
        Gcen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    pub fn down0(&self) -> Down0R {
        Down0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    pub fn down1(&self) -> Down1R {
        Down1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen0(&mut self) -> Gcen0W<SmmrSpec> {
        Gcen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen1(&mut self) -> Gcen1W<SmmrSpec> {
        Gcen1W::new(self, 1)
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    #[must_use]
    pub fn down0(&mut self) -> Down0W<SmmrSpec> {
        Down0W::new(self, 16)
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    #[must_use]
    pub fn down1(&mut self) -> Down1W<SmmrSpec> {
        Down1W::new(self, 17)
    }
}
#[doc = "PWM Stepper Motor Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmmrSpec;
impl crate::RegisterSpec for SmmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr::R`](R) reader structure"]
impl crate::Readable for SmmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smmr::W`](W) writer structure"]
impl crate::Writable for SmmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SmmrSpec {
    const RESET_VALUE: u32 = 0;
}
