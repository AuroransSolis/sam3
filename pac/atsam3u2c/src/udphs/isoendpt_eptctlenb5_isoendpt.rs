#[doc = "Register `EPTCTLENB5_ISOENDPT` writer"]
pub type W = crate::W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC>;
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub type EPT_ENABL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub type AUTO_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type INTDIS_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAX_RX` writer - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub type DATAX_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATA_RX` writer - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub type MDATA_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub type ERR_OVFLW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub type RXRDY_TXKL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub type TX_COMPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready/Transaction Error Interrupt Enable"]
pub type TXRDY_TRER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Interrupt Enable"]
pub type ERR_FL_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_CRC_NTR` writer - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
pub type ERR_CRC_NTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Interrupt Enable"]
pub type ERR_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub type BUSY_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub type SHRT_PCKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_enabl(&mut self) -> EPT_ENABL_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        EPT_ENABL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        AUTO_VALID_W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        INTDIS_DMA_W::new(self, 3)
    }
    #[doc = "Bit 6 - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn datax_rx(&mut self) -> DATAX_RX_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        DATAX_RX_W::new(self, 6)
    }
    #[doc = "Bit 7 - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn mdata_rx(&mut self) -> MDATA_RX_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        MDATA_RX_W::new(self, 7)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        ERR_OVFLW_W::new(self, 8)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        RXRDY_TXKL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        TX_COMPLT_W::new(self, 10)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TXRDY_TRER_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        TXRDY_TRER_W::new(self, 11)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        ERR_FL_ISO_W::new(self, 12)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        ERR_CRC_NTR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        ERR_FLUSH_W::new(self, 14)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        BUSY_BANK_W::new(self, 18)
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> SHRT_PCKT_W<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC> {
        SHRT_PCKT_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb5_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptctlenb5_isoendpt::W`](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
