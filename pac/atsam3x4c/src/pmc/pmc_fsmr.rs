#[doc = "Register `PMC_FSMR` reader"]
pub type R = crate::R<PmcFsmrSpec>;
#[doc = "Register `PMC_FSMR` writer"]
pub type W = crate::W<PmcFsmrSpec>;
#[doc = "Field `FSTT0` reader - Fast Start-up Input Enable 0"]
pub type Fstt0R = crate::BitReader;
#[doc = "Field `FSTT0` writer - Fast Start-up Input Enable 0"]
pub type Fstt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT1` reader - Fast Start-up Input Enable 1"]
pub type Fstt1R = crate::BitReader;
#[doc = "Field `FSTT1` writer - Fast Start-up Input Enable 1"]
pub type Fstt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT2` reader - Fast Start-up Input Enable 2"]
pub type Fstt2R = crate::BitReader;
#[doc = "Field `FSTT2` writer - Fast Start-up Input Enable 2"]
pub type Fstt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT3` reader - Fast Start-up Input Enable 3"]
pub type Fstt3R = crate::BitReader;
#[doc = "Field `FSTT3` writer - Fast Start-up Input Enable 3"]
pub type Fstt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT4` reader - Fast Start-up Input Enable 4"]
pub type Fstt4R = crate::BitReader;
#[doc = "Field `FSTT4` writer - Fast Start-up Input Enable 4"]
pub type Fstt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT5` reader - Fast Start-up Input Enable 5"]
pub type Fstt5R = crate::BitReader;
#[doc = "Field `FSTT5` writer - Fast Start-up Input Enable 5"]
pub type Fstt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT6` reader - Fast Start-up Input Enable 6"]
pub type Fstt6R = crate::BitReader;
#[doc = "Field `FSTT6` writer - Fast Start-up Input Enable 6"]
pub type Fstt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT7` reader - Fast Start-up Input Enable 7"]
pub type Fstt7R = crate::BitReader;
#[doc = "Field `FSTT7` writer - Fast Start-up Input Enable 7"]
pub type Fstt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT8` reader - Fast Start-up Input Enable 8"]
pub type Fstt8R = crate::BitReader;
#[doc = "Field `FSTT8` writer - Fast Start-up Input Enable 8"]
pub type Fstt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT9` reader - Fast Start-up Input Enable 9"]
pub type Fstt9R = crate::BitReader;
#[doc = "Field `FSTT9` writer - Fast Start-up Input Enable 9"]
pub type Fstt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT10` reader - Fast Start-up Input Enable 10"]
pub type Fstt10R = crate::BitReader;
#[doc = "Field `FSTT10` writer - Fast Start-up Input Enable 10"]
pub type Fstt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT11` reader - Fast Start-up Input Enable 11"]
pub type Fstt11R = crate::BitReader;
#[doc = "Field `FSTT11` writer - Fast Start-up Input Enable 11"]
pub type Fstt11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT12` reader - Fast Start-up Input Enable 12"]
pub type Fstt12R = crate::BitReader;
#[doc = "Field `FSTT12` writer - Fast Start-up Input Enable 12"]
pub type Fstt12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT13` reader - Fast Start-up Input Enable 13"]
pub type Fstt13R = crate::BitReader;
#[doc = "Field `FSTT13` writer - Fast Start-up Input Enable 13"]
pub type Fstt13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT14` reader - Fast Start-up Input Enable 14"]
pub type Fstt14R = crate::BitReader;
#[doc = "Field `FSTT14` writer - Fast Start-up Input Enable 14"]
pub type Fstt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT15` reader - Fast Start-up Input Enable 15"]
pub type Fstt15R = crate::BitReader;
#[doc = "Field `FSTT15` writer - Fast Start-up Input Enable 15"]
pub type Fstt15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTAL` reader - RTT Alarm Enable"]
pub type RttalR = crate::BitReader;
#[doc = "Field `RTTAL` writer - RTT Alarm Enable"]
pub type RttalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAL` reader - RTC Alarm Enable"]
pub type RtcalR = crate::BitReader;
#[doc = "Field `RTCAL` writer - RTC Alarm Enable"]
pub type RtcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBAL` reader - USB Alarm Enable"]
pub type UsbalR = crate::BitReader;
#[doc = "Field `USBAL` writer - USB Alarm Enable"]
pub type UsbalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM` reader - Low Power Mode"]
pub type LpmR = crate::BitReader;
#[doc = "Field `LPM` writer - Low Power Mode"]
pub type LpmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast Start-up Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> Fstt0R {
        Fstt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Start-up Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> Fstt1R {
        Fstt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Start-up Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> Fstt2R {
        Fstt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Start-up Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> Fstt3R {
        Fstt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Start-up Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> Fstt4R {
        Fstt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Start-up Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> Fstt5R {
        Fstt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Start-up Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> Fstt6R {
        Fstt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Start-up Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> Fstt7R {
        Fstt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Start-up Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> Fstt8R {
        Fstt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Start-up Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> Fstt9R {
        Fstt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Start-up Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> Fstt10R {
        Fstt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Start-up Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> Fstt11R {
        Fstt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Start-up Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> Fstt12R {
        Fstt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Start-up Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> Fstt13R {
        Fstt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Start-up Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> Fstt14R {
        Fstt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Start-up Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> Fstt15R {
        Fstt15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RttalR {
        RttalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RtcalR {
        RtcalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> UsbalR {
        UsbalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LpmR {
        LpmR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Start-up Input Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn fstt0(&mut self) -> Fstt0W<PmcFsmrSpec> {
        Fstt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fast Start-up Input Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn fstt1(&mut self) -> Fstt1W<PmcFsmrSpec> {
        Fstt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fast Start-up Input Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn fstt2(&mut self) -> Fstt2W<PmcFsmrSpec> {
        Fstt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fast Start-up Input Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn fstt3(&mut self) -> Fstt3W<PmcFsmrSpec> {
        Fstt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Fast Start-up Input Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn fstt4(&mut self) -> Fstt4W<PmcFsmrSpec> {
        Fstt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Fast Start-up Input Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn fstt5(&mut self) -> Fstt5W<PmcFsmrSpec> {
        Fstt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Start-up Input Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn fstt6(&mut self) -> Fstt6W<PmcFsmrSpec> {
        Fstt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Fast Start-up Input Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn fstt7(&mut self) -> Fstt7W<PmcFsmrSpec> {
        Fstt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast Start-up Input Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn fstt8(&mut self) -> Fstt8W<PmcFsmrSpec> {
        Fstt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast Start-up Input Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn fstt9(&mut self) -> Fstt9W<PmcFsmrSpec> {
        Fstt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Fast Start-up Input Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn fstt10(&mut self) -> Fstt10W<PmcFsmrSpec> {
        Fstt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Fast Start-up Input Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn fstt11(&mut self) -> Fstt11W<PmcFsmrSpec> {
        Fstt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Fast Start-up Input Enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn fstt12(&mut self) -> Fstt12W<PmcFsmrSpec> {
        Fstt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast Start-up Input Enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn fstt13(&mut self) -> Fstt13W<PmcFsmrSpec> {
        Fstt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Start-up Input Enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn fstt14(&mut self) -> Fstt14W<PmcFsmrSpec> {
        Fstt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Fast Start-up Input Enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn fstt15(&mut self) -> Fstt15W<PmcFsmrSpec> {
        Fstt15W::new(self, 15)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rttal(&mut self) -> RttalW<PmcFsmrSpec> {
        RttalW::new(self, 16)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcal(&mut self) -> RtcalW<PmcFsmrSpec> {
        RtcalW::new(self, 17)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbal(&mut self) -> UsbalW<PmcFsmrSpec> {
        UsbalW::new(self, 18)
    }
    #[doc = "Bit 20 - Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LpmW<PmcFsmrSpec> {
        LpmW::new(self, 20)
    }
}
#[doc = "Fast Start-up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_fsmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_fsmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcFsmrSpec;
impl crate::RegisterSpec for PmcFsmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_fsmr::R`](R) reader structure"]
impl crate::Readable for PmcFsmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_fsmr::W`](W) writer structure"]
impl crate::Writable for PmcFsmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_FSMR to value 0"]
impl crate::Resettable for PmcFsmrSpec {
    const RESET_VALUE: u32 = 0;
}
