#[doc = "Register `EPTCTLENB3_ISOENDPT` writer"]
pub type W = crate::W<IsoendptEptctlenb3IsoendptSpec>;
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub type EptEnablW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub type AutoValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type IntdisDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAX_RX` writer - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub type DataxRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATA_RX` writer - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub type MdataRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub type ErrOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub type TxCompltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready/Transaction Error Interrupt Enable"]
pub type TxrdyTrerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Interrupt Enable"]
pub type ErrFlIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_CRC_NTR` writer - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
pub type ErrCrcNtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Interrupt Enable"]
pub type ErrFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub type BusyBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub type ShrtPcktW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_enabl(&mut self) -> EptEnablW<IsoendptEptctlenb3IsoendptSpec> {
        EptEnablW::new(self, 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AutoValidW<IsoendptEptctlenb3IsoendptSpec> {
        AutoValidW::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> IntdisDmaW<IsoendptEptctlenb3IsoendptSpec> {
        IntdisDmaW::new(self, 3)
    }
    #[doc = "Bit 6 - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn datax_rx(&mut self) -> DataxRxW<IsoendptEptctlenb3IsoendptSpec> {
        DataxRxW::new(self, 6)
    }
    #[doc = "Bit 7 - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn mdata_rx(&mut self) -> MdataRxW<IsoendptEptctlenb3IsoendptSpec> {
        MdataRxW::new(self, 7)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ErrOvflwW<IsoendptEptctlenb3IsoendptSpec> {
        ErrOvflwW::new(self, 8)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<IsoendptEptctlenb3IsoendptSpec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TxCompltW<IsoendptEptctlenb3IsoendptSpec> {
        TxCompltW::new(self, 10)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TxrdyTrerW<IsoendptEptctlenb3IsoendptSpec> {
        TxrdyTrerW::new(self, 11)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ErrFlIsoW<IsoendptEptctlenb3IsoendptSpec> {
        ErrFlIsoW::new(self, 12)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ErrCrcNtrW<IsoendptEptctlenb3IsoendptSpec> {
        ErrCrcNtrW::new(self, 13)
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ErrFlushW<IsoendptEptctlenb3IsoendptSpec> {
        ErrFlushW::new(self, 14)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BusyBankW<IsoendptEptctlenb3IsoendptSpec> {
        BusyBankW::new(self, 18)
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> ShrtPcktW<IsoendptEptctlenb3IsoendptSpec> {
        ShrtPcktW::new(self, 31)
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb3_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoendptEptctlenb3IsoendptSpec;
impl crate::RegisterSpec for IsoendptEptctlenb3IsoendptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptctlenb3_isoendpt::W`](W) writer structure"]
impl crate::Writable for IsoendptEptctlenb3IsoendptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
