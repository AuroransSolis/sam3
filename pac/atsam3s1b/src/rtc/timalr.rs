#[doc = "Register `TIMALR` reader"]
pub type R = crate::R<TimalrSpec>;
#[doc = "Register `TIMALR` writer"]
pub type W = crate::W<TimalrSpec>;
#[doc = "Field `SEC` reader - Second Alarm"]
pub type SecR = crate::FieldReader;
#[doc = "Field `SEC` writer - Second Alarm"]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECEN` reader - Second Alarm Enable"]
pub type SecenR = crate::BitReader;
#[doc = "Field `SECEN` writer - Second Alarm Enable"]
pub type SecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIN` reader - Minute Alarm"]
pub type MinR = crate::FieldReader;
#[doc = "Field `MIN` writer - Minute Alarm"]
pub type MinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MINEN` reader - Minute Alarm Enable"]
pub type MinenR = crate::BitReader;
#[doc = "Field `MINEN` writer - Minute Alarm Enable"]
pub type MinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR` reader - Hour Alarm"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hour Alarm"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AMPM` reader - AM/PM Indicator"]
pub type AmpmR = crate::BitReader;
#[doc = "Field `AMPM` writer - AM/PM Indicator"]
pub type AmpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUREN` reader - Hour Alarm Enable"]
pub type HourenR = crate::BitReader;
#[doc = "Field `HOUREN` writer - Hour Alarm Enable"]
pub type HourenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&self) -> SecenR {
        SecenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&self) -> MinR {
        MinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&self) -> MinenR {
        MinenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&self) -> HourenR {
        HourenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SecW<TimalrSpec> {
        SecW::new(self, 0)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secen(&mut self) -> SecenW<TimalrSpec> {
        SecenW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MinW<TimalrSpec> {
        MinW::new(self, 8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minen(&mut self) -> MinenW<TimalrSpec> {
        MinenW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HourW<TimalrSpec> {
        HourW::new(self, 16)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AmpmW<TimalrSpec> {
        AmpmW::new(self, 22)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn houren(&mut self) -> HourenW<TimalrSpec> {
        HourenW::new(self, 23)
    }
}
#[doc = "Time Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timalr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timalr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimalrSpec;
impl crate::RegisterSpec for TimalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timalr::R`](R) reader structure"]
impl crate::Readable for TimalrSpec {}
#[doc = "`write(|w| ..)` method takes [`timalr::W`](W) writer structure"]
impl crate::Writable for TimalrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMALR to value 0"]
impl crate::Resettable for TimalrSpec {
    const RESET_VALUE: u32 = 0;
}
