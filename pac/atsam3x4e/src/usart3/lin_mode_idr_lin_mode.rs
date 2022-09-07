#[doc = "Register `IDR_LIN_MODE` writer"]
pub struct W(crate::W<LIN_MODE_IDR_LIN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIN_MODE_IDR_LIN_MODE_SPEC>;
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
impl From<crate::W<LIN_MODE_IDR_LIN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIN_MODE_IDR_LIN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `ENDRX` writer - "]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `ENDTX` writer - "]
pub type ENDTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FRAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Disable"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TXEMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `TXBUFE` writer - "]
pub type TXBUFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `RXBUFF` writer - "]
pub type RXBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINBK` writer - LIN Break Sent or LIN Break Received Interrupt Disable"]
pub type LINBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINID` writer - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
pub type LINID_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINTC` writer - LIN Transfer Completed Interrupt Disable"]
pub type LINTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINBE` writer - LIN Bus Error Interrupt Disable"]
pub type LINBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINISFE` writer - LIN Inconsistent Synch Field Error Interrupt Disable"]
pub type LINISFE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINIPE` writer - LIN Identifier Parity Interrupt Disable"]
pub type LINIPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINCE` writer - LIN Checksum Error Interrupt Disable"]
pub type LINCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
#[doc = "Field `LINSNRE` writer - LIN Slave Not Responding Error Interrupt Disable"]
pub type LINSNRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LIN_MODE_IDR_LIN_MODE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W<3> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn endtx(&mut self) -> ENDTX_W<4> {
        ENDTX_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PARE_W<7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 8 - Time-out Interrupt Disable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txbufe(&mut self) -> TXBUFE_W<11> {
        TXBUFE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W<12> {
        RXBUFF_W::new(self)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Disable"]
    #[inline(always)]
    pub fn linbk(&mut self) -> LINBK_W<13> {
        LINBK_W::new(self)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
    #[inline(always)]
    pub fn linid(&mut self) -> LINID_W<14> {
        LINID_W::new(self)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Disable"]
    #[inline(always)]
    pub fn lintc(&mut self) -> LINTC_W<15> {
        LINTC_W::new(self)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn linbe(&mut self) -> LINBE_W<25> {
        LINBE_W::new(self)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Disable"]
    #[inline(always)]
    pub fn linisfe(&mut self) -> LINISFE_W<26> {
        LINISFE_W::new(self)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Disable"]
    #[inline(always)]
    pub fn linipe(&mut self) -> LINIPE_W<27> {
        LINIPE_W::new(self)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Disable"]
    #[inline(always)]
    pub fn lince(&mut self) -> LINCE_W<28> {
        LINCE_W::new(self)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Disable"]
    #[inline(always)]
    pub fn linsnre(&mut self) -> LINSNRE_W<29> {
        LINSNRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lin_mode_idr_lin_mode](index.html) module"]
pub struct LIN_MODE_IDR_LIN_MODE_SPEC;
impl crate::RegisterSpec for LIN_MODE_IDR_LIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lin_mode_idr_lin_mode::W](W) writer structure"]
impl crate::Writable for LIN_MODE_IDR_LIN_MODE_SPEC {
    type Writer = W;
}
