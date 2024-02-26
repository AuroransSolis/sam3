#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Receiver Transfer"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmitter Transfer"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error"]
pub type PareR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Transmission Buffer Empty"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Receive Buffer Full"]
pub type RxbuffR = crate::BitReader;
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
    #[doc = "Bit 3 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmitter Transfer"]
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
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
