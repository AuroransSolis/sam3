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
#[doc = "Field `ACKUPD` reader - Acknowledge for Update"]
pub struct ACKUPD_R(crate::FieldReader<bool, bool>);
impl ACKUPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACKUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKUPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM` reader - Alarm Flag"]
pub struct ALARM_R(crate::FieldReader<bool, bool>);
impl ALARM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` reader - Second Event"]
pub struct SEC_R(crate::FieldReader<bool, bool>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEV` reader - Time Event"]
pub struct TIMEV_R(crate::FieldReader<bool, bool>);
impl TIMEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALEV` reader - Calendar Event"]
pub struct CALEV_R(crate::FieldReader<bool, bool>);
impl CALEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
