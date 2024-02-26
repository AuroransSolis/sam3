#[doc = "Register `SREQ` reader"]
pub type R = crate::R<SreqSpec>;
#[doc = "Register `SREQ` writer"]
pub type W = crate::W<SreqSpec>;
#[doc = "Field `SSREQ0` reader - Source Request"]
pub type Ssreq0R = crate::BitReader;
#[doc = "Field `SSREQ0` writer - Source Request"]
pub type Ssreq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ0` reader - Destination Request"]
pub type Dsreq0R = crate::BitReader;
#[doc = "Field `DSREQ0` writer - Destination Request"]
pub type Dsreq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ1` reader - Source Request"]
pub type Ssreq1R = crate::BitReader;
#[doc = "Field `SSREQ1` writer - Source Request"]
pub type Ssreq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ1` reader - Destination Request"]
pub type Dsreq1R = crate::BitReader;
#[doc = "Field `DSREQ1` writer - Destination Request"]
pub type Dsreq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ2` reader - Source Request"]
pub type Ssreq2R = crate::BitReader;
#[doc = "Field `SSREQ2` writer - Source Request"]
pub type Ssreq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ2` reader - Destination Request"]
pub type Dsreq2R = crate::BitReader;
#[doc = "Field `DSREQ2` writer - Destination Request"]
pub type Dsreq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ3` reader - Source Request"]
pub type Ssreq3R = crate::BitReader;
#[doc = "Field `SSREQ3` writer - Source Request"]
pub type Ssreq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ3` reader - Destination Request"]
pub type Dsreq3R = crate::BitReader;
#[doc = "Field `DSREQ3` writer - Destination Request"]
pub type Dsreq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ4` reader - Source Request"]
pub type Ssreq4R = crate::BitReader;
#[doc = "Field `SSREQ4` writer - Source Request"]
pub type Ssreq4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ4` reader - Destination Request"]
pub type Dsreq4R = crate::BitReader;
#[doc = "Field `DSREQ4` writer - Destination Request"]
pub type Dsreq4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ5` reader - Source Request"]
pub type Ssreq5R = crate::BitReader;
#[doc = "Field `SSREQ5` writer - Source Request"]
pub type Ssreq5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ5` reader - Destination Request"]
pub type Dsreq5R = crate::BitReader;
#[doc = "Field `DSREQ5` writer - Destination Request"]
pub type Dsreq5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&self) -> Ssreq0R {
        Ssreq0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&self) -> Dsreq0R {
        Dsreq0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&self) -> Ssreq1R {
        Ssreq1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&self) -> Dsreq1R {
        Dsreq1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&self) -> Ssreq2R {
        Ssreq2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&self) -> Dsreq2R {
        Dsreq2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&self) -> Ssreq3R {
        Ssreq3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&self) -> Dsreq3R {
        Dsreq3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&self) -> Ssreq4R {
        Ssreq4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&self) -> Dsreq4R {
        Dsreq4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&self) -> Ssreq5R {
        Ssreq5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&self) -> Dsreq5R {
        Dsreq5R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq0(&mut self) -> Ssreq0W<SreqSpec> {
        Ssreq0W::new(self, 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq0(&mut self) -> Dsreq0W<SreqSpec> {
        Dsreq0W::new(self, 1)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq1(&mut self) -> Ssreq1W<SreqSpec> {
        Ssreq1W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq1(&mut self) -> Dsreq1W<SreqSpec> {
        Dsreq1W::new(self, 3)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq2(&mut self) -> Ssreq2W<SreqSpec> {
        Ssreq2W::new(self, 4)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq2(&mut self) -> Dsreq2W<SreqSpec> {
        Dsreq2W::new(self, 5)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq3(&mut self) -> Ssreq3W<SreqSpec> {
        Ssreq3W::new(self, 6)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq3(&mut self) -> Dsreq3W<SreqSpec> {
        Dsreq3W::new(self, 7)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq4(&mut self) -> Ssreq4W<SreqSpec> {
        Ssreq4W::new(self, 8)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq4(&mut self) -> Dsreq4W<SreqSpec> {
        Dsreq4W::new(self, 9)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq5(&mut self) -> Ssreq5W<SreqSpec> {
        Ssreq5W::new(self, 10)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq5(&mut self) -> Dsreq5W<SreqSpec> {
        Dsreq5W::new(self, 11)
    }
}
#[doc = "DMAC Software Single Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SreqSpec;
impl crate::RegisterSpec for SreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sreq::R`](R) reader structure"]
impl crate::Readable for SreqSpec {}
#[doc = "`write(|w| ..)` method takes [`sreq::W`](W) writer structure"]
impl crate::Writable for SreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SREQ to value 0"]
impl crate::Resettable for SreqSpec {
    const RESET_VALUE: u32 = 0;
}
