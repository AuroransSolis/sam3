#[doc = "Register `SEQR1` reader"]
pub type R = crate::R<Seqr1Spec>;
#[doc = "Register `SEQR1` writer"]
pub type W = crate::W<Seqr1Spec>;
#[doc = "Field `USCH1` reader - User Sequence Number 1"]
pub type Usch1R = crate::FieldReader;
#[doc = "Field `USCH1` writer - User Sequence Number 1"]
pub type Usch1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH2` reader - User Sequence Number 2"]
pub type Usch2R = crate::FieldReader;
#[doc = "Field `USCH2` writer - User Sequence Number 2"]
pub type Usch2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH3` reader - User Sequence Number 3"]
pub type Usch3R = crate::FieldReader;
#[doc = "Field `USCH3` writer - User Sequence Number 3"]
pub type Usch3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH4` reader - User Sequence Number 4"]
pub type Usch4R = crate::FieldReader;
#[doc = "Field `USCH4` writer - User Sequence Number 4"]
pub type Usch4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH5` reader - User Sequence Number 5"]
pub type Usch5R = crate::FieldReader;
#[doc = "Field `USCH5` writer - User Sequence Number 5"]
pub type Usch5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH6` reader - User Sequence Number 6"]
pub type Usch6R = crate::FieldReader;
#[doc = "Field `USCH6` writer - User Sequence Number 6"]
pub type Usch6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH7` reader - User Sequence Number 7"]
pub type Usch7R = crate::FieldReader;
#[doc = "Field `USCH7` writer - User Sequence Number 7"]
pub type Usch7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USCH8` reader - User Sequence Number 8"]
pub type Usch8R = crate::FieldReader;
#[doc = "Field `USCH8` writer - User Sequence Number 8"]
pub type Usch8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&self) -> Usch1R {
        Usch1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&self) -> Usch2R {
        Usch2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&self) -> Usch3R {
        Usch3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&self) -> Usch4R {
        Usch4R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&self) -> Usch5R {
        Usch5R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&self) -> Usch6R {
        Usch6R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&self) -> Usch7R {
        Usch7R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> Usch8R {
        Usch8R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 1"]
    #[inline(always)]
    #[must_use]
    pub fn usch1(&mut self) -> Usch1W<Seqr1Spec> {
        Usch1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - User Sequence Number 2"]
    #[inline(always)]
    #[must_use]
    pub fn usch2(&mut self) -> Usch2W<Seqr1Spec> {
        Usch2W::new(self, 4)
    }
    #[doc = "Bits 8:11 - User Sequence Number 3"]
    #[inline(always)]
    #[must_use]
    pub fn usch3(&mut self) -> Usch3W<Seqr1Spec> {
        Usch3W::new(self, 8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 4"]
    #[inline(always)]
    #[must_use]
    pub fn usch4(&mut self) -> Usch4W<Seqr1Spec> {
        Usch4W::new(self, 12)
    }
    #[doc = "Bits 16:19 - User Sequence Number 5"]
    #[inline(always)]
    #[must_use]
    pub fn usch5(&mut self) -> Usch5W<Seqr1Spec> {
        Usch5W::new(self, 16)
    }
    #[doc = "Bits 20:23 - User Sequence Number 6"]
    #[inline(always)]
    #[must_use]
    pub fn usch6(&mut self) -> Usch6W<Seqr1Spec> {
        Usch6W::new(self, 20)
    }
    #[doc = "Bits 24:27 - User Sequence Number 7"]
    #[inline(always)]
    #[must_use]
    pub fn usch7(&mut self) -> Usch7W<Seqr1Spec> {
        Usch7W::new(self, 24)
    }
    #[doc = "Bits 28:31 - User Sequence Number 8"]
    #[inline(always)]
    #[must_use]
    pub fn usch8(&mut self) -> Usch8W<Seqr1Spec> {
        Usch8W::new(self, 28)
    }
}
#[doc = "Channel Sequence Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqr1Spec;
impl crate::RegisterSpec for Seqr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr1::R`](R) reader structure"]
impl crate::Readable for Seqr1Spec {}
#[doc = "`write(|w| ..)` method takes [`seqr1::W`](W) writer structure"]
impl crate::Writable for Seqr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQR1 to value 0"]
impl crate::Resettable for Seqr1Spec {
    const RESET_VALUE: u32 = 0;
}
