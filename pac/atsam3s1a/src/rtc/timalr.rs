#[doc = "Register `TIMALR` reader"]
pub type R = crate::R<TIMALR_SPEC>;
#[doc = "Register `TIMALR` writer"]
pub type W = crate::W<TIMALR_SPEC>;
#[doc = "Field `SEC` reader - Second Alarm"]
pub type SEC_R = crate::FieldReader;
#[doc = "Field `SEC` writer - Second Alarm"]
pub type SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECEN` reader - Second Alarm Enable"]
pub type SECEN_R = crate::BitReader;
#[doc = "Field `SECEN` writer - Second Alarm Enable"]
pub type SECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIN` reader - Minute Alarm"]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - Minute Alarm"]
pub type MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MINEN` reader - Minute Alarm Enable"]
pub type MINEN_R = crate::BitReader;
#[doc = "Field `MINEN` writer - Minute Alarm Enable"]
pub type MINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR` reader - Hour Alarm"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hour Alarm"]
pub type HOUR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AMPM` reader - AM/PM Indicator"]
pub type AMPM_R = crate::BitReader;
#[doc = "Field `AMPM` writer - AM/PM Indicator"]
pub type AMPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUREN` reader - Hour Alarm Enable"]
pub type HOUREN_R = crate::BitReader;
#[doc = "Field `HOUREN` writer - Hour Alarm Enable"]
pub type HOUREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&self) -> SECEN_R {
        SECEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&self) -> MINEN_R {
        MINEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&self) -> HOUREN_R {
        HOUREN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<TIMALR_SPEC> {
        SEC_W::new(self, 0)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secen(&mut self) -> SECEN_W<TIMALR_SPEC> {
        SECEN_W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<TIMALR_SPEC> {
        MIN_W::new(self, 8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minen(&mut self) -> MINEN_W<TIMALR_SPEC> {
        MINEN_W::new(self, 15)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<TIMALR_SPEC> {
        HOUR_W::new(self, 16)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AMPM_W<TIMALR_SPEC> {
        AMPM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn houren(&mut self) -> HOUREN_W<TIMALR_SPEC> {
        HOUREN_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Time Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timalr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timalr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMALR_SPEC;
impl crate::RegisterSpec for TIMALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timalr::R`](R) reader structure"]
impl crate::Readable for TIMALR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timalr::W`](W) writer structure"]
impl crate::Writable for TIMALR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMALR to value 0"]
impl crate::Resettable for TIMALR_SPEC {
    const RESET_VALUE: u32 = 0;
}
