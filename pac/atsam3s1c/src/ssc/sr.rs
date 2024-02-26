#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmit Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmission"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - "]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Ready"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `OVRUN` reader - Receive Overrun"]
pub type OvrunR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Reception"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - "]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `CP0` reader - Compare 0"]
pub type Cp0R = crate::BitReader;
#[doc = "Field `CP1` reader - Compare 1"]
pub type Cp1R = crate::BitReader;
#[doc = "Field `TXSYN` reader - Transmit Sync"]
pub type TxsynR = crate::BitReader;
#[doc = "Field `RXSYN` reader - Receive Sync"]
pub type RxsynR = crate::BitReader;
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RxenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Transmission"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun"]
    #[inline(always)]
    pub fn ovrun(&self) -> OvrunR {
        OvrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Reception"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 0"]
    #[inline(always)]
    pub fn cp0(&self) -> Cp0R {
        Cp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 1"]
    #[inline(always)]
    pub fn cp1(&self) -> Cp1R {
        Cp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Sync"]
    #[inline(always)]
    pub fn txsyn(&self) -> TxsynR {
        TxsynR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Sync"]
    #[inline(always)]
    pub fn rxsyn(&self) -> RxsynR {
        RxsynR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0xcc"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xcc;
}
