#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TXCOMP` reader - Transmission Completed (automatically set / reset)"]
pub type TxcompR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready (automatically set / reset)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready (automatically set / reset)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `SVREAD` reader - Slave Read (automatically set / reset)"]
pub type SvreadR = crate::BitReader;
#[doc = "Field `SVACC` reader - Slave Access (automatically set / reset)"]
pub type SvaccR = crate::BitReader;
#[doc = "Field `GACC` reader - General Call Access (clear on read)"]
pub type GaccR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (clear on read)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledged (clear on read)"]
pub type NackR = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration Lost (clear on read)"]
pub type ArblstR = crate::BitReader;
#[doc = "Field `SCLWS` reader - Clock Wait State (automatically set / reset)"]
pub type SclwsR = crate::BitReader;
#[doc = "Field `EOSACC` reader - End Of Slave Access (clear on read)"]
pub type EosaccR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of RX buffer"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of TX buffer"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - TX Buffer Empty"]
pub type TxbufeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Completed (automatically set / reset)"]
    #[inline(always)]
    pub fn txcomp(&self) -> TxcompR {
        TxcompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready (automatically set / reset)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready (automatically set / reset)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Read (automatically set / reset)"]
    #[inline(always)]
    pub fn svread(&self) -> SvreadR {
        SvreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Access (automatically set / reset)"]
    #[inline(always)]
    pub fn svacc(&self) -> SvaccR {
        SvaccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call Access (clear on read)"]
    #[inline(always)]
    pub fn gacc(&self) -> GaccR {
        GaccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error (clear on read)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledged (clear on read)"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost (clear on read)"]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State (automatically set / reset)"]
    #[inline(always)]
    pub fn sclws(&self) -> SclwsR {
        SclwsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access (clear on read)"]
    #[inline(always)]
    pub fn eosacc(&self) -> EosaccR {
        EosaccR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of RX buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - End of TX buffer"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 13) & 1) != 0)
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
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0xf009"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xf009;
}
