#[doc = "Register `SMMR` reader"]
pub struct R(crate::R<SMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMMR` writer"]
pub struct W(crate::W<SMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMMR_SPEC>;
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
impl From<crate::W<SMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Supply Monitor Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMTH_A {
    #[doc = "0: 1.9 V"]
    _1_9V = 0,
    #[doc = "1: 2.0 V"]
    _2_0V = 1,
    #[doc = "2: 2.1 V"]
    _2_1V = 2,
    #[doc = "3: 2.2 V"]
    _2_2V = 3,
    #[doc = "4: 2.3 V"]
    _2_3V = 4,
    #[doc = "5: 2.4 V"]
    _2_4V = 5,
    #[doc = "6: 2.5 V"]
    _2_5V = 6,
    #[doc = "7: 2.6 V"]
    _2_6V = 7,
    #[doc = "8: 2.7 V"]
    _2_7V = 8,
    #[doc = "9: 2.8 V"]
    _2_8V = 9,
    #[doc = "10: 2.9 V"]
    _2_9V = 10,
    #[doc = "11: 3.0 V"]
    _3_0V = 11,
    #[doc = "12: 3.1 V"]
    _3_1V = 12,
    #[doc = "13: 3.2 V"]
    _3_2V = 13,
    #[doc = "14: 3.3 V"]
    _3_3V = 14,
    #[doc = "15: 3.4 V"]
    _3_4V = 15,
}
impl From<SMTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SMTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMTH` reader - Supply Monitor Threshold"]
pub struct SMTH_R(crate::FieldReader<u8, SMTH_A>);
impl SMTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTH_A {
        match self.bits {
            0 => SMTH_A::_1_9V,
            1 => SMTH_A::_2_0V,
            2 => SMTH_A::_2_1V,
            3 => SMTH_A::_2_2V,
            4 => SMTH_A::_2_3V,
            5 => SMTH_A::_2_4V,
            6 => SMTH_A::_2_5V,
            7 => SMTH_A::_2_6V,
            8 => SMTH_A::_2_7V,
            9 => SMTH_A::_2_8V,
            10 => SMTH_A::_2_9V,
            11 => SMTH_A::_3_0V,
            12 => SMTH_A::_3_1V,
            13 => SMTH_A::_3_2V,
            14 => SMTH_A::_3_3V,
            15 => SMTH_A::_3_4V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_9V`"]
    #[inline(always)]
    pub fn is_1_9v(&self) -> bool {
        **self == SMTH_A::_1_9V
    }
    #[doc = "Checks if the value of the field is `_2_0V`"]
    #[inline(always)]
    pub fn is_2_0v(&self) -> bool {
        **self == SMTH_A::_2_0V
    }
    #[doc = "Checks if the value of the field is `_2_1V`"]
    #[inline(always)]
    pub fn is_2_1v(&self) -> bool {
        **self == SMTH_A::_2_1V
    }
    #[doc = "Checks if the value of the field is `_2_2V`"]
    #[inline(always)]
    pub fn is_2_2v(&self) -> bool {
        **self == SMTH_A::_2_2V
    }
    #[doc = "Checks if the value of the field is `_2_3V`"]
    #[inline(always)]
    pub fn is_2_3v(&self) -> bool {
        **self == SMTH_A::_2_3V
    }
    #[doc = "Checks if the value of the field is `_2_4V`"]
    #[inline(always)]
    pub fn is_2_4v(&self) -> bool {
        **self == SMTH_A::_2_4V
    }
    #[doc = "Checks if the value of the field is `_2_5V`"]
    #[inline(always)]
    pub fn is_2_5v(&self) -> bool {
        **self == SMTH_A::_2_5V
    }
    #[doc = "Checks if the value of the field is `_2_6V`"]
    #[inline(always)]
    pub fn is_2_6v(&self) -> bool {
        **self == SMTH_A::_2_6V
    }
    #[doc = "Checks if the value of the field is `_2_7V`"]
    #[inline(always)]
    pub fn is_2_7v(&self) -> bool {
        **self == SMTH_A::_2_7V
    }
    #[doc = "Checks if the value of the field is `_2_8V`"]
    #[inline(always)]
    pub fn is_2_8v(&self) -> bool {
        **self == SMTH_A::_2_8V
    }
    #[doc = "Checks if the value of the field is `_2_9V`"]
    #[inline(always)]
    pub fn is_2_9v(&self) -> bool {
        **self == SMTH_A::_2_9V
    }
    #[doc = "Checks if the value of the field is `_3_0V`"]
    #[inline(always)]
    pub fn is_3_0v(&self) -> bool {
        **self == SMTH_A::_3_0V
    }
    #[doc = "Checks if the value of the field is `_3_1V`"]
    #[inline(always)]
    pub fn is_3_1v(&self) -> bool {
        **self == SMTH_A::_3_1V
    }
    #[doc = "Checks if the value of the field is `_3_2V`"]
    #[inline(always)]
    pub fn is_3_2v(&self) -> bool {
        **self == SMTH_A::_3_2V
    }
    #[doc = "Checks if the value of the field is `_3_3V`"]
    #[inline(always)]
    pub fn is_3_3v(&self) -> bool {
        **self == SMTH_A::_3_3V
    }
    #[doc = "Checks if the value of the field is `_3_4V`"]
    #[inline(always)]
    pub fn is_3_4v(&self) -> bool {
        **self == SMTH_A::_3_4V
    }
}
impl core::ops::Deref for SMTH_R {
    type Target = crate::FieldReader<u8, SMTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTH` writer - Supply Monitor Threshold"]
pub struct SMTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.9 V"]
    #[inline(always)]
    pub fn _1_9v(self) -> &'a mut W {
        self.variant(SMTH_A::_1_9V)
    }
    #[doc = "2.0 V"]
    #[inline(always)]
    pub fn _2_0v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_0V)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn _2_1v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_1V)
    }
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn _2_2v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_2V)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn _2_3v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_3V)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn _2_4v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_4V)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn _2_5v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_5V)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn _2_6v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_6V)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn _2_7v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_7V)
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn _2_8v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_8V)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn _2_9v(self) -> &'a mut W {
        self.variant(SMTH_A::_2_9V)
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn _3_0v(self) -> &'a mut W {
        self.variant(SMTH_A::_3_0V)
    }
    #[doc = "3.1 V"]
    #[inline(always)]
    pub fn _3_1v(self) -> &'a mut W {
        self.variant(SMTH_A::_3_1V)
    }
    #[doc = "3.2 V"]
    #[inline(always)]
    pub fn _3_2v(self) -> &'a mut W {
        self.variant(SMTH_A::_3_2V)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn _3_3v(self) -> &'a mut W {
        self.variant(SMTH_A::_3_3V)
    }
    #[doc = "3.4 V"]
    #[inline(always)]
    pub fn _3_4v(self) -> &'a mut W {
        self.variant(SMTH_A::_3_4V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Supply Monitor Sampling Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMSMPL_A {
    #[doc = "0: Supply Monitor disabled"]
    SMD = 0,
    #[doc = "1: Continuous Supply Monitor"]
    CSM = 1,
    #[doc = "2: Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK = 2,
    #[doc = "3: Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK = 3,
    #[doc = "4: Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK = 4,
}
impl From<SMSMPL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSMPL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMSMPL` reader - Supply Monitor Sampling Period"]
pub struct SMSMPL_R(crate::FieldReader<u8, SMSMPL_A>);
impl SMSMPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMSMPL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMSMPL_A> {
        match self.bits {
            0 => Some(SMSMPL_A::SMD),
            1 => Some(SMSMPL_A::CSM),
            2 => Some(SMSMPL_A::_32SLCK),
            3 => Some(SMSMPL_A::_256SLCK),
            4 => Some(SMSMPL_A::_2048SLCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SMD`"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        **self == SMSMPL_A::SMD
    }
    #[doc = "Checks if the value of the field is `CSM`"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        **self == SMSMPL_A::CSM
    }
    #[doc = "Checks if the value of the field is `_32SLCK`"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        **self == SMSMPL_A::_32SLCK
    }
    #[doc = "Checks if the value of the field is `_256SLCK`"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        **self == SMSMPL_A::_256SLCK
    }
    #[doc = "Checks if the value of the field is `_2048SLCK`"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        **self == SMSMPL_A::_2048SLCK
    }
}
impl core::ops::Deref for SMSMPL_R {
    type Target = crate::FieldReader<u8, SMSMPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMSMPL` writer - Supply Monitor Sampling Period"]
pub struct SMSMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSMPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMSMPL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPL_A::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPL_A::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_2048SLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTEN_A {
    #[doc = "0: the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE = 1,
}
impl From<SMRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTEN` reader - Supply Monitor Reset Enable"]
pub struct SMRSTEN_R(crate::FieldReader<bool, SMRSTEN_A>);
impl SMRSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMRSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTEN_A {
        match self.bits {
            false => SMRSTEN_A::NOT_ENABLE,
            true => SMRSTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == SMRSTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SMRSTEN_A::ENABLE
    }
}
impl core::ops::Deref for SMRSTEN_R {
    type Target = crate::FieldReader<bool, SMRSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMRSTEN` writer - Supply Monitor Reset Enable"]
pub struct SMRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMRSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::NOT_ENABLE)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::ENABLE)
    }
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
#[doc = "Supply Monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMIEN_A {
    #[doc = "0: the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE = 1,
}
impl From<SMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMIEN` reader - Supply Monitor Interrupt Enable"]
pub struct SMIEN_R(crate::FieldReader<bool, SMIEN_A>);
impl SMIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIEN_A {
        match self.bits {
            false => SMIEN_A::NOT_ENABLE,
            true => SMIEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == SMIEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SMIEN_A::ENABLE
    }
}
impl core::ops::Deref for SMIEN_R {
    type Target = crate::FieldReader<bool, SMIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMIEN` writer - Supply Monitor Interrupt Enable"]
pub struct SMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIEN_A::NOT_ENABLE)
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIEN_A::ENABLE)
    }
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
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SMTH_R {
        SMTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SMSMPL_R {
        SMSMPL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SMRSTEN_R {
        SMRSTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SMIEN_R {
        SMIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&mut self) -> SMTH_W {
        SMTH_W { w: self }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&mut self) -> SMSMPL_W {
        SMSMPL_W { w: self }
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&mut self) -> SMRSTEN_W {
        SMRSTEN_W { w: self }
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&mut self) -> SMIEN_W {
        SMIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Supply Monitor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](index.html) module"]
pub struct SMMR_SPEC;
impl crate::RegisterSpec for SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smmr::R](R) reader structure"]
impl crate::Readable for SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smmr::W](W) writer structure"]
impl crate::Writable for SMMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SMMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
