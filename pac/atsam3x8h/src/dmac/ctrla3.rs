#[doc = "Register `CTRLA3` reader"]
pub struct R(crate::R<CTRLA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA3` writer"]
pub struct W(crate::W<CTRLA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA3_SPEC>;
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
impl From<crate::W<CTRLA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTSIZE` reader - Buffer Transfer Size"]
pub struct BTSIZE_R(crate::FieldReader<u16, u16>);
impl BTSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BTSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTSIZE` writer - Buffer Transfer Size"]
pub struct BTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Source Chunk Transfer Size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 4 data transferred"]
    CHK_4 = 1,
    #[doc = "2: 8 data transferred"]
    CHK_8 = 2,
    #[doc = "3: 16 data transferred"]
    CHK_16 = 3,
}
impl From<SCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCSIZE` reader - Source Chunk Transfer Size."]
pub struct SCSIZE_R(crate::FieldReader<u8, SCSIZE_A>);
impl SCSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCSIZE_A> {
        match self.bits {
            0 => Some(SCSIZE_A::CHK_1),
            1 => Some(SCSIZE_A::CHK_4),
            2 => Some(SCSIZE_A::CHK_8),
            3 => Some(SCSIZE_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        **self == SCSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        **self == SCSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        **self == SCSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        **self == SCSIZE_A::CHK_16
    }
}
impl core::ops::Deref for SCSIZE_R {
    type Target = crate::FieldReader<u8, SCSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCSIZE` writer - Source Chunk Transfer Size."]
pub struct SCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Destination Chunk Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 4 data transferred"]
    CHK_4 = 1,
    #[doc = "2: 8 data transferred"]
    CHK_8 = 2,
    #[doc = "3: 16 data transferred"]
    CHK_16 = 3,
}
impl From<DCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DCSIZE` reader - Destination Chunk Transfer Size"]
pub struct DCSIZE_R(crate::FieldReader<u8, DCSIZE_A>);
impl DCSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCSIZE_A> {
        match self.bits {
            0 => Some(DCSIZE_A::CHK_1),
            1 => Some(DCSIZE_A::CHK_4),
            2 => Some(DCSIZE_A::CHK_8),
            3 => Some(DCSIZE_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        **self == DCSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        **self == DCSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        **self == DCSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        **self == DCSIZE_A::CHK_16
    }
}
impl core::ops::Deref for DCSIZE_R {
    type Target = crate::FieldReader<u8, DCSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCSIZE` writer - Destination Chunk Transfer Size"]
pub struct DCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Transfer Width for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_WIDTH_A {
    #[doc = "0: the transfer size is set to 8-bit width"]
    BYTE = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HALF_WORD = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    WORD = 2,
}
impl From<SRC_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC_WIDTH` reader - Transfer Width for the Source"]
pub struct SRC_WIDTH_R(crate::FieldReader<u8, SRC_WIDTH_A>);
impl SRC_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_WIDTH_A> {
        match self.bits {
            0 => Some(SRC_WIDTH_A::BYTE),
            1 => Some(SRC_WIDTH_A::HALF_WORD),
            2 => Some(SRC_WIDTH_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == SRC_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        **self == SRC_WIDTH_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == SRC_WIDTH_A::WORD
    }
}
impl core::ops::Deref for SRC_WIDTH_R {
    type Target = crate::FieldReader<u8, SRC_WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_WIDTH` writer - Transfer Width for the Source"]
pub struct SRC_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SRC_WIDTH_A::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SRC_WIDTH_A::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SRC_WIDTH_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Transfer Width for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DST_WIDTH_A {
    #[doc = "0: the transfer size is set to 8-bit width"]
    BYTE = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HALF_WORD = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    WORD = 2,
}
impl From<DST_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DST_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DST_WIDTH` reader - Transfer Width for the Destination"]
pub struct DST_WIDTH_R(crate::FieldReader<u8, DST_WIDTH_A>);
impl DST_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DST_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DST_WIDTH_A> {
        match self.bits {
            0 => Some(DST_WIDTH_A::BYTE),
            1 => Some(DST_WIDTH_A::HALF_WORD),
            2 => Some(DST_WIDTH_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == DST_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        **self == DST_WIDTH_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == DST_WIDTH_A::WORD
    }
}
impl core::ops::Deref for DST_WIDTH_R {
    type Target = crate::FieldReader<u8, DST_WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_WIDTH` writer - Transfer Width for the Destination"]
pub struct DST_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DST_WIDTH_A::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DST_WIDTH_A::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DST_WIDTH_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `DONE` reader - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&self) -> BTSIZE_R {
        BTSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&self) -> SCSIZE_R {
        SCSIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&self) -> DCSIZE_R {
        DCSIZE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&self) -> SRC_WIDTH_R {
        SRC_WIDTH_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&self) -> DST_WIDTH_R {
        DST_WIDTH_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&mut self) -> BTSIZE_W {
        BTSIZE_W { w: self }
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&mut self) -> SCSIZE_W {
        SCSIZE_W { w: self }
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&mut self) -> DCSIZE_W {
        DCSIZE_W { w: self }
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&mut self) -> SRC_WIDTH_W {
        SRC_WIDTH_W { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&mut self) -> DST_WIDTH_W {
        DST_WIDTH_W { w: self }
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Control A Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla3](index.html) module"]
pub struct CTRLA3_SPEC;
impl crate::RegisterSpec for CTRLA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla3::R](R) reader structure"]
impl crate::Readable for CTRLA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla3::W](W) writer structure"]
impl crate::Writable for CTRLA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA3 to value 0"]
impl crate::Resettable for CTRLA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
