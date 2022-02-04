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
#[doc = "Field `SMCSTS` reader - NAND Flash Controller status (this field cannot be reset)"]
pub struct SMCSTS_R(crate::FieldReader<bool, bool>);
impl SMCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_RISE` reader - Selected Ready Busy Rising Edge Detected"]
pub struct RB_RISE_R(crate::FieldReader<bool, bool>);
impl RB_RISE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_RISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_RISE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_FALL` reader - Selected Ready Busy Falling Edge Detected"]
pub struct RB_FALL_R(crate::FieldReader<bool, bool>);
impl RB_FALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_FALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_FALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCBUSY` reader - NFC Busy (this field cannot be reset)"]
pub struct NFCBUSY_R(crate::FieldReader<bool, bool>);
impl NFCBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NFCBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCWR` reader - NFC Write/Read Operation (this field cannot be reset)"]
pub struct NFCWR_R(crate::FieldReader<bool, bool>);
impl NFCWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NFCWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCSID` reader - NFC Chip Select ID (this field cannot be reset)"]
pub struct NFCSID_R(crate::FieldReader<u8, u8>);
impl NFCSID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCSID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCSID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRDONE` reader - NFC Data Transfer Terminated"]
pub struct XFRDONE_R(crate::FieldReader<bool, bool>);
impl XFRDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XFRDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDDONE` reader - Command Done"]
pub struct CMDDONE_R(crate::FieldReader<bool, bool>);
impl CMDDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOE` reader - Data Timeout Error"]
pub struct DTOE_R(crate::FieldReader<bool, bool>);
impl DTOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDEF` reader - Undefined Area Error"]
pub struct UNDEF_R(crate::FieldReader<bool, bool>);
impl UNDEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWB` reader - Accessing While Busy"]
pub struct AWB_R(crate::FieldReader<bool, bool>);
impl AWB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AWB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCASE` reader - NFC Access Size Error"]
pub struct NFCASE_R(crate::FieldReader<bool, bool>);
impl NFCASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NFCASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EDGE0` reader - Ready/Busy Line 0 Edge Detected"]
pub struct RB_EDGE0_R(crate::FieldReader<bool, bool>);
impl RB_EDGE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EDGE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EDGE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - NAND Flash Controller status (this field cannot be reset)"]
    #[inline(always)]
    pub fn smcsts(&self) -> SMCSTS_R {
        SMCSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selected Ready Busy Rising Edge Detected"]
    #[inline(always)]
    pub fn rb_rise(&self) -> RB_RISE_R {
        RB_RISE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selected Ready Busy Falling Edge Detected"]
    #[inline(always)]
    pub fn rb_fall(&self) -> RB_FALL_R {
        RB_FALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NFC Busy (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcbusy(&self) -> NFCBUSY_R {
        NFCBUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NFC Write/Read Operation (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcwr(&self) -> NFCWR_R {
        NFCWR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - NFC Chip Select ID (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcsid(&self) -> NFCSID_R {
        NFCSID_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NFC Data Transfer Terminated"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command Done"]
    #[inline(always)]
    pub fn cmddone(&self) -> CMDDONE_R {
        CMDDONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Undefined Area Error"]
    #[inline(always)]
    pub fn undef(&self) -> UNDEF_R {
        UNDEF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Accessing While Busy"]
    #[inline(always)]
    pub fn awb(&self) -> AWB_R {
        AWB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - NFC Access Size Error"]
    #[inline(always)]
    pub fn nfcase(&self) -> NFCASE_R {
        NFCASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Edge Detected"]
    #[inline(always)]
    pub fn rb_edge0(&self) -> RB_EDGE0_R {
        RB_EDGE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "SMC NFC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
