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
}
#[doc = "DMAC Software Single Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
