#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Acknowledge for Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackupd {
    #[doc = "0: Time and calendar registers cannot be updated."]
    Freerun = 0,
    #[doc = "1: Time and calendar registers can be updated."]
    Update = 1,
}
impl From<Ackupd> for bool {
    #[inline(always)]
    fn from(variant: Ackupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKUPD` reader - Acknowledge for Update"]
pub type AckupdR = crate::BitReader<Ackupd>;
impl AckupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackupd {
        match self.bits {
            false => Ackupd::Freerun,
            true => Ackupd::Update,
        }
    }
    #[doc = "Time and calendar registers cannot be updated."]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == Ackupd::Freerun
    }
    #[doc = "Time and calendar registers can be updated."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Ackupd::Update
    }
}
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alarm {
    #[doc = "0: No alarm matching condition occurred."]
    NoAlarmevent = 0,
    #[doc = "1: An alarm matching condition has occurred."]
    Alarmevent = 1,
}
impl From<Alarm> for bool {
    #[inline(always)]
    fn from(variant: Alarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM` reader - Alarm Flag"]
pub type AlarmR = crate::BitReader<Alarm>;
impl AlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alarm {
        match self.bits {
            false => Alarm::NoAlarmevent,
            true => Alarm::Alarmevent,
        }
    }
    #[doc = "No alarm matching condition occurred."]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == Alarm::NoAlarmevent
    }
    #[doc = "An alarm matching condition has occurred."]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == Alarm::Alarmevent
    }
}
#[doc = "Second Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sec {
    #[doc = "0: No second event has occurred since the last clear."]
    NoSecevent = 0,
    #[doc = "1: At least one second event has occurred since the last clear."]
    Secevent = 1,
}
impl From<Sec> for bool {
    #[inline(always)]
    fn from(variant: Sec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC` reader - Second Event"]
pub type SecR = crate::BitReader<Sec>;
impl SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sec {
        match self.bits {
            false => Sec::NoSecevent,
            true => Sec::Secevent,
        }
    }
    #[doc = "No second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == Sec::NoSecevent
    }
    #[doc = "At least one second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == Sec::Secevent
    }
}
#[doc = "Time Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timev {
    #[doc = "0: No time event has occurred since the last clear."]
    NoTimevent = 0,
    #[doc = "1: At least one time event has occurred since the last clear."]
    Timevent = 1,
}
impl From<Timev> for bool {
    #[inline(always)]
    fn from(variant: Timev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEV` reader - Time Event"]
pub type TimevR = crate::BitReader<Timev>;
impl TimevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timev {
        match self.bits {
            false => Timev::NoTimevent,
            true => Timev::Timevent,
        }
    }
    #[doc = "No time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == Timev::NoTimevent
    }
    #[doc = "At least one time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == Timev::Timevent
    }
}
#[doc = "Calendar Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calev {
    #[doc = "0: No calendar event has occurred since the last clear."]
    NoCalevent = 0,
    #[doc = "1: At least one calendar event has occurred since the last clear."]
    Calevent = 1,
}
impl From<Calev> for bool {
    #[inline(always)]
    fn from(variant: Calev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALEV` reader - Calendar Event"]
pub type CalevR = crate::BitReader<Calev>;
impl CalevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calev {
        match self.bits {
            false => Calev::NoCalevent,
            true => Calev::Calevent,
        }
    }
    #[doc = "No calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == Calev::NoCalevent
    }
    #[doc = "At least one calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == Calev::Calevent
    }
}
#[doc = "Time and/or Date Free Running Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tderr {
    #[doc = "0: The internal free running counters are carrying valid values since the last read of RTC_SR."]
    Correct = 0,
    #[doc = "1: The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    ErrTimedate = 1,
}
impl From<Tderr> for bool {
    #[inline(always)]
    fn from(variant: Tderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDERR` reader - Time and/or Date Free Running Error"]
pub type TderrR = crate::BitReader<Tderr>;
impl TderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tderr {
        match self.bits {
            false => Tderr::Correct,
            true => Tderr::ErrTimedate,
        }
    }
    #[doc = "The internal free running counters are carrying valid values since the last read of RTC_SR."]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == Tderr::Correct
    }
    #[doc = "The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    #[inline(always)]
    pub fn is_err_timedate(&self) -> bool {
        *self == Tderr::ErrTimedate
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> AckupdR {
        AckupdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> AlarmR {
        AlarmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TimevR {
        TimevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CalevR {
        CalevR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error"]
    #[inline(always)]
    pub fn tderr(&self) -> TderrR {
        TderrR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
