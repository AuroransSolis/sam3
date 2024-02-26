#[doc = "Register `EPTCTLDIS1_ISOENDPT` writer"]
pub type W = crate::W<IsoendptEptctldis1IsoendptSpec>;
#[doc = "Field `EPT_DISABL` writer - Endpoint Disable"]
pub type EptDisablW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Disable"]
pub type AutoValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type IntdisDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAX_RX` writer - DATAx Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type DataxRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATA_RX` writer - MDATA Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type MdataRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Disable"]
pub type ErrOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Disable"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Disable"]
pub type TxCompltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready/Transaction Error Interrupt Disable"]
pub type TxrdyTrerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Interrupt Disable"]
pub type ErrFlIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_CRC_NTR` writer - ISO CRC Error/Number of Transaction Error Interrupt Disable"]
pub type ErrCrcNtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FLUSH` writer - bank flush error Interrupt Disable"]
pub type ErrFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Disable"]
pub type BusyBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Interrupt Disable"]
pub type ShrtPcktW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_disabl(&mut self) -> EptDisablW<IsoendptEptctldis1IsoendptSpec> {
        EptDisablW::new(self, 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Disable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AutoValidW<IsoendptEptctldis1IsoendptSpec> {
        AutoValidW::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> IntdisDmaW<IsoendptEptctldis1IsoendptSpec> {
        IntdisDmaW::new(self, 3)
    }
    #[doc = "Bit 6 - DATAx Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn datax_rx(&mut self) -> DataxRxW<IsoendptEptctldis1IsoendptSpec> {
        DataxRxW::new(self, 6)
    }
    #[doc = "Bit 7 - MDATA Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn mdata_rx(&mut self) -> MdataRxW<IsoendptEptctldis1IsoendptSpec> {
        MdataRxW::new(self, 7)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ErrOvflwW<IsoendptEptctldis1IsoendptSpec> {
        ErrOvflwW::new(self, 8)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<IsoendptEptctldis1IsoendptSpec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TxCompltW<IsoendptEptctldis1IsoendptSpec> {
        TxCompltW::new(self, 10)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TxrdyTrerW<IsoendptEptctldis1IsoendptSpec> {
        TxrdyTrerW::new(self, 11)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ErrFlIsoW<IsoendptEptctldis1IsoendptSpec> {
        ErrFlIsoW::new(self, 12)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ErrCrcNtrW<IsoendptEptctldis1IsoendptSpec> {
        ErrCrcNtrW::new(self, 13)
    }
    #[doc = "Bit 14 - bank flush error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ErrFlushW<IsoendptEptctldis1IsoendptSpec> {
        ErrFlushW::new(self, 14)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BusyBankW<IsoendptEptctldis1IsoendptSpec> {
        BusyBankW::new(self, 18)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> ShrtPcktW<IsoendptEptctldis1IsoendptSpec> {
        ShrtPcktW::new(self, 31)
    }
}
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis1_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoendptEptctldis1IsoendptSpec;
impl crate::RegisterSpec for IsoendptEptctldis1IsoendptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptctldis1_isoendpt::W`](W) writer structure"]
impl crate::Writable for IsoendptEptctldis1IsoendptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
