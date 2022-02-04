#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Page Size of the NAND Flash Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAGESIZE_A {
    #[doc = "0: Main area 512 Bytes"]
    PS512 = 0,
    #[doc = "1: Main area 1024 Bytes"]
    PS1024 = 1,
    #[doc = "2: Main area 2048 Bytes"]
    PS2048 = 2,
    #[doc = "3: Main area 4096 Bytes"]
    PS4096 = 3,
}
impl From<PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGESIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAGESIZE` reader - Page Size of the NAND Flash Device"]
pub struct PAGESIZE_R(crate::FieldReader<u8, PAGESIZE_A>);
impl PAGESIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAGESIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGESIZE_A {
        match self.bits {
            0 => PAGESIZE_A::PS512,
            1 => PAGESIZE_A::PS1024,
            2 => PAGESIZE_A::PS2048,
            3 => PAGESIZE_A::PS4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512`"]
    #[inline(always)]
    pub fn is_ps512(&self) -> bool {
        **self == PAGESIZE_A::PS512
    }
    #[doc = "Checks if the value of the field is `PS1024`"]
    #[inline(always)]
    pub fn is_ps1024(&self) -> bool {
        **self == PAGESIZE_A::PS1024
    }
    #[doc = "Checks if the value of the field is `PS2048`"]
    #[inline(always)]
    pub fn is_ps2048(&self) -> bool {
        **self == PAGESIZE_A::PS2048
    }
    #[doc = "Checks if the value of the field is `PS4096`"]
    #[inline(always)]
    pub fn is_ps4096(&self) -> bool {
        **self == PAGESIZE_A::PS4096
    }
}
impl core::ops::Deref for PAGESIZE_R {
    type Target = crate::FieldReader<u8, PAGESIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGESIZE` writer - Page Size of the NAND Flash Device"]
pub struct PAGESIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGESIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGESIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Main area 512 Bytes"]
    #[inline(always)]
    pub fn ps512(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS512)
    }
    #[doc = "Main area 1024 Bytes"]
    #[inline(always)]
    pub fn ps1024(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS1024)
    }
    #[doc = "Main area 2048 Bytes"]
    #[inline(always)]
    pub fn ps2048(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS2048)
    }
    #[doc = "Main area 4096 Bytes"]
    #[inline(always)]
    pub fn ps4096(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `WSPARE` reader - Write Spare Area"]
pub struct WSPARE_R(crate::FieldReader<bool, bool>);
impl WSPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WSPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WSPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WSPARE` writer - Write Spare Area"]
pub struct WSPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> WSPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RSPARE` reader - Read Spare Area"]
pub struct RSPARE_R(crate::FieldReader<bool, bool>);
impl RSPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSPARE` writer - Read Spare Area"]
pub struct RSPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `EDGECTRL` reader - Rising/Falling Edge Detection Control"]
pub struct EDGECTRL_R(crate::FieldReader<bool, bool>);
impl EDGECTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGECTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGECTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGECTRL` writer - Rising/Falling Edge Detection Control"]
pub struct EDGECTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGECTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RBEDGE` reader - Ready/Busy Signal Edge Detection"]
pub struct RBEDGE_R(crate::FieldReader<bool, bool>);
impl RBEDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBEDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBEDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBEDGE` writer - Ready/Busy Signal Edge Detection"]
pub struct RBEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DTOCYC` reader - Data Timeout Cycle Number"]
pub struct DTOCYC_R(crate::FieldReader<u8, u8>);
impl DTOCYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTOCYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOCYC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOCYC` writer - Data Timeout Cycle Number"]
pub struct DTOCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTOMUL_A {
    #[doc = "0: DTOCYC"]
    X1 = 0,
    #[doc = "1: DTOCYC x 16"]
    X16 = 1,
    #[doc = "2: DTOCYC x 128"]
    X128 = 2,
    #[doc = "3: DTOCYC x 256"]
    X256 = 3,
    #[doc = "4: DTOCYC x 1024"]
    X1024 = 4,
    #[doc = "5: DTOCYC x 4096"]
    X4096 = 5,
    #[doc = "6: DTOCYC x 65536"]
    X65536 = 6,
    #[doc = "7: DTOCYC x 1048576"]
    X1048576 = 7,
}
impl From<DTOMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOMUL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTOMUL` reader - Data Timeout Multiplier"]
pub struct DTOMUL_R(crate::FieldReader<u8, DTOMUL_A>);
impl DTOMUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTOMUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOMUL_A {
        match self.bits {
            0 => DTOMUL_A::X1,
            1 => DTOMUL_A::X16,
            2 => DTOMUL_A::X128,
            3 => DTOMUL_A::X256,
            4 => DTOMUL_A::X1024,
            5 => DTOMUL_A::X4096,
            6 => DTOMUL_A::X65536,
            7 => DTOMUL_A::X1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        **self == DTOMUL_A::X1
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        **self == DTOMUL_A::X16
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        **self == DTOMUL_A::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        **self == DTOMUL_A::X256
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        **self == DTOMUL_A::X1024
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        **self == DTOMUL_A::X4096
    }
    #[doc = "Checks if the value of the field is `X65536`"]
    #[inline(always)]
    pub fn is_x65536(&self) -> bool {
        **self == DTOMUL_A::X65536
    }
    #[doc = "Checks if the value of the field is `X1048576`"]
    #[inline(always)]
    pub fn is_x1048576(&self) -> bool {
        **self == DTOMUL_A::X1048576
    }
}
impl core::ops::Deref for DTOMUL_R {
    type Target = crate::FieldReader<u8, DTOMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOMUL` writer - Data Timeout Multiplier"]
pub struct DTOMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOMUL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(DTOMUL_A::X16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut W {
        self.variant(DTOMUL_A::X128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut W {
        self.variant(DTOMUL_A::X256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut W {
        self.variant(DTOMUL_A::X4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn x65536(self) -> &'a mut W {
        self.variant(DTOMUL_A::X65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn x1048576(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Page Size of the NAND Flash Device"]
    #[inline(always)]
    pub fn pagesize(&self) -> PAGESIZE_R {
        PAGESIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&self) -> WSPARE_R {
        WSPARE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&self) -> RSPARE_R {
        RSPARE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&self) -> EDGECTRL_R {
        EDGECTRL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&self) -> RBEDGE_R {
        RBEDGE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DTOCYC_R {
        DTOCYC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DTOMUL_R {
        DTOMUL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Page Size of the NAND Flash Device"]
    #[inline(always)]
    pub fn pagesize(&mut self) -> PAGESIZE_W {
        PAGESIZE_W { w: self }
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&mut self) -> WSPARE_W {
        WSPARE_W { w: self }
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&mut self) -> RSPARE_W {
        RSPARE_W { w: self }
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&mut self) -> EDGECTRL_W {
        EDGECTRL_W { w: self }
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&mut self) -> RBEDGE_W {
        RBEDGE_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&mut self) -> DTOCYC_W {
        DTOCYC_W { w: self }
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&mut self) -> DTOMUL_W {
        DTOMUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC NFC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
