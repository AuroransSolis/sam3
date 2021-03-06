#[doc = "Register `HSTPIPCFG0_HSBOHSCP` reader"]
pub struct R(crate::R<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTPIPCFG0_HSBOHSCP` writer"]
pub struct W(crate::W<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>;
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
impl From<crate::W<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOC` reader - Pipe Memory Allocate"]
pub struct ALLOC_R(crate::FieldReader<bool, bool>);
impl ALLOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALLOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOC` writer - Pipe Memory Allocate"]
pub struct ALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Pipe Banks"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PBK_A {
    #[doc = "0: Single-bank pipe"]
    _1_BANK = 0,
    #[doc = "1: Double-bank pipe"]
    _2_BANK = 1,
    #[doc = "2: Triple-bank pipe"]
    _3_BANK = 2,
}
impl From<PBK_A> for u8 {
    #[inline(always)]
    fn from(variant: PBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PBK` reader - Pipe Banks"]
pub struct PBK_R(crate::FieldReader<u8, PBK_A>);
impl PBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PBK_A> {
        match self.bits {
            0 => Some(PBK_A::_1_BANK),
            1 => Some(PBK_A::_2_BANK),
            2 => Some(PBK_A::_3_BANK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        **self == PBK_A::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        **self == PBK_A::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        **self == PBK_A::_3_BANK
    }
}
impl core::ops::Deref for PBK_R {
    type Target = crate::FieldReader<u8, PBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub struct PBK_W<'a> {
    w: &'a mut W,
}
impl<'a> PBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(PBK_A::_1_BANK)
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(PBK_A::_2_BANK)
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(PBK_A::_3_BANK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Pipe Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: 8 bytes"]
    _8_BYTE = 0,
    #[doc = "1: 16 bytes"]
    _16_BYTE = 1,
    #[doc = "2: 32 bytes"]
    _32_BYTE = 2,
    #[doc = "3: 64 bytes"]
    _64_BYTE = 3,
    #[doc = "4: 128 bytes"]
    _128_BYTE = 4,
    #[doc = "5: 256 bytes"]
    _256_BYTE = 5,
    #[doc = "6: 512 bytes"]
    _512_BYTE = 6,
    #[doc = "7: 1024 bytes"]
    _1024_BYTE = 7,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub struct PSIZE_R(crate::FieldReader<u8, PSIZE_A>);
impl PSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::_8_BYTE,
            1 => PSIZE_A::_16_BYTE,
            2 => PSIZE_A::_32_BYTE,
            3 => PSIZE_A::_64_BYTE,
            4 => PSIZE_A::_128_BYTE,
            5 => PSIZE_A::_256_BYTE,
            6 => PSIZE_A::_512_BYTE,
            7 => PSIZE_A::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == PSIZE_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == PSIZE_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == PSIZE_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == PSIZE_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        **self == PSIZE_A::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        **self == PSIZE_A::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        **self == PSIZE_A::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        **self == PSIZE_A::_1024_BYTE
    }
}
impl core::ops::Deref for PSIZE_R {
    type Target = crate::FieldReader<u8, PSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_1024_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Pipe Token"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTOKEN_A {
    #[doc = "0: SETUP"]
    SETUP = 0,
    #[doc = "1: IN"]
    IN = 1,
    #[doc = "2: OUT"]
    OUT = 2,
}
impl From<PTOKEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOKEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub struct PTOKEN_R(crate::FieldReader<u8, PTOKEN_A>);
impl PTOKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PTOKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTOKEN_A> {
        match self.bits {
            0 => Some(PTOKEN_A::SETUP),
            1 => Some(PTOKEN_A::IN),
            2 => Some(PTOKEN_A::OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        **self == PTOKEN_A::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == PTOKEN_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == PTOKEN_A::OUT
    }
}
impl core::ops::Deref for PTOKEN_R {
    type Target = crate::FieldReader<u8, PTOKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub struct PTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTOKEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKEN_A::SETUP)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKEN_A::IN)
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKEN_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub struct AUTOSW_R(crate::FieldReader<bool, bool>);
impl AUTOSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub struct AUTOSW_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Pipe Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "2: Bulk"]
    BLK = 2,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub struct PTYPE_R(crate::FieldReader<u8, PTYPE_A>);
impl PTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTYPE_A> {
        match self.bits {
            0 => Some(PTYPE_A::CTRL),
            2 => Some(PTYPE_A::BLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        **self == PTYPE_A::CTRL
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        **self == PTYPE_A::BLK
    }
}
impl core::ops::Deref for PTYPE_R {
    type Target = crate::FieldReader<u8, PTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(PTYPE_A::CTRL)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut W {
        self.variant(PTYPE_A::BLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `PEPNUM` reader - Pipe Endpoint Number"]
pub struct PEPNUM_R(crate::FieldReader<u8, u8>);
impl PEPNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PEPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEPNUM` writer - Pipe Endpoint Number"]
pub struct PEPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PINGEN` reader - Ping Enable"]
pub struct PINGEN_R(crate::FieldReader<bool, bool>);
impl PINGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PINGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINGEN` writer - Ping Enable"]
pub struct PINGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINGEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `BINTERVAL` reader - Binterval Parameter for the Bulk-Out/Ping Transaction"]
pub struct BINTERVAL_R(crate::FieldReader<u8, u8>);
impl BINTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BINTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BINTERVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BINTERVAL` writer - Binterval Parameter for the Bulk-Out/Ping Transaction"]
pub struct BINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PBK_R {
        PBK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&self) -> PEPNUM_R {
        PEPNUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&self) -> PINGEN_R {
        PINGEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Binterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&self) -> BINTERVAL_R {
        BINTERVAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&mut self) -> ALLOC_W {
        ALLOC_W { w: self }
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&mut self) -> PBK_W {
        PBK_W { w: self }
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PTOKEN_W {
        PTOKEN_W { w: self }
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&mut self) -> AUTOSW_W {
        AUTOSW_W { w: self }
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&mut self) -> PEPNUM_W {
        PEPNUM_W { w: self }
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&mut self) -> PINGEN_W {
        PINGEN_W { w: self }
    }
    #[doc = "Bits 24:31 - Binterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&mut self) -> BINTERVAL_W {
        BINTERVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Configuration Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsbohscp_hstpipcfg0_hsbohscp](index.html) module"]
pub struct HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC;
impl crate::RegisterSpec for HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsbohscp_hstpipcfg0_hsbohscp::R](R) reader structure"]
impl crate::Readable for HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsbohscp_hstpipcfg0_hsbohscp::W](W) writer structure"]
impl crate::Writable for HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC {
    type Writer = W;
}
