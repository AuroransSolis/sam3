#[doc = "Register `EPTCLRSTA4_ISOENDPT` writer"]
pub struct W(crate::W<ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>;
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
impl From<crate::W<ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub struct TOGGLESQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGGLESQ_W<'a> {
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
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
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
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
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
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Clear"]
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
#[doc = "Field `ERR_CRC_NTR` writer - Number of Transaction Error Clear"]
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
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Clear"]
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
impl W {
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    pub fn togglesq(&mut self) -> TOGGLESQ_W {
        TOGGLESQ_W { w: self }
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W {
        RXRDY_TXKL_W { w: self }
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W {
        TX_COMPLT_W { w: self }
    }
    #[doc = "Bit 12 - Error Flow Clear"]
    #[inline(always)]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W {
        ERR_FL_ISO_W { w: self }
    }
    #[doc = "Bit 13 - Number of Transaction Error Clear"]
    #[inline(always)]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W {
        ERR_CRC_NTR_W { w: self }
    }
    #[doc = "Bit 14 - Bank Flush Error Clear"]
    #[inline(always)]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W {
        ERR_FLUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptclrsta4_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isoendpt_eptclrsta4_isoendpt::W](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC {
    type Writer = W;
}
