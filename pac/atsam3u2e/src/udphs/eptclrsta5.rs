#[doc = "Register `EPTCLRSTA5` writer"]
pub struct W(crate::W<EPTCLRSTA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTCLRSTA5_SPEC>;
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
impl From<crate::W<EPTCLRSTA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTCLRSTA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Clear"]
pub struct FRCESTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCESTALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
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
#[doc = "Field `RX_SETUP` writer - Received SETUP Clear"]
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
#[doc = "Field `STALL_SNT` writer - Stall Sent Clear"]
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
#[doc = "Field `NAK_IN` writer - NAKIN Clear"]
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
#[doc = "Field `NAK_OUT` writer - NAKOUT Clear"]
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
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Clear"]
    #[inline(always)]
    pub fn frcestall(&mut self) -> FRCESTALL_W {
        FRCESTALL_W { w: self }
    }
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
    #[doc = "Bit 12 - Received SETUP Clear"]
    #[inline(always)]
    pub fn rx_setup(&mut self) -> RX_SETUP_W {
        RX_SETUP_W { w: self }
    }
    #[doc = "Bit 13 - Stall Sent Clear"]
    #[inline(always)]
    pub fn stall_snt(&mut self) -> STALL_SNT_W {
        STALL_SNT_W { w: self }
    }
    #[doc = "Bit 14 - NAKIN Clear"]
    #[inline(always)]
    pub fn nak_in(&mut self) -> NAK_IN_W {
        NAK_IN_W { w: self }
    }
    #[doc = "Bit 15 - NAKOUT Clear"]
    #[inline(always)]
    pub fn nak_out(&mut self) -> NAK_OUT_W {
        NAK_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptclrsta5](index.html) module"]
pub struct EPTCLRSTA5_SPEC;
impl crate::RegisterSpec for EPTCLRSTA5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptclrsta5::W](W) writer structure"]
impl crate::Writable for EPTCLRSTA5_SPEC {
    type Writer = W;
}
