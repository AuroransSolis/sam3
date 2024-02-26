#[doc = "Register `IER_LIN_MODE` writer"]
pub type W = crate::W<LinModeIerLinModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - "]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - "]
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
#[doc = "Field `TXBUFE` writer - "]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - "]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINBK` writer - LIN Break Sent or LIN Break Received Interrupt Enable"]
pub type LinbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINID` writer - LIN Identifier Sent or LIN Identifier Received Interrupt Enable"]
pub type LinidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINTC` writer - LIN Transfer Completed Interrupt Enable"]
pub type LintcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINBE` writer - LIN Bus Error Interrupt Enable"]
pub type LinbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINISFE` writer - LIN Inconsistent Synch Field Error Interrupt Enable"]
pub type LinisfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINIPE` writer - LIN Identifier Parity Interrupt Enable"]
pub type LinipeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCE` writer - LIN Checksum Error Interrupt Enable"]
pub type LinceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINSNRE` writer - LIN Slave Not Responding Error Interrupt Enable"]
pub type LinsnreW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<LinModeIerLinModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<LinModeIerLinModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<LinModeIerLinModeSpec> {
        EndrxW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<LinModeIerLinModeSpec> {
        EndtxW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<LinModeIerLinModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FrameW<LinModeIerLinModeSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PareW<LinModeIerLinModeSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Time-out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<LinModeIerLinModeSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<LinModeIerLinModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<LinModeIerLinModeSpec> {
        TxbufeW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<LinModeIerLinModeSpec> {
        RxbuffW::new(self, 12)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linbk(&mut self) -> LinbkW<LinModeIerLinModeSpec> {
        LinbkW::new(self, 13)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linid(&mut self) -> LinidW<LinModeIerLinModeSpec> {
        LinidW::new(self, 14)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lintc(&mut self) -> LintcW<LinModeIerLinModeSpec> {
        LintcW::new(self, 15)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linbe(&mut self) -> LinbeW<LinModeIerLinModeSpec> {
        LinbeW::new(self, 25)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linisfe(&mut self) -> LinisfeW<LinModeIerLinModeSpec> {
        LinisfeW::new(self, 26)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linipe(&mut self) -> LinipeW<LinModeIerLinModeSpec> {
        LinipeW::new(self, 27)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lince(&mut self) -> LinceW<LinModeIerLinModeSpec> {
        LinceW::new(self, 28)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linsnre(&mut self) -> LinsnreW<LinModeIerLinModeSpec> {
        LinsnreW::new(self, 29)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_mode_ier_lin_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinModeIerLinModeSpec;
impl crate::RegisterSpec for LinModeIerLinModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lin_mode_ier_lin_mode::W`](W) writer structure"]
impl crate::Writable for LinModeIerLinModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
