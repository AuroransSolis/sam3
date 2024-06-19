#[doc = "Register `MSR4` reader"]
pub type R = crate::R<Msr4Spec>;
#[doc = "Field `MTIMESTAMP` reader - Timer value"]
pub type MtimestampR = crate::FieldReader<u16>;
#[doc = "Field `MDLC` reader - Mailbox Data Length Code"]
pub type MdlcR = crate::FieldReader;
#[doc = "Field `MRTR` reader - Mailbox Remote Transmission Request"]
pub type MrtrR = crate::BitReader;
#[doc = "Field `MABT` reader - Mailbox Message Abort"]
pub type MabtR = crate::BitReader;
#[doc = "Field `MRDY` reader - Mailbox Ready"]
pub type MrdyR = crate::BitReader;
#[doc = "Field `MMI` reader - Mailbox Message Ignored"]
pub type MmiR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Timer value"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MtimestampR {
        MtimestampR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    pub fn mdlc(&self) -> MdlcR {
        MdlcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    pub fn mrtr(&self) -> MrtrR {
        MrtrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Mailbox Message Abort"]
    #[inline(always)]
    pub fn mabt(&self) -> MabtR {
        MabtR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mailbox Ready"]
    #[inline(always)]
    pub fn mrdy(&self) -> MrdyR {
        MrdyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mailbox Message Ignored"]
    #[inline(always)]
    pub fn mmi(&self) -> MmiR {
        MmiR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Mailbox Status Register (MB = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`msr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Msr4Spec;
impl crate::RegisterSpec for Msr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr4::R`](R) reader structure"]
impl crate::Readable for Msr4Spec {}
#[doc = "`reset()` method sets MSR4 to value 0"]
impl crate::Resettable for Msr4Spec {
    const RESET_VALUE: u32 = 0;
}
