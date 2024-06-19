#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit Empty Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Ready Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRUN` writer - Receive Overrun Interrupt Disable"]
pub type OvrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0` writer - Compare 0 Interrupt Disable"]
pub type Cp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1` writer - Compare 1 Interrupt Disable"]
pub type Cp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSYN` writer - Tx Sync Interrupt Enable"]
pub type TxsynW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSYN` writer - Rx Sync Interrupt Enable"]
pub type RxsynW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IdrSpec> {
        TxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IdrSpec> {
        TxemptyW::new(self, 1)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<IdrSpec> {
        RxrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrun(&mut self) -> OvrunW<IdrSpec> {
        OvrunW::new(self, 5)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> Cp0W<IdrSpec> {
        Cp0W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> Cp1W<IdrSpec> {
        Cp1W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txsyn(&mut self) -> TxsynW<IdrSpec> {
        TxsynW::new(self, 10)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsyn(&mut self) -> RxsynW<IdrSpec> {
        RxsynW::new(self, 11)
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
