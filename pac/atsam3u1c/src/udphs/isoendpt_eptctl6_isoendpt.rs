#[doc = "Register `EPTCTL6_ISOENDPT` reader"]
pub struct R(crate::R<ISOENDPT_EPTCTL6_ISOENDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOENDPT_EPTCTL6_ISOENDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOENDPT_EPTCTL6_ISOENDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOENDPT_EPTCTL6_ISOENDPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPT_ENABL` reader - Endpoint Enable"]
pub struct EPT_ENABL_R(crate::FieldReader<bool, bool>);
impl EPT_ENABL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_ENABL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_ENABL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_VALID` reader - Packet Auto-Valid Enabled"]
pub struct AUTO_VALID_R(crate::FieldReader<bool, bool>);
impl AUTO_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTDIS_DMA` reader - Interrupt Disables DMA"]
pub struct INTDIS_DMA_R(crate::FieldReader<bool, bool>);
impl INTDIS_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTDIS_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTDIS_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAX_RX` reader - DATAx Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
pub struct DATAX_RX_R(crate::FieldReader<bool, bool>);
impl DATAX_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAX_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAX_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDATA_RX` reader - MDATA Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
pub struct MDATA_RX_R(crate::FieldReader<bool, bool>);
impl MDATA_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDATA_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDATA_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_OVFLW` reader - Overflow Error Interrupt Enabled"]
pub struct ERR_OVFLW_R(crate::FieldReader<bool, bool>);
impl ERR_OVFLW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_OVFLW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_OVFLW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data Interrupt Enabled"]
pub struct RXRDY_TXKL_R(crate::FieldReader<bool, bool>);
impl RXRDY_TXKL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_TXKL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_TXKL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete Interrupt Enabled"]
pub struct TX_COMPLT_R(crate::FieldReader<bool, bool>);
impl TX_COMPLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_COMPLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_COMPLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY_TRER` reader - TX Packet Ready/Transaction Error Interrupt Enabled"]
pub struct TXRDY_TRER_R(crate::FieldReader<bool, bool>);
impl TXRDY_TRER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_TRER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_TRER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_FL_ISO` reader - Error Flow Interrupt Enabled"]
pub struct ERR_FL_ISO_R(crate::FieldReader<bool, bool>);
impl ERR_FL_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_FL_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_FL_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_CRC_NTR` reader - ISO CRC Error/Number of Transaction Error Interrupt Enabled"]
pub struct ERR_CRC_NTR_R(crate::FieldReader<bool, bool>);
impl ERR_CRC_NTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_CRC_NTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_CRC_NTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_FLUSH` reader - Bank Flush Error Interrupt Enabled"]
pub struct ERR_FLUSH_R(crate::FieldReader<bool, bool>);
impl ERR_FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_FLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_FLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY_BANK` reader - Busy Bank Interrupt Enabled"]
pub struct BUSY_BANK_R(crate::FieldReader<bool, bool>);
impl BUSY_BANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_BANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHRT_PCKT` reader - Short Packet Interrupt Enabled"]
pub struct SHRT_PCKT_R(crate::FieldReader<bool, bool>);
impl SHRT_PCKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHRT_PCKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHRT_PCKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    pub fn ept_enabl(&self) -> EPT_ENABL_R {
        EPT_ENABL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enabled"]
    #[inline(always)]
    pub fn auto_valid(&self) -> AUTO_VALID_R {
        AUTO_VALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Disables DMA"]
    #[inline(always)]
    pub fn intdis_dma(&self) -> INTDIS_DMA_R {
        INTDIS_DMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DATAx Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn datax_rx(&self) -> DATAX_RX_R {
        DATAX_RX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MDATA Interrupt Enabled (Only for High Bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn mdata_rx(&self) -> MDATA_RX_R {
        MDATA_RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ERR_OVFLW_R {
        ERR_OVFLW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enabled"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RXRDY_TXKL_R {
        RXRDY_TXKL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enabled"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TX_COMPLT_R {
        TX_COMPLT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enabled"]
    #[inline(always)]
    pub fn txrdy_trer(&self) -> TXRDY_TRER_R {
        TXRDY_TRER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enabled"]
    #[inline(always)]
    pub fn err_fl_iso(&self) -> ERR_FL_ISO_R {
        ERR_FL_ISO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_crc_ntr(&self) -> ERR_CRC_NTR_R {
        ERR_CRC_NTR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_flush(&self) -> ERR_FLUSH_R {
        ERR_FLUSH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enabled"]
    #[inline(always)]
    pub fn busy_bank(&self) -> BUSY_BANK_R {
        BUSY_BANK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Enabled"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> SHRT_PCKT_R {
        SHRT_PCKT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptctl6_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTCTL6_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCTL6_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoendpt_eptctl6_isoendpt::R](R) reader structure"]
impl crate::Readable for ISOENDPT_EPTCTL6_ISOENDPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTCTL6_ISOENDPT to value 0"]
impl crate::Resettable for ISOENDPT_EPTCTL6_ISOENDPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
