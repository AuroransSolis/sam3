#[doc = "Register `CR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CR_SPEC>);
#[doc = "Register `CR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CR_SPEC>);
#[doc = "Field `UPDTIM` reader - Update Request Time Register"]
pub type UPDTIM_R = crate::BitReader<bool>;
#[doc = "Field `UPDTIM` writer - Update Request Time Register"]
pub type UPDTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `UPDCAL` reader - Update Request Calendar Register"]
pub type UPDCAL_R = crate::BitReader<bool>;
#[doc = "Field `UPDCAL` writer - Update Request Calendar Register"]
pub type UPDCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TIMEVSEL` reader - Time Event Selection"]
pub type TIMEVSEL_R = crate::FieldReader<u8, TIMEVSEL_A>;
#[doc = "Time Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEVSEL_A {
    #[doc = "0: Minute change"]
    Minute = 0,
    #[doc = "1: Hour change"]
    Hour = 1,
    #[doc = "2: Every day at midnight"]
    Midnight = 2,
    #[doc = "3: Every day at noon"]
    Noon = 3,
}
impl From<TIMEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEVSEL_A) -> Self {
        variant as _
    }
}
impl TIMEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVSEL_A {
        match self.bits {
            0 => TIMEVSEL_A::Minute,
            1 => TIMEVSEL_A::Hour,
            2 => TIMEVSEL_A::Midnight,
            3 => TIMEVSEL_A::Noon,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Minute`"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == TIMEVSEL_A::Minute
    }
    #[doc = "Checks if the value of the field is `Hour`"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == TIMEVSEL_A::Hour
    }
    #[doc = "Checks if the value of the field is `Midnight`"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == TIMEVSEL_A::Midnight
    }
    #[doc = "Checks if the value of the field is `Noon`"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == TIMEVSEL_A::Noon
    }
}
#[doc = "Field `TIMEVSEL` writer - Time Event Selection"]
pub type TIMEVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, TIMEVSEL_A, 2, O>;
impl<'a, const O: u8> TIMEVSEL_W<'a, O> {
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::Minute)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::Hour)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::Midnight)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::Noon)
    }
}
#[doc = "Field `CALEVSEL` reader - Calendar Event Selection"]
pub type CALEVSEL_R = crate::FieldReader<u8, CALEVSEL_A>;
#[doc = "Calendar Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALEVSEL_A {
    #[doc = "0: Week change (every Monday at time 00:00:00)"]
    Week = 0,
    #[doc = "1: Month change (every 01 of each month at time 00:00:00)"]
    Month = 1,
    #[doc = "2: Year change (every January 1 at time 00:00:00)"]
    Year = 2,
}
impl From<CALEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CALEVSEL_A) -> Self {
        variant as _
    }
}
impl CALEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALEVSEL_A> {
        match self.bits {
            0 => Some(CALEVSEL_A::Week),
            1 => Some(CALEVSEL_A::Month),
            2 => Some(CALEVSEL_A::Year),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Week`"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == CALEVSEL_A::Week
    }
    #[doc = "Checks if the value of the field is `Month`"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == CALEVSEL_A::Month
    }
    #[doc = "Checks if the value of the field is `Year`"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == CALEVSEL_A::Year
    }
}
#[doc = "Field `CALEVSEL` writer - Calendar Event Selection"]
pub type CALEVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, CALEVSEL_A, 2, O>;
impl<'a, const O: u8> CALEVSEL_W<'a, O> {
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut W {
        self.variant(CALEVSEL_A::Week)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut W {
        self.variant(CALEVSEL_A::Month)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut W {
        self.variant(CALEVSEL_A::Year)
    }
}
impl R {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UPDTIM_R {
        UPDTIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UPDCAL_R {
        UPDCAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TIMEVSEL_R {
        TIMEVSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CALEVSEL_R {
        CALEVSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    #[must_use]
    pub fn updtim(&mut self) -> UPDTIM_W<0> {
        UPDTIM_W::new(self)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    #[must_use]
    pub fn updcal(&mut self) -> UPDCAL_W<1> {
        UPDCAL_W::new(self)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn timevsel(&mut self) -> TIMEVSEL_W<8> {
        TIMEVSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn calevsel(&mut self) -> CALEVSEL_W<16> {
        CALEVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
