#[doc = "Register `EPTCTL0_ISOENDPT` reader"]
pub type R = crate::R<ISOENDPT_EPTCTL0_ISOENDPT_SPEC>;
#[doc = "Field `EPT_ENABL` reader - Endpoint Enable"]
pub type EPT_ENABL_R = crate::BitReader;
#[doc = "Field `AUTO_VALID` reader - Packet Auto-Valid Enabled"]
pub type AUTO_VALID_R = crate::BitReader;
#[doc = "Field `INTDIS_DMA` reader - Interrupt Disables DMA"]
pub type INTDIS_DMA_R = crate::BitReader;
#[doc = "Field `DATAX_RX` reader - DATAx Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type DATAX_RX_R = crate::BitReader;
#[doc = "Field `MDATA_RX` reader - MDATA Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type MDATA_RX_R = crate::BitReader;
#[doc = "Field `ERR_OVFLW` reader - Overflow Error Interrupt Enabled"]
pub type ERR_OVFLW_R = crate::BitReader;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data Interrupt Enabled"]
pub type RXRDY_TXKL_R = crate::BitReader;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete Interrupt Enabled"]
pub type TX_COMPLT_R = crate::BitReader;
#[doc = "Field `TXRDY_TRER` reader - TX Packet Ready/Transaction Error Interrupt Enabled"]
pub type TXRDY_TRER_R = crate::BitReader;
#[doc = "Field `ERR_FL_ISO` reader - Error Flow Interrupt Enabled"]
pub type ERR_FL_ISO_R = crate::BitReader;
#[doc = "Field `ERR_CRC_NTR` reader - ISO CRC Error/Number of Transaction Error Interrupt Enabled"]
pub type ERR_CRC_NTR_R = crate::BitReader;
#[doc = "Field `ERR_FLUSH` reader - Bank Flush Error Interrupt Enabled"]
pub type ERR_FLUSH_R = crate::BitReader;
#[doc = "Field `BUSY_BANK` reader - Busy Bank Interrupt Enabled"]
pub type BUSY_BANK_R = crate::BitReader;
#[doc = "Field `SHRT_PCKT` reader - Short Packet Interrupt Enabled"]
pub type SHRT_PCKT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    pub fn ept_enabl(&self) -> EPT_ENABL_R {
        EPT_ENABL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enabled"]
    #[inline(always)]
    pub fn auto_valid(&self) -> AUTO_VALID_R {
        AUTO_VALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Disables DMA"]
    #[inline(always)]
    pub fn intdis_dma(&self) -> INTDIS_DMA_R {
        INTDIS_DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DATAx Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn datax_rx(&self) -> DATAX_RX_R {
        DATAX_RX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MDATA Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn mdata_rx(&self) -> MDATA_RX_R {
        MDATA_RX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ERR_OVFLW_R {
        ERR_OVFLW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enabled"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RXRDY_TXKL_R {
        RXRDY_TXKL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enabled"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TX_COMPLT_R {
        TX_COMPLT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enabled"]
    #[inline(always)]
    pub fn txrdy_trer(&self) -> TXRDY_TRER_R {
        TXRDY_TRER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enabled"]
    #[inline(always)]
    pub fn err_fl_iso(&self) -> ERR_FL_ISO_R {
        ERR_FL_ISO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_crc_ntr(&self) -> ERR_CRC_NTR_R {
        ERR_CRC_NTR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_flush(&self) -> ERR_FLUSH_R {
        ERR_FLUSH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enabled"]
    #[inline(always)]
    pub fn busy_bank(&self) -> BUSY_BANK_R {
        BUSY_BANK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Enabled"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> SHRT_PCKT_R {
        SHRT_PCKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl0_isoendpt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENDPT_EPTCTL0_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCTL0_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoendpt_eptctl0_isoendpt::R`](R) reader structure"]
impl crate::Readable for ISOENDPT_EPTCTL0_ISOENDPT_SPEC {}
#[doc = "`reset()` method sets EPTCTL0_ISOENDPT to value 0"]
impl crate::Resettable for ISOENDPT_EPTCTL0_ISOENDPT_SPEC {
    const RESET_VALUE: u32 = 0;
}
