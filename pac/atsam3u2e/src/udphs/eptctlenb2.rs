#[doc = "Register `EPTCTLENB2` writer"]
pub struct W(crate::W<EPTCTLENB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTCTLENB2_SPEC>;
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
impl From<crate::W<EPTCTLENB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTCTLENB2_SPEC>) -> Self {
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
#[doc = "Field `NYET_DIS` writer - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub struct NYET_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `TXRDY` writer - TX Packet Ready Interrupt Enable"]
pub struct TXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY_W<'a> {
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
#[doc = "Field `RX_SETUP` writer - Received SETUP"]
pub struct RX_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SETUP_W<'a> {
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
#[doc = "Field `STALL_SNT` writer - Stall Sent Interrupt Enable"]
pub struct STALL_SNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_SNT_W<'a> {
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
#[doc = "Field `NAK_IN` writer - NAKIN Interrupt Enable"]
pub struct NAK_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_IN_W<'a> {
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
#[doc = "Field `NAK_OUT` writer - NAKOUT Interrupt Enable"]
pub struct NAK_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
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
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    pub fn nyet_dis(&mut self) -> NYET_DIS_W {
        NYET_DIS_W { w: self }
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
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    pub fn rx_setup(&mut self) -> RX_SETUP_W {
        RX_SETUP_W { w: self }
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enable"]
    #[inline(always)]
    pub fn stall_snt(&mut self) -> STALL_SNT_W {
        STALL_SNT_W { w: self }
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enable"]
    #[inline(always)]
    pub fn nak_in(&mut self) -> NAK_IN_W {
        NAK_IN_W { w: self }
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enable"]
    #[inline(always)]
    pub fn nak_out(&mut self) -> NAK_OUT_W {
        NAK_OUT_W { w: self }
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
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptctlenb2](index.html) module"]
pub struct EPTCTLENB2_SPEC;
impl crate::RegisterSpec for EPTCTLENB2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptctlenb2::W](W) writer structure"]
impl crate::Writable for EPTCTLENB2_SPEC {
    type Writer = W;
}
