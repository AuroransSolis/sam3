#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MB0` reader - Mailbox 0 Event"]
pub struct MB0_R(crate::FieldReader<bool, bool>);
impl MB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB1` reader - Mailbox 1 Event"]
pub struct MB1_R(crate::FieldReader<bool, bool>);
impl MB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB2` reader - Mailbox 2 Event"]
pub struct MB2_R(crate::FieldReader<bool, bool>);
impl MB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB3` reader - Mailbox 3 Event"]
pub struct MB3_R(crate::FieldReader<bool, bool>);
impl MB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB4` reader - Mailbox 4 Event"]
pub struct MB4_R(crate::FieldReader<bool, bool>);
impl MB4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB5` reader - Mailbox 5 Event"]
pub struct MB5_R(crate::FieldReader<bool, bool>);
impl MB5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB6` reader - Mailbox 6 Event"]
pub struct MB6_R(crate::FieldReader<bool, bool>);
impl MB6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB7` reader - Mailbox 7 Event"]
pub struct MB7_R(crate::FieldReader<bool, bool>);
impl MB7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRA` reader - Error Active Mode"]
pub struct ERRA_R(crate::FieldReader<bool, bool>);
impl ERRA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WARN` reader - Warning Limit"]
pub struct WARN_R(crate::FieldReader<bool, bool>);
impl WARN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WARN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WARN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRP` reader - Error Passive Mode"]
pub struct ERRP_R(crate::FieldReader<bool, bool>);
impl ERRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFF` reader - Bus Off Mode"]
pub struct BOFF_R(crate::FieldReader<bool, bool>);
impl BOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` reader - CAN controller in Low power Mode"]
pub struct SLEEP_R(crate::FieldReader<bool, bool>);
impl SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` reader - CAN controller is not in Low power Mode"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVF` reader - Timer Overflow"]
pub struct TOVF_R(crate::FieldReader<bool, bool>);
impl TOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTP` reader - "]
pub struct TSTP_R(crate::FieldReader<bool, bool>);
impl TSTP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERR` reader - Mailbox CRC Error"]
pub struct CERR_R(crate::FieldReader<bool, bool>);
impl CERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERR` reader - Mailbox Stuffing Error"]
pub struct SERR_R(crate::FieldReader<bool, bool>);
impl SERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AERR` reader - Acknowledgment Error"]
pub struct AERR_R(crate::FieldReader<bool, bool>);
impl AERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERR` reader - Form Error"]
pub struct FERR_R(crate::FieldReader<bool, bool>);
impl FERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERR` reader - Bit Error"]
pub struct BERR_R(crate::FieldReader<bool, bool>);
impl BERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBSY` reader - Receiver busy"]
pub struct RBSY_R(crate::FieldReader<bool, bool>);
impl RBSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBSY` reader - Transmitter busy"]
pub struct TBSY_R(crate::FieldReader<bool, bool>);
impl TBSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVLSY` reader - Overload busy"]
pub struct OVLSY_R(crate::FieldReader<bool, bool>);
impl OVLSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVLSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVLSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Mailbox 0 Event"]
    #[inline(always)]
    pub fn mb0(&self) -> MB0_R {
        MB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Event"]
    #[inline(always)]
    pub fn mb1(&self) -> MB1_R {
        MB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mailbox 2 Event"]
    #[inline(always)]
    pub fn mb2(&self) -> MB2_R {
        MB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mailbox 3 Event"]
    #[inline(always)]
    pub fn mb3(&self) -> MB3_R {
        MB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mailbox 4 Event"]
    #[inline(always)]
    pub fn mb4(&self) -> MB4_R {
        MB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mailbox 5 Event"]
    #[inline(always)]
    pub fn mb5(&self) -> MB5_R {
        MB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mailbox 6 Event"]
    #[inline(always)]
    pub fn mb6(&self) -> MB6_R {
        MB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mailbox 7 Event"]
    #[inline(always)]
    pub fn mb7(&self) -> MB7_R {
        MB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Error Active Mode"]
    #[inline(always)]
    pub fn erra(&self) -> ERRA_R {
        ERRA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Warning Limit"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Error Passive Mode"]
    #[inline(always)]
    pub fn errp(&self) -> ERRP_R {
        ERRP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bus Off Mode"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CAN controller in Low power Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CAN controller is not in Low power Mode"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer Overflow"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tstp(&self) -> TSTP_R {
        TSTP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mailbox CRC Error"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Mailbox Stuffing Error"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Acknowledgment Error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Form Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit Error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Receiver busy"]
    #[inline(always)]
    pub fn rbsy(&self) -> RBSY_R {
        RBSY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmitter busy"]
    #[inline(always)]
    pub fn tbsy(&self) -> TBSY_R {
        TBSY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Overload busy"]
    #[inline(always)]
    pub fn ovlsy(&self) -> OVLSY_R {
        OVLSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
