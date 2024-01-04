#[doc = "Register `IMR1` reader"]
pub type R = crate::R<IMR1_SPEC>;
#[doc = "Field `CHID0` reader - Counter Event on Channel 0 Interrupt Mask"]
pub type CHID0_R = crate::BitReader;
#[doc = "Field `CHID1` reader - Counter Event on Channel 1 Interrupt Mask"]
pub type CHID1_R = crate::BitReader;
#[doc = "Field `CHID2` reader - Counter Event on Channel 2 Interrupt Mask"]
pub type CHID2_R = crate::BitReader;
#[doc = "Field `CHID3` reader - Counter Event on Channel 3 Interrupt Mask"]
pub type CHID3_R = crate::BitReader;
#[doc = "Field `FCHID0` reader - Fault Protection Trigger on Channel 0 Interrupt Mask"]
pub type FCHID0_R = crate::BitReader;
#[doc = "Field `FCHID1` reader - Fault Protection Trigger on Channel 1 Interrupt Mask"]
pub type FCHID1_R = crate::BitReader;
#[doc = "Field `FCHID2` reader - Fault Protection Trigger on Channel 2 Interrupt Mask"]
pub type FCHID2_R = crate::BitReader;
#[doc = "Field `FCHID3` reader - Fault Protection Trigger on Channel 3 Interrupt Mask"]
pub type FCHID3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid0(&self) -> FCHID0_R {
        FCHID0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid1(&self) -> FCHID1_R {
        FCHID1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid2(&self) -> FCHID2_R {
        FCHID2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid3(&self) -> FCHID3_R {
        FCHID3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr1::R`](R) reader structure"]
impl crate::Readable for IMR1_SPEC {}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
