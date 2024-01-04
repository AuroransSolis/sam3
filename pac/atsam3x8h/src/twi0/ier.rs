#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Enable"]
pub type TXCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Enable"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Enable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Enable"]
pub type SVACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Enable"]
pub type GACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Enable"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Enable"]
pub type ARBLST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Enable"]
pub type SCL_WS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Enable"]
pub type EOSACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TXCOMP_W<IER_SPEC> {
        TXCOMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IER_SPEC> {
        RXRDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IER_SPEC> {
        TXRDY_W::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn svacc(&mut self) -> SVACC_W<IER_SPEC> {
        SVACC_W::new(self, 4)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gacc(&mut self) -> GACC_W<IER_SPEC> {
        GACC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<IER_SPEC> {
        OVRE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<IER_SPEC> {
        NACK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<IER_SPEC> {
        ARBLST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_ws(&mut self) -> SCL_WS_W<IER_SPEC> {
        SCL_WS_W::new(self, 10)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eosacc(&mut self) -> EOSACC_W<IER_SPEC> {
        EOSACC_W::new(self, 11)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IER_SPEC> {
        ENDRX_W::new(self, 12)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IER_SPEC> {
        ENDTX_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IER_SPEC> {
        RXBUFF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IER_SPEC> {
        TXBUFE_W::new(self, 15)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
