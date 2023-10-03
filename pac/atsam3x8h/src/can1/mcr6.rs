#[doc = "Register `MCR6` writer"]
pub type W = crate::W<MCR6_SPEC>;
#[doc = "Field `MDLC` writer - Mailbox Data Length Code"]
pub type MDLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MRTR` writer - Mailbox Remote Transmission Request"]
pub type MRTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACR` writer - Abort Request for Mailbox x"]
pub type MACR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCR` writer - Mailbox Transfer Command"]
pub type MTCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn mdlc(&mut self) -> MDLC_W<MCR6_SPEC, 16> {
        MDLC_W::new(self)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn mrtr(&mut self) -> MRTR_W<MCR6_SPEC, 20> {
        MRTR_W::new(self)
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    #[must_use]
    pub fn macr(&mut self) -> MACR_W<MCR6_SPEC, 22> {
        MACR_W::new(self)
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn mtcr(&mut self) -> MTCR_W<MCR6_SPEC, 23> {
        MTCR_W::new(self)
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
#[doc = "Mailbox Control Register (MB = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR6_SPEC;
impl crate::RegisterSpec for MCR6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcr6::W`](W) writer structure"]
impl crate::Writable for MCR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
