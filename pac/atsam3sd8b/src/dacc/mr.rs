#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader<TRGEN_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger mode disabled. DACC in free running mode."]
    Dis = 0,
    #[doc = "1: External trigger mode enabled."]
    En = 1,
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
    pub const fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::Dis,
            true => TRGEN_A::En,
        }
    }
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN_A::Dis
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN_A::En
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, REG> = crate::BitWriter<'a, REG, TRGEN_A>;
impl<'a, REG> TRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TRGEN_A::Dis)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TRGEN_A::En)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader;
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WORD_R = crate::BitReader<WORD_A>;
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WORD_A {
    #[doc = "0: Half-Word transfer"]
    Half = 0,
    #[doc = "1: Word Transfer"]
    Word = 1,
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
    pub const fn variant(&self) -> WORD_A {
        match self.bits {
            false => WORD_A::Half,
            true => WORD_A::Word,
        }
    }
    #[doc = "Half-Word transfer"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == WORD_A::Half
    }
    #[doc = "Word Transfer"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == WORD_A::Word
    }
}
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WORD_W<'a, REG> = crate::BitWriter<'a, REG, WORD_A>;
impl<'a, REG> WORD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-Word transfer"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_A::Half)
    }
    #[doc = "Word Transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_A::Word)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTWKUP` reader - Fast Wake up Mode"]
