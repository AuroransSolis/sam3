#[doc = "Register `EPTCTLENB1_ISOENDPT` writer"]
pub struct W(crate::W<ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>;
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
impl From<crate::W<ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub struct EPT_ENABL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_ENABL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub struct AUTO_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_VALID_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub struct INTDIS_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTDIS_DMA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DATAX_RX` writer - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub struct DATAX_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAX_RX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MDATA_RX` writer - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
pub struct MDATA_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> MDATA_RX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub struct ERR_OVFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_OVFLW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub struct RXRDY_TXKL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY_TXKL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub struct TX_COMPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_COMPLT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TXRDY_TRER` writer - TX Packet Ready/Transaction Error Interrupt Enable"]
pub struct TXRDY_TRER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY_TRER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Interrupt Enable"]
pub struct ERR_FL_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_FL_ISO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `ERR_CRC_NTR` writer - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
pub struct ERR_CRC_NTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_CRC_NTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Interrupt Enable"]
pub struct ERR_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_FLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub struct BUSY_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_BANK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub struct SHRT_PCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHRT_PCKT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    pub fn ept_enabl(&mut self) -> EPT_ENABL_W {
        EPT_ENABL_W { w: self }
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W {
        AUTO_VALID_W { w: self }
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W {
        INTDIS_DMA_W { w: self }
    }
    #[doc = "Bit 6 - DATAx Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn datax_rx(&mut self) -> DATAX_RX_W {
        DATAX_RX_W { w: self }
    }
    #[doc = "Bit 7 - MDATA Interrupt Enable (Only for high bandwidth Isochronous OUT endpoints)"]
    #[inline(always)]
    pub fn mdata_rx(&mut self) -> MDATA_RX_W {
        MDATA_RX_W { w: self }
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W {
        ERR_OVFLW_W { w: self }
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W {
        RXRDY_TXKL_W { w: self }
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W {
        TX_COMPLT_W { w: self }
    }
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy_trer(&mut self) -> TXRDY_TRER_W {
        TXRDY_TRER_W { w: self }
    }
    #[doc = "Bit 12 - Error Flow Interrupt Enable"]
    #[inline(always)]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W {
        ERR_FL_ISO_W { w: self }
    }
    #[doc = "Bit 13 - ISO CRC Error/Number of Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W {
        ERR_CRC_NTR_W { w: self }
    }
    #[doc = "Bit 14 - Bank Flush Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W {
        ERR_FLUSH_W { w: self }
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W {
        BUSY_BANK_W { w: self }
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shrt_pckt(&mut self) -> SHRT_PCKT_W {
        SHRT_PCKT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptctlenb1_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isoendpt_eptctlenb1_isoendpt::W](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC {
    type Writer = W;
}
