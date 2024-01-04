#[doc = "Register `EPTSETSTA1` writer"]
pub type W = crate::W<EPTSETSTA1_SPEC>;
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Set"]
pub type FRCESTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
pub type RXRDY_TXKL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Set"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn frcestall(&mut self) -> FRCESTALL_W<EPTSETSTA1_SPEC> {
        FRCESTALL_W::new(self, 5)
    }
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<EPTSETSTA1_SPEC> {
        RXRDY_TXKL_W::new(self, 9)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<EPTSETSTA1_SPEC> {
        TXRDY_W::new(self, 11)
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
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPTSETSTA1_SPEC;
impl crate::RegisterSpec for EPTSETSTA1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptsetsta1::W`](W) writer structure"]
impl crate::Writable for EPTSETSTA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
