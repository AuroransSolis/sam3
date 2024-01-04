#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `MB0` writer - Mailbox 0 Interrupt Enable"]
pub type MB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB1` writer - Mailbox 1 Interrupt Enable"]
pub type MB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB2` writer - Mailbox 2 Interrupt Enable"]
pub type MB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB3` writer - Mailbox 3 Interrupt Enable"]
pub type MB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB4` writer - Mailbox 4 Interrupt Enable"]
pub type MB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB5` writer - Mailbox 5 Interrupt Enable"]
pub type MB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB6` writer - Mailbox 6 Interrupt Enable"]
pub type MB6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB7` writer - Mailbox 7 Interrupt Enable"]
pub type MB7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRA` writer - Error Active Mode Interrupt Enable"]
pub type ERRA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - Warning Limit Interrupt Enable"]
pub type WARN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRP` writer - Error Passive Mode Interrupt Enable"]
pub type ERRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF` writer - Bus Off Mode Interrupt Enable"]
pub type BOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` writer - Sleep Interrupt Enable"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Wakeup Interrupt Enable"]
pub type WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOVF` writer - Timer Overflow Interrupt Enable"]
pub type TOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTP` writer - TimeStamp Interrupt Enable"]
pub type TSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERR` writer - CRC Error Interrupt Enable"]
pub type CERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERR` writer - Stuffing Error Interrupt Enable"]
pub type SERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AERR` writer - Acknowledgment Error Interrupt Enable"]
pub type AERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Form Error Interrupt Enable"]
pub type FERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` writer - Bit Error Interrupt Enable"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> MB0_W<IER_SPEC> {
        MB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> MB1_W<IER_SPEC> {
        MB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> MB2_W<IER_SPEC> {
        MB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> MB3_W<IER_SPEC> {
        MB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> MB4_W<IER_SPEC> {
        MB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> MB5_W<IER_SPEC> {
        MB5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> MB6_W<IER_SPEC> {
        MB6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> MB7_W<IER_SPEC> {
        MB7_W::new(self, 7)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erra(&mut self) -> ERRA_W<IER_SPEC> {
        ERRA_W::new(self, 16)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WARN_W<IER_SPEC> {
        WARN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errp(&mut self) -> ERRP_W<IER_SPEC> {
        ERRP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boff(&mut self) -> BOFF_W<IER_SPEC> {
        BOFF_W::new(self, 19)
    }
    #[doc = "Bit 20 - Sleep Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<IER_SPEC> {
        SLEEP_W::new(self, 20)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<IER_SPEC> {
        WAKEUP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tovf(&mut self) -> TOVF_W<IER_SPEC> {
        TOVF_W::new(self, 22)
    }
    #[doc = "Bit 23 - TimeStamp Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstp(&mut self) -> TSTP_W<IER_SPEC> {
        TSTP_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CERR_W<IER_SPEC> {
        CERR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SERR_W<IER_SPEC> {
        SERR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AERR_W<IER_SPEC> {
        AERR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Form Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IER_SPEC> {
        FERR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<IER_SPEC> {
        BERR_W::new(self, 28)
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
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
