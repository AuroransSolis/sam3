#[doc = "Register `MCR5` writer"]
pub type W = crate::W<MCR5_SPEC>;
#[doc = "Field `MDLC` writer - Mailbox Data Length Code"]
pub type MDLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRTR` writer - Mailbox Remote Transmission Request"]
pub type MRTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACR` writer - Abort Request for Mailbox x"]
pub type MACR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTCR` writer - Mailbox Transfer Command"]
pub type MTCR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn mdlc(&mut self) -> MDLC_W<MCR5_SPEC> {
        MDLC_W::new(self, 16)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn mrtr(&mut self) -> MRTR_W<MCR5_SPEC> {
        MRTR_W::new(self, 20)
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    #[must_use]
    pub fn macr(&mut self) -> MACR_W<MCR5_SPEC> {
        MACR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn mtcr(&mut self) -> MTCR_W<MCR5_SPEC> {
        MTCR_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mailbox Control Register (MB = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR5_SPEC;
impl crate::RegisterSpec for MCR5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcr5::W`](W) writer structure"]
impl crate::Writable for MCR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
