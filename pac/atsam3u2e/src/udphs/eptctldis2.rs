#[doc = "Register `EPTCTLDIS2` writer"]
pub type W = crate::W<Eptctldis2Spec>;
#[doc = "Field `EPT_DISABL` writer - Endpoint Disable"]
pub type EptDisablW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Disable"]
pub type AutoValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type IntdisDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET_DIS` writer - NYET Enable (Only for High Speed Bulk OUT endpoints)"]
pub type NyetDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Disable"]
pub type ErrOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Disable"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Disable"]
pub type TxCompltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SETUP` writer - Received SETUP Interrupt Disable"]
pub type RxSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Interrupt Disable"]
pub type StallSntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_IN` writer - NAKIN Interrupt Disable"]
pub type NakInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Interrupt Disable"]
pub type NakOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Disable"]
pub type BusyBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Interrupt Disable"]
pub type ShrtPcktW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_disabl(&mut self) -> EptDisablW<Eptctldis2Spec> {
        EptDisablW::new(self, 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Disable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AutoValidW<Eptctldis2Spec> {
        AutoValidW::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> IntdisDmaW<Eptctldis2Spec> {
        IntdisDmaW::new(self, 3)
    }
    #[doc = "Bit 4 - NYET Enable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_dis(&mut self) -> NyetDisW<Eptctldis2Spec> {
        NyetDisW::new(self, 4)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ErrOvflwW<Eptctldis2Spec> {
        ErrOvflwW::new(self, 8)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<Eptctldis2Spec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TxCompltW<Eptctldis2Spec> {
        TxCompltW::new(self, 10)
    }
    #[doc = "Bit 11 - TX Packet Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<Eptctldis2Spec> {
        TxrdyW::new(self, 11)
    }
    #[doc = "Bit 12 - Received SETUP Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RxSetupW<Eptctldis2Spec> {
        RxSetupW::new(self, 12)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> StallSntW<Eptctldis2Spec> {
        StallSntW::new(self, 13)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NakInW<Eptctldis2Spec> {
        NakInW::new(self, 14)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NakOutW<Eptctldis2Spec> {
        NakOutW::new(self, 15)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BusyBankW<Eptctldis2Spec> {
        BusyBankW::new(self, 18)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> ShrtPcktW<Eptctldis2Spec> {
        ShrtPcktW::new(self, 31)
    }
}
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptctldis2Spec;
impl crate::RegisterSpec for Eptctldis2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptctldis2::W`](W) writer structure"]
impl crate::Writable for Eptctldis2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
