#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHID0` reader - Channel ID"]
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
#[doc = "Field `CHID1` reader - Channel ID"]
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
#[doc = "Field `CHID2` reader - Channel ID"]
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
#[doc = "Field `CHID3` reader - Channel ID"]
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
#[doc = "Field `CHID4` reader - Channel ID"]
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
#[doc = "Field `CHID5` reader - Channel ID"]
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
#[doc = "Field `CHID6` reader - Channel ID"]
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
#[doc = "Field `CHID7` reader - Channel ID"]
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
impl R {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel ID"]
    #[inline(always)]
    pub fn chid4(&self) -> CHID4_R {
        CHID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel ID"]
    #[inline(always)]
    pub fn chid5(&self) -> CHID5_R {
        CHID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel ID"]
    #[inline(always)]
    pub fn chid6(&self) -> CHID6_R {
        CHID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel ID"]
    #[inline(always)]
    pub fn chid7(&self) -> CHID7_R {
        CHID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "PWM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
