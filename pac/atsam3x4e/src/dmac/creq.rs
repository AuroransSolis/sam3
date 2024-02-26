#[doc = "Register `CREQ` reader"]
pub type R = crate::R<CreqSpec>;
#[doc = "Register `CREQ` writer"]
pub type W = crate::W<CreqSpec>;
#[doc = "Field `SCREQ0` reader - Source Chunk Request"]
pub type Screq0R = crate::BitReader;
#[doc = "Field `SCREQ0` writer - Source Chunk Request"]
pub type Screq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCREQ0` reader - Destination Chunk Request"]
pub type Dcreq0R = crate::BitReader;
#[doc = "Field `DCREQ0` writer - Destination Chunk Request"]
pub type Dcreq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCREQ1` reader - Source Chunk Request"]
pub type Screq1R = crate::BitReader;
#[doc = "Field `SCREQ1` writer - Source Chunk Request"]
pub type Screq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCREQ1` reader - Destination Chunk Request"]
pub type Dcreq1R = crate::BitReader;
#[doc = "Field `DCREQ1` writer - Destination Chunk Request"]
pub type Dcreq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCREQ2` reader - Source Chunk Request"]
pub type Screq2R = crate::BitReader;
#[doc = "Field `SCREQ2` writer - Source Chunk Request"]
pub type Screq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCREQ2` reader - Destination Chunk Request"]
pub type Dcreq2R = crate::BitReader;
#[doc = "Field `DCREQ2` writer - Destination Chunk Request"]
pub type Dcreq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCREQ3` reader - Source Chunk Request"]
pub type Screq3R = crate::BitReader;
#[doc = "Field `SCREQ3` writer - Source Chunk Request"]
pub type Screq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCREQ3` reader - Destination Chunk Request"]
pub type Dcreq3R = crate::BitReader;
#[doc = "Field `DCREQ3` writer - Destination Chunk Request"]
pub type Dcreq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCREQ4` reader - Source Chunk Request"]
pub type Screq4R = crate::BitReader;
#[doc = "Field `SCREQ4` writer - Source Chunk Request"]
pub type Screq4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCREQ4` reader - Destination Chunk Request"]
pub type Dcreq4R = crate::BitReader;
#[doc = "Field `DCREQ4` writer - Destination Chunk Request"]
pub type Dcreq4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCREQ5` reader - Source Chunk Request"]
pub type Screq5R = crate::BitReader;
#[doc = "Field `SCREQ5` writer - Source Chunk Request"]
pub type Screq5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCREQ5` reader - Destination Chunk Request"]
pub type Dcreq5R = crate::BitReader;
#[doc = "Field `DCREQ5` writer - Destination Chunk Request"]
pub type Dcreq5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&self) -> Screq0R {
        Screq0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&self) -> Dcreq0R {
        Dcreq0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&self) -> Screq1R {
        Screq1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&self) -> Dcreq1R {
        Dcreq1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&self) -> Screq2R {
        Screq2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&self) -> Dcreq2R {
        Dcreq2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&self) -> Screq3R {
        Screq3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&self) -> Dcreq3R {
        Dcreq3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&self) -> Screq4R {
        Screq4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&self) -> Dcreq4R {
        Dcreq4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&self) -> Screq5R {
        Screq5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&self) -> Dcreq5R {
        Dcreq5R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn screq0(&mut self) -> Screq0W<CreqSpec> {
        Screq0W::new(self, 0)
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn dcreq0(&mut self) -> Dcreq0W<CreqSpec> {
        Dcreq0W::new(self, 1)
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn screq1(&mut self) -> Screq1W<CreqSpec> {
        Screq1W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn dcreq1(&mut self) -> Dcreq1W<CreqSpec> {
        Dcreq1W::new(self, 3)
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn screq2(&mut self) -> Screq2W<CreqSpec> {
        Screq2W::new(self, 4)
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn dcreq2(&mut self) -> Dcreq2W<CreqSpec> {
        Dcreq2W::new(self, 5)
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn screq3(&mut self) -> Screq3W<CreqSpec> {
        Screq3W::new(self, 6)
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn dcreq3(&mut self) -> Dcreq3W<CreqSpec> {
        Dcreq3W::new(self, 7)
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn screq4(&mut self) -> Screq4W<CreqSpec> {
        Screq4W::new(self, 8)
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn dcreq4(&mut self) -> Dcreq4W<CreqSpec> {
        Dcreq4W::new(self, 9)
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn screq5(&mut self) -> Screq5W<CreqSpec> {
        Screq5W::new(self, 10)
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    #[must_use]
    pub fn dcreq5(&mut self) -> Dcreq5W<CreqSpec> {
        Dcreq5W::new(self, 11)
    }
}
#[doc = "DMAC Software Chunk Transfer Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`creq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`creq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CreqSpec;
impl crate::RegisterSpec for CreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`creq::R`](R) reader structure"]
impl crate::Readable for CreqSpec {}
#[doc = "`write(|w| ..)` method takes [`creq::W`](W) writer structure"]
impl crate::Writable for CreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CREQ to value 0"]
impl crate::Resettable for CreqSpec {
    const RESET_VALUE: u32 = 0;
}
