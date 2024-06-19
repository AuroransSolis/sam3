#[doc = "Register `TIMR` reader"]
pub type R = crate::R<TimrSpec>;
#[doc = "Register `TIMR` writer"]
pub type W = crate::W<TimrSpec>;
#[doc = "Field `SEC` reader - Current Second"]
pub type SecR = crate::FieldReader;
#[doc = "Field `SEC` writer - Current Second"]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MIN` reader - Current Minute"]
pub type MinR = crate::FieldReader;
#[doc = "Field `MIN` writer - Current Minute"]
pub type MinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HOUR` reader - Current Hour"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - Current Hour"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AMPM` reader - Ante Meridiem Post Meridiem Indicator"]
pub type AmpmR = crate::BitReader;
#[doc = "Field `AMPM` writer - Ante Meridiem Post Meridiem Indicator"]
pub type AmpmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Current Second"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Current Minute"]
    #[inline(always)]
    pub fn min(&self) -> MinR {
        MinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Current Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Ante Meridiem Post Meridiem Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Second"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SecW<TimrSpec> {
        SecW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Current Minute"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MinW<TimrSpec> {
        MinW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Current Hour"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HourW<TimrSpec> {
        HourW::new(self, 16)
    }
    #[doc = "Bit 22 - Ante Meridiem Post Meridiem Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AmpmW<TimrSpec> {
        AmpmW::new(self, 22)
    }
}
#[doc = "Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimrSpec;
impl crate::RegisterSpec for TimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timr::R`](R) reader structure"]
impl crate::Readable for TimrSpec {}
#[doc = "`write(|w| ..)` method takes [`timr::W`](W) writer structure"]
impl crate::Writable for TimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMR to value 0"]
impl crate::Resettable for TimrSpec {
    const RESET_VALUE: u32 = 0;
}
