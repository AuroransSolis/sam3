#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RXRDY` reader - Mask RXRDY Interrupt"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Disable TXRDY Interrupt"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `ENDRX` reader - Mask End of Receive Transfer Interrupt"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - Mask End of Transmit Interrupt"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `OVRE` reader - Mask Overrun Error Interrupt"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Mask Framing Error Interrupt"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Mask Parity Error Interrupt"]
pub type PareR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Mask TXEMPTY Interrupt"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Mask TXBUFE Interrupt"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Mask RXBUFF Interrupt"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mask RXRDY Interrupt"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable TXRDY Interrupt"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask End of Receive Transfer Interrupt"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask End of Transmit Interrupt"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Overrun Error Interrupt"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask Framing Error Interrupt"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask Parity Error Interrupt"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask TXEMPTY Interrupt"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask TXBUFE Interrupt"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask RXBUFF Interrupt"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
