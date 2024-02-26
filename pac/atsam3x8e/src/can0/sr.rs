#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `MB0` reader - Mailbox 0 Event"]
pub type Mb0R = crate::BitReader;
#[doc = "Field `MB1` reader - Mailbox 1 Event"]
pub type Mb1R = crate::BitReader;
#[doc = "Field `MB2` reader - Mailbox 2 Event"]
pub type Mb2R = crate::BitReader;
#[doc = "Field `MB3` reader - Mailbox 3 Event"]
pub type Mb3R = crate::BitReader;
#[doc = "Field `MB4` reader - Mailbox 4 Event"]
pub type Mb4R = crate::BitReader;
#[doc = "Field `MB5` reader - Mailbox 5 Event"]
pub type Mb5R = crate::BitReader;
#[doc = "Field `MB6` reader - Mailbox 6 Event"]
pub type Mb6R = crate::BitReader;
#[doc = "Field `MB7` reader - Mailbox 7 Event"]
pub type Mb7R = crate::BitReader;
#[doc = "Field `ERRA` reader - Error Active Mode"]
pub type ErraR = crate::BitReader;
#[doc = "Field `WARN` reader - Warning Limit"]
pub type WarnR = crate::BitReader;
#[doc = "Field `ERRP` reader - Error Passive Mode"]
pub type ErrpR = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus Off Mode"]
pub type BoffR = crate::BitReader;
#[doc = "Field `SLEEP` reader - CAN controller in Low power Mode"]
pub type SleepR = crate::BitReader;
#[doc = "Field `WAKEUP` reader - CAN controller is not in Low power Mode"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `TOVF` reader - Timer Overflow"]
pub type TovfR = crate::BitReader;
#[doc = "Field `TSTP` reader - "]
pub type TstpR = crate::BitReader;
#[doc = "Field `CERR` reader - Mailbox CRC Error"]
pub type CerrR = crate::BitReader;
#[doc = "Field `SERR` reader - Mailbox Stuffing Error"]
pub type SerrR = crate::BitReader;
#[doc = "Field `AERR` reader - Acknowledgment Error"]
pub type AerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Form Error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `BERR` reader - Bit Error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `RBSY` reader - Receiver busy"]
pub type RbsyR = crate::BitReader;
#[doc = "Field `TBSY` reader - Transmitter busy"]
pub type TbsyR = crate::BitReader;
#[doc = "Field `OVLSY` reader - Overload busy"]
pub type OvlsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mailbox 0 Event"]
    #[inline(always)]
    pub fn mb0(&self) -> Mb0R {
        Mb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Event"]
    #[inline(always)]
    pub fn mb1(&self) -> Mb1R {
        Mb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 2 Event"]
    #[inline(always)]
    pub fn mb2(&self) -> Mb2R {
        Mb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 3 Event"]
    #[inline(always)]
    pub fn mb3(&self) -> Mb3R {
        Mb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mailbox 4 Event"]
    #[inline(always)]
    pub fn mb4(&self) -> Mb4R {
        Mb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mailbox 5 Event"]
    #[inline(always)]
    pub fn mb5(&self) -> Mb5R {
        Mb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mailbox 6 Event"]
    #[inline(always)]
    pub fn mb6(&self) -> Mb6R {
        Mb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 7 Event"]
    #[inline(always)]
    pub fn mb7(&self) -> Mb7R {
        Mb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Active Mode"]
    #[inline(always)]
    pub fn erra(&self) -> ErraR {
        ErraR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Warning Limit"]
    #[inline(always)]
    pub fn warn(&self) -> WarnR {
        WarnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Error Passive Mode"]
    #[inline(always)]
    pub fn errp(&self) -> ErrpR {
        ErrpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus Off Mode"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CAN controller in Low power Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN controller is not in Low power Mode"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer Overflow"]
    #[inline(always)]
    pub fn tovf(&self) -> TovfR {
        TovfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tstp(&self) -> TstpR {
        TstpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mailbox CRC Error"]
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mailbox Stuffing Error"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Acknowledgment Error"]
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Form Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receiver busy"]
    #[inline(always)]
    pub fn rbsy(&self) -> RbsyR {
        RbsyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmitter busy"]
    #[inline(always)]
    pub fn tbsy(&self) -> TbsyR {
        TbsyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Overload busy"]
    #[inline(always)]
    pub fn ovlsy(&self) -> OvlsyR {
        OvlsyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
