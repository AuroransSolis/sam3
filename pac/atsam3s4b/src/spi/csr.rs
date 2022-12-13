#[doc = "Register `CSR[%s]` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CSR_SPEC>);
#[doc = "Register `CSR[%s]` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CSR_SPEC>);
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub type NCPHA_R = crate::BitReader<bool>;
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub type NCPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CSNAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CSNAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub type CSAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub type CSAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub type BITS_R = crate::FieldReader<u8, BITS_A>;
#[doc = "Bits Per Transfer"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITS_A {
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
impl From<BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITS_A) -> Self {
        variant as _
    }
}
impl BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITS_A> {
        match self.bits {
            0 => Some(BITS_A::_8Bit),
            1 => Some(BITS_A::_9Bit),
            2 => Some(BITS_A::_10Bit),
            3 => Some(BITS_A::_11Bit),
            4 => Some(BITS_A::_12Bit),
            5 => Some(BITS_A::_13Bit),
            6 => Some(BITS_A::_14Bit),
            7 => Some(BITS_A::_15Bit),
            8 => Some(BITS_A::_16Bit),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8Bit`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == BITS_A::_8Bit
    }
    #[doc = "Checks if the value of the field is `_9Bit`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == BITS_A::_9Bit
    }
    #[doc = "Checks if the value of the field is `_10Bit`"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == BITS_A::_10Bit
    }
    #[doc = "Checks if the value of the field is `_11Bit`"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == BITS_A::_11Bit
    }
    #[doc = "Checks if the value of the field is `_12Bit`"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == BITS_A::_12Bit
    }
    #[doc = "Checks if the value of the field is `_13Bit`"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == BITS_A::_13Bit
    }
    #[doc = "Checks if the value of the field is `_14Bit`"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == BITS_A::_14Bit
    }
    #[doc = "Checks if the value of the field is `_15Bit`"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == BITS_A::_15Bit
    }
    #[doc = "Checks if the value of the field is `_16Bit`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == BITS_A::_16Bit
    }
}
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, BITS_A, 4, O>;
impl<'a, const O: u8> BITS_W<'a, O> {
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(BITS_A::_8Bit)
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(BITS_A::_9Bit)
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(BITS_A::_10Bit)
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(BITS_A::_11Bit)
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(BITS_A::_12Bit)
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(BITS_A::_13Bit)
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(BITS_A::_14Bit)
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(BITS_A::_15Bit)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(BITS_A::_16Bit)
    }
}
#[doc = "Field `SCBR` reader - Serial Clock Baud Rate"]
pub type SCBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBR` writer - Serial Clock Baud Rate"]
pub type SCBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub type DLYBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub type DLYBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NCPHA_R {
        NCPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&self) -> CSNAAT_R {
        CSNAAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CSAAT_R {
        CSAAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> SCBR_R {
        SCBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<0> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ncpha(&mut self) -> NCPHA_W<1> {
        NCPHA_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn csnaat(&mut self) -> CSNAAT_W<2> {
        CSNAAT_W::new(self)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn csaat(&mut self) -> CSAAT_W<3> {
        CSAAT_W::new(self)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BITS_W<4> {
        BITS_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn scbr(&mut self) -> SCBR_W<8> {
        SCBR_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs(&mut self) -> DLYBS_W<16> {
        DLYBS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DLYBCT_W<24> {
        DLYBCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