pub type FASTWKUP_R = crate::BitReader;
#[doc = "Field `FASTWKUP` writer - Fast Wake up Mode"]
pub type FASTWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFRESH` reader - Refresh Period"]
pub type REFRESH_R = crate::FieldReader;
#[doc = "Field `REFRESH` writer - Refresh Period"]
pub type REFRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USER_SEL` reader - User Channel Selection"]
pub type USER_SEL_R = crate::FieldReader<USER_SEL_A>;
#[doc = "User Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USER_SEL_A {
    #[doc = "0: Channel 0"]
    Channel0 = 0,
    #[doc = "1: Channel 1"]
    Channel1 = 1,
}
impl From<USER_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USER_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USER_SEL_A {
    type Ux = u8;
}
impl USER_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USER_SEL_A> {
        match self.bits {
            0 => Some(USER_SEL_A::Channel0),
            1 => Some(USER_SEL_A::Channel1),
            _ => None,
        }
    }
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == USER_SEL_A::Channel0
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USER_SEL_A::Channel1
    }
}
#[doc = "Field `USER_SEL` writer - User Channel Selection"]
pub type USER_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USER_SEL_A>;
impl<'a, REG> USER_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(USER_SEL_A::Channel0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(USER_SEL_A::Channel1)
    }
}
#[doc = "Field `TAG` reader - Tag Selection Mode"]
pub type TAG_R = crate::BitReader<TAG_A>;
#[doc = "Tag Selection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAG_A {
    #[doc = "0: Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    Dis = 0,
    #[doc = "1: Tag selection mode enabled"]
    En = 1,
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
    pub const fn variant(&self) -> TAG_A {
        match self.bits {
            false => TAG_A::Dis,
            true => TAG_A::En,
        }
    }
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAG_A::Dis
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAG_A::En
    }
}
#[doc = "Field `TAG` writer - Tag Selection Mode"]
pub type TAG_W<'a, REG> = crate::BitWriter<'a, REG, TAG_A>;
impl<'a, REG> TAG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TAG_A::Dis)
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TAG_A::En)
    }
}
#[doc = "Field `MAXS` reader - Max Speed Mode"]
pub type MAXS_R = crate::BitReader<MAXS_A>;
#[doc = "Max Speed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXS_A {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Max Speed Mode enabled"]
    Maximum = 1,
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
    pub const fn variant(&self) -> MAXS_A {
        match self.bits {
            false => MAXS_A::Normal,
            true => MAXS_A::Maximum,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MAXS_A::Normal
    }
    #[doc = "Max Speed Mode enabled"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS_A::Maximum
    }
}
#[doc = "Field `MAXS` writer - Max Speed Mode"]
pub type MAXS_W<'a, REG> = crate::BitWriter<'a, REG, MAXS_A>;
impl<'a, REG> MAXS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MAXS_A::Normal)
    }
    #[doc = "Max Speed Mode enabled"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(MAXS_A::Maximum)
    }
}
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub type STARTUP_R = crate::FieldReader<STARTUP_A>;
#[doc = "Startup Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for STARTUP_A {
    type Ux = u8;
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STARTUP_A> {
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
    #[doc = "0 periods of DACClock"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STARTUP_A::_0
    }
    #[doc = "8 periods of DACClock"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == STARTUP_A::_8
    }
    #[doc = "16 periods of DACClock"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == STARTUP_A::_16
    }
    #[doc = "24 periods of DACClock"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == STARTUP_A::_24
    }
    #[doc = "64 periods of DACClock"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == STARTUP_A::_64
    }
    #[doc = "80 periods of DACClock"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == STARTUP_A::_80
    }
    #[doc = "96 periods of DACClock"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == STARTUP_A::_96
    }
    #[doc = "112 periods of DACClock"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        *self == STARTUP_A::_112
    }
    #[doc = "512 periods of DACClock"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == STARTUP_A::_512
    }
    #[doc = "576 periods of DACClock"]
    #[inline(always)]
    pub fn is_576(&self) -> bool {
        *self == STARTUP_A::_576
    }
    #[doc = "640 periods of DACClock"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        *self == STARTUP_A::_640
    }
    #[doc = "704 periods of DACClock"]
    #[inline(always)]
    pub fn is_704(&self) -> bool {
        *self == STARTUP_A::_704
    }
    #[doc = "768 periods of DACClock"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == STARTUP_A::_768
    }
    #[doc = "832 periods of DACClock"]
    #[inline(always)]
    pub fn is_832(&self) -> bool {
        *self == STARTUP_A::_832
    }
    #[doc = "896 periods of DACClock"]
    #[inline(always)]
    pub fn is_896(&self) -> bool {
        *self == STARTUP_A::_896
    }
    #[doc = "960 periods of DACClock"]
    #[inline(always)]
    pub fn is_960(&self) -> bool {
        *self == STARTUP_A::_960
    }
    #[doc = "1024 periods of DACClock"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == STARTUP_A::_1024
    }
    #[doc = "1088 periods of DACClock"]
    #[inline(always)]
    pub fn is_1088(&self) -> bool {
        *self == STARTUP_A::_1088
    }
    #[doc = "1152 periods of DACClock"]
    #[inline(always)]
    pub fn is_1152(&self) -> bool {
        *self == STARTUP_A::_1152
    }
    #[doc = "1216 periods of DACClock"]
    #[inline(always)]
    pub fn is_1216(&self) -> bool {
        *self == STARTUP_A::_1216
    }
    #[doc = "1280 periods of DACClock"]
    #[inline(always)]
    pub fn is_1280(&self) -> bool {
        *self == STARTUP_A::_1280
    }
    #[doc = "1344 periods of DACClock"]
    #[inline(always)]
    pub fn is_1344(&self) -> bool {
        *self == STARTUP_A::_1344
    }
    #[doc = "1408 periods of DACClock"]
    #[inline(always)]
    pub fn is_1408(&self) -> bool {
        *self == STARTUP_A::_1408
    }
    #[doc = "1472 periods of DACClock"]
    #[inline(always)]
    pub fn is_1472(&self) -> bool {
        *self == STARTUP_A::_1472
    }
    #[doc = "1536 periods of DACClock"]
    #[inline(always)]
    pub fn is_1536(&self) -> bool {
        *self == STARTUP_A::_1536
    }
    #[doc = "1600 periods of DACClock"]
    #[inline(always)]
    pub fn is_1600(&self) -> bool {
        *self == STARTUP_A::_1600
    }
    #[doc = "1664 periods of DACClock"]
    #[inline(always)]
    pub fn is_1664(&self) -> bool {
        *self == STARTUP_A::_1664
    }
    #[doc = "1728 periods of DACClock"]
    #[inline(always)]
    pub fn is_1728(&self) -> bool {
        *self == STARTUP_A::_1728
    }
    #[doc = "1792 periods of DACClock"]
    #[inline(always)]
    pub fn is_1792(&self) -> bool {
        *self == STARTUP_A::_1792
    }
    #[doc = "1856 periods of DACClock"]
    #[inline(always)]
    pub fn is_1856(&self) -> bool {
        *self == STARTUP_A::_1856
    }
    #[doc = "1920 periods of DACClock"]
    #[inline(always)]
    pub fn is_1920(&self) -> bool {
        *self == STARTUP_A::_1920
    }
    #[doc = "1984 periods of DACClock"]
    #[inline(always)]
    pub fn is_1984(&self) -> bool {
        *self == STARTUP_A::_1984
    }
}
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub type STARTUP_W<'a, REG> = crate::FieldWriter<'a, REG, 6, STARTUP_A>;
impl<'a, REG> STARTUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 periods of DACClock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_0)
    }
    #[doc = "8 periods of DACClock"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_8)
    }
    #[doc = "16 periods of DACClock"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_16)
    }
    #[doc = "24 periods of DACClock"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_24)
    }
    #[doc = "64 periods of DACClock"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_64)
    }
    #[doc = "80 periods of DACClock"]
    #[inline(always)]
    pub fn _80(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_80)
    }
    #[doc = "96 periods of DACClock"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_96)
    }
    #[doc = "112 periods of DACClock"]
    #[inline(always)]
    pub fn _112(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_112)
    }
    #[doc = "512 periods of DACClock"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_512)
    }
    #[doc = "576 periods of DACClock"]
    #[inline(always)]
    pub fn _576(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_576)
    }
    #[doc = "640 periods of DACClock"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_640)
    }
    #[doc = "704 periods of DACClock"]
    #[inline(always)]
    pub fn _704(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_704)
    }
    #[doc = "768 periods of DACClock"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_768)
    }
    #[doc = "832 periods of DACClock"]
    #[inline(always)]
    pub fn _832(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_832)
    }
    #[doc = "896 periods of DACClock"]
    #[inline(always)]
    pub fn _896(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_896)
    }
    #[doc = "960 periods of DACClock"]
    #[inline(always)]
    pub fn _960(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_960)
    }
    #[doc = "1024 periods of DACClock"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1024)
    }
    #[doc = "1088 periods of DACClock"]
    #[inline(always)]
    pub fn _1088(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1088)
    }
    #[doc = "1152 periods of DACClock"]
    #[inline(always)]
    pub fn _1152(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1152)
    }
    #[doc = "1216 periods of DACClock"]
    #[inline(always)]
    pub fn _1216(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1216)
    }
    #[doc = "1280 periods of DACClock"]
    #[inline(always)]
    pub fn _1280(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1280)
    }
    #[doc = "1344 periods of DACClock"]
    #[inline(always)]
    pub fn _1344(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1344)
    }
    #[doc = "1408 periods of DACClock"]
    #[inline(always)]
    pub fn _1408(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1408)
    }
    #[doc = "1472 periods of DACClock"]
    #[inline(always)]
    pub fn _1472(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1472)
    }
    #[doc = "1536 periods of DACClock"]
    #[inline(always)]
    pub fn _1536(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1536)
    }
    #[doc = "1600 periods of DACClock"]
    #[inline(always)]
    pub fn _1600(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1600)
    }
    #[doc = "1664 periods of DACClock"]
    #[inline(always)]
    pub fn _1664(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1664)
    }
    #[doc = "1728 periods of DACClock"]
    #[inline(always)]
    pub fn _1728(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1728)
    }
    #[doc = "1792 periods of DACClock"]
    #[inline(always)]
    pub fn _1792(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1792)
    }
    #[doc = "1856 periods of DACClock"]
    #[inline(always)]
    pub fn _1856(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1856)
    }
    #[doc = "1920 periods of DACClock"]
    #[inline(always)]
    pub fn _1920(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::_1920)
    }
    #[doc = "1984 periods of DACClock"]
    #[inline(always)]
    pub fn _1984(self) -> &'a mut crate::W<REG> {
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
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<MR_SPEC> {
        TRGEN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<MR_SPEC> {
        TRGSEL_W::new(self, 1)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WORD_W<MR_SPEC> {
        WORD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<MR_SPEC> {
        SLEEP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fastwkup(&mut self) -> FASTWKUP_W<MR_SPEC> {
        FASTWKUP_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<MR_SPEC> {
        REFRESH_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn user_sel(&mut self) -> USER_SEL_W<MR_SPEC> {
        USER_SEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<MR_SPEC> {
        TAG_W::new(self, 20)
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn maxs(&mut self) -> MAXS_W<MR_SPEC> {
        MAXS_W::new(self, 21)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<MR_SPEC> {
        STARTUP_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: u32 = 0;
}
