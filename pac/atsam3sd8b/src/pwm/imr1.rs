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
#[doc = "Field `FCHID0` reader - Fault Protection Trigger on Channel 0 Interrupt Mask"]
pub type Fchid0R = crate::BitReader;
#[doc = "Field `FCHID1` reader - Fault Protection Trigger on Channel 1 Interrupt Mask"]
pub type Fchid1R = crate::BitReader;
#[doc = "Field `FCHID2` reader - Fault Protection Trigger on Channel 2 Interrupt Mask"]
pub type Fchid2R = crate::BitReader;
#[doc = "Field `FCHID3` reader - Fault Protection Trigger on Channel 3 Interrupt Mask"]
pub type Fchid3R = crate::BitReader;
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
