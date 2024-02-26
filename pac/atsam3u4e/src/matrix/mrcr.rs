#[doc = "Register `MRCR` reader"]
pub type R = crate::R<MrcrSpec>;
#[doc = "Register `MRCR` writer"]
pub type W = crate::W<MrcrSpec>;
#[doc = "Field `RCB0` reader - Remap Command Bit for AHB Master 0"]
pub type Rcb0R = crate::BitReader;
#[doc = "Field `RCB0` writer - Remap Command Bit for AHB Master 0"]
pub type Rcb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB1` reader - Remap Command Bit for AHB Master 1"]
pub type Rcb1R = crate::BitReader;
#[doc = "Field `RCB1` writer - Remap Command Bit for AHB Master 1"]
pub type Rcb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB2` reader - Remap Command Bit for AHB Master 2"]
pub type Rcb2R = crate::BitReader;
#[doc = "Field `RCB2` writer - Remap Command Bit for AHB Master 2"]
pub type Rcb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB3` reader - Remap Command Bit for AHB Master 3"]
pub type Rcb3R = crate::BitReader;
#[doc = "Field `RCB3` writer - Remap Command Bit for AHB Master 3"]
pub type Rcb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB4` reader - Remap Command Bit for AHB Master 4"]
pub type Rcb4R = crate::BitReader;
#[doc = "Field `RCB4` writer - Remap Command Bit for AHB Master 4"]
pub type Rcb4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remap Command Bit for AHB Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> Rcb0R {
        Rcb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for AHB Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> Rcb1R {
        Rcb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for AHB Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> Rcb2R {
        Rcb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for AHB Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> Rcb3R {
        Rcb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for AHB Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> Rcb4R {
        Rcb4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for AHB Master 0"]
    #[inline(always)]
    #[must_use]
    pub fn rcb0(&mut self) -> Rcb0W<MrcrSpec> {
        Rcb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for AHB Master 1"]
    #[inline(always)]
    #[must_use]
    pub fn rcb1(&mut self) -> Rcb1W<MrcrSpec> {
        Rcb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Remap Command Bit for AHB Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn rcb2(&mut self) -> Rcb2W<MrcrSpec> {
        Rcb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Remap Command Bit for AHB Master 3"]
    #[inline(always)]
    #[must_use]
    pub fn rcb3(&mut self) -> Rcb3W<MrcrSpec> {
        Rcb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Remap Command Bit for AHB Master 4"]
    #[inline(always)]
    #[must_use]
    pub fn rcb4(&mut self) -> Rcb4W<MrcrSpec> {
        Rcb4W::new(self, 4)
    }
}
#[doc = "Master Remap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrcrSpec;
impl crate::RegisterSpec for MrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcr::R`](R) reader structure"]
impl crate::Readable for MrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcr::W`](W) writer structure"]
impl crate::Writable for MrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MrcrSpec {
    const RESET_VALUE: u32 = 0;
}
