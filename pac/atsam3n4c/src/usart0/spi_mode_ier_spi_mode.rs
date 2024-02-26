#[doc = "Register `IER_SPI_MODE` writer"]
pub type W = crate::W<SpiModeIerSpiModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Enable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<SpiModeIerSpiModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<SpiModeIerSpiModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<SpiModeIerSpiModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<SpiModeIerSpiModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UnreW<SpiModeIerSpiModeSpec> {
        UnreW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_ier_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiModeIerSpiModeSpec;
impl crate::RegisterSpec for SpiModeIerSpiModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mode_ier_spi_mode::W`](W) writer structure"]
impl crate::Writable for SpiModeIerSpiModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
