#[doc = "Register `DEVEPT` reader"]
pub type R = crate::R<DeveptSpec>;
#[doc = "Register `DEVEPT` writer"]
pub type W = crate::W<DeveptSpec>;
#[doc = "Field `EPEN0` reader - Endpoint 0 Enable"]
pub type Epen0R = crate::BitReader;
#[doc = "Field `EPEN0` writer - Endpoint 0 Enable"]
pub type Epen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN1` reader - Endpoint 1 Enable"]
pub type Epen1R = crate::BitReader;
#[doc = "Field `EPEN1` writer - Endpoint 1 Enable"]
pub type Epen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN2` reader - Endpoint 2 Enable"]
pub type Epen2R = crate::BitReader;
#[doc = "Field `EPEN2` writer - Endpoint 2 Enable"]
pub type Epen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN3` reader - Endpoint 3 Enable"]
pub type Epen3R = crate::BitReader;
#[doc = "Field `EPEN3` writer - Endpoint 3 Enable"]
pub type Epen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN4` reader - Endpoint 4 Enable"]
pub type Epen4R = crate::BitReader;
#[doc = "Field `EPEN4` writer - Endpoint 4 Enable"]
pub type Epen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN5` reader - Endpoint 5 Enable"]
pub type Epen5R = crate::BitReader;
#[doc = "Field `EPEN5` writer - Endpoint 5 Enable"]
pub type Epen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN6` reader - Endpoint 6 Enable"]
pub type Epen6R = crate::BitReader;
#[doc = "Field `EPEN6` writer - Endpoint 6 Enable"]
pub type Epen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN7` reader - Endpoint 7 Enable"]
pub type Epen7R = crate::BitReader;
#[doc = "Field `EPEN7` writer - Endpoint 7 Enable"]
pub type Epen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN8` reader - Endpoint 8 Enable"]
pub type Epen8R = crate::BitReader;
#[doc = "Field `EPEN8` writer - Endpoint 8 Enable"]
pub type Epen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST0` reader - Endpoint 0 Reset"]
pub type Eprst0R = crate::BitReader;
#[doc = "Field `EPRST0` writer - Endpoint 0 Reset"]
pub type Eprst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST1` reader - Endpoint 1 Reset"]
pub type Eprst1R = crate::BitReader;
#[doc = "Field `EPRST1` writer - Endpoint 1 Reset"]
pub type Eprst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST2` reader - Endpoint 2 Reset"]
pub type Eprst2R = crate::BitReader;
#[doc = "Field `EPRST2` writer - Endpoint 2 Reset"]
pub type Eprst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST3` reader - Endpoint 3 Reset"]
pub type Eprst3R = crate::BitReader;
#[doc = "Field `EPRST3` writer - Endpoint 3 Reset"]
pub type Eprst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST4` reader - Endpoint 4 Reset"]
pub type Eprst4R = crate::BitReader;
#[doc = "Field `EPRST4` writer - Endpoint 4 Reset"]
pub type Eprst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST5` reader - Endpoint 5 Reset"]
pub type Eprst5R = crate::BitReader;
#[doc = "Field `EPRST5` writer - Endpoint 5 Reset"]
pub type Eprst5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST6` reader - Endpoint 6 Reset"]
pub type Eprst6R = crate::BitReader;
#[doc = "Field `EPRST6` writer - Endpoint 6 Reset"]
pub type Eprst6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST7` reader - Endpoint 7 Reset"]
pub type Eprst7R = crate::BitReader;
#[doc = "Field `EPRST7` writer - Endpoint 7 Reset"]
pub type Eprst7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRST8` reader - Endpoint 8 Reset"]
pub type Eprst8R = crate::BitReader;
#[doc = "Field `EPRST8` writer - Endpoint 8 Reset"]
pub type Eprst8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> Epen0R {
        Epen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> Epen1R {
        Epen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> Epen2R {
        Epen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> Epen3R {
        Epen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> Epen4R {
        Epen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> Epen5R {
        Epen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> Epen6R {
        Epen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> Epen7R {
        Epen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&self) -> Epen8R {
        Epen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&self) -> Eprst0R {
        Eprst0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&self) -> Eprst1R {
        Eprst1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&self) -> Eprst2R {
        Eprst2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&self) -> Eprst3R {
        Eprst3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&self) -> Eprst4R {
        Eprst4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&self) -> Eprst5R {
        Eprst5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&self) -> Eprst6R {
        Eprst6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&self) -> Eprst7R {
        Eprst7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&self) -> Eprst8R {
        Eprst8R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen0(&mut self) -> Epen0W<DeveptSpec> {
        Epen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen1(&mut self) -> Epen1W<DeveptSpec> {
        Epen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen2(&mut self) -> Epen2W<DeveptSpec> {
        Epen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen3(&mut self) -> Epen3W<DeveptSpec> {
        Epen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen4(&mut self) -> Epen4W<DeveptSpec> {
        Epen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen5(&mut self) -> Epen5W<DeveptSpec> {
        Epen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen6(&mut self) -> Epen6W<DeveptSpec> {
        Epen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen7(&mut self) -> Epen7W<DeveptSpec> {
        Epen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen8(&mut self) -> Epen8W<DeveptSpec> {
        Epen8W::new(self, 8)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst0(&mut self) -> Eprst0W<DeveptSpec> {
        Eprst0W::new(self, 16)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst1(&mut self) -> Eprst1W<DeveptSpec> {
        Eprst1W::new(self, 17)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst2(&mut self) -> Eprst2W<DeveptSpec> {
        Eprst2W::new(self, 18)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst3(&mut self) -> Eprst3W<DeveptSpec> {
        Eprst3W::new(self, 19)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst4(&mut self) -> Eprst4W<DeveptSpec> {
        Eprst4W::new(self, 20)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst5(&mut self) -> Eprst5W<DeveptSpec> {
        Eprst5W::new(self, 21)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst6(&mut self) -> Eprst6W<DeveptSpec> {
        Eprst6W::new(self, 22)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst7(&mut self) -> Eprst7W<DeveptSpec> {
        Eprst7W::new(self, 23)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst8(&mut self) -> Eprst8W<DeveptSpec> {
        Eprst8W::new(self, 24)
    }
}
#[doc = "Device Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devept::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devept::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptSpec;
impl crate::RegisterSpec for DeveptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devept::R`](R) reader structure"]
impl crate::Readable for DeveptSpec {}
#[doc = "`write(|w| ..)` method takes [`devept::W`](W) writer structure"]
impl crate::Writable for DeveptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVEPT to value 0"]
impl crate::Resettable for DeveptSpec {
    const RESET_VALUE: u32 = 0;
}
