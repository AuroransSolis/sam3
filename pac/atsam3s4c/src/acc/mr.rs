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
#[doc = "SELection for MINUS comparator input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELMINUS_A {
    #[doc = "0: SelectTS"]
    TS = 0,
    #[doc = "1: Select ADVREF"]
    ADVREF = 1,
    #[doc = "2: Select DAC0"]
    DAC0 = 2,
    #[doc = "3: Select DAC1"]
    DAC1 = 3,
    #[doc = "4: Select AD0"]
    AD0 = 4,
    #[doc = "5: Select AD1"]
    AD1 = 5,
    #[doc = "6: Select AD2"]
    AD2 = 6,
    #[doc = "7: Select AD3"]
    AD3 = 7,
}
impl From<SELMINUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMINUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELMINUS` reader - SELection for MINUS comparator input"]
pub struct SELMINUS_R(crate::FieldReader<u8, SELMINUS_A>);
impl SELMINUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELMINUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMINUS_A {
        match self.bits {
            0 => SELMINUS_A::TS,
            1 => SELMINUS_A::ADVREF,
            2 => SELMINUS_A::DAC0,
            3 => SELMINUS_A::DAC1,
            4 => SELMINUS_A::AD0,
            5 => SELMINUS_A::AD1,
            6 => SELMINUS_A::AD2,
            7 => SELMINUS_A::AD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == SELMINUS_A::TS
    }
    #[doc = "Checks if the value of the field is `ADVREF`"]
    #[inline(always)]
    pub fn is_advref(&self) -> bool {
        **self == SELMINUS_A::ADVREF
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        **self == SELMINUS_A::DAC0
    }
    #[doc = "Checks if the value of the field is `DAC1`"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        **self == SELMINUS_A::DAC1
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        **self == SELMINUS_A::AD0
    }
    #[doc = "Checks if the value of the field is `AD1`"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        **self == SELMINUS_A::AD1
    }
    #[doc = "Checks if the value of the field is `AD2`"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        **self == SELMINUS_A::AD2
    }
    #[doc = "Checks if the value of the field is `AD3`"]
    #[inline(always)]
    pub fn is_ad3(&self) -> bool {
        **self == SELMINUS_A::AD3
    }
}
impl core::ops::Deref for SELMINUS_R {
    type Target = crate::FieldReader<u8, SELMINUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELMINUS` writer - SELection for MINUS comparator input"]
pub struct SELMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELMINUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELMINUS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SelectTS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(SELMINUS_A::TS)
    }
    #[doc = "Select ADVREF"]
    #[inline(always)]
    pub fn advref(self) -> &'a mut W {
        self.variant(SELMINUS_A::ADVREF)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SELMINUS_A::DAC0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut W {
        self.variant(SELMINUS_A::DAC1)
    }
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(SELMINUS_A::AD0)
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut W {
        self.variant(SELMINUS_A::AD1)
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut W {
        self.variant(SELMINUS_A::AD2)
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn ad3(self) -> &'a mut W {
        self.variant(SELMINUS_A::AD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "SELection for PLUS comparator input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELPLUS_A {
    #[doc = "0: Select AD0"]
    AD0 = 0,
    #[doc = "1: Select AD1"]
    AD1 = 1,
    #[doc = "2: Select AD2"]
    AD2 = 2,
    #[doc = "3: Select AD3"]
    AD3 = 3,
    #[doc = "4: Select AD4"]
    AD4 = 4,
    #[doc = "5: Select AD5"]
    AD5 = 5,
    #[doc = "6: Select AD6"]
    AD6 = 6,
    #[doc = "7: Select AD7"]
    AD7 = 7,
}
impl From<SELPLUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELPLUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELPLUS` reader - SELection for PLUS comparator input"]
pub struct SELPLUS_R(crate::FieldReader<u8, SELPLUS_A>);
impl SELPLUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELPLUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELPLUS_A {
        match self.bits {
            0 => SELPLUS_A::AD0,
            1 => SELPLUS_A::AD1,
            2 => SELPLUS_A::AD2,
            3 => SELPLUS_A::AD3,
            4 => SELPLUS_A::AD4,
            5 => SELPLUS_A::AD5,
            6 => SELPLUS_A::AD6,
            7 => SELPLUS_A::AD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        **self == SELPLUS_A::AD0
    }
    #[doc = "Checks if the value of the field is `AD1`"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        **self == SELPLUS_A::AD1
    }
    #[doc = "Checks if the value of the field is `AD2`"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        **self == SELPLUS_A::AD2
    }
    #[doc = "Checks if the value of the field is `AD3`"]
    #[inline(always)]
    pub fn is_ad3(&self) -> bool {
        **self == SELPLUS_A::AD3
    }
    #[doc = "Checks if the value of the field is `AD4`"]
    #[inline(always)]
    pub fn is_ad4(&self) -> bool {
        **self == SELPLUS_A::AD4
    }
    #[doc = "Checks if the value of the field is `AD5`"]
    #[inline(always)]
    pub fn is_ad5(&self) -> bool {
        **self == SELPLUS_A::AD5
    }
    #[doc = "Checks if the value of the field is `AD6`"]
    #[inline(always)]
    pub fn is_ad6(&self) -> bool {
        **self == SELPLUS_A::AD6
    }
    #[doc = "Checks if the value of the field is `AD7`"]
    #[inline(always)]
    pub fn is_ad7(&self) -> bool {
        **self == SELPLUS_A::AD7
    }
}
impl core::ops::Deref for SELPLUS_R {
    type Target = crate::FieldReader<u8, SELPLUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELPLUS` writer - SELection for PLUS comparator input"]
pub struct SELPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELPLUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELPLUS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD0)
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD1)
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD2)
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn ad3(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD3)
    }
    #[doc = "Select AD4"]
    #[inline(always)]
    pub fn ad4(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD4)
    }
    #[doc = "Select AD5"]
    #[inline(always)]
    pub fn ad5(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD5)
    }
    #[doc = "Select AD6"]
    #[inline(always)]
    pub fn ad6(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD6)
    }
    #[doc = "Select AD7"]
    #[inline(always)]
    pub fn ad7(self) -> &'a mut W {
        self.variant(SELPLUS_A::AD7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Analog Comparator ENable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACEN_A {
    #[doc = "0: Analog Comparator Disabled."]
    DIS = 0,
    #[doc = "1: Analog Comparator Enabled."]
    EN = 1,
}
impl From<ACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACEN` reader - Analog Comparator ENable"]
pub struct ACEN_R(crate::FieldReader<bool, ACEN_A>);
impl ACEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACEN_A {
        match self.bits {
            false => ACEN_A::DIS,
            true => ACEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ACEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ACEN_A::EN
    }
}
impl core::ops::Deref for ACEN_R {
    type Target = crate::FieldReader<bool, ACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACEN` writer - Analog Comparator ENable"]
pub struct ACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog Comparator Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACEN_A::DIS)
    }
    #[doc = "Analog Comparator Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACEN_A::EN)
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
#[doc = "EDGE TYPe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGETYP_A {
    #[doc = "0: only rising edge of comparator output"]
    RISING = 0,
    #[doc = "1: falling edge of comparator output"]
    FALLING = 1,
    #[doc = "2: any edge of comparator output"]
    ANY = 2,
}
impl From<EDGETYP_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGETYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDGETYP` reader - EDGE TYPe"]
pub struct EDGETYP_R(crate::FieldReader<u8, EDGETYP_A>);
impl EDGETYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGETYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EDGETYP_A> {
        match self.bits {
            0 => Some(EDGETYP_A::RISING),
            1 => Some(EDGETYP_A::FALLING),
            2 => Some(EDGETYP_A::ANY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == EDGETYP_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == EDGETYP_A::FALLING
    }
    #[doc = "Checks if the value of the field is `ANY`"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        **self == EDGETYP_A::ANY
    }
}
impl core::ops::Deref for EDGETYP_R {
    type Target = crate::FieldReader<u8, EDGETYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGETYP` writer - EDGE TYPe"]
