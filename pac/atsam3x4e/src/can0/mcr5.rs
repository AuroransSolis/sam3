#[doc = "Register `MCR5` writer"]
pub type W = crate::W<Mcr5Spec>;
#[doc = "Field `MDLC` writer - Mailbox Data Length Code"]
pub type MdlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRTR` writer - Mailbox Remote Transmission Request"]
pub type MrtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACR` writer - Abort Request for Mailbox x"]
pub type MacrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTCR` writer - Mailbox Transfer Command"]
pub type MtcrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn mdlc(&mut self) -> MdlcW<Mcr5Spec> {
        MdlcW::new(self, 16)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn mrtr(&mut self) -> MrtrW<Mcr5Spec> {
        MrtrW::new(self, 20)
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    #[must_use]
    pub fn macr(&mut self) -> MacrW<Mcr5Spec> {
        MacrW::new(self, 22)
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn mtcr(&mut self) -> MtcrW<Mcr5Spec> {
        MtcrW::new(self, 23)
    }
}
#[doc = "Mailbox Control Register (MB = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr5Spec;
impl crate::RegisterSpec for Mcr5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcr5::W`](W) writer structure"]
impl crate::Writable for Mcr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
