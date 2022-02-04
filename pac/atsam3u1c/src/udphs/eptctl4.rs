#[doc = "Register `EPTCTL4` reader"]
pub struct R(crate::R<EPTCTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTCTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTCTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTCTL4_SPEC>) -> Self {
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
#[doc = "Field `AUTO_VALID` reader - Packet Auto-Valid Enabled (Not for CONTROL Endpoints)"]
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
#[doc = "Field `NYET_DIS` reader - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub struct NYET_DIS_R(crate::FieldReader<bool, bool>);
impl NYET_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NYET_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYET_DIS_R {
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
#[doc = "Field `TXRDY` reader - TX Packet Ready Interrupt Enabled"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SETUP` reader - Received SETUP Interrupt Enabled"]
pub struct RX_SETUP_R(crate::FieldReader<bool, bool>);
impl RX_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_SNT` reader - Stall Sent Interrupt Enabled"]
pub struct STALL_SNT_R(crate::FieldReader<bool, bool>);
impl STALL_SNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_SNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_SNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK_IN` reader - NAKIN Interrupt Enabled"]
pub struct NAK_IN_R(crate::FieldReader<bool, bool>);
impl NAK_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAK_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK_OUT` reader - NAKOUT Interrupt Enabled"]
pub struct NAK_OUT_R(crate::FieldReader<bool, bool>);
impl NAK_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAK_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_OUT_R {
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
    #[doc = "Bit 1 - Packet Auto-Valid Enabled (Not for CONTROL Endpoints)"]
    #[inline(always)]
    pub fn auto_valid(&self) -> AUTO_VALID_R {
        AUTO_VALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Disables DMA"]
    #[inline(always)]
    pub fn intdis_dma(&self) -> INTDIS_DMA_R {
        INTDIS_DMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    pub fn nyet_dis(&self) -> NYET_DIS_R {
        NYET_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
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
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enabled"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Received SETUP Interrupt Enabled"]
    #[inline(always)]
    pub fn rx_setup(&self) -> RX_SETUP_R {
        RX_SETUP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enabled"]
    #[inline(always)]
    pub fn stall_snt(&self) -> STALL_SNT_R {
        STALL_SNT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enabled"]
    #[inline(always)]
    pub fn nak_in(&self) -> NAK_IN_R {
        NAK_IN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enabled"]
    #[inline(always)]
    pub fn nak_out(&self) -> NAK_OUT_R {
        NAK_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
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
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptctl4](index.html) module"]
pub struct EPTCTL4_SPEC;
impl crate::RegisterSpec for EPTCTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptctl4::R](R) reader structure"]
impl crate::Readable for EPTCTL4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTCTL4 to value 0"]
impl crate::Resettable for EPTCTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
