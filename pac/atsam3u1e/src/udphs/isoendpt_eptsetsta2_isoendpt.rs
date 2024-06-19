#[doc = "Register `EPTSETSTA2_ISOENDPT` writer"]
pub type W = crate::W<IsoendptEptsetsta2IsoendptSpec>;
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready Set"]
pub type TxrdyTrerW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<IsoendptEptsetsta2IsoendptSpec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TxrdyTrerW<IsoendptEptsetsta2IsoendptSpec> {
        TxrdyTrerW::new(self, 11)
    }
}
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta2_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoendptEptsetsta2IsoendptSpec;
impl crate::RegisterSpec for IsoendptEptsetsta2IsoendptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptsetsta2_isoendpt::W`](W) writer structure"]
impl crate::Writable for IsoendptEptsetsta2IsoendptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
