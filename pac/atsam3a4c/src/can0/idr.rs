#[doc = "Register `IDR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<IDR_SPEC>);
#[doc = "Field `MB0` writer - Mailbox 0 Interrupt Disable"]
pub type MB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB1` writer - Mailbox 1 Interrupt Disable"]
pub type MB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB2` writer - Mailbox 2 Interrupt Disable"]
pub type MB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB3` writer - Mailbox 3 Interrupt Disable"]
pub type MB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB4` writer - Mailbox 4 Interrupt Disable"]
pub type MB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB5` writer - Mailbox 5 Interrupt Disable"]
pub type MB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB6` writer - Mailbox 6 Interrupt Disable"]
pub type MB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `MB7` writer - Mailbox 7 Interrupt Disable"]
pub type MB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ERRA` writer - Error Active Mode Interrupt Disable"]
pub type ERRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `WARN` writer - Warning Limit Interrupt Disable"]
pub type WARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ERRP` writer - Error Passive Mode Interrupt Disable"]
pub type ERRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `BOFF` writer - Bus Off Mode Interrupt Disable"]
pub type BOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SLEEP` writer - Sleep Interrupt Disable"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `WAKEUP` writer - Wakeup Interrupt Disable"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `TOVF` writer - Timer Overflow Interrupt"]
pub type TOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `TSTP` writer - TimeStamp Interrupt Disable"]
pub type TSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `CERR` writer - CRC Error Interrupt Disable"]
pub type CERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SERR` writer - Stuffing Error Interrupt Disable"]
pub type SERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `AERR` writer - Acknowledgment Error Interrupt Disable"]
pub type AERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `FERR` writer - Form Error Interrupt Disable"]
pub type FERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `BERR` writer - Bit Error Interrupt Disable"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> MB0_W<0> {
        MB0_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> MB1_W<1> {
        MB1_W::new(self)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> MB2_W<2> {
        MB2_W::new(self)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> MB3_W<3> {
        MB3_W::new(self)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> MB4_W<4> {
        MB4_W::new(self)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> MB5_W<5> {
        MB5_W::new(self)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> MB6_W<6> {
        MB6_W::new(self)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> MB7_W<7> {
        MB7_W::new(self)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn erra(&mut self) -> ERRA_W<16> {
        ERRA_W::new(self)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WARN_W<17> {
        WARN_W::new(self)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn errp(&mut self) -> ERRP_W<18> {
        ERRP_W::new(self)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn boff(&mut self) -> BOFF_W<19> {
        BOFF_W::new(self)
    }
    #[doc = "Bit 20 - Sleep Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<20> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<21> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tovf(&mut self) -> TOVF_W<22> {
        TOVF_W::new(self)
    }
    #[doc = "Bit 23 - TimeStamp Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tstp(&mut self) -> TSTP_W<23> {
        TSTP_W::new(self)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CERR_W<24> {
        CERR_W::new(self)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SERR_W<25> {
        SERR_W::new(self)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AERR_W<26> {
        AERR_W::new(self)
    }
    #[doc = "Bit 27 - Form Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<27> {
        FERR_W::new(self)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<28> {
        BERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
