#[doc = "Register `IMR1` reader"]
pub type R = crate::R<Imr1Spec>;
#[doc = "Field `CHID0` reader - Counter Event on Channel 0 Interrupt Mask"]
pub type Chid0R = crate::BitReader;
#[doc = "Field `CHID1` reader - Counter Event on Channel 1 Interrupt Mask"]
pub type Chid1R = crate::BitReader;
#[doc = "Field `CHID2` reader - Counter Event on Channel 2 Interrupt Mask"]
pub type Chid2R = crate::BitReader;
#[doc = "Field `CHID3` reader - Counter Event on Channel 3 Interrupt Mask"]
pub type Chid3R = crate::BitReader;
#[doc = "Field `CHID4` reader - Counter Event on Channel 4 Interrupt Mask"]
pub type Chid4R = crate::BitReader;
#[doc = "Field `CHID5` reader - Counter Event on Channel 5 Interrupt Mask"]
pub type Chid5R = crate::BitReader;
#[doc = "Field `CHID6` reader - Counter Event on Channel 6 Interrupt Mask"]
pub type Chid6R = crate::BitReader;
#[doc = "Field `CHID7` reader - Counter Event on Channel 7 Interrupt Mask"]
pub type Chid7R = crate::BitReader;
#[doc = "Field `FCHID0` reader - Fault Protection Trigger on Channel 0 Interrupt Mask"]
pub type Fchid0R = crate::BitReader;
#[doc = "Field `FCHID1` reader - Fault Protection Trigger on Channel 1 Interrupt Mask"]
pub type Fchid1R = crate::BitReader;
#[doc = "Field `FCHID2` reader - Fault Protection Trigger on Channel 2 Interrupt Mask"]
pub type Fchid2R = crate::BitReader;
#[doc = "Field `FCHID3` reader - Fault Protection Trigger on Channel 3 Interrupt Mask"]
pub type Fchid3R = crate::BitReader;
#[doc = "Field `FCHID4` reader - Fault Protection Trigger on Channel 4 Interrupt Mask"]
pub type Fchid4R = crate::BitReader;
#[doc = "Field `FCHID5` reader - Fault Protection Trigger on Channel 5 Interrupt Mask"]
pub type Fchid5R = crate::BitReader;
#[doc = "Field `FCHID6` reader - Fault Protection Trigger on Channel 6 Interrupt Mask"]
pub type Fchid6R = crate::BitReader;
#[doc = "Field `FCHID7` reader - Fault Protection Trigger on Channel 7 Interrupt Mask"]
pub type Fchid7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn chid0(&self) -> Chid0R {
        Chid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn chid1(&self) -> Chid1R {
        Chid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn chid2(&self) -> Chid2R {
        Chid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn chid3(&self) -> Chid3R {
        Chid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter Event on Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn chid4(&self) -> Chid4R {
        Chid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Event on Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn chid5(&self) -> Chid5R {
        Chid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Event on Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn chid6(&self) -> Chid6R {
        Chid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Event on Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn chid7(&self) -> Chid7R {
        Chid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid0(&self) -> Fchid0R {
        Fchid0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid1(&self) -> Fchid1R {
        Fchid1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid2(&self) -> Fchid2R {
        Fchid2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid3(&self) -> Fchid3R {
        Fchid3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid4(&self) -> Fchid4R {
        Fchid4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid5(&self) -> Fchid5R {
        Fchid5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid6(&self) -> Fchid6R {
        Fchid6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid7(&self) -> Fchid7R {
        Fchid7R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr1Spec;
impl crate::RegisterSpec for Imr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr1::R`](R) reader structure"]
impl crate::Readable for Imr1Spec {}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for Imr1Spec {
    const RESET_VALUE: u32 = 0;
}
