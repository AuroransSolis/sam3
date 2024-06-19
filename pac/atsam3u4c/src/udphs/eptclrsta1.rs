#[doc = "Register `EPTCLRSTA1` writer"]
pub type W = crate::W<Eptclrsta1Spec>;
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Clear"]
pub type FrcestallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TogglesqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TxCompltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SETUP` writer - Received SETUP Clear"]
pub type RxSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Clear"]
pub type StallSntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_IN` writer - NAKIN Clear"]
pub type NakInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Clear"]
pub type NakOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn frcestall(&mut self) -> FrcestallW<Eptclrsta1Spec> {
        FrcestallW::new(self, 5)
    }
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn togglesq(&mut self) -> TogglesqW<Eptclrsta1Spec> {
        TogglesqW::new(self, 6)
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<Eptclrsta1Spec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TxCompltW<Eptclrsta1Spec> {
        TxCompltW::new(self, 10)
    }
    #[doc = "Bit 12 - Received SETUP Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RxSetupW<Eptclrsta1Spec> {
        RxSetupW::new(self, 12)
    }
    #[doc = "Bit 13 - Stall Sent Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> StallSntW<Eptclrsta1Spec> {
        StallSntW::new(self, 13)
    }
    #[doc = "Bit 14 - NAKIN Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NakInW<Eptclrsta1Spec> {
        NakInW::new(self, 14)
    }
    #[doc = "Bit 15 - NAKOUT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NakOutW<Eptclrsta1Spec> {
        NakOutW::new(self, 15)
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptclrsta1Spec;
impl crate::RegisterSpec for Eptclrsta1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptclrsta1::W`](W) writer structure"]
impl crate::Writable for Eptclrsta1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
