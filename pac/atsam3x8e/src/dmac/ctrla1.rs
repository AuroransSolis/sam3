#[doc = "Register `CTRLA1` reader"]
pub struct R(crate::R<CTRLA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA1` writer"]
pub struct W(crate::W<CTRLA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA1_SPEC>;
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
impl From<crate::W<CTRLA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTSIZE` reader - Buffer Transfer Size"]
pub type BTSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BTSIZE` writer - Buffer Transfer Size"]
pub type BTSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA1_SPEC, u16, u16, 16, O>;
#[doc = "Field `SCSIZE` reader - Source Chunk Transfer Size."]
pub type SCSIZE_R = crate::FieldReader<u8, SCSIZE_A>;
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
impl SCSIZE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SCSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == SCSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == SCSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == SCSIZE_A::CHK_16
    }
}
#[doc = "Field `SCSIZE` writer - Source Chunk Transfer Size."]
pub type SCSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA1_SPEC, u8, SCSIZE_A, 3, O>;
impl<'a, const O: u8> SCSIZE_W<'a, O> {
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
}
#[doc = "Field `DCSIZE` reader - Destination Chunk Transfer Size"]
pub type DCSIZE_R = crate::FieldReader<u8, DCSIZE_A>;
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
impl DCSIZE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DCSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == DCSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == DCSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == DCSIZE_A::CHK_16
    }
}
#[doc = "Field `DCSIZE` writer - Destination Chunk Transfer Size"]
pub type DCSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA1_SPEC, u8, DCSIZE_A, 3, O>;
impl<'a, const O: u8> DCSIZE_W<'a, O> {
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
}
#[doc = "Field `SRC_WIDTH` reader - Transfer Width for the Source"]
pub type SRC_WIDTH_R = crate::FieldReader<u8, SRC_WIDTH_A>;
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
impl SRC_WIDTH_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SRC_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRC_WIDTH_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRC_WIDTH_A::WORD
    }
}
#[doc = "Field `SRC_WIDTH` writer - Transfer Width for the Source"]
pub type SRC_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLA1_SPEC, u8, SRC_WIDTH_A, 2, O>;
impl<'a, const O: u8> SRC_WIDTH_W<'a, O> {
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
}
#[doc = "Field `DST_WIDTH` reader - Transfer Width for the Destination"]
pub type DST_WIDTH_R = crate::FieldReader<u8, DST_WIDTH_A>;
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
impl DST_WIDTH_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DST_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DST_WIDTH_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DST_WIDTH_A::WORD
    }
}
#[doc = "Field `DST_WIDTH` writer - Transfer Width for the Destination"]
pub type DST_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLA1_SPEC, u8, DST_WIDTH_A, 2, O>;
impl<'a, const O: u8> DST_WIDTH_W<'a, O> {
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
}
#[doc = "Field `DONE` reader - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&self) -> BTSIZE_R {
        BTSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&self) -> SCSIZE_R {
        SCSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&self) -> DCSIZE_R {
        DCSIZE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&self) -> SRC_WIDTH_R {
        SRC_WIDTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&self) -> DST_WIDTH_R {
        DST_WIDTH_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&mut self) -> BTSIZE_W<0> {
        BTSIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&mut self) -> SCSIZE_W<16> {
        SCSIZE_W::new(self)
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&mut self) -> DCSIZE_W<20> {
        DCSIZE_W::new(self)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&mut self) -> SRC_WIDTH_W<24> {
        SRC_WIDTH_W::new(self)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&mut self) -> DST_WIDTH_W<28> {
        DST_WIDTH_W::new(self)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<31> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Control A Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla1](index.html) module"]
pub struct CTRLA1_SPEC;
impl crate::RegisterSpec for CTRLA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla1::R](R) reader structure"]
impl crate::Readable for CTRLA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla1::W](W) writer structure"]
impl crate::Writable for CTRLA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA1 to value 0"]
impl crate::Resettable for CTRLA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
