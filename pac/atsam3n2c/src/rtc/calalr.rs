#[doc = "Register `CALALR` reader"]
pub type R = crate::R<CALALR_SPEC>;
#[doc = "Register `CALALR` writer"]
pub type W = crate::W<CALALR_SPEC>;
#[doc = "Field `MONTH` reader - Month Alarm"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month Alarm"]
pub type MONTH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MTHEN` reader - Month Alarm Enable"]
pub type MTHEN_R = crate::BitReader;
#[doc = "Field `MTHEN` writer - Month Alarm Enable"]
pub type MTHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATE` reader - Date Alarm"]
pub type DATE_R = crate::FieldReader;
#[doc = "Field `DATE` writer - Date Alarm"]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATEEN` reader - Date Alarm Enable"]
pub type DATEEN_R = crate::BitReader;
#[doc = "Field `DATEEN` writer - Date Alarm Enable"]
pub type DATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    pub fn mthen(&self) -> MTHEN_R {
        MTHEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    pub fn dateen(&self) -> DATEEN_R {
        DATEEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<CALALR_SPEC> {
        MONTH_W::new(self, 16)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mthen(&mut self) -> MTHEN_W<CALALR_SPEC> {
        MTHEN_W::new(self, 23)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<CALALR_SPEC> {
        DATE_W::new(self, 24)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dateen(&mut self) -> DATEEN_W<CALALR_SPEC> {
        DATEEN_W::new(self, 31)
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
#[doc = "Calendar Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calalr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calalr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALALR_SPEC;
impl crate::RegisterSpec for CALALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calalr::R`](R) reader structure"]
impl crate::Readable for CALALR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calalr::W`](W) writer structure"]
impl crate::Writable for CALALR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALALR to value 0x0101_0000"]
impl crate::Resettable for CALALR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0000;
}
