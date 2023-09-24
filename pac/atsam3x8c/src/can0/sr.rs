#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `MB0` reader - Mailbox 0 Event"]
pub type MB0_R = crate::BitReader;
#[doc = "Field `MB1` reader - Mailbox 1 Event"]
pub type MB1_R = crate::BitReader;
#[doc = "Field `MB2` reader - Mailbox 2 Event"]
pub type MB2_R = crate::BitReader;
#[doc = "Field `MB3` reader - Mailbox 3 Event"]
pub type MB3_R = crate::BitReader;
#[doc = "Field `MB4` reader - Mailbox 4 Event"]
pub type MB4_R = crate::BitReader;
#[doc = "Field `MB5` reader - Mailbox 5 Event"]
pub type MB5_R = crate::BitReader;
#[doc = "Field `MB6` reader - Mailbox 6 Event"]
pub type MB6_R = crate::BitReader;
#[doc = "Field `MB7` reader - Mailbox 7 Event"]
pub type MB7_R = crate::BitReader;
#[doc = "Field `ERRA` reader - Error Active Mode"]
pub type ERRA_R = crate::BitReader;
#[doc = "Field `WARN` reader - Warning Limit"]
pub type WARN_R = crate::BitReader;
#[doc = "Field `ERRP` reader - Error Passive Mode"]
pub type ERRP_R = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus Off Mode"]
pub type BOFF_R = crate::BitReader;
#[doc = "Field `SLEEP` reader - CAN controller in Low power Mode"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `WAKEUP` reader - CAN controller is not in Low power Mode"]
pub type WAKEUP_R = crate::BitReader;
#[doc = "Field `TOVF` reader - Timer Overflow"]
pub type TOVF_R = crate::BitReader;
#[doc = "Field `TSTP` reader - "]
pub type TSTP_R = crate::BitReader;
#[doc = "Field `CERR` reader - Mailbox CRC Error"]
pub type CERR_R = crate::BitReader;
#[doc = "Field `SERR` reader - Mailbox Stuffing Error"]
pub type SERR_R = crate::BitReader;
#[doc = "Field `AERR` reader - Acknowledgment Error"]
pub type AERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Form Error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `BERR` reader - Bit Error"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `RBSY` reader - Receiver busy"]
pub type RBSY_R = crate::BitReader;
#[doc = "Field `TBSY` reader - Transmitter busy"]
pub type TBSY_R = crate::BitReader;
#[doc = "Field `OVLSY` reader - Overload busy"]
pub type OVLSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mailbox 0 Event"]
    #[inline(always)]
    pub fn mb0(&self) -> MB0_R {
        MB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Event"]
    #[inline(always)]
    pub fn mb1(&self) -> MB1_R {
        MB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 2 Event"]
    #[inline(always)]
    pub fn mb2(&self) -> MB2_R {
        MB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 3 Event"]
    #[inline(always)]
    pub fn mb3(&self) -> MB3_R {
        MB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mailbox 4 Event"]
    #[inline(always)]
    pub fn mb4(&self) -> MB4_R {
        MB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mailbox 5 Event"]
    #[inline(always)]
    pub fn mb5(&self) -> MB5_R {
        MB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mailbox 6 Event"]
    #[inline(always)]
    pub fn mb6(&self) -> MB6_R {
        MB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 7 Event"]
    #[inline(always)]
    pub fn mb7(&self) -> MB7_R {
        MB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Active Mode"]
    #[inline(always)]
    pub fn erra(&self) -> ERRA_R {
        ERRA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Warning Limit"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Error Passive Mode"]
    #[inline(always)]
    pub fn errp(&self) -> ERRP_R {
        ERRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus Off Mode"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CAN controller in Low power Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN controller is not in Low power Mode"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer Overflow"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tstp(&self) -> TSTP_R {
        TSTP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mailbox CRC Error"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mailbox Stuffing Error"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Acknowledgment Error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Form Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receiver busy"]
    #[inline(always)]
    pub fn rbsy(&self) -> RBSY_R {
        RBSY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmitter busy"]
    #[inline(always)]
    pub fn tbsy(&self) -> TBSY_R {
        TBSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Overload busy"]
    #[inline(always)]
    pub fn ovlsy(&self) -> OVLSY_R {
        OVLSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
