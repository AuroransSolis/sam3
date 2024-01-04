#[doc = "Register `IER_LIN_MODE` writer"]
pub type W = crate::W<LIN_MODE_IER_LIN_MODE_SPEC>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - "]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - "]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Enable"]
pub type FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Enable"]
pub type PARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Enable"]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TXEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - "]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - "]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINBK` writer - LIN Break Sent or LIN Break Received Interrupt Enable"]
pub type LINBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINID` writer - LIN Identifier Sent or LIN Identifier Received Interrupt Enable"]
pub type LINID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINTC` writer - LIN Transfer Completed Interrupt Enable"]
pub type LINTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINBE` writer - LIN Bus Error Interrupt Enable"]
pub type LINBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINISFE` writer - LIN Inconsistent Synch Field Error Interrupt Enable"]
pub type LINISFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINIPE` writer - LIN Identifier Parity Interrupt Enable"]
pub type LINIPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCE` writer - LIN Checksum Error Interrupt Enable"]
pub type LINCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINSNRE` writer - LIN Slave Not Responding Error Interrupt Enable"]
pub type LINSNRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        RXRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        TXRDY_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        ENDRX_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        ENDTX_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        OVRE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        FRAME_W::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        PARE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Time-out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        TIMEOUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        TXEMPTY_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        TXBUFE_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        RXBUFF_W::new(self, 12)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linbk(&mut self) -> LINBK_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINBK_W::new(self, 13)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linid(&mut self) -> LINID_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINID_W::new(self, 14)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lintc(&mut self) -> LINTC_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINTC_W::new(self, 15)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linbe(&mut self) -> LINBE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINBE_W::new(self, 25)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linisfe(&mut self) -> LINISFE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINISFE_W::new(self, 26)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linipe(&mut self) -> LINIPE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINIPE_W::new(self, 27)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lince(&mut self) -> LINCE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINCE_W::new(self, 28)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linsnre(&mut self) -> LINSNRE_W<LIN_MODE_IER_LIN_MODE_SPEC> {
        LINSNRE_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_mode_ier_lin_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIN_MODE_IER_LIN_MODE_SPEC;
impl crate::RegisterSpec for LIN_MODE_IER_LIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lin_mode_ier_lin_mode::W`](W) writer structure"]
impl crate::Writable for LIN_MODE_IER_LIN_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
