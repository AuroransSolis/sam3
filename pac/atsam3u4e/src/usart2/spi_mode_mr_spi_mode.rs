#[doc = "Register `MR_SPI_MODE` reader"]
pub type R = crate::R<SPI_MODE_MR_SPI_MODE_SPEC>;
#[doc = "Register `MR_SPI_MODE` writer"]
pub type W = crate::W<SPI_MODE_MR_SPI_MODE_SPEC>;
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type USART_MODE_R = crate::FieldReader<USART_MODE_A>;
#[doc = "USART Mode of Operation"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART_MODE_A {
    #[doc = "14: SPI Master"]
    SpiMaster = 14,
    #[doc = "15: SPI Slave"]
    SpiSlave = 15,
}
impl From<USART_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: USART_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART_MODE_A {
    type Ux = u8;
}
impl USART_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART_MODE_A> {
        match self.bits {
            14 => Some(USART_MODE_A::SpiMaster),
            15 => Some(USART_MODE_A::SpiSlave),
            _ => None,
        }
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODE_A::SpiMaster
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODE_A::SpiSlave
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type USART_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, USART_MODE_A>;
impl<'a, REG> USART_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::SpiMaster)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::SpiSlave)
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type USCLKS_R = crate::FieldReader<USCLKS_A>;
#[doc = "Clock Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USCLKS_A {
    #[doc = "0: Master Clock MCK is selected"]
    Mck = 0,
    #[doc = "1: Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    Div = 1,
    #[doc = "3: Serial Clock SLK is selected"]
    Sck = 3,
}
impl From<USCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USCLKS_A {
    type Ux = u8;
}
impl USCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USCLKS_A> {
        match self.bits {
            0 => Some(USCLKS_A::Mck),
            1 => Some(USCLKS_A::Div),
            3 => Some(USCLKS_A::Sck),
            _ => None,
        }
    }
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKS_A::Mck
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == USCLKS_A::Div
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKS_A::Sck
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type USCLKS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USCLKS_A>;
impl<'a, REG> USCLKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(USCLKS_A::Mck)
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(USCLKS_A::Div)
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(USCLKS_A::Sck)
    }
}
#[doc = "Field `CHRL` reader - Character Length."]
pub type CHRL_R = crate::FieldReader<CHRL_A>;
#[doc = "Character Length."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHRL_A {
    #[doc = "3: Character length is 8 bits"]
    _8Bit = 3,
}
impl From<CHRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHRL_A {
    type Ux = u8;
}
impl CHRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHRL_A> {
        match self.bits {
            3 => Some(CHRL_A::_8Bit),
            _ => None,
        }
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRL_A::_8Bit
    }
}
#[doc = "Field `CHRL` writer - Character Length."]
pub type CHRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CHRL_A>;
impl<'a, REG> CHRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHRL_A::_8Bit)
    }
}
#[doc = "Field `CPHA` reader - SPI Clock Phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - SPI Clock Phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRDBT` reader - Wait Read Data Before Transfer"]
pub type WRDBT_R = crate::BitReader;
#[doc = "Field `WRDBT` writer - Wait Read Data Before Transfer"]
pub type WRDBT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> USART_MODE_R {
        USART_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&self) -> WRDBT_R {
        WRDBT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn usart_mode(&mut self) -> USART_MODE_W<SPI_MODE_MR_SPI_MODE_SPEC> {
        USART_MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> USCLKS_W<SPI_MODE_MR_SPI_MODE_SPEC> {
        USCLKS_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> CHRL_W<SPI_MODE_MR_SPI_MODE_SPEC> {
        CHRL_W::new(self, 6)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPI_MODE_MR_SPI_MODE_SPEC> {
        CPHA_W::new(self, 8)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPI_MODE_MR_SPI_MODE_SPEC> {
        CPOL_W::new(self, 16)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wrdbt(&mut self) -> WRDBT_W<SPI_MODE_MR_SPI_MODE_SPEC> {
        WRDBT_W::new(self, 20)
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
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_mr_spi_mode::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_mr_spi_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MODE_MR_SPI_MODE_SPEC;
impl crate::RegisterSpec for SPI_MODE_MR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mode_mr_spi_mode::R`](R) reader structure"]
impl crate::Readable for SPI_MODE_MR_SPI_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mode_mr_spi_mode::W`](W) writer structure"]
impl crate::Writable for SPI_MODE_MR_SPI_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
