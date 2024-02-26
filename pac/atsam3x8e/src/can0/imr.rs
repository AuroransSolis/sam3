#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `MB0` reader - Mailbox 0 Interrupt Mask"]
pub type Mb0R = crate::BitReader;
#[doc = "Field `MB1` reader - Mailbox 1 Interrupt Mask"]
pub type Mb1R = crate::BitReader;
#[doc = "Field `MB2` reader - Mailbox 2 Interrupt Mask"]
pub type Mb2R = crate::BitReader;
#[doc = "Field `MB3` reader - Mailbox 3 Interrupt Mask"]
pub type Mb3R = crate::BitReader;
#[doc = "Field `MB4` reader - Mailbox 4 Interrupt Mask"]
pub type Mb4R = crate::BitReader;
#[doc = "Field `MB5` reader - Mailbox 5 Interrupt Mask"]
pub type Mb5R = crate::BitReader;
#[doc = "Field `MB6` reader - Mailbox 6 Interrupt Mask"]
pub type Mb6R = crate::BitReader;
#[doc = "Field `MB7` reader - Mailbox 7 Interrupt Mask"]
pub type Mb7R = crate::BitReader;
#[doc = "Field `ERRA` reader - Error Active Mode Interrupt Mask"]
pub type ErraR = crate::BitReader;
#[doc = "Field `WARN` reader - Warning Limit Interrupt Mask"]
pub type WarnR = crate::BitReader;
#[doc = "Field `ERRP` reader - Error Passive Mode Interrupt Mask"]
pub type ErrpR = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus Off Mode Interrupt Mask"]
pub type BoffR = crate::BitReader;
#[doc = "Field `SLEEP` reader - Sleep Interrupt Mask"]
pub type SleepR = crate::BitReader;
#[doc = "Field `WAKEUP` reader - Wakeup Interrupt Mask"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `TOVF` reader - Timer Overflow Interrupt Mask"]
pub type TovfR = crate::BitReader;
#[doc = "Field `TSTP` reader - Timestamp Interrupt Mask"]
pub type TstpR = crate::BitReader;
#[doc = "Field `CERR` reader - CRC Error Interrupt Mask"]
pub type CerrR = crate::BitReader;
#[doc = "Field `SERR` reader - Stuffing Error Interrupt Mask"]
pub type SerrR = crate::BitReader;
#[doc = "Field `AERR` reader - Acknowledgment Error Interrupt Mask"]
pub type AerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Form Error Interrupt Mask"]
pub type FerrR = crate::BitReader;
#[doc = "Field `BERR` reader - Bit Error Interrupt Mask"]
pub type BerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Mask"]
    #[inline(always)]
    pub fn mb0(&self) -> Mb0R {
        Mb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Mask"]
    #[inline(always)]
    pub fn mb1(&self) -> Mb1R {
        Mb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Mask"]
    #[inline(always)]
    pub fn mb2(&self) -> Mb2R {
        Mb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Mask"]
    #[inline(always)]
    pub fn mb3(&self) -> Mb3R {
        Mb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Mask"]
    #[inline(always)]
    pub fn mb4(&self) -> Mb4R {
        Mb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Mask"]
    #[inline(always)]
    pub fn mb5(&self) -> Mb5R {
        Mb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Mask"]
    #[inline(always)]
    pub fn mb6(&self) -> Mb6R {
        Mb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Mask"]
    #[inline(always)]
    pub fn mb7(&self) -> Mb7R {
        Mb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Mask"]
    #[inline(always)]
    pub fn erra(&self) -> ErraR {
        ErraR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Mask"]
    #[inline(always)]
    pub fn warn(&self) -> WarnR {
        WarnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Mask"]
    #[inline(always)]
    pub fn errp(&self) -> ErrpR {
        ErrpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Mask"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Sleep Interrupt Mask"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn tovf(&self) -> TovfR {
        TovfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tstp(&self) -> TstpR {
        TstpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Mask"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Mask"]
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Form Error Interrupt Mask"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
