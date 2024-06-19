#[doc = "Register `EPTSETSTA2` writer"]
pub type W = crate::W<Eptsetsta2Spec>;
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Set"]
pub type FrcestallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Set"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn frcestall(&mut self) -> FrcestallW<Eptsetsta2Spec> {
        FrcestallW::new(self, 5)
    }
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<Eptsetsta2Spec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<Eptsetsta2Spec> {
        TxrdyW::new(self, 11)
    }
}
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptsetsta2Spec;
impl crate::RegisterSpec for Eptsetsta2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptsetsta2::W`](W) writer structure"]
impl crate::Writable for Eptsetsta2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
