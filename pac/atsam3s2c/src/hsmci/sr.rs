#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `CMDRDY` reader - Command Ready"]
pub type CmdrdyR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `BLKE` reader - Data Block Ended"]
pub type BlkeR = crate::BitReader;
#[doc = "Field `DTIP` reader - Data Transfer in Progress"]
pub type DtipR = crate::BitReader;
#[doc = "Field `NOTBUSY` reader - HSMCI Not Busy"]
pub type NotbusyR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of RX Buffer"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of TX Buffer"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `SDIOIRQA` reader - SDIO Interrupt for Slot A"]
pub type SdioirqaR = crate::BitReader;
#[doc = "Field `SDIOWAIT` reader - SDIO Read Wait Operation Status"]
pub type SdiowaitR = crate::BitReader;
#[doc = "Field `CSRCV` reader - CE-ATA Completion Signal Received"]
pub type CsrcvR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - TX Buffer Empty"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RINDE` reader - Response Index Error"]
pub type RindeR = crate::BitReader;
#[doc = "Field `RDIRE` reader - Response Direction Error"]
pub type RdireR = crate::BitReader;
#[doc = "Field `RCRCE` reader - Response CRC Error"]
pub type RcrceR = crate::BitReader;
#[doc = "Field `RENDE` reader - Response End Bit Error"]
pub type RendeR = crate::BitReader;
#[doc = "Field `RTOE` reader - Response Time-out Error"]
pub type RtoeR = crate::BitReader;
#[doc = "Field `DCRCE` reader - Data CRC Error"]
pub type DcrceR = crate::BitReader;
#[doc = "Field `DTOE` reader - Data Time-out Error"]
pub type DtoeR = crate::BitReader;
#[doc = "Field `CSTOE` reader - Completion Signal Time-out Error"]
pub type CstoeR = crate::BitReader;
#[doc = "Field `FIFOEMPTY` reader - FIFO empty flag"]
pub type FifoemptyR = crate::BitReader;
#[doc = "Field `XFRDONE` reader - Transfer Done flag"]
pub type XfrdoneR = crate::BitReader;
#[doc = "Field `ACKRCV` reader - Boot Operation Acknowledge Received"]
pub type AckrcvR = crate::BitReader;
#[doc = "Field `ACKRCVE` reader - Boot Operation Acknowledge Error"]
pub type AckrcveR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun"]
pub type OvreR = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun"]
pub type UnreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Ready"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CmdrdyR {
        CmdrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended"]
    #[inline(always)]
    pub fn blke(&self) -> BlkeR {
        BlkeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress"]
    #[inline(always)]
    pub fn dtip(&self) -> DtipR {
        DtipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSMCI Not Busy"]
    #[inline(always)]
    pub fn notbusy(&self) -> NotbusyR {
        NotbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of TX Buffer"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SdioirqaR {
        SdioirqaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SdiowaitR {
        SdiowaitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CE-ATA Completion Signal Received"]
    #[inline(always)]
    pub fn csrcv(&self) -> CsrcvR {
        CsrcvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Response Index Error"]
    #[inline(always)]
    pub fn rinde(&self) -> RindeR {
        RindeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error"]
    #[inline(always)]
    pub fn rdire(&self) -> RdireR {
        RdireR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error"]
    #[inline(always)]
    pub fn rcrce(&self) -> RcrceR {
        RcrceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error"]
    #[inline(always)]
    pub fn rende(&self) -> RendeR {
        RendeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error"]
    #[inline(always)]
    pub fn rtoe(&self) -> RtoeR {
        RtoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn dcrce(&self) -> DcrceR {
        DcrceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DtoeR {
        DtoeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error"]
    #[inline(always)]
    pub fn cstoe(&self) -> CstoeR {
        CstoeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - FIFO empty flag"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FifoemptyR {
        FifoemptyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transfer Done flag"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XfrdoneR {
        XfrdoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received"]
    #[inline(always)]
    pub fn ackrcv(&self) -> AckrcvR {
        AckrcvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error"]
    #[inline(always)]
    pub fn ackrcve(&self) -> AckrcveR {
        AckrcveR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Overrun"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Underrun"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0xc0e5"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xc0e5;
}
