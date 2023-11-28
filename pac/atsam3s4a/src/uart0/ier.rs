#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RXRDY` writer - Enable RXRDY Interrupt"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Enable TXRDY Interrupt"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - Enable End of Receive Transfer Interrupt"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - Enable End of Transmit Interrupt"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Enable Overrun Error Interrupt"]
pub type OVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Enable Framing Error Interrupt"]
pub type FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Enable Parity Error Interrupt"]
pub type PARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Enable TXEMPTY Interrupt"]
pub type TXEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Enable Buffer Empty Interrupt"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Enable Buffer Full Interrupt"]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable RXRDY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IER_SPEC> {
        RXRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable TXRDY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IER_SPEC> {
        TXRDY_W::new(self, 1)
    }
    #[doc = "Bit 3 - Enable End of Receive Transfer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IER_SPEC> {
        ENDRX_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable End of Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IER_SPEC> {
        ENDTX_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Overrun Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<IER_SPEC> {
        OVRE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Framing Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<IER_SPEC> {
        FRAME_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Parity Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<IER_SPEC> {
        PARE_W::new(self, 7)
    }
    #[doc = "Bit 9 - Enable TXEMPTY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<IER_SPEC> {
        TXEMPTY_W::new(self, 9)
    }
    #[doc = "Bit 11 - Enable Buffer Empty Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IER_SPEC> {
        TXBUFE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Buffer Full Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IER_SPEC> {
        RXBUFF_W::new(self, 12)
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
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
