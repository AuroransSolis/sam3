#[doc = "Register `SEQR2` reader"]
pub type R = crate::R<Seqr2Spec>;
#[doc = "Register `SEQR2` writer"]
pub type W = crate::W<Seqr2Spec>;
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub type Usch9R = crate::FieldReader;
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub type Usch9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub type Usch10R = crate::FieldReader;
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub type Usch10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub type Usch11R = crate::FieldReader;
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub type Usch11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH12` reader - User Sequence Number 12"]
pub type Usch12R = crate::FieldReader;
#[doc = "Field `USCH12` writer - User Sequence Number 12"]
pub type Usch12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH13` reader - User Sequence Number 13"]
pub type Usch13R = crate::FieldReader;
#[doc = "Field `USCH13` writer - User Sequence Number 13"]
pub type Usch13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH14` reader - User Sequence Number 14"]
pub type Usch14R = crate::FieldReader;
#[doc = "Field `USCH14` writer - User Sequence Number 14"]
pub type Usch14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH15` reader - User Sequence Number 15"]
pub type Usch15R = crate::FieldReader;
#[doc = "Field `USCH15` writer - User Sequence Number 15"]
pub type Usch15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH16` reader - User Sequence Number 16"]
pub type Usch16R = crate::FieldReader;
#[doc = "Field `USCH16` writer - User Sequence Number 16"]
pub type Usch16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> Usch9R {
        Usch9R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> Usch10R {
        Usch10R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> Usch11R {
        Usch11R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - User Sequence Number 12"]
    #[inline(always)]
    pub fn usch12(&self) -> Usch12R {
        Usch12R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - User Sequence Number 13"]
    #[inline(always)]
    pub fn usch13(&self) -> Usch13R {
        Usch13R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - User Sequence Number 14"]
    #[inline(always)]
    pub fn usch14(&self) -> Usch14R {
        Usch14R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - User Sequence Number 15"]
    #[inline(always)]
    pub fn usch15(&self) -> Usch15R {
        Usch15R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - User Sequence Number 16"]
    #[inline(always)]
    pub fn usch16(&self) -> Usch16R {
        Usch16R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - User Sequence Number 9"]
    #[inline(always)]
    #[must_use]
    pub fn usch9(&mut self) -> Usch9W<Seqr2Spec> {
        Usch9W::new(self, 0)
    }
    #[doc = "Bits 4:6 - User Sequence Number 10"]
    #[inline(always)]
    #[must_use]
    pub fn usch10(&mut self) -> Usch10W<Seqr2Spec> {
        Usch10W::new(self, 4)
    }
    #[doc = "Bits 8:10 - User Sequence Number 11"]
    #[inline(always)]
    #[must_use]
    pub fn usch11(&mut self) -> Usch11W<Seqr2Spec> {
        Usch11W::new(self, 8)
    }
    #[doc = "Bits 12:14 - User Sequence Number 12"]
    #[inline(always)]
    #[must_use]
    pub fn usch12(&mut self) -> Usch12W<Seqr2Spec> {
        Usch12W::new(self, 12)
    }
    #[doc = "Bits 16:18 - User Sequence Number 13"]
    #[inline(always)]
    #[must_use]
    pub fn usch13(&mut self) -> Usch13W<Seqr2Spec> {
        Usch13W::new(self, 16)
    }
    #[doc = "Bits 20:22 - User Sequence Number 14"]
    #[inline(always)]
    #[must_use]
    pub fn usch14(&mut self) -> Usch14W<Seqr2Spec> {
        Usch14W::new(self, 20)
    }
    #[doc = "Bits 24:26 - User Sequence Number 15"]
    #[inline(always)]
    #[must_use]
    pub fn usch15(&mut self) -> Usch15W<Seqr2Spec> {
        Usch15W::new(self, 24)
    }
    #[doc = "Bits 28:30 - User Sequence Number 16"]
    #[inline(always)]
    #[must_use]
    pub fn usch16(&mut self) -> Usch16W<Seqr2Spec> {
        Usch16W::new(self, 28)
    }
}
#[doc = "Channel Sequence Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqr2Spec;
impl crate::RegisterSpec for Seqr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr2::R`](R) reader structure"]
impl crate::Readable for Seqr2Spec {}
#[doc = "`write(|w| ..)` method takes [`seqr2::W`](W) writer structure"]
impl crate::Writable for Seqr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQR2 to value 0"]
impl crate::Resettable for Seqr2Spec {
    const RESET_VALUE: u32 = 0;
}
