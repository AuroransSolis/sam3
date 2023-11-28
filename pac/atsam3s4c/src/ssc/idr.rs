#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit Empty Interrupt Disable"]
pub type TXEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmission Interrupt Disable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - "]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Ready Interrupt Disable"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRUN` writer - Receive Overrun Interrupt Disable"]
pub type OVRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Reception Interrupt Disable"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - "]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0` writer - Compare 0 Interrupt Disable"]
pub type CP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1` writer - Compare 1 Interrupt Disable"]
pub type CP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSYN` writer - Tx Sync Interrupt Enable"]
pub type TXSYN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSYN` writer - Rx Sync Interrupt Enable"]
pub type RXSYN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IDR_SPEC> {
        TXRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<IDR_SPEC> {
        TXEMPTY_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Transmission Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IDR_SPEC> {
        ENDTX_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IDR_SPEC> {
        TXBUFE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IDR_SPEC> {
        RXRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrun(&mut self) -> OVRUN_W<IDR_SPEC> {
        OVRUN_W::new(self, 5)
    }
    #[doc = "Bit 6 - End of Reception Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IDR_SPEC> {
        ENDRX_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IDR_SPEC> {
        RXBUFF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<IDR_SPEC> {
        CP0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<IDR_SPEC> {
        CP1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txsyn(&mut self) -> TXSYN_W<IDR_SPEC> {
        TXSYN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsyn(&mut self) -> RXSYN_W<IDR_SPEC> {
        RXSYN_W::new(self, 11)
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
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
