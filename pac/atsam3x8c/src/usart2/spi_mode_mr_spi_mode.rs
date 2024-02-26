#[doc = "Register `MR_SPI_MODE` reader"]
pub type R = crate::R<SpiModeMrSpiModeSpec>;
#[doc = "Register `MR_SPI_MODE` writer"]
pub type W = crate::W<SpiModeMrSpiModeSpec>;
#[doc = "USART Mode of Operation"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsartMode {
    #[doc = "14: SPI master"]
    SpiMaster = 14,
    #[doc = "15: SPI Slave"]
    SpiSlave = 15,
}
impl From<UsartMode> for u8 {
    #[inline(always)]
    fn from(variant: UsartMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsartMode {
    type Ux = u8;
}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type UsartModeR = crate::FieldReader<UsartMode>;
impl UsartModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UsartMode> {
        match self.bits {
            14 => Some(UsartMode::SpiMaster),
            15 => Some(UsartMode::SpiSlave),
            _ => None,
        }
    }
    #[doc = "SPI master"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UsartMode::SpiMaster
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == UsartMode::SpiSlave
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type UsartModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, UsartMode>;
impl<'a, REG> UsartModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::SpiMaster)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::SpiSlave)
    }
}
#[doc = "Clock Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usclks {
    #[doc = "0: master Clock MCK is selected"]
    Mck = 0,
    #[doc = "1: Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    Div = 1,
    #[doc = "3: Serial Clock SLK is selected"]
    Sck = 3,
}
impl From<Usclks> for u8 {
    #[inline(always)]
    fn from(variant: Usclks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usclks {
    type Ux = u8;
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type UsclksR = crate::FieldReader<Usclks>;
impl UsclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usclks> {
        match self.bits {
            0 => Some(Usclks::Mck),
            1 => Some(Usclks::Div),
            3 => Some(Usclks::Sck),
            _ => None,
        }
    }
    #[doc = "master Clock MCK is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Usclks::Mck
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == Usclks::Div
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == Usclks::Sck
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type UsclksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usclks>;
impl<'a, REG> UsclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "master Clock MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclks::Mck)
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(Usclks::Div)
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclks::Sck)
    }
}
#[doc = "Character Length"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chrl {
    #[doc = "3: Character length is 8 bits"]
    _8Bit = 3,
}
impl From<Chrl> for u8 {
    #[inline(always)]
    fn from(variant: Chrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chrl {
    type Ux = u8;
}
#[doc = "Field `CHRL` reader - Character Length"]
pub type ChrlR = crate::FieldReader<Chrl>;
impl ChrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chrl> {
        match self.bits {
            3 => Some(Chrl::_8Bit),
            _ => None,
        }
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Chrl::_8Bit
    }
}
#[doc = "Field `CHRL` writer - Character Length"]
pub type ChrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chrl>;
impl<'a, REG> ChrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrl::_8Bit)
    }
}
#[doc = "Field `CPHA` reader - SPI Clock Phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - SPI Clock Phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRDBT` reader - Wait Read Data Before Transfer"]
pub type WrdbtR = crate::BitReader;
#[doc = "Field `WRDBT` writer - Wait Read Data Before Transfer"]
pub type WrdbtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> UsartModeR {
        UsartModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> UsclksR {
        UsclksR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&self) -> ChrlR {
        ChrlR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&self) -> WrdbtR {
        WrdbtR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn usart_mode(&mut self) -> UsartModeW<SpiModeMrSpiModeSpec> {
        UsartModeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> UsclksW<SpiModeMrSpiModeSpec> {
        UsclksW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> ChrlW<SpiModeMrSpiModeSpec> {
        ChrlW::new(self, 6)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<SpiModeMrSpiModeSpec> {
        CphaW::new(self, 8)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<SpiModeMrSpiModeSpec> {
        CpolW::new(self, 16)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wrdbt(&mut self) -> WrdbtW<SpiModeMrSpiModeSpec> {
        WrdbtW::new(self, 20)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_mr_spi_mode::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_mr_spi_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiModeMrSpiModeSpec;
impl crate::RegisterSpec for SpiModeMrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mode_mr_spi_mode::R`](R) reader structure"]
impl crate::Readable for SpiModeMrSpiModeSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_mode_mr_spi_mode::W`](W) writer structure"]
impl crate::Writable for SpiModeMrSpiModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
