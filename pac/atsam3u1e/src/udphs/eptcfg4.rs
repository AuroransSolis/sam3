#[doc = "Register `EPTCFG4` reader"]
pub struct R(crate::R<EPTCFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTCFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTCFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTCFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPTCFG4` writer"]
pub struct W(crate::W<EPTCFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTCFG4_SPEC>;
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
impl From<crate::W<EPTCFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTCFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPT_SIZE` reader - Endpoint Size"]
pub type EPT_SIZE_R = crate::FieldReader<u8, EPT_SIZE_A>;
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPT_SIZE_A {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
}
impl From<EPT_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPT_SIZE_A) -> Self {
        variant as _
    }
}
impl EPT_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPT_SIZE_A {
        match self.bits {
            0 => EPT_SIZE_A::_8,
            1 => EPT_SIZE_A::_16,
            2 => EPT_SIZE_A::_32,
            3 => EPT_SIZE_A::_64,
            4 => EPT_SIZE_A::_128,
            5 => EPT_SIZE_A::_256,
            6 => EPT_SIZE_A::_512,
            7 => EPT_SIZE_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == EPT_SIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == EPT_SIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == EPT_SIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == EPT_SIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == EPT_SIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == EPT_SIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == EPT_SIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == EPT_SIZE_A::_1024
    }
}
#[doc = "Field `EPT_SIZE` writer - Endpoint Size"]
pub type EPT_SIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EPTCFG4_SPEC, u8, EPT_SIZE_A, 3, O>;
impl<'a, const O: u8> EPT_SIZE_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_8)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_16)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_32)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_512)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(EPT_SIZE_A::_1024)
    }
}
#[doc = "Field `EPT_DIR` reader - Endpoint Direction"]
pub type EPT_DIR_R = crate::BitReader<bool>;
#[doc = "Field `EPT_DIR` writer - Endpoint Direction"]
pub type EPT_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCFG4_SPEC, bool, O>;
#[doc = "Field `EPT_TYPE` reader - Endpoint Type"]
pub type EPT_TYPE_R = crate::FieldReader<u8, EPT_TYPE_A>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPT_TYPE_A {
    #[doc = "0: Control endpoint"]
    CTRL8 = 0,
    #[doc = "1: Isochronous endpoint"]
    ISO = 1,
    #[doc = "2: Bulk endpoint"]
    BULK = 2,
    #[doc = "3: Interrupt endpoint"]
    INT = 3,
}
impl From<EPT_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPT_TYPE_A) -> Self {
        variant as _
    }
}
impl EPT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPT_TYPE_A {
        match self.bits {
            0 => EPT_TYPE_A::CTRL8,
            1 => EPT_TYPE_A::ISO,
            2 => EPT_TYPE_A::BULK,
            3 => EPT_TYPE_A::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL8`"]
    #[inline(always)]
    pub fn is_ctrl8(&self) -> bool {
        *self == EPT_TYPE_A::CTRL8
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EPT_TYPE_A::ISO
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EPT_TYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == EPT_TYPE_A::INT
    }
}
#[doc = "Field `EPT_TYPE` writer - Endpoint Type"]
pub type EPT_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EPTCFG4_SPEC, u8, EPT_TYPE_A, 2, O>;
impl<'a, const O: u8> EPT_TYPE_W<'a, O> {
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn ctrl8(self) -> &'a mut W {
        self.variant(EPT_TYPE_A::CTRL8)
    }
    #[doc = "Isochronous endpoint"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPT_TYPE_A::ISO)
    }
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPT_TYPE_A::BULK)
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(EPT_TYPE_A::INT)
    }
}
#[doc = "Field `BK_NUMBER` reader - Number of Banks"]
pub type BK_NUMBER_R = crate::FieldReader<u8, BK_NUMBER_A>;
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BK_NUMBER_A {
    #[doc = "0: Zero bank, the endpoint is not mapped in memory"]
    _0 = 0,
    #[doc = "1: One bank (bank 0)"]
    _1 = 1,
    #[doc = "2: Double bank (Ping-Pong: bank0/bank1)"]
    _2 = 2,
    #[doc = "3: Triple bank (bank0/bank1/bank2)"]
    _3 = 3,
}
impl From<BK_NUMBER_A> for u8 {
    #[inline(always)]
    fn from(variant: BK_NUMBER_A) -> Self {
        variant as _
    }
}
impl BK_NUMBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK_NUMBER_A {
        match self.bits {
            0 => BK_NUMBER_A::_0,
            1 => BK_NUMBER_A::_1,
            2 => BK_NUMBER_A::_2,
            3 => BK_NUMBER_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BK_NUMBER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BK_NUMBER_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == BK_NUMBER_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == BK_NUMBER_A::_3
    }
}
#[doc = "Field `BK_NUMBER` writer - Number of Banks"]
pub type BK_NUMBER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EPTCFG4_SPEC, u8, BK_NUMBER_A, 2, O>;
impl<'a, const O: u8> BK_NUMBER_W<'a, O> {
    #[doc = "Zero bank, the endpoint is not mapped in memory"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BK_NUMBER_A::_0)
    }
    #[doc = "One bank (bank 0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BK_NUMBER_A::_1)
    }
    #[doc = "Double bank (Ping-Pong: bank0/bank1)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BK_NUMBER_A::_2)
    }
    #[doc = "Triple bank (bank0/bank1/bank2)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BK_NUMBER_A::_3)
    }
}
#[doc = "Field `NB_TRANS` reader - Number Of Transaction per Microframe"]
pub type NB_TRANS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NB_TRANS` writer - Number Of Transaction per Microframe"]
pub type NB_TRANS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPTCFG4_SPEC, u8, u8, 2, O>;
#[doc = "Field `EPT_MAPD` reader - Endpoint Mapped"]
pub type EPT_MAPD_R = crate::BitReader<bool>;
#[doc = "Field `EPT_MAPD` writer - Endpoint Mapped"]
pub type EPT_MAPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCFG4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Endpoint Size"]
    #[inline(always)]
    pub fn ept_size(&self) -> EPT_SIZE_R {
        EPT_SIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Endpoint Direction"]
    #[inline(always)]
    pub fn ept_dir(&self) -> EPT_DIR_R {
        EPT_DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Endpoint Type"]
    #[inline(always)]
    pub fn ept_type(&self) -> EPT_TYPE_R {
        EPT_TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Number of Banks"]
    #[inline(always)]
    pub fn bk_number(&self) -> BK_NUMBER_R {
        BK_NUMBER_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Number Of Transaction per Microframe"]
    #[inline(always)]
    pub fn nb_trans(&self) -> NB_TRANS_R {
        NB_TRANS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 31 - Endpoint Mapped"]
    #[inline(always)]
    pub fn ept_mapd(&self) -> EPT_MAPD_R {
        EPT_MAPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Endpoint Size"]
    #[inline(always)]
    pub fn ept_size(&mut self) -> EPT_SIZE_W<0> {
        EPT_SIZE_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint Direction"]
    #[inline(always)]
    pub fn ept_dir(&mut self) -> EPT_DIR_W<3> {
        EPT_DIR_W::new(self)
    }
    #[doc = "Bits 4:5 - Endpoint Type"]
    #[inline(always)]
    pub fn ept_type(&mut self) -> EPT_TYPE_W<4> {
        EPT_TYPE_W::new(self)
    }
    #[doc = "Bits 6:7 - Number of Banks"]
    #[inline(always)]
    pub fn bk_number(&mut self) -> BK_NUMBER_W<6> {
        BK_NUMBER_W::new(self)
    }
    #[doc = "Bits 8:9 - Number Of Transaction per Microframe"]
    #[inline(always)]
    pub fn nb_trans(&mut self) -> NB_TRANS_W<8> {
        NB_TRANS_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint Mapped"]
    #[inline(always)]
    pub fn ept_mapd(&mut self) -> EPT_MAPD_W<31> {
        EPT_MAPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptcfg4](index.html) module"]
pub struct EPTCFG4_SPEC;
impl crate::RegisterSpec for EPTCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptcfg4::R](R) reader structure"]
impl crate::Readable for EPTCFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eptcfg4::W](W) writer structure"]
impl crate::Writable for EPTCFG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPTCFG4 to value 0"]
impl crate::Resettable for EPTCFG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
