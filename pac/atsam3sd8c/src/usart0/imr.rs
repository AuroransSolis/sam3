#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `RXBRK` reader - Receiver Break Interrupt Mask"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Receive Transfer Interrupt Mask (available in all USART modes of operation)"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmit Interrupt Mask (available in all USART modes of operation)"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error Interrupt Mask"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error Interrupt Mask"]
pub type PareR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Time-out Interrupt Mask"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `ITER` reader - Max number of Repetitions Reached Interrupt Mask"]
pub type IterR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Buffer Empty Interrupt Mask (available in all USART modes of operation)"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Buffer Full Interrupt Mask (available in all USART modes of operation)"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `NACK` reader - Non AcknowledgeInterrupt Mask"]
pub type NackR = crate::BitReader;
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Mask"]
pub type RiicR = crate::BitReader;
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Mask"]
pub type DsricR = crate::BitReader;
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Interrupt Mask"]
pub type DcdicR = crate::BitReader;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Interrupt Mask"]
pub type CtsicR = crate::BitReader;
#[doc = "Field `MANE` reader - Manchester Error Interrupt Mask"]
pub type ManeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Mask"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Receive Transfer Interrupt Mask (available in all USART modes of operation)"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmit Interrupt Mask (available in all USART modes of operation)"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time-out Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Max number of Repetitions Reached Interrupt Mask"]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Mask (available in all USART modes of operation)"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Mask (available in all USART modes of operation)"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Non AcknowledgeInterrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RiicR {
        RiicR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DsricR {
        DsricR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DcdicR {
        DcdicR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn ctsic(&self) -> CtsicR {
        CtsicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Mask"]
    #[inline(always)]
    pub fn mane(&self) -> ManeR {
        ManeR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
