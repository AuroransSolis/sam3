#[doc = "Register `IDR_SPI_MODE` writer"]
pub type W = crate::W<SPI_MODE_IDR_SPI_MODE_SPEC>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - "]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - "]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TXEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Disable"]
pub type UNRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - "]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - "]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        RXRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        TXRDY_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        ENDRX_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        ENDTX_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        OVRE_W::new(self, 5)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        TXEMPTY_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        UNRE_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        TXBUFE_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<SPI_MODE_IDR_SPI_MODE_SPEC> {
        RXBUFF_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_idr_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MODE_IDR_SPI_MODE_SPEC;
impl crate::RegisterSpec for SPI_MODE_IDR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mode_idr_spi_mode::W`](W) writer structure"]
impl crate::Writable for SPI_MODE_IDR_SPI_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
