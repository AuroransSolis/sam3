#[doc = "Register `EPTCTLENB0_ISOENDPT` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC>);
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub type EPT_ENABL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub type AUTO_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type INTDIS_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `DATAX_RX` writer - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub type DATAX_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `MDATA_RX` writer - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub type MDATA_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub type ERR_OVFLW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub type RXRDY_TXKL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub type TX_COMPLT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready/Transaction Error Interrupt Enable"]
pub type TXRDY_TRER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Interrupt Enable"]
pub type ERR_FL_ISO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_CRC_NTR` writer - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
pub type ERR_CRC_NTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Interrupt Enable"]
pub type ERR_FLUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub type BUSY_BANK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub type SHRT_PCKT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_enabl(&mut self) -> EPT_ENABL_W<0> {
        EPT_ENABL_W::new(self)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W<1> {
        AUTO_VALID_W::new(self)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W<3> {
        INTDIS_DMA_W::new(self)
    }
    #[doc = "Bit 6 - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn datax_rx(&mut self) -> DATAX_RX_W<6> {
        DATAX_RX_W::new(self)
    }
    #[doc = "Bit 7 - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn mdata_rx(&mut self) -> MDATA_RX_W<7> {
        MDATA_RX_W::new(self)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W<8> {
        ERR_OVFLW_W::new(self)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<10> {
        TX_COMPLT_W::new(self)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy_trer(&mut self) -> TXRDY_TRER_W<11> {
        TXRDY_TRER_W::new(self)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W<12> {
        ERR_FL_ISO_W::new(self)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W<13> {
        ERR_CRC_NTR_W::new(self)
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W<14> {
        ERR_FLUSH_W::new(self)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W<18> {
        BUSY_BANK_W::new(self)
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> SHRT_PCKT_W<31> {
        SHRT_PCKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptctlenb0_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isoendpt_eptctlenb0_isoendpt::W](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
