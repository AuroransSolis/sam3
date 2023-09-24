#[doc = "Register `CSR_SPI_MODE` reader"]
pub type R = crate::R<SPI_MODE_CSR_SPI_MODE_SPEC>;
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `ENDRX` reader - "]
pub type ENDRX_R = crate::BitReader;
#[doc = "Field `ENDTX` reader - "]
pub type ENDTX_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error"]
pub type UNRE_R = crate::BitReader;
#[doc = "Field `TXBUFE` reader - "]
pub type TXBUFE_R = crate::BitReader;
#[doc = "Field `RXBUFF` reader - "]
pub type RXBUFF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_csr_spi_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MODE_CSR_SPI_MODE_SPEC;
impl crate::RegisterSpec for SPI_MODE_CSR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mode_csr_spi_mode::R`](R) reader structure"]
impl crate::Readable for SPI_MODE_CSR_SPI_MODE_SPEC {}