pub struct EDGETYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGETYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGETYP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGETYP_A::RISING)
    }
    #[doc = "falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGETYP_A::FALLING)
    }
    #[doc = "any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut W {
        self.variant(EDGETYP_A::ANY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "INVert comparator output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Analog Comparator output is directly processed."]
    DIS = 0,
    #[doc = "1: Analog Comparator output is inverted prior to being processed."]
    EN = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - INVert comparator output"]
pub struct INV_R(crate::FieldReader<bool, INV_A>);
impl INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::DIS,
            true => INV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == INV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == INV_A::EN
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, INV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - INVert comparator output"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog Comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INV_A::DIS)
    }
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INV_A::EN)
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
#[doc = "SELection of Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELFS_A {
    #[doc = "0: the CF flag is used to drive the FAULT output."]
    CF = 0,
    #[doc = "1: the output of the Analog Comparator flag is used to drive the FAULT output."]
    OUTPUT = 1,
}
impl From<SELFS_A> for bool {
    #[inline(always)]
    fn from(variant: SELFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELFS` reader - SELection of Fault Source"]
pub struct SELFS_R(crate::FieldReader<bool, SELFS_A>);
impl SELFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SELFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELFS_A {
        match self.bits {
            false => SELFS_A::CF,
            true => SELFS_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CF`"]
    #[inline(always)]
    pub fn is_cf(&self) -> bool {
        **self == SELFS_A::CF
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == SELFS_A::OUTPUT
    }
}
impl core::ops::Deref for SELFS_R {
    type Target = crate::FieldReader<bool, SELFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFS` writer - SELection of Fault Source"]
pub struct SELFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the CF flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn cf(self) -> &'a mut W {
        self.variant(SELFS_A::CF)
    }
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SELFS_A::OUTPUT)
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
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: the FAULT output is tied to 0."]
    DIS = 0,
    #[doc = "1: the FAULT output is driven by the signal defined by SELFS."]
    EN = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Fault Enable"]
pub struct FE_R(crate::FieldReader<bool, FE_A>);
impl FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::DIS,
            true => FE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FE_A::EN
    }
}
impl core::ops::Deref for FE_R {
    type Target = crate::FieldReader<bool, FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FE` writer - Fault Enable"]
pub struct FE_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FE_A::DIS)
    }
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FE_A::EN)
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
impl R {
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline(always)]
    pub fn selminus(&self) -> SELMINUS_R {
        SELMINUS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline(always)]
    pub fn selplus(&self) -> SELPLUS_R {
        SELPLUS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EDGETYP_R {
        EDGETYP_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SELFS_R {
        SELFS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline(always)]
    pub fn selminus(&mut self) -> SELMINUS_W {
        SELMINUS_W { w: self }
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline(always)]
    pub fn selplus(&mut self) -> SELPLUS_W {
        SELPLUS_W { w: self }
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline(always)]
    pub fn acen(&mut self) -> ACEN_W {
        ACEN_W { w: self }
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline(always)]
    pub fn edgetyp(&mut self) -> EDGETYP_W {
        EDGETYP_W { w: self }
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline(always)]
    pub fn selfs(&mut self) -> SELFS_W {
        SELFS_W { w: self }
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
