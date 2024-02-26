#[doc = "Register `IMR_SPI_MODE` reader"]
pub type R = crate::R<SpiModeImrSpiModeSpec>;
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OvreR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNRE` reader - SPI Underrun Error Interrupt Mask"]
pub type UnreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_imr_spi_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiModeImrSpiModeSpec;
impl crate::RegisterSpec for SpiModeImrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mode_imr_spi_mode::R`](R) reader structure"]
impl crate::Readable for SpiModeImrSpiModeSpec {}
#[doc = "`reset()` method sets IMR_SPI_MODE to value 0"]
impl crate::Resettable for SpiModeImrSpiModeSpec {
    const RESET_VALUE: u32 = 0;
}
