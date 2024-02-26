#[doc = "Register `EPTSETSTA1` writer"]
pub type W = crate::W<Eptsetsta1Spec>;
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
    pub fn frcestall(&mut self) -> FrcestallW<Eptsetsta1Spec> {
        FrcestallW::new(self, 5)
    }
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<Eptsetsta1Spec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<Eptsetsta1Spec> {
        TxrdyW::new(self, 11)
    }
}
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptsetsta1Spec;
impl crate::RegisterSpec for Eptsetsta1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptsetsta1::W`](W) writer structure"]
impl crate::Writable for Eptsetsta1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
