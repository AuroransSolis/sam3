#[doc = "Register `WUIR` reader"]
pub struct R(crate::R<WUIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUIR` writer"]
pub struct W(crate::W<WUIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUIR_SPEC>;
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
impl From<crate::W<WUIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wake-up Input Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN0_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN0` reader - Wake-up Input Enable 0"]
pub struct WKUPEN0_R(crate::FieldReader<bool, WKUPEN0_A>);
impl WKUPEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN0_A {
        match self.bits {
            false => WKUPEN0_A::DISABLE,
            true => WKUPEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN0_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN0_R {
    type Target = crate::FieldReader<bool, WKUPEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN0` writer - Wake-up Input Enable 0"]
pub struct WKUPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN0_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Wake-up Input Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN1_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN1` reader - Wake-up Input Enable 1"]
pub struct WKUPEN1_R(crate::FieldReader<bool, WKUPEN1_A>);
impl WKUPEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN1_A {
        match self.bits {
            false => WKUPEN1_A::DISABLE,
            true => WKUPEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN1_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN1_R {
    type Target = crate::FieldReader<bool, WKUPEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN1` writer - Wake-up Input Enable 1"]
pub struct WKUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN1_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Wake-up Input Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN2_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN2` reader - Wake-up Input Enable 2"]
pub struct WKUPEN2_R(crate::FieldReader<bool, WKUPEN2_A>);
impl WKUPEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN2_A {
        match self.bits {
            false => WKUPEN2_A::DISABLE,
            true => WKUPEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN2_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN2_R {
    type Target = crate::FieldReader<bool, WKUPEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN2` writer - Wake-up Input Enable 2"]
pub struct WKUPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN2_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Wake-up Input Enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN3_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN3` reader - Wake-up Input Enable 3"]
pub struct WKUPEN3_R(crate::FieldReader<bool, WKUPEN3_A>);
impl WKUPEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN3_A {
        match self.bits {
            false => WKUPEN3_A::DISABLE,
            true => WKUPEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN3_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN3_R {
    type Target = crate::FieldReader<bool, WKUPEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN3` writer - Wake-up Input Enable 3"]
pub struct WKUPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN3_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Wake-up Input Enable 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN4_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN4` reader - Wake-up Input Enable 4"]
pub struct WKUPEN4_R(crate::FieldReader<bool, WKUPEN4_A>);
impl WKUPEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN4_A {
        match self.bits {
            false => WKUPEN4_A::DISABLE,
            true => WKUPEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN4_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN4_R {
    type Target = crate::FieldReader<bool, WKUPEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN4` writer - Wake-up Input Enable 4"]
pub struct WKUPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN4_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN4_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Wake-up Input Enable 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN5_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN5` reader - Wake-up Input Enable 5"]
pub struct WKUPEN5_R(crate::FieldReader<bool, WKUPEN5_A>);
impl WKUPEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN5_A {
        match self.bits {
            false => WKUPEN5_A::DISABLE,
            true => WKUPEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN5_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN5_R {
    type Target = crate::FieldReader<bool, WKUPEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN5` writer - Wake-up Input Enable 5"]
pub struct WKUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN5_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN5_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Wake-up Input Enable 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN6_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN6_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN6` reader - Wake-up Input Enable 6"]
pub struct WKUPEN6_R(crate::FieldReader<bool, WKUPEN6_A>);
impl WKUPEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN6_A {
        match self.bits {
            false => WKUPEN6_A::DISABLE,
            true => WKUPEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN6_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN6_R {
    type Target = crate::FieldReader<bool, WKUPEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN6` writer - Wake-up Input Enable 6"]
pub struct WKUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN6_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN6_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Wake-up Input Enable 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN7_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN7_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN7` reader - Wake-up Input Enable 7"]
pub struct WKUPEN7_R(crate::FieldReader<bool, WKUPEN7_A>);
impl WKUPEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN7_A {
        match self.bits {
            false => WKUPEN7_A::DISABLE,
            true => WKUPEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN7_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN7_R {
    type Target = crate::FieldReader<bool, WKUPEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN7` writer - Wake-up Input Enable 7"]
pub struct WKUPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN7_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN7_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Wake-up Input Enable 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN8_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN8_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN8` reader - Wake-up Input Enable 8"]
pub struct WKUPEN8_R(crate::FieldReader<bool, WKUPEN8_A>);
impl WKUPEN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN8_A {
        match self.bits {
            false => WKUPEN8_A::DISABLE,
            true => WKUPEN8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN8_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN8_R {
    type Target = crate::FieldReader<bool, WKUPEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN8` writer - Wake-up Input Enable 8"]
pub struct WKUPEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN8_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN8_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Wake-up Input Enable 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN9_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN9_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN9` reader - Wake-up Input Enable 9"]
pub struct WKUPEN9_R(crate::FieldReader<bool, WKUPEN9_A>);
impl WKUPEN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN9_A {
        match self.bits {
            false => WKUPEN9_A::DISABLE,
            true => WKUPEN9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN9_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN9_R {
    type Target = crate::FieldReader<bool, WKUPEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN9` writer - Wake-up Input Enable 9"]
pub struct WKUPEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN9_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN9_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Wake-up Input Enable 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN10_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN10_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN10` reader - Wake-up Input Enable 10"]
pub struct WKUPEN10_R(crate::FieldReader<bool, WKUPEN10_A>);
impl WKUPEN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN10_A {
        match self.bits {
            false => WKUPEN10_A::DISABLE,
            true => WKUPEN10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN10_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN10_R {
    type Target = crate::FieldReader<bool, WKUPEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN10` writer - Wake-up Input Enable 10"]
pub struct WKUPEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN10_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN10_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Wake-up Input Enable 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN11_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN11_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN11` reader - Wake-up Input Enable 11"]
pub struct WKUPEN11_R(crate::FieldReader<bool, WKUPEN11_A>);
impl WKUPEN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN11_A {
        match self.bits {
            false => WKUPEN11_A::DISABLE,
            true => WKUPEN11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN11_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN11_R {
    type Target = crate::FieldReader<bool, WKUPEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN11` writer - Wake-up Input Enable 11"]
pub struct WKUPEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN11_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN11_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Wake-up Input Enable 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN12_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN12_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN12` reader - Wake-up Input Enable 12"]
pub struct WKUPEN12_R(crate::FieldReader<bool, WKUPEN12_A>);
impl WKUPEN12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN12_A {
        match self.bits {
            false => WKUPEN12_A::DISABLE,
            true => WKUPEN12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN12_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN12_R {
    type Target = crate::FieldReader<bool, WKUPEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN12` writer - Wake-up Input Enable 12"]
pub struct WKUPEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN12_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN12_A::ENABLE)
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
#[doc = "Wake-up Input Enable 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN13_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN13_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN13` reader - Wake-up Input Enable 13"]
pub struct WKUPEN13_R(crate::FieldReader<bool, WKUPEN13_A>);
impl WKUPEN13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN13_A {
        match self.bits {
            false => WKUPEN13_A::DISABLE,
            true => WKUPEN13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN13_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN13_R {
    type Target = crate::FieldReader<bool, WKUPEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN13` writer - Wake-up Input Enable 13"]
pub struct WKUPEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN13_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN13_A::ENABLE)
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
#[doc = "Wake-up Input Enable 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN14_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN14_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN14` reader - Wake-up Input Enable 14"]
pub struct WKUPEN14_R(crate::FieldReader<bool, WKUPEN14_A>);
impl WKUPEN14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN14_A {
        match self.bits {
            false => WKUPEN14_A::DISABLE,
            true => WKUPEN14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN14_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN14_R {
    type Target = crate::FieldReader<bool, WKUPEN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN14` writer - Wake-up Input Enable 14"]
pub struct WKUPEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN14_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN14_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Wake-up Input Enable 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN15_A {
    #[doc = "0: the corresponding wake-up input has no wake-up effect."]
    DISABLE = 0,
    #[doc = "1: the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN15_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN15` reader - Wake-up Input Enable 15"]
pub struct WKUPEN15_R(crate::FieldReader<bool, WKUPEN15_A>);
impl WKUPEN15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPEN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN15_A {
        match self.bits {
            false => WKUPEN15_A::DISABLE,
            true => WKUPEN15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKUPEN15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKUPEN15_A::ENABLE
    }
}
impl core::ops::Deref for WKUPEN15_R {
    type Target = crate::FieldReader<bool, WKUPEN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPEN15` writer - Wake-up Input Enable 15"]
pub struct WKUPEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN15_A::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN15_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Wake-up Input Type 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT0_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT0_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT0` reader - Wake-up Input Type 0"]
pub struct WKUPT0_R(crate::FieldReader<bool, WKUPT0_A>);
impl WKUPT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT0_A {
        match self.bits {
            false => WKUPT0_A::HIGH_TO_LOW,
            true => WKUPT0_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT0_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT0_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT0_R {
    type Target = crate::FieldReader<bool, WKUPT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT0` writer - Wake-up Input Type 0"]
pub struct WKUPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT0_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT0_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Wake-up Input Type 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT1_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT1_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT1` reader - Wake-up Input Type 1"]
pub struct WKUPT1_R(crate::FieldReader<bool, WKUPT1_A>);
impl WKUPT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT1_A {
        match self.bits {
            false => WKUPT1_A::HIGH_TO_LOW,
            true => WKUPT1_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT1_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT1_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT1_R {
    type Target = crate::FieldReader<bool, WKUPT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT1` writer - Wake-up Input Type 1"]
pub struct WKUPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT1_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT1_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Wake-up Input Type 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT2_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT2_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT2` reader - Wake-up Input Type 2"]
pub struct WKUPT2_R(crate::FieldReader<bool, WKUPT2_A>);
impl WKUPT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT2_A {
        match self.bits {
            false => WKUPT2_A::HIGH_TO_LOW,
            true => WKUPT2_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT2_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT2_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT2_R {
    type Target = crate::FieldReader<bool, WKUPT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT2` writer - Wake-up Input Type 2"]
pub struct WKUPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT2_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT2_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Wake-up Input Type 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT3_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT3_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT3` reader - Wake-up Input Type 3"]
pub struct WKUPT3_R(crate::FieldReader<bool, WKUPT3_A>);
impl WKUPT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT3_A {
        match self.bits {
            false => WKUPT3_A::HIGH_TO_LOW,
            true => WKUPT3_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT3_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT3_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT3_R {
    type Target = crate::FieldReader<bool, WKUPT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT3` writer - Wake-up Input Type 3"]
pub struct WKUPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT3_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT3_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Wake-up Input Type 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT4_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT4_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT4` reader - Wake-up Input Type 4"]
pub struct WKUPT4_R(crate::FieldReader<bool, WKUPT4_A>);
impl WKUPT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT4_A {
        match self.bits {
            false => WKUPT4_A::HIGH_TO_LOW,
            true => WKUPT4_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT4_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT4_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT4_R {
    type Target = crate::FieldReader<bool, WKUPT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT4` writer - Wake-up Input Type 4"]
pub struct WKUPT4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT4_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT4_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Wake-up Input Type 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT5_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT5_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT5` reader - Wake-up Input Type 5"]
pub struct WKUPT5_R(crate::FieldReader<bool, WKUPT5_A>);
impl WKUPT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT5_A {
        match self.bits {
            false => WKUPT5_A::HIGH_TO_LOW,
            true => WKUPT5_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT5_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT5_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT5_R {
    type Target = crate::FieldReader<bool, WKUPT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT5` writer - Wake-up Input Type 5"]
pub struct WKUPT5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT5_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT5_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Wake-up Input Type 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT6_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT6_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT6` reader - Wake-up Input Type 6"]
pub struct WKUPT6_R(crate::FieldReader<bool, WKUPT6_A>);
impl WKUPT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT6_A {
        match self.bits {
            false => WKUPT6_A::HIGH_TO_LOW,
            true => WKUPT6_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT6_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT6_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT6_R {
    type Target = crate::FieldReader<bool, WKUPT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT6` writer - Wake-up Input Type 6"]
pub struct WKUPT6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT6_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT6_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Wake-up Input Type 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT7_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT7_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT7` reader - Wake-up Input Type 7"]
pub struct WKUPT7_R(crate::FieldReader<bool, WKUPT7_A>);
impl WKUPT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT7_A {
        match self.bits {
            false => WKUPT7_A::HIGH_TO_LOW,
            true => WKUPT7_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT7_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT7_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT7_R {
    type Target = crate::FieldReader<bool, WKUPT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT7` writer - Wake-up Input Type 7"]
pub struct WKUPT7_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT7_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT7_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Wake-up Input Type 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT8_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT8_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT8` reader - Wake-up Input Type 8"]
pub struct WKUPT8_R(crate::FieldReader<bool, WKUPT8_A>);
impl WKUPT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT8_A {
        match self.bits {
            false => WKUPT8_A::HIGH_TO_LOW,
            true => WKUPT8_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT8_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT8_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT8_R {
    type Target = crate::FieldReader<bool, WKUPT8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT8` writer - Wake-up Input Type 8"]
pub struct WKUPT8_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT8_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT8_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Wake-up Input Type 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT9_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT9_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT9` reader - Wake-up Input Type 9"]
pub struct WKUPT9_R(crate::FieldReader<bool, WKUPT9_A>);
impl WKUPT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT9_A {
        match self.bits {
            false => WKUPT9_A::HIGH_TO_LOW,
            true => WKUPT9_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT9_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT9_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT9_R {
    type Target = crate::FieldReader<bool, WKUPT9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT9` writer - Wake-up Input Type 9"]
pub struct WKUPT9_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT9_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT9_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Wake-up Input Type 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT10_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT10_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT10` reader - Wake-up Input Type 10"]
pub struct WKUPT10_R(crate::FieldReader<bool, WKUPT10_A>);
impl WKUPT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT10_A {
        match self.bits {
            false => WKUPT10_A::HIGH_TO_LOW,
            true => WKUPT10_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT10_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT10_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT10_R {
    type Target = crate::FieldReader<bool, WKUPT10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT10` writer - Wake-up Input Type 10"]
pub struct WKUPT10_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT10_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT10_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Wake-up Input Type 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT11_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT11_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT11` reader - Wake-up Input Type 11"]
pub struct WKUPT11_R(crate::FieldReader<bool, WKUPT11_A>);
impl WKUPT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT11_A {
        match self.bits {
            false => WKUPT11_A::HIGH_TO_LOW,
            true => WKUPT11_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT11_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT11_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT11_R {
    type Target = crate::FieldReader<bool, WKUPT11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT11` writer - Wake-up Input Type 11"]
pub struct WKUPT11_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT11_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT11_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Wake-up Input Type 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT12_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT12_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT12` reader - Wake-up Input Type 12"]
pub struct WKUPT12_R(crate::FieldReader<bool, WKUPT12_A>);
impl WKUPT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT12_A {
        match self.bits {
            false => WKUPT12_A::HIGH_TO_LOW,
            true => WKUPT12_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT12_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT12_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT12_R {
    type Target = crate::FieldReader<bool, WKUPT12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT12` writer - Wake-up Input Type 12"]
pub struct WKUPT12_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT12_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT12_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Wake-up Input Type 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT13_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT13_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT13` reader - Wake-up Input Type 13"]
pub struct WKUPT13_R(crate::FieldReader<bool, WKUPT13_A>);
impl WKUPT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT13_A {
        match self.bits {
            false => WKUPT13_A::HIGH_TO_LOW,
            true => WKUPT13_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT13_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT13_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT13_R {
    type Target = crate::FieldReader<bool, WKUPT13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT13` writer - Wake-up Input Type 13"]
pub struct WKUPT13_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT13_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT13_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Wake-up Input Type 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT14_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT14_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT14` reader - Wake-up Input Type 14"]
pub struct WKUPT14_R(crate::FieldReader<bool, WKUPT14_A>);
impl WKUPT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT14_A {
        match self.bits {
            false => WKUPT14_A::HIGH_TO_LOW,
            true => WKUPT14_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT14_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT14_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT14_R {
    type Target = crate::FieldReader<bool, WKUPT14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT14` writer - Wake-up Input Type 14"]
pub struct WKUPT14_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT14_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT14_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Wake-up Input Type 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT15_A {
    #[doc = "0: a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW = 0,
    #[doc = "1: a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH = 1,
}
impl From<WKUPT15_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT15` reader - Wake-up Input Type 15"]
pub struct WKUPT15_R(crate::FieldReader<bool, WKUPT15_A>);
impl WKUPT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPT15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT15_A {
        match self.bits {
            false => WKUPT15_A::HIGH_TO_LOW,
            true => WKUPT15_A::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline(always)]
    pub fn is_high_to_low(&self) -> bool {
        **self == WKUPT15_A::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline(always)]
    pub fn is_low_to_high(&self) -> bool {
        **self == WKUPT15_A::LOW_TO_HIGH
    }
}
impl core::ops::Deref for WKUPT15_R {
    type Target = crate::FieldReader<bool, WKUPT15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPT15` writer - Wake-up Input Type 15"]
pub struct WKUPT15_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT15_A::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT15_A::LOW_TO_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Input Enable 0"]
    #[inline(always)]
    pub fn wkupen0(&self) -> WKUPEN0_R {
        WKUPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake-up Input Enable 1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-up Input Enable 2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake-up Input Enable 3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-up Input Enable 4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake-up Input Enable 5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-up Input Enable 6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wake-up Input Enable 7"]
    #[inline(always)]
    pub fn wkupen7(&self) -> WKUPEN7_R {
        WKUPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-up Input Enable 8"]
    #[inline(always)]
    pub fn wkupen8(&self) -> WKUPEN8_R {
        WKUPEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wake-up Input Enable 9"]
    #[inline(always)]
    pub fn wkupen9(&self) -> WKUPEN9_R {
        WKUPEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wake-up Input Enable 10"]
    #[inline(always)]
    pub fn wkupen10(&self) -> WKUPEN10_R {
        WKUPEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wake-up Input Enable 11"]
    #[inline(always)]
    pub fn wkupen11(&self) -> WKUPEN11_R {
        WKUPEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wake-up Input Enable 12"]
    #[inline(always)]
    pub fn wkupen12(&self) -> WKUPEN12_R {
        WKUPEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wake-up Input Enable 13"]
    #[inline(always)]
    pub fn wkupen13(&self) -> WKUPEN13_R {
        WKUPEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wake-up Input Enable 14"]
    #[inline(always)]
    pub fn wkupen14(&self) -> WKUPEN14_R {
        WKUPEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wake-up Input Enable 15"]
    #[inline(always)]
    pub fn wkupen15(&self) -> WKUPEN15_R {
        WKUPEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Wake-up Input Type 0"]
    #[inline(always)]
    pub fn wkupt0(&self) -> WKUPT0_R {
        WKUPT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake-up Input Type 1"]
    #[inline(always)]
    pub fn wkupt1(&self) -> WKUPT1_R {
        WKUPT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Wake-up Input Type 2"]
    #[inline(always)]
    pub fn wkupt2(&self) -> WKUPT2_R {
        WKUPT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Wake-up Input Type 3"]
    #[inline(always)]
    pub fn wkupt3(&self) -> WKUPT3_R {
        WKUPT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Wake-up Input Type 4"]
    #[inline(always)]
    pub fn wkupt4(&self) -> WKUPT4_R {
        WKUPT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Wake-up Input Type 5"]
    #[inline(always)]
    pub fn wkupt5(&self) -> WKUPT5_R {
        WKUPT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Wake-up Input Type 6"]
    #[inline(always)]
    pub fn wkupt6(&self) -> WKUPT6_R {
        WKUPT6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Wake-up Input Type 7"]
    #[inline(always)]
    pub fn wkupt7(&self) -> WKUPT7_R {
        WKUPT7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Wake-up Input Type 8"]
    #[inline(always)]
    pub fn wkupt8(&self) -> WKUPT8_R {
        WKUPT8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Wake-up Input Type 9"]
    #[inline(always)]
    pub fn wkupt9(&self) -> WKUPT9_R {
        WKUPT9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Wake-up Input Type 10"]
    #[inline(always)]
    pub fn wkupt10(&self) -> WKUPT10_R {
        WKUPT10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Wake-up Input Type 11"]
    #[inline(always)]
    pub fn wkupt11(&self) -> WKUPT11_R {
        WKUPT11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Wake-up Input Type 12"]
    #[inline(always)]
    pub fn wkupt12(&self) -> WKUPT12_R {
        WKUPT12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Wake-up Input Type 13"]
    #[inline(always)]
    pub fn wkupt13(&self) -> WKUPT13_R {
        WKUPT13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Wake-up Input Type 14"]
    #[inline(always)]
    pub fn wkupt14(&self) -> WKUPT14_R {
        WKUPT14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wake-up Input Type 15"]
    #[inline(always)]
    pub fn wkupt15(&self) -> WKUPT15_R {
        WKUPT15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Input Enable 0"]
    #[inline(always)]
    pub fn wkupen0(&mut self) -> WKUPEN0_W {
        WKUPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Wake-up Input Enable 1"]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W {
        WKUPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Wake-up Input Enable 2"]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W {
        WKUPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Wake-up Input Enable 3"]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W {
        WKUPEN3_W { w: self }
    }
    #[doc = "Bit 4 - Wake-up Input Enable 4"]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W {
        WKUPEN4_W { w: self }
    }
    #[doc = "Bit 5 - Wake-up Input Enable 5"]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W {
        WKUPEN5_W { w: self }
    }
    #[doc = "Bit 6 - Wake-up Input Enable 6"]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W {
        WKUPEN6_W { w: self }
    }
    #[doc = "Bit 7 - Wake-up Input Enable 7"]
    #[inline(always)]
    pub fn wkupen7(&mut self) -> WKUPEN7_W {
        WKUPEN7_W { w: self }
    }
    #[doc = "Bit 8 - Wake-up Input Enable 8"]
    #[inline(always)]
    pub fn wkupen8(&mut self) -> WKUPEN8_W {
        WKUPEN8_W { w: self }
    }
    #[doc = "Bit 9 - Wake-up Input Enable 9"]
    #[inline(always)]
    pub fn wkupen9(&mut self) -> WKUPEN9_W {
        WKUPEN9_W { w: self }
    }
    #[doc = "Bit 10 - Wake-up Input Enable 10"]
    #[inline(always)]
    pub fn wkupen10(&mut self) -> WKUPEN10_W {
        WKUPEN10_W { w: self }
    }
    #[doc = "Bit 11 - Wake-up Input Enable 11"]
    #[inline(always)]
    pub fn wkupen11(&mut self) -> WKUPEN11_W {
        WKUPEN11_W { w: self }
    }
    #[doc = "Bit 12 - Wake-up Input Enable 12"]
    #[inline(always)]
    pub fn wkupen12(&mut self) -> WKUPEN12_W {
        WKUPEN12_W { w: self }
    }
    #[doc = "Bit 13 - Wake-up Input Enable 13"]
    #[inline(always)]
    pub fn wkupen13(&mut self) -> WKUPEN13_W {
        WKUPEN13_W { w: self }
    }
    #[doc = "Bit 14 - Wake-up Input Enable 14"]
    #[inline(always)]
    pub fn wkupen14(&mut self) -> WKUPEN14_W {
        WKUPEN14_W { w: self }
    }
    #[doc = "Bit 15 - Wake-up Input Enable 15"]
    #[inline(always)]
    pub fn wkupen15(&mut self) -> WKUPEN15_W {
        WKUPEN15_W { w: self }
    }
    #[doc = "Bit 16 - Wake-up Input Type 0"]
    #[inline(always)]
    pub fn wkupt0(&mut self) -> WKUPT0_W {
        WKUPT0_W { w: self }
    }
    #[doc = "Bit 17 - Wake-up Input Type 1"]
    #[inline(always)]
    pub fn wkupt1(&mut self) -> WKUPT1_W {
        WKUPT1_W { w: self }
    }
    #[doc = "Bit 18 - Wake-up Input Type 2"]
    #[inline(always)]
    pub fn wkupt2(&mut self) -> WKUPT2_W {
        WKUPT2_W { w: self }
    }
    #[doc = "Bit 19 - Wake-up Input Type 3"]
    #[inline(always)]
    pub fn wkupt3(&mut self) -> WKUPT3_W {
        WKUPT3_W { w: self }
    }
    #[doc = "Bit 20 - Wake-up Input Type 4"]
    #[inline(always)]
    pub fn wkupt4(&mut self) -> WKUPT4_W {
        WKUPT4_W { w: self }
    }
    #[doc = "Bit 21 - Wake-up Input Type 5"]
    #[inline(always)]
    pub fn wkupt5(&mut self) -> WKUPT5_W {
        WKUPT5_W { w: self }
    }
    #[doc = "Bit 22 - Wake-up Input Type 6"]
    #[inline(always)]
    pub fn wkupt6(&mut self) -> WKUPT6_W {
        WKUPT6_W { w: self }
    }
    #[doc = "Bit 23 - Wake-up Input Type 7"]
    #[inline(always)]
    pub fn wkupt7(&mut self) -> WKUPT7_W {
        WKUPT7_W { w: self }
    }
    #[doc = "Bit 24 - Wake-up Input Type 8"]
    #[inline(always)]
    pub fn wkupt8(&mut self) -> WKUPT8_W {
        WKUPT8_W { w: self }
    }
    #[doc = "Bit 25 - Wake-up Input Type 9"]
    #[inline(always)]
    pub fn wkupt9(&mut self) -> WKUPT9_W {
        WKUPT9_W { w: self }
    }
    #[doc = "Bit 26 - Wake-up Input Type 10"]
    #[inline(always)]
    pub fn wkupt10(&mut self) -> WKUPT10_W {
        WKUPT10_W { w: self }
    }
    #[doc = "Bit 27 - Wake-up Input Type 11"]
    #[inline(always)]
    pub fn wkupt11(&mut self) -> WKUPT11_W {
        WKUPT11_W { w: self }
    }
    #[doc = "Bit 28 - Wake-up Input Type 12"]
    #[inline(always)]
    pub fn wkupt12(&mut self) -> WKUPT12_W {
        WKUPT12_W { w: self }
    }
    #[doc = "Bit 29 - Wake-up Input Type 13"]
    #[inline(always)]
    pub fn wkupt13(&mut self) -> WKUPT13_W {
        WKUPT13_W { w: self }
    }
    #[doc = "Bit 30 - Wake-up Input Type 14"]
    #[inline(always)]
    pub fn wkupt14(&mut self) -> WKUPT14_W {
        WKUPT14_W { w: self }
    }
    #[doc = "Bit 31 - Wake-up Input Type 15"]
    #[inline(always)]
    pub fn wkupt15(&mut self) -> WKUPT15_W {
        WKUPT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Wake-up Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuir](index.html) module"]
pub struct WUIR_SPEC;
impl crate::RegisterSpec for WUIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuir::R](R) reader structure"]
impl crate::Readable for WUIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuir::W](W) writer structure"]
impl crate::Writable for WUIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUIR to value 0"]
impl crate::Resettable for WUIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
