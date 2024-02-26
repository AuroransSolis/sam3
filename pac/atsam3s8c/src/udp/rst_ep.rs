#[doc = "Register `RST_EP` reader"]
pub type R = crate::R<RstEpSpec>;
#[doc = "Register `RST_EP` writer"]
pub type W = crate::W<RstEpSpec>;
#[doc = "Field `EP0` reader - Reset Endpoint 0"]
pub type Ep0R = crate::BitReader;
#[doc = "Field `EP0` writer - Reset Endpoint 0"]
pub type Ep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1` reader - Reset Endpoint 1"]
pub type Ep1R = crate::BitReader;
#[doc = "Field `EP1` writer - Reset Endpoint 1"]
pub type Ep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2` reader - Reset Endpoint 2"]
pub type Ep2R = crate::BitReader;
#[doc = "Field `EP2` writer - Reset Endpoint 2"]
pub type Ep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3` reader - Reset Endpoint 3"]
pub type Ep3R = crate::BitReader;
#[doc = "Field `EP3` writer - Reset Endpoint 3"]
pub type Ep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4` reader - Reset Endpoint 4"]
pub type Ep4R = crate::BitReader;
#[doc = "Field `EP4` writer - Reset Endpoint 4"]
pub type Ep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5` reader - Reset Endpoint 5"]
pub type Ep5R = crate::BitReader;
#[doc = "Field `EP5` writer - Reset Endpoint 5"]
pub type Ep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6` reader - Reset Endpoint 6"]
pub type Ep6R = crate::BitReader;
#[doc = "Field `EP6` writer - Reset Endpoint 6"]
pub type Ep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7` reader - Reset Endpoint 7"]
pub type Ep7R = crate::BitReader;
#[doc = "Field `EP7` writer - Reset Endpoint 7"]
pub type Ep7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset Endpoint 0"]
    #[inline(always)]
    pub fn ep0(&self) -> Ep0R {
        Ep0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset Endpoint 1"]
    #[inline(always)]
    pub fn ep1(&self) -> Ep1R {
        Ep1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Endpoint 2"]
    #[inline(always)]
    pub fn ep2(&self) -> Ep2R {
        Ep2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Endpoint 3"]
    #[inline(always)]
    pub fn ep3(&self) -> Ep3R {
        Ep3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Endpoint 4"]
    #[inline(always)]
    pub fn ep4(&self) -> Ep4R {
        Ep4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset Endpoint 5"]
    #[inline(always)]
    pub fn ep5(&self) -> Ep5R {
        Ep5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Endpoint 6"]
    #[inline(always)]
    pub fn ep6(&self) -> Ep6R {
        Ep6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset Endpoint 7"]
    #[inline(always)]
    pub fn ep7(&self) -> Ep7R {
        Ep7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Endpoint 0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0(&mut self) -> Ep0W<RstEpSpec> {
        Ep0W::new(self, 0)
    }
    #[doc = "Bit 1 - Reset Endpoint 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep1(&mut self) -> Ep1W<RstEpSpec> {
        Ep1W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset Endpoint 2"]
    #[inline(always)]
    #[must_use]
    pub fn ep2(&mut self) -> Ep2W<RstEpSpec> {
        Ep2W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Endpoint 3"]
    #[inline(always)]
    #[must_use]
    pub fn ep3(&mut self) -> Ep3W<RstEpSpec> {
        Ep3W::new(self, 3)
    }
    #[doc = "Bit 4 - Reset Endpoint 4"]
    #[inline(always)]
    #[must_use]
    pub fn ep4(&mut self) -> Ep4W<RstEpSpec> {
        Ep4W::new(self, 4)
    }
    #[doc = "Bit 5 - Reset Endpoint 5"]
    #[inline(always)]
    #[must_use]
    pub fn ep5(&mut self) -> Ep5W<RstEpSpec> {
        Ep5W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset Endpoint 6"]
    #[inline(always)]
    #[must_use]
    pub fn ep6(&mut self) -> Ep6W<RstEpSpec> {
        Ep6W::new(self, 6)
    }
    #[doc = "Bit 7 - Reset Endpoint 7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7(&mut self) -> Ep7W<RstEpSpec> {
        Ep7W::new(self, 7)
    }
}
#[doc = "Reset Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_ep::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_ep::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstEpSpec;
impl crate::RegisterSpec for RstEpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_ep::R`](R) reader structure"]
impl crate::Readable for RstEpSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_ep::W`](W) writer structure"]
impl crate::Writable for RstEpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_EP to value 0"]
impl crate::Resettable for RstEpSpec {
    const RESET_VALUE: u32 = 0;
}
