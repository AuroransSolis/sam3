#[doc = "Register `EPTCTLDIS2_ISOENDPT` writer"]
pub struct W(crate::W<ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPT_DISABL` writer - Endpoint Disable"]
pub type EPT_DISABL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Disable"]
pub type AUTO_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type INTDIS_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `DATAX_RX` writer - DATAx Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type DATAX_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `MDATA_RX` writer - MDATA Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
pub type MDATA_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Disable"]
pub type ERR_OVFLW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Disable"]
pub type RXRDY_TXKL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Disable"]
pub type TX_COMPLT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready/Transaction Error Interrupt Disable"]
pub type TXRDY_TRER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Interrupt Disable"]
pub type ERR_FL_ISO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_CRC_NTR` writer - ISO CRC Error/Number of Transaction Error Interrupt Disable"]
pub type ERR_CRC_NTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_FLUSH` writer - bank flush error Interrupt Disable"]
pub type ERR_FLUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Disable"]
pub type BUSY_BANK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Interrupt Disable"]
pub type SHRT_PCKT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Endpoint Disable"]
    #[inline(always)]
    pub fn ept_disabl(&mut self) -> EPT_DISABL_W<0> {
        EPT_DISABL_W::new(self)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Disable"]
    #[inline(always)]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W<1> {
        AUTO_VALID_W::new(self)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W<3> {
        INTDIS_DMA_W::new(self)
    }
    #[doc = "Bit 6 - DATAx Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn datax_rx(&mut self) -> DATAX_RX_W<6> {
        DATAX_RX_W::new(self)
    }
    #[doc = "Bit 7 - MDATA Interrupt Disable (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn mdata_rx(&mut self) -> MDATA_RX_W<7> {
        MDATA_RX_W::new(self)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Disable"]
    #[inline(always)]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W<8> {
        ERR_OVFLW_W::new(self)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Disable"]
    #[inline(always)]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<10> {
        TX_COMPLT_W::new(self)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy_trer(&mut self) -> TXRDY_TRER_W<11> {
        TXRDY_TRER_W::new(self)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Disable"]
    #[inline(always)]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W<12> {
        ERR_FL_ISO_W::new(self)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Disable"]
    #[inline(always)]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W<13> {
        ERR_CRC_NTR_W::new(self)
    }
    #[doc = "Bit 14 - bank flush error Interrupt Disable"]
    #[inline(always)]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W<14> {
        ERR_FLUSH_W::new(self)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Disable"]
    #[inline(always)]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W<18> {
        BUSY_BANK_W::new(self)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Disable"]
    #[inline(always)]
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
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptctldis2_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isoendpt_eptctldis2_isoendpt::W](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC {
    type Writer = W;
}
