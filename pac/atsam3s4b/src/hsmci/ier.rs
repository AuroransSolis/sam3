#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `CMDRDY` writer - Command Ready Interrupt Enable"]
pub type CmdrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKE` writer - Data Block Ended Interrupt Enable"]
pub type BlkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIP` writer - Data Transfer in Progress Interrupt Enable"]
pub type DtipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTBUSY` writer - Data Not Busy Interrupt Enable"]
pub type NotbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIRQA` writer - SDIO Interrupt for Slot A Interrupt Enable"]
pub type SdioirqaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOWAIT` writer - SDIO Read Wait Operation Status Interrupt Enable"]
pub type SdiowaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRCV` writer - Completion Signal Received Interrupt Enable"]
pub type CsrcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINDE` writer - Response Index Error Interrupt Enable"]
pub type RindeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIRE` writer - Response Direction Error Interrupt Enable"]
pub type RdireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCE` writer - Response CRC Error Interrupt Enable"]
pub type RcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RENDE` writer - Response End Bit Error Interrupt Enable"]
pub type RendeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOE` writer - Response Time-out Error Interrupt Enable"]
pub type RtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCE` writer - Data CRC Error Interrupt Enable"]
pub type DcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Time-out Error Interrupt Enable"]
pub type DtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTOE` writer - Completion Signal Timeout Error Interrupt Enable"]
pub type CstoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEMPTY` writer - FIFO empty Interrupt enable"]
pub type FifoemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt enable"]
pub type XfrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRCV` writer - Boot Acknowledge Interrupt Enable"]
pub type AckrcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRCVE` writer - Boot Acknowledge Error Interrupt Enable"]
pub type AckrcveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Interrupt Enable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Command Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrdy(&mut self) -> CmdrdyW<IerSpec> {
        CmdrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IerSpec> {
        RxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IerSpec> {
        TxrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blke(&mut self) -> BlkeW<IerSpec> {
        BlkeW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtip(&mut self) -> DtipW<IerSpec> {
        DtipW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn notbusy(&mut self) -> NotbusyW<IerSpec> {
        NotbusyW::new(self, 5)
    }
    #[doc = "Bit 6 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IerSpec> {
        EndrxW::new(self, 6)
    }
    #[doc = "Bit 7 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IerSpec> {
        EndtxW::new(self, 7)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioirqa(&mut self) -> SdioirqaW<IerSpec> {
        SdioirqaW::new(self, 8)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdiowait(&mut self) -> SdiowaitW<IerSpec> {
        SdiowaitW::new(self, 12)
    }
    #[doc = "Bit 13 - Completion Signal Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csrcv(&mut self) -> CsrcvW<IerSpec> {
        CsrcvW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IerSpec> {
        RxbuffW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IerSpec> {
        TxbufeW::new(self, 15)
    }
    #[doc = "Bit 16 - Response Index Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rinde(&mut self) -> RindeW<IerSpec> {
        RindeW::new(self, 16)
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdire(&mut self) -> RdireW<IerSpec> {
        RdireW::new(self, 17)
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcrce(&mut self) -> RcrceW<IerSpec> {
        RcrceW::new(self, 18)
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rende(&mut self) -> RendeW<IerSpec> {
        RendeW::new(self, 19)
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoe(&mut self) -> RtoeW<IerSpec> {
        RtoeW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcrce(&mut self) -> DcrceW<IerSpec> {
        DcrceW::new(self, 21)
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DtoeW<IerSpec> {
        DtoeW::new(self, 22)
    }
    #[doc = "Bit 23 - Completion Signal Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cstoe(&mut self) -> CstoeW<IerSpec> {
        CstoeW::new(self, 23)
    }
    #[doc = "Bit 26 - FIFO empty Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FifoemptyW<IerSpec> {
        FifoemptyW::new(self, 26)
    }
    #[doc = "Bit 27 - Transfer Done Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XfrdoneW<IerSpec> {
        XfrdoneW::new(self, 27)
    }
    #[doc = "Bit 28 - Boot Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcv(&mut self) -> AckrcvW<IerSpec> {
        AckrcvW::new(self, 28)
    }
    #[doc = "Bit 29 - Boot Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcve(&mut self) -> AckrcveW<IerSpec> {
        AckrcveW::new(self, 29)
    }
    #[doc = "Bit 30 - Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<IerSpec> {
        OvreW::new(self, 30)
    }
    #[doc = "Bit 31 - Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UnreW<IerSpec> {
        UnreW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
