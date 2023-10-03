#[doc = "Register `EPTSETSTA3_ISOENDPT` writer"]
pub type W = crate::W<ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC>;
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
pub type RXRDY_TXKL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready Set"]
pub type TXRDY_TRER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC, 9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TXRDY_TRER_W<ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC, 11> {
        TXRDY_TRER_W::new(self)
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
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta3_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptsetsta3_isoendpt::W`](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
