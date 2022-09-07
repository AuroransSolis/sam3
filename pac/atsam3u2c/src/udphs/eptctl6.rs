#[doc = "Register `EPTCTL6` reader"]
pub struct R(crate::R<EPTCTL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTCTL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTCTL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTCTL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPT_ENABL` reader - Endpoint Enable"]
pub type EPT_ENABL_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_VALID` reader - Packet Auto-Valid Enabled (Not for CONTROL Endpoints)"]
pub type AUTO_VALID_R = crate::BitReader<bool>;
#[doc = "Field `INTDIS_DMA` reader - Interrupt Disables DMA"]
pub type INTDIS_DMA_R = crate::BitReader<bool>;
#[doc = "Field `NYET_DIS` reader - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub type NYET_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ERR_OVFLW` reader - Overflow Error Interrupt Enabled"]
pub type ERR_OVFLW_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data Interrupt Enabled"]
pub type RXRDY_TXKL_R = crate::BitReader<bool>;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete Interrupt Enabled"]
pub type TX_COMPLT_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - TX Packet Ready Interrupt Enabled"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `RX_SETUP` reader - Received SETUP Interrupt Enabled"]
pub type RX_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `STALL_SNT` reader - Stall Sent Interrupt Enabled"]
pub type STALL_SNT_R = crate::BitReader<bool>;
#[doc = "Field `NAK_IN` reader - NAKIN Interrupt Enabled"]
pub type NAK_IN_R = crate::BitReader<bool>;
#[doc = "Field `NAK_OUT` reader - NAKOUT Interrupt Enabled"]
pub type NAK_OUT_R = crate::BitReader<bool>;
#[doc = "Field `BUSY_BANK` reader - Busy Bank Interrupt Enabled"]
pub type BUSY_BANK_R = crate::BitReader<bool>;
#[doc = "Field `SHRT_PCKT` reader - Short Packet Interrupt Enabled"]
pub type SHRT_PCKT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    pub fn ept_enabl(&self) -> EPT_ENABL_R {
        EPT_ENABL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enabled (Not for CONTROL Endpoints)"]
    #[inline(always)]
    pub fn auto_valid(&self) -> AUTO_VALID_R {
        AUTO_VALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Disables DMA"]
    #[inline(always)]
    pub fn intdis_dma(&self) -> INTDIS_DMA_R {
        INTDIS_DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    pub fn nyet_dis(&self) -> NYET_DIS_R {
        NYET_DIS_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enabled"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Received SETUP Interrupt Enabled"]
    #[inline(always)]
    pub fn rx_setup(&self) -> RX_SETUP_R {
        RX_SETUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enabled"]
    #[inline(always)]
    pub fn stall_snt(&self) -> STALL_SNT_R {
        STALL_SNT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enabled"]
    #[inline(always)]
    pub fn nak_in(&self) -> NAK_IN_R {
        NAK_IN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enabled"]
    #[inline(always)]
    pub fn nak_out(&self) -> NAK_OUT_R {
        NAK_OUT_R::new(((self.bits >> 15) & 1) != 0)
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
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptctl6](index.html) module"]
pub struct EPTCTL6_SPEC;
impl crate::RegisterSpec for EPTCTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptctl6::R](R) reader structure"]
impl crate::Readable for EPTCTL6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTCTL6 to value 0"]
impl crate::Resettable for EPTCTL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
