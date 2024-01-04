#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `ACKUPD` reader - Acknowledge for Update"]
pub type ACKUPD_R = crate::BitReader<ACKUPD_A>;
#[doc = "Acknowledge for Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKUPD_A {
    #[doc = "0: Time and calendar registers cannot be updated."]
    Freerun = 0,
    #[doc = "1: Time and calendar registers can be updated."]
    Update = 1,
}
impl From<ACKUPD_A> for bool {
    #[inline(always)]
    fn from(variant: ACKUPD_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACKUPD_A {
        match self.bits {
            false => ACKUPD_A::Freerun,
            true => ACKUPD_A::Update,
        }
    }
    #[doc = "Time and calendar registers cannot be updated."]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPD_A::Freerun
    }
    #[doc = "Time and calendar registers can be updated."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ACKUPD_A::Update
    }
}
#[doc = "Field `ALARM` reader - Alarm Flag"]
pub type ALARM_R = crate::BitReader<ALARM_A>;
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM_A {
    #[doc = "0: No alarm matching condition occurred."]
    NoAlarmevent = 0,
    #[doc = "1: An alarm matching condition has occurred."]
    Alarmevent = 1,
}
impl From<ALARM_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALARM_A {
        match self.bits {
            false => ALARM_A::NoAlarmevent,
            true => ALARM_A::Alarmevent,
        }
    }
    #[doc = "No alarm matching condition occurred."]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARM_A::NoAlarmevent
    }
    #[doc = "An alarm matching condition has occurred."]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == ALARM_A::Alarmevent
    }
}
#[doc = "Field `SEC` reader - Second Event"]
pub type SEC_R = crate::BitReader<SEC_A>;
#[doc = "Second Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_A {
    #[doc = "0: No second event has occurred since the last clear."]
    NoSecevent = 0,
    #[doc = "1: At least one second event has occurred since the last clear."]
    Secevent = 1,
}
impl From<SEC_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEC_A {
        match self.bits {
            false => SEC_A::NoSecevent,
            true => SEC_A::Secevent,
        }
    }
    #[doc = "No second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == SEC_A::NoSecevent
    }
    #[doc = "At least one second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == SEC_A::Secevent
    }
}
#[doc = "Field `TIMEV` reader - Time Event"]
pub type TIMEV_R = crate::BitReader<TIMEV_A>;
#[doc = "Time Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEV_A {
    #[doc = "0: No time event has occurred since the last clear."]
    NoTimevent = 0,
    #[doc = "1: At least one time event has occurred since the last clear."]
    Timevent = 1,
}
impl From<TIMEV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEV_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMEV_A {
        match self.bits {
            false => TIMEV_A::NoTimevent,
            true => TIMEV_A::Timevent,
        }
    }
    #[doc = "No time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEV_A::NoTimevent
    }
    #[doc = "At least one time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == TIMEV_A::Timevent
    }
}
#[doc = "Field `CALEV` reader - Calendar Event"]
pub type CALEV_R = crate::BitReader<CALEV_A>;
#[doc = "Calendar Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEV_A {
    #[doc = "0: No calendar event has occurred since the last clear."]
    NoCalevent = 0,
    #[doc = "1: At least one calendar event has occurred since the last clear."]
    Calevent = 1,
}
impl From<CALEV_A> for bool {
    #[inline(always)]
    fn from(variant: CALEV_A) -> Self {
        variant as u8 != 0
    }
}
impl CALEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALEV_A {
        match self.bits {
            false => CALEV_A::NoCalevent,
            true => CALEV_A::Calevent,
        }
    }
    #[doc = "No calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEV_A::NoCalevent
    }
    #[doc = "At least one calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == CALEV_A::Calevent
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
