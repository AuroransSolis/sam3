#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `MB0` writer - Mailbox 0 Interrupt Disable"]
pub type MB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB1` writer - Mailbox 1 Interrupt Disable"]
pub type MB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB2` writer - Mailbox 2 Interrupt Disable"]
pub type MB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB3` writer - Mailbox 3 Interrupt Disable"]
pub type MB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB4` writer - Mailbox 4 Interrupt Disable"]
pub type MB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB5` writer - Mailbox 5 Interrupt Disable"]
pub type MB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB6` writer - Mailbox 6 Interrupt Disable"]
pub type MB6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB7` writer - Mailbox 7 Interrupt Disable"]
pub type MB7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRA` writer - Error Active Mode Interrupt Disable"]
pub type ERRA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - Warning Limit Interrupt Disable"]
pub type WARN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRP` writer - Error Passive Mode Interrupt Disable"]
pub type ERRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF` writer - Bus Off Mode Interrupt Disable"]
pub type BOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` writer - Sleep Interrupt Disable"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Wakeup Interrupt Disable"]
pub type WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOVF` writer - Timer Overflow Interrupt"]
pub type TOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTP` writer - TimeStamp Interrupt Disable"]
pub type TSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERR` writer - CRC Error Interrupt Disable"]
pub type CERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERR` writer - Stuffing Error Interrupt Disable"]
pub type SERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AERR` writer - Acknowledgment Error Interrupt Disable"]
pub type AERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Form Error Interrupt Disable"]
pub type FERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` writer - Bit Error Interrupt Disable"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> MB0_W<IDR_SPEC> {
        MB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> MB1_W<IDR_SPEC> {
        MB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> MB2_W<IDR_SPEC> {
        MB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> MB3_W<IDR_SPEC> {
        MB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> MB4_W<IDR_SPEC> {
        MB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> MB5_W<IDR_SPEC> {
        MB5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> MB6_W<IDR_SPEC> {
        MB6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> MB7_W<IDR_SPEC> {
        MB7_W::new(self, 7)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn erra(&mut self) -> ERRA_W<IDR_SPEC> {
        ERRA_W::new(self, 16)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WARN_W<IDR_SPEC> {
        WARN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn errp(&mut self) -> ERRP_W<IDR_SPEC> {
        ERRP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn boff(&mut self) -> BOFF_W<IDR_SPEC> {
        BOFF_W::new(self, 19)
    }
    #[doc = "Bit 20 - Sleep Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<IDR_SPEC> {
        SLEEP_W::new(self, 20)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<IDR_SPEC> {
        WAKEUP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tovf(&mut self) -> TOVF_W<IDR_SPEC> {
        TOVF_W::new(self, 22)
    }
    #[doc = "Bit 23 - TimeStamp Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tstp(&mut self) -> TSTP_W<IDR_SPEC> {
        TSTP_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CERR_W<IDR_SPEC> {
        CERR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SERR_W<IDR_SPEC> {
        SERR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AERR_W<IDR_SPEC> {
        AERR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Form Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IDR_SPEC> {
        FERR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<IDR_SPEC> {
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
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
