#[doc = "Register `MSR2` reader"]
pub struct R(crate::R<MSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MTIMESTAMP` reader - Timer value"]
pub struct MTIMESTAMP_R(crate::FieldReader<u16, u16>);
impl MTIMESTAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MTIMESTAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIMESTAMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDLC` reader - Mailbox Data Length Code"]
pub struct MDLC_R(crate::FieldReader<u8, u8>);
impl MDLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MDLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRTR` reader - Mailbox Remote Transmission Request"]
pub struct MRTR_R(crate::FieldReader<bool, bool>);
impl MRTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MRTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MABT` reader - Mailbox Message Abort"]
pub struct MABT_R(crate::FieldReader<bool, bool>);
impl MABT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MABT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MABT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRDY` reader - Mailbox Ready"]
pub struct MRDY_R(crate::FieldReader<bool, bool>);
impl MRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMI` reader - Mailbox Message Ignored"]
pub struct MMI_R(crate::FieldReader<bool, bool>);
impl MMI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer value"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MTIMESTAMP_R {
        MTIMESTAMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    pub fn mdlc(&self) -> MDLC_R {
        MDLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    pub fn mrtr(&self) -> MRTR_R {
        MRTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mailbox Message Abort"]
    #[inline(always)]
    pub fn mabt(&self) -> MABT_R {
        MABT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Mailbox Ready"]
    #[inline(always)]
    pub fn mrdy(&self) -> MRDY_R {
        MRDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mailbox Message Ignored"]
    #[inline(always)]
    pub fn mmi(&self) -> MMI_R {
        MMI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Mailbox Status Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr2](index.html) module"]
pub struct MSR2_SPEC;
impl crate::RegisterSpec for MSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr2::R](R) reader structure"]
impl crate::Readable for MSR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR2 to value 0"]
impl crate::Resettable for MSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
