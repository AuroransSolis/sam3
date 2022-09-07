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
pub type FRCESTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTSETSTA4_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - KILL Bank Set (for IN Endpoint)"]
pub type RXRDY_TXKL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTSETSTA4_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Set"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTSETSTA4_SPEC, bool, O>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Set"]
    #[inline(always)]
    pub fn frcestall(&mut self) -> FRCESTALL_W<5> {
        FRCESTALL_W::new(self)
    }
    #[doc = "Bit 9 - KILL Bank Set (for IN Endpoint)"]
    #[inline(always)]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 11 - TX Packet Ready Set"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W<11> {
        TXRDY_W::new(self)
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
