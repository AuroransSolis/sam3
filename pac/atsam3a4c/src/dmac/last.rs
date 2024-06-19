#[doc = "Register `LAST` reader"]
pub type R = crate::R<LastSpec>;
#[doc = "Register `LAST` writer"]
pub type W = crate::W<LastSpec>;
#[doc = "Field `SLAST0` reader - Source Last"]
pub type Slast0R = crate::BitReader;
#[doc = "Field `SLAST0` writer - Source Last"]
pub type Slast0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAST0` reader - Destination Last"]
pub type Dlast0R = crate::BitReader;
#[doc = "Field `DLAST0` writer - Destination Last"]
pub type Dlast0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAST1` reader - Source Last"]
pub type Slast1R = crate::BitReader;
#[doc = "Field `SLAST1` writer - Source Last"]
pub type Slast1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAST1` reader - Destination Last"]
pub type Dlast1R = crate::BitReader;
#[doc = "Field `DLAST1` writer - Destination Last"]
pub type Dlast1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAST2` reader - Source Last"]
pub type Slast2R = crate::BitReader;
#[doc = "Field `SLAST2` writer - Source Last"]
pub type Slast2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAST2` reader - Destination Last"]
pub type Dlast2R = crate::BitReader;
#[doc = "Field `DLAST2` writer - Destination Last"]
pub type Dlast2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAST3` reader - Source Last"]
pub type Slast3R = crate::BitReader;
#[doc = "Field `SLAST3` writer - Source Last"]
pub type Slast3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAST3` reader - Destination Last"]
pub type Dlast3R = crate::BitReader;
#[doc = "Field `DLAST3` writer - Destination Last"]
pub type Dlast3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAST4` reader - Source Last"]
pub type Slast4R = crate::BitReader;
#[doc = "Field `SLAST4` writer - Source Last"]
pub type Slast4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAST4` reader - Destination Last"]
pub type Dlast4R = crate::BitReader;
#[doc = "Field `DLAST4` writer - Destination Last"]
pub type Dlast4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAST5` reader - Source Last"]
pub type Slast5R = crate::BitReader;
#[doc = "Field `SLAST5` writer - Source Last"]
pub type Slast5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAST5` reader - Destination Last"]
pub type Dlast5R = crate::BitReader;
#[doc = "Field `DLAST5` writer - Destination Last"]
pub type Dlast5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&self) -> Slast0R {
        Slast0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&self) -> Dlast0R {
        Dlast0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&self) -> Slast1R {
        Slast1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&self) -> Dlast1R {
        Dlast1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&self) -> Slast2R {
        Slast2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&self) -> Dlast2R {
        Dlast2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&self) -> Slast3R {
        Slast3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&self) -> Dlast3R {
        Dlast3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&self) -> Slast4R {
        Slast4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&self) -> Dlast4R {
        Dlast4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&self) -> Slast5R {
        Slast5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&self) -> Dlast5R {
        Dlast5R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    #[must_use]
    pub fn slast0(&mut self) -> Slast0W<LastSpec> {
        Slast0W::new(self, 0)
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    #[must_use]
    pub fn dlast0(&mut self) -> Dlast0W<LastSpec> {
        Dlast0W::new(self, 1)
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    #[must_use]
    pub fn slast1(&mut self) -> Slast1W<LastSpec> {
        Slast1W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    #[must_use]
    pub fn dlast1(&mut self) -> Dlast1W<LastSpec> {
        Dlast1W::new(self, 3)
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    #[must_use]
    pub fn slast2(&mut self) -> Slast2W<LastSpec> {
        Slast2W::new(self, 4)
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    #[must_use]
    pub fn dlast2(&mut self) -> Dlast2W<LastSpec> {
        Dlast2W::new(self, 5)
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    #[must_use]
    pub fn slast3(&mut self) -> Slast3W<LastSpec> {
        Slast3W::new(self, 6)
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    #[must_use]
    pub fn dlast3(&mut self) -> Dlast3W<LastSpec> {
        Dlast3W::new(self, 7)
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    #[must_use]
    pub fn slast4(&mut self) -> Slast4W<LastSpec> {
        Slast4W::new(self, 8)
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    #[must_use]
    pub fn dlast4(&mut self) -> Dlast4W<LastSpec> {
        Dlast4W::new(self, 9)
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    #[must_use]
    pub fn slast5(&mut self) -> Slast5W<LastSpec> {
        Slast5W::new(self, 10)
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    #[must_use]
    pub fn dlast5(&mut self) -> Dlast5W<LastSpec> {
        Dlast5W::new(self, 11)
    }
}
#[doc = "DMAC Software Last Transfer Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`last::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LastSpec;
impl crate::RegisterSpec for LastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`last::R`](R) reader structure"]
impl crate::Readable for LastSpec {}
#[doc = "`write(|w| ..)` method takes [`last::W`](W) writer structure"]
impl crate::Writable for LastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAST to value 0"]
impl crate::Resettable for LastSpec {
    const RESET_VALUE: u32 = 0;
}
