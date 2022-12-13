#[doc = "Register `MSR5` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MSR5_SPEC>);
#[doc = "Field `MTIMESTAMP` reader - Timer value"]
pub type MTIMESTAMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MDLC` reader - Mailbox Data Length Code"]
pub type MDLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRTR` reader - Mailbox Remote Transmission Request"]
pub type MRTR_R = crate::BitReader<bool>;
#[doc = "Field `MABT` reader - Mailbox Message Abort"]
pub type MABT_R = crate::BitReader<bool>;
#[doc = "Field `MRDY` reader - Mailbox Ready"]
pub type MRDY_R = crate::BitReader<bool>;
#[doc = "Field `MMI` reader - Mailbox Message Ignored"]
pub type MMI_R = crate::BitReader<bool>;
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
        MRTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Mailbox Message Abort"]
    #[inline(always)]
    pub fn mabt(&self) -> MABT_R {
        MABT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mailbox Ready"]
    #[inline(always)]
    pub fn mrdy(&self) -> MRDY_R {
        MRDY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mailbox Message Ignored"]
    #[inline(always)]
    pub fn mmi(&self) -> MMI_R {
        MMI_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Mailbox Status Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr5](index.html) module"]
pub struct MSR5_SPEC;
impl crate::RegisterSpec for MSR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr5::R](R) reader structure"]
impl crate::Readable for MSR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR5 to value 0"]
impl crate::Resettable for MSR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
