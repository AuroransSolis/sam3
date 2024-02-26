#[doc = "Register `HSTPIP` reader"]
pub type R = crate::R<HstpipSpec>;
#[doc = "Register `HSTPIP` writer"]
pub type W = crate::W<HstpipSpec>;
#[doc = "Field `PEN0` reader - Pipe 0 Enable"]
pub type Pen0R = crate::BitReader;
#[doc = "Field `PEN0` writer - Pipe 0 Enable"]
pub type Pen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN1` reader - Pipe 1 Enable"]
pub type Pen1R = crate::BitReader;
#[doc = "Field `PEN1` writer - Pipe 1 Enable"]
pub type Pen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN2` reader - Pipe 2 Enable"]
pub type Pen2R = crate::BitReader;
#[doc = "Field `PEN2` writer - Pipe 2 Enable"]
pub type Pen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN3` reader - Pipe 3 Enable"]
pub type Pen3R = crate::BitReader;
#[doc = "Field `PEN3` writer - Pipe 3 Enable"]
pub type Pen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN4` reader - Pipe 4 Enable"]
pub type Pen4R = crate::BitReader;
#[doc = "Field `PEN4` writer - Pipe 4 Enable"]
pub type Pen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN5` reader - Pipe 5 Enable"]
pub type Pen5R = crate::BitReader;
#[doc = "Field `PEN5` writer - Pipe 5 Enable"]
pub type Pen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN6` reader - Pipe 6 Enable"]
pub type Pen6R = crate::BitReader;
#[doc = "Field `PEN6` writer - Pipe 6 Enable"]
pub type Pen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN7` reader - Pipe 7 Enable"]
pub type Pen7R = crate::BitReader;
#[doc = "Field `PEN7` writer - Pipe 7 Enable"]
pub type Pen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN8` reader - Pipe 8 Enable"]
pub type Pen8R = crate::BitReader;
#[doc = "Field `PEN8` writer - Pipe 8 Enable"]
pub type Pen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST0` reader - Pipe 0 Reset"]
pub type Prst0R = crate::BitReader;
#[doc = "Field `PRST0` writer - Pipe 0 Reset"]
pub type Prst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST1` reader - Pipe 1 Reset"]
pub type Prst1R = crate::BitReader;
#[doc = "Field `PRST1` writer - Pipe 1 Reset"]
pub type Prst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST2` reader - Pipe 2 Reset"]
pub type Prst2R = crate::BitReader;
#[doc = "Field `PRST2` writer - Pipe 2 Reset"]
pub type Prst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST3` reader - Pipe 3 Reset"]
pub type Prst3R = crate::BitReader;
#[doc = "Field `PRST3` writer - Pipe 3 Reset"]
pub type Prst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST4` reader - Pipe 4 Reset"]
pub type Prst4R = crate::BitReader;
#[doc = "Field `PRST4` writer - Pipe 4 Reset"]
pub type Prst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST5` reader - Pipe 5 Reset"]
pub type Prst5R = crate::BitReader;
#[doc = "Field `PRST5` writer - Pipe 5 Reset"]
pub type Prst5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST6` reader - Pipe 6 Reset"]
pub type Prst6R = crate::BitReader;
#[doc = "Field `PRST6` writer - Pipe 6 Reset"]
pub type Prst6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST7` reader - Pipe 7 Reset"]
pub type Prst7R = crate::BitReader;
#[doc = "Field `PRST7` writer - Pipe 7 Reset"]
pub type Prst7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST8` reader - Pipe 8 Reset"]
pub type Prst8R = crate::BitReader;
#[doc = "Field `PRST8` writer - Pipe 8 Reset"]
pub type Prst8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> Pen0R {
        Pen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> Pen1R {
        Pen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> Pen2R {
        Pen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> Pen3R {
        Pen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> Pen4R {
        Pen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> Pen5R {
        Pen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> Pen6R {
        Pen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> Pen7R {
        Pen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    pub fn pen8(&self) -> Pen8R {
        Pen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    pub fn prst0(&self) -> Prst0R {
        Prst0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    pub fn prst1(&self) -> Prst1R {
        Prst1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    pub fn prst2(&self) -> Prst2R {
        Prst2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    pub fn prst3(&self) -> Prst3R {
        Prst3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    pub fn prst4(&self) -> Prst4R {
        Prst4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    pub fn prst5(&self) -> Prst5R {
        Prst5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    pub fn prst6(&self) -> Prst6R {
        Prst6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    pub fn prst7(&self) -> Prst7R {
        Prst7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    pub fn prst8(&self) -> Prst8R {
        Prst8R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen0(&mut self) -> Pen0W<HstpipSpec> {
        Pen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen1(&mut self) -> Pen1W<HstpipSpec> {
        Pen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen2(&mut self) -> Pen2W<HstpipSpec> {
        Pen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen3(&mut self) -> Pen3W<HstpipSpec> {
        Pen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen4(&mut self) -> Pen4W<HstpipSpec> {
        Pen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen5(&mut self) -> Pen5W<HstpipSpec> {
        Pen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen6(&mut self) -> Pen6W<HstpipSpec> {
        Pen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen7(&mut self) -> Pen7W<HstpipSpec> {
        Pen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen8(&mut self) -> Pen8W<HstpipSpec> {
        Pen8W::new(self, 8)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst0(&mut self) -> Prst0W<HstpipSpec> {
        Prst0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst1(&mut self) -> Prst1W<HstpipSpec> {
        Prst1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst2(&mut self) -> Prst2W<HstpipSpec> {
        Prst2W::new(self, 18)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst3(&mut self) -> Prst3W<HstpipSpec> {
        Prst3W::new(self, 19)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst4(&mut self) -> Prst4W<HstpipSpec> {
        Prst4W::new(self, 20)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst5(&mut self) -> Prst5W<HstpipSpec> {
        Prst5W::new(self, 21)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst6(&mut self) -> Prst6W<HstpipSpec> {
        Prst6W::new(self, 22)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst7(&mut self) -> Prst7W<HstpipSpec> {
        Prst7W::new(self, 23)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst8(&mut self) -> Prst8W<HstpipSpec> {
        Prst8W::new(self, 24)
    }
}
#[doc = "Host Pipe Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipSpec;
impl crate::RegisterSpec for HstpipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpip::R`](R) reader structure"]
impl crate::Readable for HstpipSpec {}
#[doc = "`write(|w| ..)` method takes [`hstpip::W`](W) writer structure"]
impl crate::Writable for HstpipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTPIP to value 0"]
impl crate::Resettable for HstpipSpec {
    const RESET_VALUE: u32 = 0;
}
