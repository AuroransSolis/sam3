#[doc = "Register `OVER` reader"]
pub type R = crate::R<OverSpec>;
#[doc = "Field `OVRE0` reader - Overrun Error 0"]
pub type Ovre0R = crate::BitReader;
#[doc = "Field `OVRE1` reader - Overrun Error 1"]
pub type Ovre1R = crate::BitReader;
#[doc = "Field `OVRE2` reader - Overrun Error 2"]
pub type Ovre2R = crate::BitReader;
#[doc = "Field `OVRE3` reader - Overrun Error 3"]
pub type Ovre3R = crate::BitReader;
#[doc = "Field `OVRE4` reader - Overrun Error 4"]
pub type Ovre4R = crate::BitReader;
#[doc = "Field `OVRE5` reader - Overrun Error 5"]
pub type Ovre5R = crate::BitReader;
#[doc = "Field `OVRE6` reader - Overrun Error 6"]
pub type Ovre6R = crate::BitReader;
#[doc = "Field `OVRE7` reader - Overrun Error 7"]
pub type Ovre7R = crate::BitReader;
#[doc = "Field `OVRE8` reader - Overrun Error 8"]
pub type Ovre8R = crate::BitReader;
#[doc = "Field `OVRE9` reader - Overrun Error 9"]
pub type Ovre9R = crate::BitReader;
#[doc = "Field `OVRE10` reader - Overrun Error 10"]
pub type Ovre10R = crate::BitReader;
#[doc = "Field `OVRE11` reader - Overrun Error 11"]
pub type Ovre11R = crate::BitReader;
#[doc = "Field `OVRE12` reader - Overrun Error 12"]
pub type Ovre12R = crate::BitReader;
#[doc = "Field `OVRE13` reader - Overrun Error 13"]
pub type Ovre13R = crate::BitReader;
#[doc = "Field `OVRE14` reader - Overrun Error 14"]
pub type Ovre14R = crate::BitReader;
#[doc = "Field `OVRE15` reader - Overrun Error 15"]
pub type Ovre15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> Ovre0R {
        Ovre0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> Ovre1R {
        Ovre1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> Ovre2R {
        Ovre2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> Ovre3R {
        Ovre3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> Ovre4R {
        Ovre4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> Ovre5R {
        Ovre5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> Ovre6R {
        Ovre6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> Ovre7R {
        Ovre7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overrun Error 8"]
    #[inline(always)]
    pub fn ovre8(&self) -> Ovre8R {
        Ovre8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Overrun Error 9"]
    #[inline(always)]
    pub fn ovre9(&self) -> Ovre9R {
        Ovre9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun Error 10"]
    #[inline(always)]
    pub fn ovre10(&self) -> Ovre10R {
        Ovre10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun Error 11"]
    #[inline(always)]
    pub fn ovre11(&self) -> Ovre11R {
        Ovre11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Error 12"]
    #[inline(always)]
    pub fn ovre12(&self) -> Ovre12R {
        Ovre12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun Error 13"]
    #[inline(always)]
    pub fn ovre13(&self) -> Ovre13R {
        Ovre13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Overrun Error 14"]
    #[inline(always)]
    pub fn ovre14(&self) -> Ovre14R {
        Ovre14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overrun Error 15"]
    #[inline(always)]
    pub fn ovre15(&self) -> Ovre15R {
        Ovre15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Overrun Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OverSpec;
impl crate::RegisterSpec for OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`over::R`](R) reader structure"]
impl crate::Readable for OverSpec {}
#[doc = "`reset()` method sets OVER to value 0"]
impl crate::Resettable for OverSpec {
    const RESET_VALUE: u32 = 0;
}
