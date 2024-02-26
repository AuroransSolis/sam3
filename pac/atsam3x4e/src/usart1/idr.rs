#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Disable"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Transfer Interrupt Disable (available in all USART modes of operation)"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Interrupt Disable (available in all USART modes of operation)"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Disable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` writer - Max Number of Repetitions Reached Interrupt Disable"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Buffer Empty Interrupt Disable (available in all USART modes of operation)"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Buffer Full Interrupt Disable (available in all USART modes of operation)"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Disable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Disable"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Disable"]
pub type ManeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IdrSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IdrSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RxbrkW<IdrSpec> {
        RxbrkW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Receive Transfer Interrupt Disable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IdrSpec> {
        EndrxW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Transmit Interrupt Disable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IdrSpec> {
        EndtxW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<IdrSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FrameW<IdrSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PareW<IdrSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Time-out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<IdrSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IdrSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iter(&mut self) -> IterW<IdrSpec> {
        IterW::new(self, 10)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Disable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IdrSpec> {
        TxbufeW::new(self, 11)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Disable (available in all USART modes of operation)"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IdrSpec> {
        RxbuffW::new(self, 12)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IdrSpec> {
        NackW::new(self, 13)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CtsicW<IdrSpec> {
        CtsicW::new(self, 19)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mane(&mut self) -> ManeW<IdrSpec> {
        ManeW::new(self, 24)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
