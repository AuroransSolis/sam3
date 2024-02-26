#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `MB0` writer - Mailbox 0 Interrupt Enable"]
pub type Mb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB1` writer - Mailbox 1 Interrupt Enable"]
pub type Mb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB2` writer - Mailbox 2 Interrupt Enable"]
pub type Mb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB3` writer - Mailbox 3 Interrupt Enable"]
pub type Mb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB4` writer - Mailbox 4 Interrupt Enable"]
pub type Mb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB5` writer - Mailbox 5 Interrupt Enable"]
pub type Mb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB6` writer - Mailbox 6 Interrupt Enable"]
pub type Mb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB7` writer - Mailbox 7 Interrupt Enable"]
pub type Mb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRA` writer - Error Active Mode Interrupt Enable"]
pub type ErraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - Warning Limit Interrupt Enable"]
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRP` writer - Error Passive Mode Interrupt Enable"]
pub type ErrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF` writer - Bus Off Mode Interrupt Enable"]
pub type BoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` writer - Sleep Interrupt Enable"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Wakeup Interrupt Enable"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOVF` writer - Timer Overflow Interrupt Enable"]
pub type TovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTP` writer - TimeStamp Interrupt Enable"]
pub type TstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERR` writer - CRC Error Interrupt Enable"]
pub type CerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERR` writer - Stuffing Error Interrupt Enable"]
pub type SerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AERR` writer - Acknowledgment Error Interrupt Enable"]
pub type AerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Form Error Interrupt Enable"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` writer - Bit Error Interrupt Enable"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> Mb0W<IerSpec> {
        Mb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> Mb1W<IerSpec> {
        Mb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> Mb2W<IerSpec> {
        Mb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> Mb3W<IerSpec> {
        Mb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> Mb4W<IerSpec> {
        Mb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> Mb5W<IerSpec> {
        Mb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> Mb6W<IerSpec> {
        Mb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> Mb7W<IerSpec> {
        Mb7W::new(self, 7)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erra(&mut self) -> ErraW<IerSpec> {
        ErraW::new(self, 16)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WarnW<IerSpec> {
        WarnW::new(self, 17)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errp(&mut self) -> ErrpW<IerSpec> {
        ErrpW::new(self, 18)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boff(&mut self) -> BoffW<IerSpec> {
        BoffW::new(self, 19)
    }
    #[doc = "Bit 20 - Sleep Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<IerSpec> {
        SleepW::new(self, 20)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IerSpec> {
        WakeupW::new(self, 21)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tovf(&mut self) -> TovfW<IerSpec> {
        TovfW::new(self, 22)
    }
    #[doc = "Bit 23 - TimeStamp Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstp(&mut self) -> TstpW<IerSpec> {
        TstpW::new(self, 23)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CerrW<IerSpec> {
        CerrW::new(self, 24)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SerrW<IerSpec> {
        SerrW::new(self, 25)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AerrW<IerSpec> {
        AerrW::new(self, 26)
    }
    #[doc = "Bit 27 - Form Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FerrW<IerSpec> {
        FerrW::new(self, 27)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BerrW<IerSpec> {
        BerrW::new(self, 28)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
