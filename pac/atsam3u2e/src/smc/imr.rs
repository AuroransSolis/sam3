#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_RISE` reader - Ready Busy Rising Edge Detection Interrupt Mask"]
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
#[doc = "Field `RB_FALL` reader - Ready Busy Falling Edge Detection Interrupt Mask"]
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
#[doc = "Field `XFRDONE` reader - Transfer Done Interrupt Mask"]
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
#[doc = "Field `CMDDONE` reader - Command Done Interrupt Mask"]
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
#[doc = "Field `DTOE` reader - Data Timeout Error Interrupt Mask"]
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
#[doc = "Field `UNDEF` reader - Undefined Area Access Interrupt Mask5"]
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
#[doc = "Field `AWB` reader - Accessing While Busy Interrupt Mask"]
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
#[doc = "Field `NFCASE` reader - NFC Access Size Error Interrupt Mask"]
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
#[doc = "Field `RB_EDGE0` reader - Ready/Busy Line 0 Interrupt Mask"]
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
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Mask"]
    #[inline(always)]
    pub fn rb_rise(&self) -> RB_RISE_R {
        RB_RISE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Mask"]
    #[inline(always)]
    pub fn rb_fall(&self) -> RB_FALL_R {
        RB_FALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Mask"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command Done Interrupt Mask"]
    #[inline(always)]
    pub fn cmddone(&self) -> CMDDONE_R {
        CMDDONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Mask5"]
    #[inline(always)]
    pub fn undef(&self) -> UNDEF_R {
        UNDEF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Mask"]
    #[inline(always)]
    pub fn awb(&self) -> AWB_R {
        AWB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Mask"]
    #[inline(always)]
    pub fn nfcase(&self) -> NFCASE_R {
        NFCASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Mask"]
    #[inline(always)]
    pub fn rb_edge0(&self) -> RB_EDGE0_R {
        RB_EDGE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "SMC NFC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
