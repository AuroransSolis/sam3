#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHID0` reader - Counter Event on Channel 0 Interrupt Mask"]
pub struct CHID0_R(crate::FieldReader<bool, bool>);
impl CHID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID1` reader - Counter Event on Channel 1 Interrupt Mask"]
pub struct CHID1_R(crate::FieldReader<bool, bool>);
impl CHID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID2` reader - Counter Event on Channel 2 Interrupt Mask"]
pub struct CHID2_R(crate::FieldReader<bool, bool>);
impl CHID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID3` reader - Counter Event on Channel 3 Interrupt Mask"]
pub struct CHID3_R(crate::FieldReader<bool, bool>);
impl CHID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID4` reader - Counter Event on Channel 4 Interrupt Mask"]
pub struct CHID4_R(crate::FieldReader<bool, bool>);
impl CHID4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID5` reader - Counter Event on Channel 5 Interrupt Mask"]
pub struct CHID5_R(crate::FieldReader<bool, bool>);
impl CHID5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID6` reader - Counter Event on Channel 6 Interrupt Mask"]
pub struct CHID6_R(crate::FieldReader<bool, bool>);
impl CHID6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID7` reader - Counter Event on Channel 7 Interrupt Mask"]
pub struct CHID7_R(crate::FieldReader<bool, bool>);
impl CHID7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID0` reader - Fault Protection Trigger on Channel 0 Interrupt Mask"]
pub struct FCHID0_R(crate::FieldReader<bool, bool>);
impl FCHID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID1` reader - Fault Protection Trigger on Channel 1 Interrupt Mask"]
pub struct FCHID1_R(crate::FieldReader<bool, bool>);
impl FCHID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID2` reader - Fault Protection Trigger on Channel 2 Interrupt Mask"]
pub struct FCHID2_R(crate::FieldReader<bool, bool>);
impl FCHID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID3` reader - Fault Protection Trigger on Channel 3 Interrupt Mask"]
pub struct FCHID3_R(crate::FieldReader<bool, bool>);
impl FCHID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID4` reader - Fault Protection Trigger on Channel 4 Interrupt Mask"]
pub struct FCHID4_R(crate::FieldReader<bool, bool>);
impl FCHID4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID5` reader - Fault Protection Trigger on Channel 5 Interrupt Mask"]
pub struct FCHID5_R(crate::FieldReader<bool, bool>);
impl FCHID5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID6` reader - Fault Protection Trigger on Channel 6 Interrupt Mask"]
pub struct FCHID6_R(crate::FieldReader<bool, bool>);
impl FCHID6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID7` reader - Fault Protection Trigger on Channel 7 Interrupt Mask"]
pub struct FCHID7_R(crate::FieldReader<bool, bool>);
impl FCHID7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counter Event on Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn chid4(&self) -> CHID4_R {
        CHID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter Event on Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn chid5(&self) -> CHID5_R {
        CHID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter Event on Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn chid6(&self) -> CHID6_R {
        CHID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Event on Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn chid7(&self) -> CHID7_R {
        CHID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid0(&self) -> FCHID0_R {
        FCHID0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid1(&self) -> FCHID1_R {
        FCHID1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid2(&self) -> FCHID2_R {
        FCHID2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid3(&self) -> FCHID3_R {
        FCHID3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid4(&self) -> FCHID4_R {
        FCHID4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid5(&self) -> FCHID5_R {
        FCHID5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid6(&self) -> FCHID6_R {
        FCHID6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid7(&self) -> FCHID7_R {
        FCHID7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
