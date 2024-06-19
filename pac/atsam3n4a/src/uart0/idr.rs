#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RXRDY` writer - Disable RXRDY Interrupt"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Disable TXRDY Interrupt"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - Disable End of Receive Transfer Interrupt"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - Disable End of Transmit Interrupt"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Disable Overrun Error Interrupt"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Disable Framing Error Interrupt"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Disable Parity Error Interrupt"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Disable TXEMPTY Interrupt"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Disable Buffer Empty Interrupt"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Disable Buffer Full Interrupt"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable RXRDY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IdrSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable TXRDY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IdrSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 3 - Disable End of Receive Transfer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IdrSpec> {
        EndrxW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable End of Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IdrSpec> {
        EndtxW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Overrun Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<IdrSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Framing Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FrameW<IdrSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable Parity Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PareW<IdrSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 9 - Disable TXEMPTY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IdrSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 11 - Disable Buffer Empty Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IdrSpec> {
        TxbufeW::new(self, 11)
    }
    #[doc = "Bit 12 - Disable Buffer Full Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IdrSpec> {
        RxbuffW::new(self, 12)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
