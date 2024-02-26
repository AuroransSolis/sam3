#[doc = "Register `EPTCTLENB4` writer"]
pub type W = crate::W<Eptctlenb4Spec>;
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub type EptEnablW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub type AutoValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type IntdisDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET_DIS` writer - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub type NyetDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub type ErrOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub type TxCompltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SETUP` writer - Received SETUP"]
pub type RxSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Interrupt Enable"]
pub type StallSntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_IN` writer - NAKIN Interrupt Enable"]
pub type NakInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Interrupt Enable"]
pub type NakOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub type BusyBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub type ShrtPcktW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_enabl(&mut self) -> EptEnablW<Eptctlenb4Spec> {
        EptEnablW::new(self, 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AutoValidW<Eptctlenb4Spec> {
        AutoValidW::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> IntdisDmaW<Eptctlenb4Spec> {
        IntdisDmaW::new(self, 3)
    }
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_dis(&mut self) -> NyetDisW<Eptctlenb4Spec> {
        NyetDisW::new(self, 4)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ErrOvflwW<Eptctlenb4Spec> {
        ErrOvflwW::new(self, 8)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<Eptctlenb4Spec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TxCompltW<Eptctlenb4Spec> {
        TxCompltW::new(self, 10)
    }
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<Eptctlenb4Spec> {
        TxrdyW::new(self, 11)
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RxSetupW<Eptctlenb4Spec> {
        RxSetupW::new(self, 12)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> StallSntW<Eptctlenb4Spec> {
        StallSntW::new(self, 13)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NakInW<Eptctlenb4Spec> {
        NakInW::new(self, 14)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NakOutW<Eptctlenb4Spec> {
        NakOutW::new(self, 15)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BusyBankW<Eptctlenb4Spec> {
        BusyBankW::new(self, 18)
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> ShrtPcktW<Eptctlenb4Spec> {
        ShrtPcktW::new(self, 31)
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptctlenb4Spec;
impl crate::RegisterSpec for Eptctlenb4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptctlenb4::W`](W) writer structure"]
impl crate::Writable for Eptctlenb4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
