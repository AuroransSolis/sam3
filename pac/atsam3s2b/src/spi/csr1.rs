#[doc = "Register `CSR1` reader"]
pub type R = crate::R<Csr1Spec>;
#[doc = "Register `CSR1` writer"]
pub type W = crate::W<Csr1Spec>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub type NcphaR = crate::BitReader;
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub type NcphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CsnaatR = crate::BitReader;
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CsnaatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub type CsaatR = crate::BitReader;
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub type CsaatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bits Per Transfer"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bits {
    #[doc = "0: 8 bits for transfer"]
    _8Bit = 0,
    #[doc = "1: 9 bits for transfer"]
    _9Bit = 1,
    #[doc = "2: 10 bits for transfer"]
    _10Bit = 2,
    #[doc = "3: 11 bits for transfer"]
    _11Bit = 3,
    #[doc = "4: 12 bits for transfer"]
    _12Bit = 4,
    #[doc = "5: 13 bits for transfer"]
    _13Bit = 5,
    #[doc = "6: 14 bits for transfer"]
    _14Bit = 6,
    #[doc = "7: 15 bits for transfer"]
    _15Bit = 7,
    #[doc = "8: 16 bits for transfer"]
    _16Bit = 8,
}
impl From<Bits> for u8 {
    #[inline(always)]
    fn from(variant: Bits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bits {
    type Ux = u8;
}
impl crate::IsEnum for Bits {}
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub type BitsR = crate::FieldReader<Bits>;
impl BitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bits> {
        match self.bits {
            0 => Some(Bits::_8Bit),
            1 => Some(Bits::_9Bit),
            2 => Some(Bits::_10Bit),
            3 => Some(Bits::_11Bit),
            4 => Some(Bits::_12Bit),
            5 => Some(Bits::_13Bit),
            6 => Some(Bits::_14Bit),
            7 => Some(Bits::_15Bit),
            8 => Some(Bits::_16Bit),
            _ => None,
        }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Bits::_8Bit
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Bits::_9Bit
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == Bits::_10Bit
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == Bits::_11Bit
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == Bits::_12Bit
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == Bits::_13Bit
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == Bits::_14Bit
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == Bits::_15Bit
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Bits::_16Bit
    }
}
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bits>;
impl<'a, REG> BitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_8Bit)
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_9Bit)
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_10Bit)
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_11Bit)
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_12Bit)
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_13Bit)
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_14Bit)
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_15Bit)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_16Bit)
    }
}
#[doc = "Field `SCBR` reader - Serial Clock Baud Rate"]
pub type ScbrR = crate::FieldReader;
#[doc = "Field `SCBR` writer - Serial Clock Baud Rate"]
pub type ScbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub type DlybsR = crate::FieldReader;
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub type DlybsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DlybctR = crate::FieldReader;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DlybctW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NcphaR {
        NcphaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&self) -> CsnaatR {
        CsnaatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CsaatR {
        CsaatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> ScbrR {
        ScbrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DlybsR {
        DlybsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DlybctR {
        DlybctR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<Csr1Spec> {
        CpolW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ncpha(&mut self) -> NcphaW<Csr1Spec> {
        NcphaW::new(self, 1)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn csnaat(&mut self) -> CsnaatW<Csr1Spec> {
        CsnaatW::new(self, 2)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn csaat(&mut self) -> CsaatW<Csr1Spec> {
        CsaatW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BitsW<Csr1Spec> {
        BitsW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn scbr(&mut self) -> ScbrW<Csr1Spec> {
        ScbrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs(&mut self) -> DlybsW<Csr1Spec> {
        DlybsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DlybctW<Csr1Spec> {
        DlybctW::new(self, 24)
    }
}
#[doc = "Chip Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr1Spec;
impl crate::RegisterSpec for Csr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for Csr1Spec {}
#[doc = "`write(|w| ..)` method takes [`csr1::W`](W) writer structure"]
impl crate::Writable for Csr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
