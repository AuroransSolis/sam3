#[doc = "Register `CALR` reader"]
pub type R = crate::R<CalrSpec>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CalrSpec>;
#[doc = "Field `CENT` reader - Current Century"]
pub type CentR = crate::FieldReader;
#[doc = "Field `CENT` writer - Current Century"]
pub type CentW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `YEAR` reader - Current Year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - Current Year"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MONTH` reader - Current Month"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - Current Month"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAY` reader - Current Day in Current Week"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - Current Day in Current Week"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATE` reader - Current Day in Current Month"]
pub type DateR = crate::FieldReader;
#[doc = "Field `DATE` writer - Current Day in Current Month"]
pub type DateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:6 - Current Century"]
    #[inline(always)]
    pub fn cent(&self) -> CentR {
        CentR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Current Year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Current Month"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Current Day in Current Week"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:29 - Current Day in Current Month"]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Century"]
    #[inline(always)]
    #[must_use]
    pub fn cent(&mut self) -> CentW<CalrSpec> {
        CentW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Current Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YearW<CalrSpec> {
        YearW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Current Month"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MonthW<CalrSpec> {
        MonthW::new(self, 16)
    }
    #[doc = "Bits 21:23 - Current Day in Current Week"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DayW<CalrSpec> {
        DayW::new(self, 21)
    }
    #[doc = "Bits 24:29 - Current Day in Current Month"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DateW<CalrSpec> {
        DateW::new(self, 24)
    }
}
#[doc = "Calendar Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalrSpec;
impl crate::RegisterSpec for CalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calr::R`](R) reader structure"]
impl crate::Readable for CalrSpec {}
#[doc = "`write(|w| ..)` method takes [`calr::W`](W) writer structure"]
impl crate::Writable for CalrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALR to value 0x0181_0720"]
impl crate::Resettable for CalrSpec {
    const RESET_VALUE: u32 = 0x0181_0720;
}
