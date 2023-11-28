#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `CMDRDY` writer - Command Ready Interrupt Disable"]
pub type CMDRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKE` writer - Data Block Ended Interrupt Disable"]
pub type BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIP` writer - Data Transfer in Progress Interrupt Disable"]
pub type DTIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTBUSY` writer - Data Not Busy Interrupt Disable"]
pub type NOTBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Disable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIRQA` writer - SDIO Interrupt for Slot A Interrupt Disable"]
pub type SDIOIRQA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOWAIT` writer - SDIO Read Wait Operation Status Interrupt Disable"]
pub type SDIOWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRCV` writer - Completion Signal received interrupt Disable"]
pub type CSRCV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Disable"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINDE` writer - Response Index Error Interrupt Disable"]
pub type RINDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIRE` writer - Response Direction Error Interrupt Disable"]
pub type RDIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCE` writer - Response CRC Error Interrupt Disable"]
pub type RCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RENDE` writer - Response End Bit Error Interrupt Disable"]
pub type RENDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOE` writer - Response Time-out Error Interrupt Disable"]
pub type RTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCE` writer - Data CRC Error Interrupt Disable"]
pub type DCRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Time-out Error Interrupt Disable"]
pub type DTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTOE` writer - Completion Signal Time out Error Interrupt Disable"]
pub type CSTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTY` writer - FIFO empty Interrupt Disable"]
pub type FIFOEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Disable"]
pub type XFRDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRCV` writer - Boot Acknowledge Interrupt Disable"]
pub type ACKRCV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRCVE` writer - Boot Acknowledge Error Interrupt Disable"]
pub type ACKRCVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Interrupt Disable"]
pub type OVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Interrupt Disable"]
pub type UNRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Command Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrdy(&mut self) -> CMDRDY_W<IDR_SPEC> {
        CMDRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IDR_SPEC> {
        RXRDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IDR_SPEC> {
        TXRDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blke(&mut self) -> BLKE_W<IDR_SPEC> {
        BLKE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtip(&mut self) -> DTIP_W<IDR_SPEC> {
        DTIP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn notbusy(&mut self) -> NOTBUSY_W<IDR_SPEC> {
        NOTBUSY_W::new(self, 5)
    }
    #[doc = "Bit 6 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IDR_SPEC> {
        ENDRX_W::new(self, 6)
    }
    #[doc = "Bit 7 - End of Transmit Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IDR_SPEC> {
        ENDTX_W::new(self, 7)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioirqa(&mut self) -> SDIOIRQA_W<IDR_SPEC> {
        SDIOIRQA_W::new(self, 8)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sdiowait(&mut self) -> SDIOWAIT_W<IDR_SPEC> {
        SDIOWAIT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Completion Signal received interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn csrcv(&mut self) -> CSRCV_W<IDR_SPEC> {
        CSRCV_W::new(self, 13)
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
    #[doc = "Bit 16 - Response Index Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rinde(&mut self) -> RINDE_W<IDR_SPEC> {
        RINDE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rdire(&mut self) -> RDIRE_W<IDR_SPEC> {
        RDIRE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rcrce(&mut self) -> RCRCE_W<IDR_SPEC> {
        RCRCE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rende(&mut self) -> RENDE_W<IDR_SPEC> {
        RENDE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoe(&mut self) -> RTOE_W<IDR_SPEC> {
        RTOE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dcrce(&mut self) -> DCRCE_W<IDR_SPEC> {
        DCRCE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<IDR_SPEC> {
        DTOE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Completion Signal Time out Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cstoe(&mut self) -> CSTOE_W<IDR_SPEC> {
        CSTOE_W::new(self, 23)
    }
    #[doc = "Bit 26 - FIFO empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<IDR_SPEC> {
        FIFOEMPTY_W::new(self, 26)
    }
    #[doc = "Bit 27 - Transfer Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XFRDONE_W<IDR_SPEC> {
        XFRDONE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Boot Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcv(&mut self) -> ACKRCV_W<IDR_SPEC> {
        ACKRCV_W::new(self, 28)
    }
    #[doc = "Bit 29 - Boot Acknowledge Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcve(&mut self) -> ACKRCVE_W<IDR_SPEC> {
        ACKRCVE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<IDR_SPEC> {
        OVRE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Underrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<IDR_SPEC> {
        UNRE_W::new(self, 31)
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
