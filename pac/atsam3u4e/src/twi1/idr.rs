#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Disable"]
pub type TXCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Disable"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Disable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Disable"]
pub type SVACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Disable"]
pub type GACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Disable"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Disable"]
pub type ARBLST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Disable"]
pub type SCL_WS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Disable"]
pub type EOSACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Disable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Disable"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TXCOMP_W<IDR_SPEC> {
        TXCOMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IDR_SPEC> {
        RXRDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IDR_SPEC> {
        TXRDY_W::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn svacc(&mut self) -> SVACC_W<IDR_SPEC> {
        SVACC_W::new(self, 4)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn gacc(&mut self) -> GACC_W<IDR_SPEC> {
        GACC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<IDR_SPEC> {
        OVRE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<IDR_SPEC> {
        NACK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<IDR_SPEC> {
        ARBLST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_ws(&mut self) -> SCL_WS_W<IDR_SPEC> {
        SCL_WS_W::new(self, 10)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eosacc(&mut self) -> EOSACC_W<IDR_SPEC> {
        EOSACC_W::new(self, 11)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IDR_SPEC> {
        ENDRX_W::new(self, 12)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IDR_SPEC> {
        ENDTX_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IDR_SPEC> {
        RXBUFF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IDR_SPEC> {
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
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
