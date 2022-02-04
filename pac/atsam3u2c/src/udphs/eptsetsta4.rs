#[doc = "Register `EPTSETSTA4` writer"]
pub struct W(crate::W<EPTSETSTA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTSETSTA4_SPEC>;
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
impl From<crate::W<EPTSETSTA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTSETSTA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Set"]
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
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
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
#[doc = "Field `TXRDY` writer - TX Packet Ready Set"]
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
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Set"]
    #[inline(always)]
    pub fn frcestall(&mut self) -> FRCESTALL_W {
        FRCESTALL_W { w: self }
    }
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W {
        RXRDY_TXKL_W { w: self }
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptsetsta4](index.html) module"]
pub struct EPTSETSTA4_SPEC;
impl crate::RegisterSpec for EPTSETSTA4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptsetsta4::W](W) writer structure"]
impl crate::Writable for EPTSETSTA4_SPEC {
    type Writer = W;
}
