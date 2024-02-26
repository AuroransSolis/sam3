#[doc = "Register `CSR_SPI_MODE` reader"]
pub type R = crate::R<SpiModeCsrSpiModeSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OvreR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error"]
pub type UnreR = crate::BitReader;
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
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_csr_spi_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiModeCsrSpiModeSpec;
impl crate::RegisterSpec for SpiModeCsrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mode_csr_spi_mode::R`](R) reader structure"]
impl crate::Readable for SpiModeCsrSpiModeSpec {}
