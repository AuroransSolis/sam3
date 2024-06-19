#[doc = "Register `ELMR1` reader"]
pub type R = crate::R<Elmr1Spec>;
#[doc = "Register `ELMR1` writer"]
pub type W = crate::W<Elmr1Spec>;
#[doc = "Field `CSEL0` reader - Comparison 0 Selection"]
pub type Csel0R = crate::BitReader;
#[doc = "Field `CSEL0` writer - Comparison 0 Selection"]
pub type Csel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL1` reader - Comparison 1 Selection"]
pub type Csel1R = crate::BitReader;
#[doc = "Field `CSEL1` writer - Comparison 1 Selection"]
pub type Csel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL2` reader - Comparison 2 Selection"]
pub type Csel2R = crate::BitReader;
#[doc = "Field `CSEL2` writer - Comparison 2 Selection"]
pub type Csel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL3` reader - Comparison 3 Selection"]
pub type Csel3R = crate::BitReader;
#[doc = "Field `CSEL3` writer - Comparison 3 Selection"]
pub type Csel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL4` reader - Comparison 4 Selection"]
pub type Csel4R = crate::BitReader;
#[doc = "Field `CSEL4` writer - Comparison 4 Selection"]
pub type Csel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL5` reader - Comparison 5 Selection"]
pub type Csel5R = crate::BitReader;
#[doc = "Field `CSEL5` writer - Comparison 5 Selection"]
pub type Csel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL6` reader - Comparison 6 Selection"]
pub type Csel6R = crate::BitReader;
#[doc = "Field `CSEL6` writer - Comparison 6 Selection"]
pub type Csel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL7` reader - Comparison 7 Selection"]
pub type Csel7R = crate::BitReader;
#[doc = "Field `CSEL7` writer - Comparison 7 Selection"]
pub type Csel7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    pub fn csel0(&self) -> Csel0R {
        Csel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    pub fn csel1(&self) -> Csel1R {
        Csel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    pub fn csel2(&self) -> Csel2R {
        Csel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    pub fn csel3(&self) -> Csel3R {
        Csel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    pub fn csel4(&self) -> Csel4R {
        Csel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    pub fn csel5(&self) -> Csel5R {
        Csel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    pub fn csel6(&self) -> Csel6R {
        Csel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    pub fn csel7(&self) -> Csel7R {
        Csel7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel0(&mut self) -> Csel0W<Elmr1Spec> {
        Csel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel1(&mut self) -> Csel1W<Elmr1Spec> {
        Csel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel2(&mut self) -> Csel2W<Elmr1Spec> {
        Csel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel3(&mut self) -> Csel3W<Elmr1Spec> {
        Csel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel4(&mut self) -> Csel4W<Elmr1Spec> {
        Csel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel5(&mut self) -> Csel5W<Elmr1Spec> {
        Csel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel6(&mut self) -> Csel6W<Elmr1Spec> {
        Csel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel7(&mut self) -> Csel7W<Elmr1Spec> {
        Csel7W::new(self, 7)
    }
}
#[doc = "PWM Event Line 0 Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`elmr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Elmr1Spec;
impl crate::RegisterSpec for Elmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elmr1::R`](R) reader structure"]
impl crate::Readable for Elmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`elmr1::W`](W) writer structure"]
impl crate::Writable for Elmr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
