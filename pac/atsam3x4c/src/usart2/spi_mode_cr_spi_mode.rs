#[doc = "Register `CR_SPI_MODE` writer"]
pub type W = crate::W<SpiModeCrSpiModeSpec>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RstrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RsttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RststaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCS` writer - Force SPI Chip Select"]
pub type FcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCS` writer - Release SPI Chip Select"]
pub type RcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RstrxW<SpiModeCrSpiModeSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RsttxW<SpiModeCrSpiModeSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<SpiModeCrSpiModeSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RxdisW<SpiModeCrSpiModeSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<SpiModeCrSpiModeSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TxdisW<SpiModeCrSpiModeSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RststaW<SpiModeCrSpiModeSpec> {
        RststaW::new(self, 8)
    }
    #[doc = "Bit 18 - Force SPI Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FcsW<SpiModeCrSpiModeSpec> {
        FcsW::new(self, 18)
    }
    #[doc = "Bit 19 - Release SPI Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RcsW<SpiModeCrSpiModeSpec> {
        RcsW::new(self, 19)
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mode_cr_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiModeCrSpiModeSpec;
impl crate::RegisterSpec for SpiModeCrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mode_cr_spi_mode::W`](W) writer structure"]
impl crate::Writable for SpiModeCrSpiModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
