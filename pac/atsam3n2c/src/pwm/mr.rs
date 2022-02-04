#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: CLKA, CLKB clock is turned off"]
    CLK_OFF = 0,
    #[doc = "1: CLKA, CLKB clock is clock selected by PREA, PREB"]
    CLK_DIV1 = 1,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVA` reader - CLKA, CLKB Divide Factor"]
pub struct DIVA_R(crate::FieldReader<u8, DIVA_A>);
impl DIVA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVA_A> {
        match self.bits {
            0 => Some(DIVA_A::CLK_OFF),
            1 => Some(DIVA_A::CLK_DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OFF`"]
    #[inline(always)]
    pub fn is_clk_off(&self) -> bool {
        **self == DIVA_A::CLK_OFF
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1`"]
    #[inline(always)]
    pub fn is_clk_div1(&self) -> bool {
        **self == DIVA_A::CLK_DIV1
    }
}
impl core::ops::Deref for DIVA_R {
    type Target = crate::FieldReader<u8, DIVA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVA` writer - CLKA, CLKB Divide Factor"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn clk_off(self) -> &'a mut W {
        self.variant(DIVA_A::CLK_OFF)
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn clk_div1(self) -> &'a mut W {
        self.variant(DIVA_A::CLK_DIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREA_A {
    #[doc = "0: Master Clock"]
    MCK = 0,
    #[doc = "1: Master Clock divided by 2"]
    MCKDIV2 = 1,
    #[doc = "2: Master Clock divided by 4"]
    MCKDIV4 = 2,
    #[doc = "3: Master Clock divided by 8"]
    MCKDIV8 = 3,
    #[doc = "4: Master Clock divided by 16"]
    MCKDIV16 = 4,
    #[doc = "5: Master Clock divided by 32"]
    MCKDIV32 = 5,
    #[doc = "6: Master Clock divided by 64"]
    MCKDIV64 = 6,
    #[doc = "7: Master Clock divided by 128"]
    MCKDIV128 = 7,
    #[doc = "8: Master Clock divided by 256"]
    MCKDIV256 = 8,
    #[doc = "9: Master Clock divided by 512"]
    MCKDIV512 = 9,
    #[doc = "10: Master Clock divided by 1024"]
    MCKDIV1024 = 10,
}
impl From<PREA_A> for u8 {
    #[inline(always)]
    fn from(variant: PREA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PREA` reader - "]
pub struct PREA_R(crate::FieldReader<u8, PREA_A>);
impl PREA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PREA_A> {
        match self.bits {
            0 => Some(PREA_A::MCK),
            1 => Some(PREA_A::MCKDIV2),
            2 => Some(PREA_A::MCKDIV4),
            3 => Some(PREA_A::MCKDIV8),
            4 => Some(PREA_A::MCKDIV16),
            5 => Some(PREA_A::MCKDIV32),
            6 => Some(PREA_A::MCKDIV64),
            7 => Some(PREA_A::MCKDIV128),
            8 => Some(PREA_A::MCKDIV256),
            9 => Some(PREA_A::MCKDIV512),
            10 => Some(PREA_A::MCKDIV1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == PREA_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        **self == PREA_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        **self == PREA_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV8`"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        **self == PREA_A::MCKDIV8
    }
    #[doc = "Checks if the value of the field is `MCKDIV16`"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        **self == PREA_A::MCKDIV16
    }
    #[doc = "Checks if the value of the field is `MCKDIV32`"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        **self == PREA_A::MCKDIV32
    }
    #[doc = "Checks if the value of the field is `MCKDIV64`"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        **self == PREA_A::MCKDIV64
    }
    #[doc = "Checks if the value of the field is `MCKDIV128`"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        **self == PREA_A::MCKDIV128
    }
    #[doc = "Checks if the value of the field is `MCKDIV256`"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        **self == PREA_A::MCKDIV256
    }
    #[doc = "Checks if the value of the field is `MCKDIV512`"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        **self == PREA_A::MCKDIV512
    }
    #[doc = "Checks if the value of the field is `MCKDIV1024`"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        **self == PREA_A::MCKDIV1024
    }
}
impl core::ops::Deref for PREA_R {
    type Target = crate::FieldReader<u8, PREA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREA` writer - "]
pub struct PREA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(PREA_A::MCK)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut W {
        self.variant(PREA_A::MCKDIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVB_A {
    #[doc = "0: CLKA, CLKB clock is turned off"]
    CLK_OFF = 0,
    #[doc = "1: CLKA, CLKB clock is clock selected by PREA, PREB"]
    CLK_DIV1 = 1,
}
impl From<DIVB_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVB` reader - CLKA, CLKB Divide Factor"]
pub struct DIVB_R(crate::FieldReader<u8, DIVB_A>);
impl DIVB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVB_A> {
        match self.bits {
            0 => Some(DIVB_A::CLK_OFF),
            1 => Some(DIVB_A::CLK_DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OFF`"]
    #[inline(always)]
    pub fn is_clk_off(&self) -> bool {
        **self == DIVB_A::CLK_OFF
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1`"]
    #[inline(always)]
    pub fn is_clk_div1(&self) -> bool {
        **self == DIVB_A::CLK_DIV1
    }
}
impl core::ops::Deref for DIVB_R {
    type Target = crate::FieldReader<u8, DIVB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVB` writer - CLKA, CLKB Divide Factor"]
pub struct DIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn clk_off(self) -> &'a mut W {
        self.variant(DIVB_A::CLK_OFF)
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn clk_div1(self) -> &'a mut W {
        self.variant(DIVB_A::CLK_DIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREB_A {
    #[doc = "0: Master Clock"]
    MCK = 0,
    #[doc = "1: Master Clock divided by 2"]
    MCKDIV2 = 1,
    #[doc = "2: Master Clock divided by 4"]
    MCKDIV4 = 2,
    #[doc = "3: Master Clock divided by 8"]
    MCKDIV8 = 3,
    #[doc = "4: Master Clock divided by 16"]
    MCKDIV16 = 4,
    #[doc = "5: Master Clock divided by 32"]
    MCKDIV32 = 5,
    #[doc = "6: Master Clock divided by 64"]
    MCKDIV64 = 6,
    #[doc = "7: Master Clock divided by 128"]
    MCKDIV128 = 7,
    #[doc = "8: Master Clock divided by 256"]
    MCKDIV256 = 8,
    #[doc = "9: Master Clock divided by 512"]
    MCKDIV512 = 9,
    #[doc = "10: Master Clock divided by 1024"]
    MCKDIV1024 = 10,
}
impl From<PREB_A> for u8 {
    #[inline(always)]
    fn from(variant: PREB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PREB` reader - "]
pub struct PREB_R(crate::FieldReader<u8, PREB_A>);
impl PREB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PREB_A> {
        match self.bits {
            0 => Some(PREB_A::MCK),
            1 => Some(PREB_A::MCKDIV2),
            2 => Some(PREB_A::MCKDIV4),
            3 => Some(PREB_A::MCKDIV8),
            4 => Some(PREB_A::MCKDIV16),
            5 => Some(PREB_A::MCKDIV32),
            6 => Some(PREB_A::MCKDIV64),
            7 => Some(PREB_A::MCKDIV128),
            8 => Some(PREB_A::MCKDIV256),
            9 => Some(PREB_A::MCKDIV512),
            10 => Some(PREB_A::MCKDIV1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == PREB_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        **self == PREB_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        **self == PREB_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV8`"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        **self == PREB_A::MCKDIV8
    }
    #[doc = "Checks if the value of the field is `MCKDIV16`"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        **self == PREB_A::MCKDIV16
    }
    #[doc = "Checks if the value of the field is `MCKDIV32`"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        **self == PREB_A::MCKDIV32
    }
    #[doc = "Checks if the value of the field is `MCKDIV64`"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        **self == PREB_A::MCKDIV64
    }
    #[doc = "Checks if the value of the field is `MCKDIV128`"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        **self == PREB_A::MCKDIV128
    }
    #[doc = "Checks if the value of the field is `MCKDIV256`"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        **self == PREB_A::MCKDIV256
    }
    #[doc = "Checks if the value of the field is `MCKDIV512`"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        **self == PREB_A::MCKDIV512
    }
    #[doc = "Checks if the value of the field is `MCKDIV1024`"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        **self == PREB_A::MCKDIV1024
    }
}
impl core::ops::Deref for PREB_R {
    type Target = crate::FieldReader<u8, PREB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREB` writer - "]
pub struct PREB_W<'a> {
    w: &'a mut W,
}
impl<'a> PREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(PREB_A::MCK)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut W {
        self.variant(PREB_A::MCKDIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn prea(&self) -> PREA_R {
        PREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn preb(&self) -> PREB_R {
        PREB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn prea(&mut self) -> PREA_W {
        PREA_W { w: self }
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W {
        DIVB_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn preb(&mut self) -> PREB_W {
        PREB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
