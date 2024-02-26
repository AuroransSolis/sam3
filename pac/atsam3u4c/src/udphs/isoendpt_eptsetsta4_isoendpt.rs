#[doc = "Register `EPTSETSTA4_ISOENDPT` writer"]
pub type W = crate::W<IsoendptEptsetsta4IsoendptSpec>;
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready Set"]
pub type TxrdyTrerW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<IsoendptEptsetsta4IsoendptSpec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TxrdyTrerW<IsoendptEptsetsta4IsoendptSpec> {
        TxrdyTrerW::new(self, 11)
    }
}
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta4_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoendptEptsetsta4IsoendptSpec;
impl crate::RegisterSpec for IsoendptEptsetsta4IsoendptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptsetsta4_isoendpt::W`](W) writer structure"]
impl crate::Writable for IsoendptEptsetsta4IsoendptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
