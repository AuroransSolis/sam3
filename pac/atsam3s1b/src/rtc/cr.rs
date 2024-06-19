#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `UPDTIM` reader - Update Request Time Register"]
pub type UpdtimR = crate::BitReader;
#[doc = "Field `UPDTIM` writer - Update Request Time Register"]
pub type UpdtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDCAL` reader - Update Request Calendar Register"]
pub type UpdcalR = crate::BitReader;
#[doc = "Field `UPDCAL` writer - Update Request Calendar Register"]
pub type UpdcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Time Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timevsel {
    #[doc = "0: Minute change"]
    Minute = 0,
    #[doc = "1: Hour change"]
    Hour = 1,
    #[doc = "2: Every day at midnight"]
    Midnight = 2,
    #[doc = "3: Every day at noon"]
    Noon = 3,
}
impl From<Timevsel> for u8 {
    #[inline(always)]
    fn from(variant: Timevsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timevsel {
    type Ux = u8;
}
impl crate::IsEnum for Timevsel {}
#[doc = "Field `TIMEVSEL` reader - Time Event Selection"]
pub type TimevselR = crate::FieldReader<Timevsel>;
impl TimevselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timevsel {
        match self.bits {
            0 => Timevsel::Minute,
            1 => Timevsel::Hour,
            2 => Timevsel::Midnight,
            3 => Timevsel::Noon,
            _ => unreachable!(),
        }
    }
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == Timevsel::Minute
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == Timevsel::Hour
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == Timevsel::Midnight
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == Timevsel::Noon
    }
}
#[doc = "Field `TIMEVSEL` writer - Time Event Selection"]
pub type TimevselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timevsel, crate::Safe>;
impl<'a, REG> TimevselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut crate::W<REG> {
        self.variant(Timevsel::Minute)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut crate::W<REG> {
        self.variant(Timevsel::Hour)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut crate::W<REG> {
        self.variant(Timevsel::Midnight)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut crate::W<REG> {
        self.variant(Timevsel::Noon)
    }
}
#[doc = "Calendar Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Calevsel {
    #[doc = "0: Week change (every Monday at time 00:00:00)"]
    Week = 0,
    #[doc = "1: Month change (every 01 of each month at time 00:00:00)"]
    Month = 1,
    #[doc = "2: Year change (every January 1 at time 00:00:00)"]
    Year = 2,
}
impl From<Calevsel> for u8 {
    #[inline(always)]
    fn from(variant: Calevsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Calevsel {
    type Ux = u8;
}
impl crate::IsEnum for Calevsel {}
#[doc = "Field `CALEVSEL` reader - Calendar Event Selection"]
pub type CalevselR = crate::FieldReader<Calevsel>;
impl CalevselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Calevsel> {
        match self.bits {
            0 => Some(Calevsel::Week),
            1 => Some(Calevsel::Month),
            2 => Some(Calevsel::Year),
            _ => None,
        }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == Calevsel::Week
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == Calevsel::Month
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == Calevsel::Year
    }
}
#[doc = "Field `CALEVSEL` writer - Calendar Event Selection"]
pub type CalevselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Calevsel>;
impl<'a, REG> CalevselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut crate::W<REG> {
        self.variant(Calevsel::Week)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut crate::W<REG> {
        self.variant(Calevsel::Month)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut crate::W<REG> {
        self.variant(Calevsel::Year)
    }
}
impl R {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UpdtimR {
        UpdtimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UpdcalR {
        UpdcalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TimevselR {
        TimevselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CalevselR {
        CalevselR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    #[must_use]
    pub fn updtim(&mut self) -> UpdtimW<CrSpec> {
        UpdtimW::new(self, 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    #[must_use]
    pub fn updcal(&mut self) -> UpdcalW<CrSpec> {
        UpdcalW::new(self, 1)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn timevsel(&mut self) -> TimevselW<CrSpec> {
        TimevselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn calevsel(&mut self) -> CalevselW<CrSpec> {
        CalevselW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
