#[doc = "Register `CALALR` reader"]
pub type R = crate::R<CalalrSpec>;
#[doc = "Register `CALALR` writer"]
pub type W = crate::W<CalalrSpec>;
#[doc = "Field `MONTH` reader - Month Alarm"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month Alarm"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MTHEN` reader - Month Alarm Enable"]
pub type MthenR = crate::BitReader;
#[doc = "Field `MTHEN` writer - Month Alarm Enable"]
pub type MthenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATE` reader - Date Alarm"]
pub type DateR = crate::FieldReader;
#[doc = "Field `DATE` writer - Date Alarm"]
pub type DateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATEEN` reader - Date Alarm Enable"]
pub type DateenR = crate::BitReader;
#[doc = "Field `DATEEN` writer - Date Alarm Enable"]
pub type DateenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    pub fn mthen(&self) -> MthenR {
        MthenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    pub fn dateen(&self) -> DateenR {
        DateenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MonthW<CalalrSpec> {
        MonthW::new(self, 16)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mthen(&mut self) -> MthenW<CalalrSpec> {
        MthenW::new(self, 23)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DateW<CalalrSpec> {
        DateW::new(self, 24)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dateen(&mut self) -> DateenW<CalalrSpec> {
        DateenW::new(self, 31)
    }
}
#[doc = "Calendar Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calalr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calalr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalalrSpec;
impl crate::RegisterSpec for CalalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calalr::R`](R) reader structure"]
impl crate::Readable for CalalrSpec {}
#[doc = "`write(|w| ..)` method takes [`calalr::W`](W) writer structure"]
impl crate::Writable for CalalrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALALR to value 0x0101_0000"]
impl crate::Resettable for CalalrSpec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
