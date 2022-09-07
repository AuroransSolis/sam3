#[doc = "Register `ISR1` reader"]
pub struct R(crate::R<ISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHID0` reader - Counter Event on Channel 0"]
pub type CHID0_R = crate::BitReader<bool>;
#[doc = "Field `CHID1` reader - Counter Event on Channel 1"]
pub type CHID1_R = crate::BitReader<bool>;
#[doc = "Field `CHID2` reader - Counter Event on Channel 2"]
pub type CHID2_R = crate::BitReader<bool>;
#[doc = "Field `CHID3` reader - Counter Event on Channel 3"]
pub type CHID3_R = crate::BitReader<bool>;
#[doc = "Field `CHID4` reader - Counter Event on Channel 4"]
pub type CHID4_R = crate::BitReader<bool>;
#[doc = "Field `CHID5` reader - Counter Event on Channel 5"]
pub type CHID5_R = crate::BitReader<bool>;
#[doc = "Field `CHID6` reader - Counter Event on Channel 6"]
pub type CHID6_R = crate::BitReader<bool>;
#[doc = "Field `CHID7` reader - Counter Event on Channel 7"]
pub type CHID7_R = crate::BitReader<bool>;
#[doc = "Field `FCHID0` reader - Fault Protection Trigger on Channel 0"]
pub type FCHID0_R = crate::BitReader<bool>;
#[doc = "Field `FCHID1` reader - Fault Protection Trigger on Channel 1"]
pub type FCHID1_R = crate::BitReader<bool>;
#[doc = "Field `FCHID2` reader - Fault Protection Trigger on Channel 2"]
pub type FCHID2_R = crate::BitReader<bool>;
#[doc = "Field `FCHID3` reader - Fault Protection Trigger on Channel 3"]
pub type FCHID3_R = crate::BitReader<bool>;
#[doc = "Field `FCHID4` reader - Fault Protection Trigger on Channel 4"]
pub type FCHID4_R = crate::BitReader<bool>;
#[doc = "Field `FCHID5` reader - Fault Protection Trigger on Channel 5"]
pub type FCHID5_R = crate::BitReader<bool>;
#[doc = "Field `FCHID6` reader - Fault Protection Trigger on Channel 6"]
pub type FCHID6_R = crate::BitReader<bool>;
#[doc = "Field `FCHID7` reader - Fault Protection Trigger on Channel 7"]
pub type FCHID7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter Event on Channel 4"]
    #[inline(always)]
    pub fn chid4(&self) -> CHID4_R {
        CHID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Event on Channel 5"]
    #[inline(always)]
    pub fn chid5(&self) -> CHID5_R {
        CHID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Event on Channel 6"]
    #[inline(always)]
    pub fn chid6(&self) -> CHID6_R {
        CHID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Event on Channel 7"]
    #[inline(always)]
    pub fn chid7(&self) -> CHID7_R {
        CHID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0"]
    #[inline(always)]
    pub fn fchid0(&self) -> FCHID0_R {
        FCHID0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1"]
    #[inline(always)]
    pub fn fchid1(&self) -> FCHID1_R {
        FCHID1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2"]
    #[inline(always)]
    pub fn fchid2(&self) -> FCHID2_R {
        FCHID2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3"]
    #[inline(always)]
    pub fn fchid3(&self) -> FCHID3_R {
        FCHID3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4"]
    #[inline(always)]
    pub fn fchid4(&self) -> FCHID4_R {
        FCHID4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5"]
    #[inline(always)]
    pub fn fchid5(&self) -> FCHID5_R {
        FCHID5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6"]
    #[inline(always)]
    pub fn fchid6(&self) -> FCHID6_R {
        FCHID6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7"]
    #[inline(always)]
    pub fn fchid7(&self) -> FCHID7_R {
        FCHID7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr1](index.html) module"]
pub struct ISR1_SPEC;
impl crate::RegisterSpec for ISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr1::R](R) reader structure"]
impl crate::Readable for ISR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR1 to value 0"]
impl crate::Resettable for ISR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
