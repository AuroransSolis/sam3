#[doc = "Register `MR_SPI_MODE` reader"]
pub struct R(crate::R<SPI_MODE_MR_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MODE_MR_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MODE_MR_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MODE_MR_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR_SPI_MODE` writer"]
pub struct W(crate::W<SPI_MODE_MR_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MODE_MR_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MODE_MR_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MODE_MR_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type USART_MODE_R = crate::FieldReader<u8, USART_MODE_A>;
#[doc = "USART Mode of Operation"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART_MODE_A {
    #[doc = "14: SPI Master"]
    SPI_MASTER = 14,
    #[doc = "15: SPI Slave"]
    SPI_SLAVE = 15,
}
impl From<USART_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: USART_MODE_A) -> Self {
        variant as _
    }
}
impl USART_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART_MODE_A> {
        match self.bits {
            14 => Some(USART_MODE_A::SPI_MASTER),
            15 => Some(USART_MODE_A::SPI_SLAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODE_A::SPI_SLAVE
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type USART_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_MODE_SPEC, u8, USART_MODE_A, 4, O>;
impl<'a, const O: u8> USART_MODE_W<'a, O> {
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODE_A::SPI_MASTER)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODE_A::SPI_SLAVE)
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type USCLKS_R = crate::FieldReader<u8, USCLKS_A>;
#[doc = "Clock Selection"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USCLKS_A {
    #[doc = "0: Master Clock MCK is selected"]
    MCK = 0,
    #[doc = "1: Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    DIV = 1,
    #[doc = "3: Serial Clock SLK is selected"]
    SCK = 3,
}
impl From<USCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKS_A) -> Self {
        variant as _
    }
}
impl USCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USCLKS_A> {
        match self.bits {
            0 => Some(USCLKS_A::MCK),
            1 => Some(USCLKS_A::DIV),
            3 => Some(USCLKS_A::SCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == USCLKS_A::DIV
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKS_A::SCK
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type USCLKS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_MODE_SPEC, u8, USCLKS_A, 2, O>;
impl<'a, const O: u8> USCLKS_W<'a, O> {
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKS_A::MCK)
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKS_A::DIV)
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKS_A::SCK)
    }
}
#[doc = "Field `CHRL` reader - Character Length."]
pub type CHRL_R = crate::FieldReader<u8, CHRL_A>;
#[doc = "Character Length."]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHRL_A {
    #[doc = "3: Character length is 8 bits"]
    _8_BIT = 3,
}
impl From<CHRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRL_A) -> Self {
        variant as _
    }
}
impl CHRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHRL_A> {
        match self.bits {
            3 => Some(CHRL_A::_8_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRL_A::_8_BIT
    }
}
#[doc = "Field `CHRL` writer - Character Length."]
pub type CHRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_MODE_SPEC, u8, CHRL_A, 2, O>;
impl<'a, const O: u8> CHRL_W<'a, O> {
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_8_BIT)
    }
}
#[doc = "Field `CPHA` reader - SPI Clock Phase"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - SPI Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `WRDBT` reader - Wait Read Data Before Transfer"]
pub type WRDBT_R = crate::BitReader<bool>;
#[doc = "Field `WRDBT` writer - Wait Read Data Before Transfer"]
pub type WRDBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_MODE_SPEC, bool, O>;
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
    pub fn usart_mode(&mut self) -> USART_MODE_W<0> {
        USART_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&mut self) -> USCLKS_W<4> {
        USCLKS_W::new(self)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&mut self) -> CHRL_W<6> {
        CHRL_W::new(self)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<8> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<16> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&mut self) -> WRDBT_W<20> {
        WRDBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mode_mr_spi_mode](index.html) module"]
pub struct SPI_MODE_MR_SPI_MODE_SPEC;
impl crate::RegisterSpec for SPI_MODE_MR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mode_mr_spi_mode::R](R) reader structure"]
impl crate::Readable for SPI_MODE_MR_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mode_mr_spi_mode::W](W) writer structure"]
impl crate::Writable for SPI_MODE_MR_SPI_MODE_SPEC {
    type Writer = W;
}
