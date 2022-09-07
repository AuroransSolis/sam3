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
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader<TRGEN_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger mode disabled. DACC in free running mode."]
    DIS = 0,
    #[doc = "1: External trigger mode enabled."]
    EN = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::DIS,
            true => TRGEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN_A::EN
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TRGEN_A, O>;
impl<'a, const O: u8> TRGEN_W<'a, O> {
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN_A::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN_A::EN)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader<u8, TRGSEL_A>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: External trigger"]
    TRGSEL0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    TRGSEL1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    TRGSEL2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    TRGSEL3 = 3,
    #[doc = "4: PWM Event Line 0"]
    TRGSEL4 = 4,
    #[doc = "5: PWM Event Line 1"]
    TRGSEL5 = 5,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            0 => Some(TRGSEL_A::TRGSEL0),
            1 => Some(TRGSEL_A::TRGSEL1),
            2 => Some(TRGSEL_A::TRGSEL2),
            3 => Some(TRGSEL_A::TRGSEL3),
            4 => Some(TRGSEL_A::TRGSEL4),
            5 => Some(TRGSEL_A::TRGSEL5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL_A::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL_A::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL_A::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL_A::TRGSEL3
    }
    #[doc = "Checks if the value of the field is `TRGSEL4`"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == TRGSEL_A::TRGSEL4
    }
    #[doc = "Checks if the value of the field is `TRGSEL5`"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == TRGSEL_A::TRGSEL5
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, TRGSEL_A, 3, O>;
impl<'a, const O: u8> TRGSEL_W<'a, O> {
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL3)
    }
    #[doc = "PWM Event Line 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL4)
    }
    #[doc = "PWM Event Line 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL5)
    }
}
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WORD_R = crate::BitReader<WORD_A>;
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORD_A {
    #[doc = "0: Half-Word transfer"]
    HALF = 0,
    #[doc = "1: Word Transfer"]
    WORD = 1,
}
impl From<WORD_A> for bool {
    #[inline(always)]
    fn from(variant: WORD_A) -> Self {
        variant as u8 != 0
    }
}
impl WORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_A {
        match self.bits {
            false => WORD_A::HALF,
            true => WORD_A::WORD,
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == WORD_A::HALF
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == WORD_A::WORD
    }
}
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, WORD_A, O>;
impl<'a, const O: u8> WORD_W<'a, O> {
    #[doc = "Half-Word transfer"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(WORD_A::HALF)
    }
    #[doc = "Word Transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(WORD_A::WORD)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SLEEP_R = crate::BitReader<SLEEP_A>;
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal Mode: The DAC Core and reference voltage circuitry are kept ON between conversions."]
    DISABLED = 0,
    #[doc = "1: Sleep Mode: The DAC Core and/or reference voltage circuitry are OFF between conversions."]
    ENABLED = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::DISABLED,
            true => SLEEP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_A::ENABLED
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, SLEEP_A, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "Normal Mode: The DAC Core and reference voltage circuitry are kept ON between conversions."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEP_A::DISABLED)
    }
    #[doc = "Sleep Mode: The DAC Core and/or reference voltage circuitry are OFF between conversions."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLEEP_A::ENABLED)
    }
}
#[doc = "Field `FASTWKUP` reader - Fast Wake up Mode"]
pub type FASTWKUP_R = crate::BitReader<FASTWKUP_A>;
#[doc = "Fast Wake up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTWKUP_A {
    #[doc = "0: Normal Sleep Mode: The sleep mode is defined by the SLEEP bit. Voltage reference is OFF between conversions."]
    STAMODE = 0,
    #[doc = "1: Fast Wake up after Sleep Mode: the voltage reference is kept ON between conversions but DAC Core is OFF"]
    FASTWAKEUP = 1,
}
impl From<FASTWKUP_A> for bool {
    #[inline(always)]
    fn from(variant: FASTWKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl FASTWKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FASTWKUP_A {
        match self.bits {
            false => FASTWKUP_A::STAMODE,
            true => FASTWKUP_A::FASTWAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `STAMODE`"]
    #[inline(always)]
    pub fn is_stamode(&self) -> bool {
        *self == FASTWKUP_A::STAMODE
    }
    #[doc = "Checks if the value of the field is `FASTWAKEUP`"]
    #[inline(always)]
    pub fn is_fastwakeup(&self) -> bool {
        *self == FASTWKUP_A::FASTWAKEUP
    }
}
#[doc = "Field `FASTWKUP` writer - Fast Wake up Mode"]
pub type FASTWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, FASTWKUP_A, O>;
impl<'a, const O: u8> FASTWKUP_W<'a, O> {
    #[doc = "Normal Sleep Mode: The sleep mode is defined by the SLEEP bit. Voltage reference is OFF between conversions."]
    #[inline(always)]
    pub fn stamode(self) -> &'a mut W {
        self.variant(FASTWKUP_A::STAMODE)
    }
    #[doc = "Fast Wake up after Sleep Mode: the voltage reference is kept ON between conversions but DAC Core is OFF"]
    #[inline(always)]
    pub fn fastwakeup(self) -> &'a mut W {
        self.variant(FASTWKUP_A::FASTWAKEUP)
    }
}
#[doc = "Field `REFRESH` reader - Refresh Period"]
pub type REFRESH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESH` writer - Refresh Period"]
pub type REFRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
#[doc = "Field `USER_SEL` reader - User Channel Selection"]
pub type USER_SEL_R = crate::FieldReader<u8, USER_SEL_A>;
#[doc = "User Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USER_SEL_A {
    #[doc = "0: Channel 0"]
    CHANNEL0 = 0,
    #[doc = "1: Channel 1"]
    CHANNEL1 = 1,
}
impl From<USER_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USER_SEL_A) -> Self {
        variant as _
    }
}
impl USER_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USER_SEL_A> {
        match self.bits {
            0 => Some(USER_SEL_A::CHANNEL0),
            1 => Some(USER_SEL_A::CHANNEL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == USER_SEL_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USER_SEL_A::CHANNEL1
    }
}
#[doc = "Field `USER_SEL` writer - User Channel Selection"]
pub type USER_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, USER_SEL_A, 2, O>;
impl<'a, const O: u8> USER_SEL_W<'a, O> {
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(USER_SEL_A::CHANNEL0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(USER_SEL_A::CHANNEL1)
    }
}
#[doc = "Field `TAG` reader - Tag Selection Mode"]
pub type TAG_R = crate::BitReader<TAG_A>;
#[doc = "Tag Selection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAG_A {
    #[doc = "0: Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    DIS = 0,
    #[doc = "1: Tag selection mode enabled"]
    EN = 1,
}
impl From<TAG_A> for bool {
    #[inline(always)]
    fn from(variant: TAG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAG_A {
        match self.bits {
            false => TAG_A::DIS,
            true => TAG_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAG_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAG_A::EN
    }
}
#[doc = "Field `TAG` writer - Tag Selection Mode"]
pub type TAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TAG_A, O>;
impl<'a, const O: u8> TAG_W<'a, O> {
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAG_A::DIS)
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAG_A::EN)
    }
}
#[doc = "Field `MAXS` reader - Max Speed Mode"]
pub type MAXS_R = crate::BitReader<MAXS_A>;
#[doc = "Max Speed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS_A {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Max Speed Mode enabled"]
    MAXIMUM = 1,
}
impl From<MAXS_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS_A) -> Self {
        variant as u8 != 0
    }
}
impl MAXS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS_A {
        match self.bits {
            false => MAXS_A::NORMAL,
            true => MAXS_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MAXS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS_A::MAXIMUM
    }
}
#[doc = "Field `MAXS` writer - Max Speed Mode"]
pub type MAXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MAXS_A, O>;
impl<'a, const O: u8> MAXS_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MAXS_A::NORMAL)
    }
    #[doc = "Max Speed Mode enabled"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS_A::MAXIMUM)
    }
}
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub type STARTUP_R = crate::FieldReader<u8, STARTUP_A>;
#[doc = "Startup Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of DACClock"]
    _0 = 0,
    #[doc = "1: 8 periods of DACClock"]
    _8 = 1,
    #[doc = "2: 16 periods of DACClock"]
    _16 = 2,
    #[doc = "3: 24 periods of DACClock"]
    _24 = 3,
    #[doc = "4: 64 periods of DACClock"]
    _64 = 4,
    #[doc = "5: 80 periods of DACClock"]
    _80 = 5,
    #[doc = "6: 96 periods of DACClock"]
    _96 = 6,
    #[doc = "7: 112 periods of DACClock"]
    _112 = 7,
    #[doc = "8: 512 periods of DACClock"]
    _512 = 8,
    #[doc = "9: 576 periods of DACClock"]
    _576 = 9,
    #[doc = "10: 640 periods of DACClock"]
    _640 = 10,
    #[doc = "11: 704 periods of DACClock"]
    _704 = 11,
    #[doc = "12: 768 periods of DACClock"]
    _768 = 12,
    #[doc = "13: 832 periods of DACClock"]
    _832 = 13,
    #[doc = "14: 896 periods of DACClock"]
    _896 = 14,
    #[doc = "15: 960 periods of DACClock"]
    _960 = 15,
    #[doc = "16: 1024 periods of DACClock"]
    _1024 = 16,
    #[doc = "17: 1088 periods of DACClock"]
    _1088 = 17,
    #[doc = "18: 1152 periods of DACClock"]
    _1152 = 18,
    #[doc = "19: 1216 periods of DACClock"]
    _1216 = 19,
    #[doc = "20: 1280 periods of DACClock"]
    _1280 = 20,
    #[doc = "21: 1344 periods of DACClock"]
    _1344 = 21,
    #[doc = "22: 1408 periods of DACClock"]
    _1408 = 22,
    #[doc = "23: 1472 periods of DACClock"]
    _1472 = 23,
    #[doc = "24: 1536 periods of DACClock"]
    _1536 = 24,
    #[doc = "25: 1600 periods of DACClock"]
    _1600 = 25,
    #[doc = "26: 1664 periods of DACClock"]
    _1664 = 26,
    #[doc = "27: 1728 periods of DACClock"]
    _1728 = 27,
    #[doc = "28: 1792 periods of DACClock"]
    _1792 = 28,
    #[doc = "29: 1856 periods of DACClock"]
    _1856 = 29,
    #[doc = "30: 1920 periods of DACClock"]
    _1920 = 30,
    #[doc = "31: 1984 periods of DACClock"]
    _1984 = 31,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTUP_A> {
        match self.bits {
            0 => Some(STARTUP_A::_0),
            1 => Some(STARTUP_A::_8),
            2 => Some(STARTUP_A::_16),
            3 => Some(STARTUP_A::_24),
            4 => Some(STARTUP_A::_64),
            5 => Some(STARTUP_A::_80),
            6 => Some(STARTUP_A::_96),
            7 => Some(STARTUP_A::_112),
            8 => Some(STARTUP_A::_512),
            9 => Some(STARTUP_A::_576),
            10 => Some(STARTUP_A::_640),
            11 => Some(STARTUP_A::_704),
            12 => Some(STARTUP_A::_768),
            13 => Some(STARTUP_A::_832),
            14 => Some(STARTUP_A::_896),
            15 => Some(STARTUP_A::_960),
            16 => Some(STARTUP_A::_1024),
            17 => Some(STARTUP_A::_1088),
            18 => Some(STARTUP_A::_1152),
            19 => Some(STARTUP_A::_1216),
            20 => Some(STARTUP_A::_1280),
            21 => Some(STARTUP_A::_1344),
            22 => Some(STARTUP_A::_1408),
            23 => Some(STARTUP_A::_1472),
            24 => Some(STARTUP_A::_1536),
            25 => Some(STARTUP_A::_1600),
            26 => Some(STARTUP_A::_1664),
            27 => Some(STARTUP_A::_1728),
            28 => Some(STARTUP_A::_1792),
            29 => Some(STARTUP_A::_1856),
            30 => Some(STARTUP_A::_1920),
            31 => Some(STARTUP_A::_1984),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STARTUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == STARTUP_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == STARTUP_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == STARTUP_A::_24
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == STARTUP_A::_64
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == STARTUP_A::_80
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == STARTUP_A::_96
    }
    #[doc = "Checks if the value of the field is `_112`"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        *self == STARTUP_A::_112
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == STARTUP_A::_512
    }
    #[doc = "Checks if the value of the field is `_576`"]
    #[inline(always)]
    pub fn is_576(&self) -> bool {
        *self == STARTUP_A::_576
    }
    #[doc = "Checks if the value of the field is `_640`"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        *self == STARTUP_A::_640
    }
    #[doc = "Checks if the value of the field is `_704`"]
    #[inline(always)]
    pub fn is_704(&self) -> bool {
        *self == STARTUP_A::_704
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == STARTUP_A::_768
    }
    #[doc = "Checks if the value of the field is `_832`"]
    #[inline(always)]
    pub fn is_832(&self) -> bool {
        *self == STARTUP_A::_832
    }
    #[doc = "Checks if the value of the field is `_896`"]
    #[inline(always)]
    pub fn is_896(&self) -> bool {
        *self == STARTUP_A::_896
    }
    #[doc = "Checks if the value of the field is `_960`"]
    #[inline(always)]
    pub fn is_960(&self) -> bool {
        *self == STARTUP_A::_960
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == STARTUP_A::_1024
    }
    #[doc = "Checks if the value of the field is `_1088`"]
    #[inline(always)]
    pub fn is_1088(&self) -> bool {
        *self == STARTUP_A::_1088
    }
    #[doc = "Checks if the value of the field is `_1152`"]
    #[inline(always)]
    pub fn is_1152(&self) -> bool {
        *self == STARTUP_A::_1152
    }
    #[doc = "Checks if the value of the field is `_1216`"]
    #[inline(always)]
    pub fn is_1216(&self) -> bool {
        *self == STARTUP_A::_1216
    }
    #[doc = "Checks if the value of the field is `_1280`"]
    #[inline(always)]
    pub fn is_1280(&self) -> bool {
        *self == STARTUP_A::_1280
    }
    #[doc = "Checks if the value of the field is `_1344`"]
    #[inline(always)]
    pub fn is_1344(&self) -> bool {
        *self == STARTUP_A::_1344
    }
    #[doc = "Checks if the value of the field is `_1408`"]
    #[inline(always)]
    pub fn is_1408(&self) -> bool {
        *self == STARTUP_A::_1408
    }
    #[doc = "Checks if the value of the field is `_1472`"]
    #[inline(always)]
    pub fn is_1472(&self) -> bool {
        *self == STARTUP_A::_1472
    }
    #[doc = "Checks if the value of the field is `_1536`"]
    #[inline(always)]
    pub fn is_1536(&self) -> bool {
        *self == STARTUP_A::_1536
    }
    #[doc = "Checks if the value of the field is `_1600`"]
    #[inline(always)]
    pub fn is_1600(&self) -> bool {
        *self == STARTUP_A::_1600
    }
    #[doc = "Checks if the value of the field is `_1664`"]
    #[inline(always)]
    pub fn is_1664(&self) -> bool {
        *self == STARTUP_A::_1664
    }
    #[doc = "Checks if the value of the field is `_1728`"]
    #[inline(always)]
    pub fn is_1728(&self) -> bool {
        *self == STARTUP_A::_1728
    }
    #[doc = "Checks if the value of the field is `_1792`"]
    #[inline(always)]
    pub fn is_1792(&self) -> bool {
        *self == STARTUP_A::_1792
    }
    #[doc = "Checks if the value of the field is `_1856`"]
    #[inline(always)]
    pub fn is_1856(&self) -> bool {
        *self == STARTUP_A::_1856
    }
    #[doc = "Checks if the value of the field is `_1920`"]
    #[inline(always)]
    pub fn is_1920(&self) -> bool {
        *self == STARTUP_A::_1920
    }
    #[doc = "Checks if the value of the field is `_1984`"]
    #[inline(always)]
    pub fn is_1984(&self) -> bool {
        *self == STARTUP_A::_1984
    }
}
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, STARTUP_A, 6, O>;
impl<'a, const O: u8> STARTUP_W<'a, O> {
    #[doc = "0 periods of DACClock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTUP_A::_0)
    }
    #[doc = "8 periods of DACClock"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(STARTUP_A::_8)
    }
    #[doc = "16 periods of DACClock"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(STARTUP_A::_16)
    }
    #[doc = "24 periods of DACClock"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(STARTUP_A::_24)
    }
    #[doc = "64 periods of DACClock"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(STARTUP_A::_64)
    }
    #[doc = "80 periods of DACClock"]
    #[inline(always)]
    pub fn _80(self) -> &'a mut W {
        self.variant(STARTUP_A::_80)
    }
    #[doc = "96 periods of DACClock"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut W {
        self.variant(STARTUP_A::_96)
    }
    #[doc = "112 periods of DACClock"]
    #[inline(always)]
    pub fn _112(self) -> &'a mut W {
        self.variant(STARTUP_A::_112)
    }
    #[doc = "512 periods of DACClock"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(STARTUP_A::_512)
    }
    #[doc = "576 periods of DACClock"]
    #[inline(always)]
    pub fn _576(self) -> &'a mut W {
        self.variant(STARTUP_A::_576)
    }
    #[doc = "640 periods of DACClock"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut W {
        self.variant(STARTUP_A::_640)
    }
    #[doc = "704 periods of DACClock"]
    #[inline(always)]
    pub fn _704(self) -> &'a mut W {
        self.variant(STARTUP_A::_704)
    }
    #[doc = "768 periods of DACClock"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut W {
        self.variant(STARTUP_A::_768)
    }
    #[doc = "832 periods of DACClock"]
    #[inline(always)]
    pub fn _832(self) -> &'a mut W {
        self.variant(STARTUP_A::_832)
    }
    #[doc = "896 periods of DACClock"]
    #[inline(always)]
    pub fn _896(self) -> &'a mut W {
        self.variant(STARTUP_A::_896)
    }
    #[doc = "960 periods of DACClock"]
    #[inline(always)]
    pub fn _960(self) -> &'a mut W {
        self.variant(STARTUP_A::_960)
    }
    #[doc = "1024 periods of DACClock"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(STARTUP_A::_1024)
    }
    #[doc = "1088 periods of DACClock"]
    #[inline(always)]
    pub fn _1088(self) -> &'a mut W {
        self.variant(STARTUP_A::_1088)
    }
    #[doc = "1152 periods of DACClock"]
    #[inline(always)]
    pub fn _1152(self) -> &'a mut W {
        self.variant(STARTUP_A::_1152)
    }
    #[doc = "1216 periods of DACClock"]
    #[inline(always)]
    pub fn _1216(self) -> &'a mut W {
        self.variant(STARTUP_A::_1216)
    }
    #[doc = "1280 periods of DACClock"]
    #[inline(always)]
    pub fn _1280(self) -> &'a mut W {
        self.variant(STARTUP_A::_1280)
    }
    #[doc = "1344 periods of DACClock"]
    #[inline(always)]
    pub fn _1344(self) -> &'a mut W {
        self.variant(STARTUP_A::_1344)
    }
    #[doc = "1408 periods of DACClock"]
    #[inline(always)]
    pub fn _1408(self) -> &'a mut W {
        self.variant(STARTUP_A::_1408)
    }
    #[doc = "1472 periods of DACClock"]
    #[inline(always)]
    pub fn _1472(self) -> &'a mut W {
        self.variant(STARTUP_A::_1472)
    }
    #[doc = "1536 periods of DACClock"]
    #[inline(always)]
    pub fn _1536(self) -> &'a mut W {
        self.variant(STARTUP_A::_1536)
    }
    #[doc = "1600 periods of DACClock"]
    #[inline(always)]
    pub fn _1600(self) -> &'a mut W {
        self.variant(STARTUP_A::_1600)
    }
    #[doc = "1664 periods of DACClock"]
    #[inline(always)]
    pub fn _1664(self) -> &'a mut W {
        self.variant(STARTUP_A::_1664)
    }
    #[doc = "1728 periods of DACClock"]
    #[inline(always)]
    pub fn _1728(self) -> &'a mut W {
        self.variant(STARTUP_A::_1728)
    }
    #[doc = "1792 periods of DACClock"]
    #[inline(always)]
    pub fn _1792(self) -> &'a mut W {
        self.variant(STARTUP_A::_1792)
    }
    #[doc = "1856 periods of DACClock"]
    #[inline(always)]
    pub fn _1856(self) -> &'a mut W {
        self.variant(STARTUP_A::_1856)
    }
    #[doc = "1920 periods of DACClock"]
    #[inline(always)]
    pub fn _1920(self) -> &'a mut W {
        self.variant(STARTUP_A::_1920)
    }
    #[doc = "1984 periods of DACClock"]
    #[inline(always)]
    pub fn _1984(self) -> &'a mut W {
        self.variant(STARTUP_A::_1984)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    pub fn user_sel(&self) -> USER_SEL_R {
        USER_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline(always)]
    pub fn maxs(&self) -> MAXS_R {
        MAXS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&mut self) -> TRGEN_W<0> {
        TRGEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W<1> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W<4> {
        WORD_W::new(self)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<5> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    pub fn fastwkup(&mut self) -> FASTWKUP_W<6> {
        FASTWKUP_W::new(self)
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W<8> {
        REFRESH_W::new(self)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    pub fn user_sel(&mut self) -> USER_SEL_W<16> {
        USER_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W<20> {
        TAG_W::new(self)
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline(always)]
    pub fn maxs(&mut self) -> MAXS_W<21> {
        MAXS_W::new(self)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W<24> {
        STARTUP_W::new(self)
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
