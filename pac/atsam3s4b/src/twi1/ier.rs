#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Enable"]
pub type TxcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Enable"]
pub type SvaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Enable"]
pub type GaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Enable"]
pub type ArblstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Enable"]
pub type SclWsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Enable"]
pub type EosaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TxcompW<IerSpec> {
        TxcompW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IerSpec> {
        RxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IerSpec> {
        TxrdyW::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn svacc(&mut self) -> SvaccW<IerSpec> {
        SvaccW::new(self, 4)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gacc(&mut self) -> GaccW<IerSpec> {
        GaccW::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<IerSpec> {
        OvreW::new(self, 6)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IerSpec> {
        NackW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ArblstW<IerSpec> {
        ArblstW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_ws(&mut self) -> SclWsW<IerSpec> {
        SclWsW::new(self, 10)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eosacc(&mut self) -> EosaccW<IerSpec> {
        EosaccW::new(self, 11)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IerSpec> {
        EndrxW::new(self, 12)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IerSpec> {
        EndtxW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IerSpec> {
        RxbuffW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IerSpec> {
        TxbufeW::new(self, 15)
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
