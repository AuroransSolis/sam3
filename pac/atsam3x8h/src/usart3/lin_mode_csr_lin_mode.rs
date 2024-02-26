#[doc = "Register `CSR_LIN_MODE` reader"]
pub type R = crate::R<LinModeCsrLinModeSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `ENDRX` reader - "]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - "]
pub type EndtxR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error"]
pub type PareR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Receiver Time-out"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - "]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - "]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `LINBK` reader - LIN Break Sent or LIN Break Received"]
pub type LinbkR = crate::BitReader;
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received"]
pub type LinidR = crate::BitReader;
#[doc = "Field `LINTC` reader - LIN Transfer Completed"]
pub type LintcR = crate::BitReader;
#[doc = "Field `LINBLS` reader - LIN Bus Line Status"]
pub type LinblsR = crate::BitReader;
#[doc = "Field `LINBE` reader - LIN Bit Error"]
pub type LinbeR = crate::BitReader;
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error"]
pub type LinisfeR = crate::BitReader;
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Error"]
pub type LinipeR = crate::BitReader;
#[doc = "Field `LINCE` reader - LIN Checksum Error"]
pub type LinceR = crate::BitReader;
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error"]
pub type LinsnreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Time-out"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received"]
    #[inline(always)]
    pub fn linbk(&self) -> LinbkR {
        LinbkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received"]
    #[inline(always)]
    pub fn linid(&self) -> LinidR {
        LinidR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Completed"]
    #[inline(always)]
    pub fn lintc(&self) -> LintcR {
        LintcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - LIN Bus Line Status"]
    #[inline(always)]
    pub fn linbls(&self) -> LinblsR {
        LinblsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Bit Error"]
    #[inline(always)]
    pub fn linbe(&self) -> LinbeR {
        LinbeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error"]
    #[inline(always)]
    pub fn linisfe(&self) -> LinisfeR {
        LinisfeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Error"]
    #[inline(always)]
    pub fn linipe(&self) -> LinipeR {
        LinipeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error"]
    #[inline(always)]
    pub fn lince(&self) -> LinceR {
        LinceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error"]
    #[inline(always)]
    pub fn linsnre(&self) -> LinsnreR {
        LinsnreR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_mode_csr_lin_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinModeCsrLinModeSpec;
impl crate::RegisterSpec for LinModeCsrLinModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lin_mode_csr_lin_mode::R`](R) reader structure"]
impl crate::Readable for LinModeCsrLinModeSpec {}
