#[doc = "Register `EPTCTL3_ISOENDPT` reader"]
pub type R = crate::R<IsoendptEptctl3IsoendptSpec>;
#[doc = "Field `EPT_ENABL` reader - Endpoint Enable"]
pub type EptEnablR = crate::BitReader;
#[doc = "Field `AUTO_VALID` reader - Packet Auto-Valid Enabled"]
pub type AutoValidR = crate::BitReader;
#[doc = "Field `INTDIS_DMA` reader - Interrupt Disables DMA"]
pub type IntdisDmaR = crate::BitReader;
#[doc = "Field `DATAX_RX` reader - DATAx Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type DataxRxR = crate::BitReader;
#[doc = "Field `MDATA_RX` reader - MDATA Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type MdataRxR = crate::BitReader;
#[doc = "Field `ERR_OVFLW` reader - Overflow Error Interrupt Enabled"]
pub type ErrOvflwR = crate::BitReader;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data Interrupt Enabled"]
pub type RxrdyTxklR = crate::BitReader;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete Interrupt Enabled"]
pub type TxCompltR = crate::BitReader;
#[doc = "Field `TXRDY_TRER` reader - TX Packet Ready/Transaction Error Interrupt Enabled"]
pub type TxrdyTrerR = crate::BitReader;
#[doc = "Field `ERR_FL_ISO` reader - Error Flow Interrupt Enabled"]
pub type ErrFlIsoR = crate::BitReader;
#[doc = "Field `ERR_CRC_NTR` reader - ISO CRC Error/Number of Transaction Error Interrupt Enabled"]
pub type ErrCrcNtrR = crate::BitReader;
#[doc = "Field `ERR_FLUSH` reader - Bank Flush Error Interrupt Enabled"]
pub type ErrFlushR = crate::BitReader;
#[doc = "Field `BUSY_BANK` reader - Busy Bank Interrupt Enabled"]
pub type BusyBankR = crate::BitReader;
#[doc = "Field `SHRT_PCKT` reader - Short Packet Interrupt Enabled"]
pub type ShrtPcktR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    pub fn ept_enabl(&self) -> EptEnablR {
        EptEnablR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enabled"]
    #[inline(always)]
    pub fn auto_valid(&self) -> AutoValidR {
        AutoValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Disables DMA"]
    #[inline(always)]
    pub fn intdis_dma(&self) -> IntdisDmaR {
        IntdisDmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DATAx Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn datax_rx(&self) -> DataxRxR {
        DataxRxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MDATA Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn mdata_rx(&self) -> MdataRxR {
        MdataRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ErrOvflwR {
        ErrOvflwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enabled"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RxrdyTxklR {
        RxrdyTxklR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enabled"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TxCompltR {
        TxCompltR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enabled"]
    #[inline(always)]
    pub fn txrdy_trer(&self) -> TxrdyTrerR {
        TxrdyTrerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enabled"]
    #[inline(always)]
    pub fn err_fl_iso(&self) -> ErrFlIsoR {
        ErrFlIsoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_crc_ntr(&self) -> ErrCrcNtrR {
        ErrCrcNtrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_flush(&self) -> ErrFlushR {
        ErrFlushR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enabled"]
    #[inline(always)]
    pub fn busy_bank(&self) -> BusyBankR {
        BusyBankR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Enabled"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> ShrtPcktR {
        ShrtPcktR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl3_isoendpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoendptEptctl3IsoendptSpec;
impl crate::RegisterSpec for IsoendptEptctl3IsoendptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoendpt_eptctl3_isoendpt::R`](R) reader structure"]
impl crate::Readable for IsoendptEptctl3IsoendptSpec {}
#[doc = "`reset()` method sets EPTCTL3_ISOENDPT to value 0"]
impl crate::Resettable for IsoendptEptctl3IsoendptSpec {
    const RESET_VALUE: u32 = 0;
}
