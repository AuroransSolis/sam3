#[doc = "Register `PMC_FSMR` reader"]
pub type R = crate::R<PMC_FSMR_SPEC>;
#[doc = "Register `PMC_FSMR` writer"]
pub type W = crate::W<PMC_FSMR_SPEC>;
#[doc = "Field `FSTT0` reader - Fast Start-up Input Enable 0"]
pub type FSTT0_R = crate::BitReader;
#[doc = "Field `FSTT0` writer - Fast Start-up Input Enable 0"]
pub type FSTT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT1` reader - Fast Start-up Input Enable 1"]
pub type FSTT1_R = crate::BitReader;
#[doc = "Field `FSTT1` writer - Fast Start-up Input Enable 1"]
pub type FSTT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT2` reader - Fast Start-up Input Enable 2"]
pub type FSTT2_R = crate::BitReader;
#[doc = "Field `FSTT2` writer - Fast Start-up Input Enable 2"]
pub type FSTT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT3` reader - Fast Start-up Input Enable 3"]
pub type FSTT3_R = crate::BitReader;
#[doc = "Field `FSTT3` writer - Fast Start-up Input Enable 3"]
pub type FSTT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT4` reader - Fast Start-up Input Enable 4"]
pub type FSTT4_R = crate::BitReader;
#[doc = "Field `FSTT4` writer - Fast Start-up Input Enable 4"]
pub type FSTT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT5` reader - Fast Start-up Input Enable 5"]
pub type FSTT5_R = crate::BitReader;
#[doc = "Field `FSTT5` writer - Fast Start-up Input Enable 5"]
pub type FSTT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT6` reader - Fast Start-up Input Enable 6"]
pub type FSTT6_R = crate::BitReader;
#[doc = "Field `FSTT6` writer - Fast Start-up Input Enable 6"]
pub type FSTT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT7` reader - Fast Start-up Input Enable 7"]
pub type FSTT7_R = crate::BitReader;
#[doc = "Field `FSTT7` writer - Fast Start-up Input Enable 7"]
pub type FSTT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT8` reader - Fast Start-up Input Enable 8"]
pub type FSTT8_R = crate::BitReader;
#[doc = "Field `FSTT8` writer - Fast Start-up Input Enable 8"]
pub type FSTT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT9` reader - Fast Start-up Input Enable 9"]
pub type FSTT9_R = crate::BitReader;
#[doc = "Field `FSTT9` writer - Fast Start-up Input Enable 9"]
pub type FSTT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT10` reader - Fast Start-up Input Enable 10"]
pub type FSTT10_R = crate::BitReader;
#[doc = "Field `FSTT10` writer - Fast Start-up Input Enable 10"]
pub type FSTT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT11` reader - Fast Start-up Input Enable 11"]
pub type FSTT11_R = crate::BitReader;
#[doc = "Field `FSTT11` writer - Fast Start-up Input Enable 11"]
pub type FSTT11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT12` reader - Fast Start-up Input Enable 12"]
pub type FSTT12_R = crate::BitReader;
#[doc = "Field `FSTT12` writer - Fast Start-up Input Enable 12"]
pub type FSTT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT13` reader - Fast Start-up Input Enable 13"]
pub type FSTT13_R = crate::BitReader;
#[doc = "Field `FSTT13` writer - Fast Start-up Input Enable 13"]
pub type FSTT13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT14` reader - Fast Start-up Input Enable 14"]
pub type FSTT14_R = crate::BitReader;
#[doc = "Field `FSTT14` writer - Fast Start-up Input Enable 14"]
pub type FSTT14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT15` reader - Fast Start-up Input Enable 15"]
pub type FSTT15_R = crate::BitReader;
#[doc = "Field `FSTT15` writer - Fast Start-up Input Enable 15"]
pub type FSTT15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTAL` reader - RTT Alarm Enable"]
pub type RTTAL_R = crate::BitReader;
#[doc = "Field `RTTAL` writer - RTT Alarm Enable"]
pub type RTTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAL` reader - RTC Alarm Enable"]
pub type RTCAL_R = crate::BitReader;
#[doc = "Field `RTCAL` writer - RTC Alarm Enable"]
pub type RTCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBAL` reader - USB Alarm Enable"]
pub type USBAL_R = crate::BitReader;
#[doc = "Field `USBAL` writer - USB Alarm Enable"]
pub type USBAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM` reader - Low Power Mode"]
pub type LPM_R = crate::BitReader;
#[doc = "Field `LPM` writer - Low Power Mode"]
pub type LPM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast Start-up Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> FSTT0_R {
        FSTT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Start-up Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> FSTT1_R {
        FSTT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Start-up Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> FSTT2_R {
        FSTT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Start-up Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> FSTT3_R {
        FSTT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Start-up Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> FSTT4_R {
        FSTT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Start-up Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> FSTT5_R {
        FSTT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Start-up Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> FSTT6_R {
        FSTT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Start-up Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> FSTT7_R {
        FSTT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Start-up Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> FSTT8_R {
        FSTT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Start-up Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> FSTT9_R {
        FSTT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Start-up Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> FSTT10_R {
        FSTT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Start-up Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> FSTT11_R {
        FSTT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Start-up Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> FSTT12_R {
        FSTT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Start-up Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> FSTT13_R {
        FSTT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Start-up Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> FSTT14_R {
        FSTT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Start-up Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> FSTT15_R {
        FSTT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RTTAL_R {
        RTTAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RTCAL_R {
        RTCAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> USBAL_R {
        USBAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Start-up Input Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn fstt0(&mut self) -> FSTT0_W<PMC_FSMR_SPEC> {
        FSTT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fast Start-up Input Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn fstt1(&mut self) -> FSTT1_W<PMC_FSMR_SPEC> {
        FSTT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fast Start-up Input Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn fstt2(&mut self) -> FSTT2_W<PMC_FSMR_SPEC> {
        FSTT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fast Start-up Input Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn fstt3(&mut self) -> FSTT3_W<PMC_FSMR_SPEC> {
        FSTT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fast Start-up Input Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn fstt4(&mut self) -> FSTT4_W<PMC_FSMR_SPEC> {
        FSTT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Fast Start-up Input Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn fstt5(&mut self) -> FSTT5_W<PMC_FSMR_SPEC> {
        FSTT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Start-up Input Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn fstt6(&mut self) -> FSTT6_W<PMC_FSMR_SPEC> {
        FSTT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fast Start-up Input Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn fstt7(&mut self) -> FSTT7_W<PMC_FSMR_SPEC> {
        FSTT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast Start-up Input Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn fstt8(&mut self) -> FSTT8_W<PMC_FSMR_SPEC> {
        FSTT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast Start-up Input Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn fstt9(&mut self) -> FSTT9_W<PMC_FSMR_SPEC> {
        FSTT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Fast Start-up Input Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn fstt10(&mut self) -> FSTT10_W<PMC_FSMR_SPEC> {
        FSTT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Fast Start-up Input Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn fstt11(&mut self) -> FSTT11_W<PMC_FSMR_SPEC> {
        FSTT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fast Start-up Input Enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn fstt12(&mut self) -> FSTT12_W<PMC_FSMR_SPEC> {
        FSTT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast Start-up Input Enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn fstt13(&mut self) -> FSTT13_W<PMC_FSMR_SPEC> {
        FSTT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Start-up Input Enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn fstt14(&mut self) -> FSTT14_W<PMC_FSMR_SPEC> {
        FSTT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Fast Start-up Input Enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn fstt15(&mut self) -> FSTT15_W<PMC_FSMR_SPEC> {
        FSTT15_W::new(self, 15)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rttal(&mut self) -> RTTAL_W<PMC_FSMR_SPEC> {
        RTTAL_W::new(self, 16)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcal(&mut self) -> RTCAL_W<PMC_FSMR_SPEC> {
        RTCAL_W::new(self, 17)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbal(&mut self) -> USBAL_W<PMC_FSMR_SPEC> {
        USBAL_W::new(self, 18)
    }
    #[doc = "Bit 20 - Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<PMC_FSMR_SPEC> {
        LPM_W::new(self, 20)
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
#[doc = "Fast Start-up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_fsmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_fsmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_FSMR_SPEC;
impl crate::RegisterSpec for PMC_FSMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_fsmr::R`](R) reader structure"]
impl crate::Readable for PMC_FSMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_fsmr::W`](W) writer structure"]
impl crate::Writable for PMC_FSMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_FSMR to value 0"]
impl crate::Resettable for PMC_FSMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
