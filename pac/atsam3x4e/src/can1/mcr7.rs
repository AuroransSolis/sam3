#[doc = "Register `MCR7` writer"]
pub type W = crate::W<Mcr7Spec>;
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
    pub fn mdlc(&mut self) -> MdlcW<Mcr7Spec> {
        MdlcW::new(self, 16)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn mrtr(&mut self) -> MrtrW<Mcr7Spec> {
        MrtrW::new(self, 20)
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    #[must_use]
    pub fn macr(&mut self) -> MacrW<Mcr7Spec> {
        MacrW::new(self, 22)
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn mtcr(&mut self) -> MtcrW<Mcr7Spec> {
        MtcrW::new(self, 23)
    }
}
#[doc = "Mailbox Control Register (MB = 7)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr7Spec;
impl crate::RegisterSpec for Mcr7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcr7::W`](W) writer structure"]
impl crate::Writable for Mcr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
