#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Enable"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Transfer Interrupt Enable (available in all USART modes of operation)"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Interrupt Enable (available in all USART modes of operation)"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Enable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Enable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Enable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` writer - Max number of Repetitions Reached Interrupt Enable"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Buffer Empty Interrupt Enable (available in all USART modes of operation)"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Buffer Full Interrupt Enable (available in all USART modes of operation)"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Non AcknowledgeInterrupt Enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Enable"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IerSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IerSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RxbrkW<IerSpec> {
        RxbrkW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Receive Transfer Interrupt Enable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IerSpec> {
        EndrxW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Transmit Interrupt Enable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IerSpec> {
        EndtxW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<IerSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FrameW<IerSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PareW<IerSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Time-out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<IerSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IerSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Max number of Repetitions Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iter(&mut self) -> IterW<IerSpec> {
        IterW::new(self, 10)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Enable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IerSpec> {
        TxbufeW::new(self, 11)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Enable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IerSpec> {
        RxbuffW::new(self, 12)
    }
    #[doc = "Bit 13 - Non AcknowledgeInterrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IerSpec> {
        NackW::new(self, 13)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CtsicW<IerSpec> {
        CtsicW::new(self, 19)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
