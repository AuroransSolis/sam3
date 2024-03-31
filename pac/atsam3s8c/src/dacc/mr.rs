#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgen {
    #[doc = "0: External trigger mode disabled. DACC in free running mode."]
    Dis = 0,
    #[doc = "1: External trigger mode enabled."]
    En = 1,
}
impl From<Trgen> for bool {
    #[inline(always)]
    fn from(variant: Trgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TrgenR = crate::BitReader<Trgen>;
impl TrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgen {
        match self.bits {
            false => Trgen::Dis,
            true => Trgen::En,
        }
    }
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgen::Dis
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Trgen::En
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TrgenW<'a, REG> = crate::BitWriter<'a, REG, Trgen>;
impl<'a, REG> TrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen::Dis)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen::En)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TrgselR = crate::FieldReader;
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Word {
    #[doc = "0: Half-Word transfer"]
    Half = 0,
    #[doc = "1: Word Transfer"]
    Word = 1,
}
impl From<Word> for bool {
    #[inline(always)]
    fn from(variant: Word) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WordR = crate::BitReader<Word>;
impl WordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Word {
        match self.bits {
            false => Word::Half,
            true => Word::Word,
        }
    }
    #[doc = "Half-Word transfer"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Word::Half
    }
    #[doc = "Word Transfer"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Word::Word
    }
}
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WordW<'a, REG> = crate::BitWriter<'a, REG, Word>;
impl<'a, REG> WordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-Word transfer"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Word::Half)
    }
    #[doc = "Word Transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Word::Word)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTWKUP` reader - Fast Wake up Mode"]
pub type FastwkupR = crate::BitReader;
#[doc = "Field `FASTWKUP` writer - Fast Wake up Mode"]
pub type FastwkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFRESH` reader - Refresh Period"]
pub type RefreshR = crate::FieldReader;
#[doc = "Field `REFRESH` writer - Refresh Period"]
pub type RefreshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "User Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UserSel {
    #[doc = "0: Channel 0"]
    Channel0 = 0,
    #[doc = "1: Channel 1"]
    Channel1 = 1,
}
impl From<UserSel> for u8 {
    #[inline(always)]
    fn from(variant: UserSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UserSel {
    type Ux = u8;
}
impl crate::IsEnum for UserSel {}
#[doc = "Field `USER_SEL` reader - User Channel Selection"]
pub type UserSelR = crate::FieldReader<UserSel>;
impl UserSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UserSel> {
        match self.bits {
            0 => Some(UserSel::Channel0),
            1 => Some(UserSel::Channel1),
            _ => None,
        }
    }
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == UserSel::Channel0
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == UserSel::Channel1
    }
}
#[doc = "Field `USER_SEL` writer - User Channel Selection"]
pub type UserSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, UserSel>;
impl<'a, REG> UserSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(UserSel::Channel0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(UserSel::Channel1)
    }
}
#[doc = "Tag Selection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag {
    #[doc = "0: Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    Dis = 0,
    #[doc = "1: Tag selection mode enabled"]
    En = 1,
}
impl From<Tag> for bool {
    #[inline(always)]
    fn from(variant: Tag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAG` reader - Tag Selection Mode"]
pub type TagR = crate::BitReader<Tag>;
impl TagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tag {
        match self.bits {
            false => Tag::Dis,
            true => Tag::En,
        }
    }
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tag::Dis
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tag::En
    }
}
#[doc = "Field `TAG` writer - Tag Selection Mode"]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG, Tag>;
impl<'a, REG> TagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tag::Dis)
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tag::En)
    }
}
#[doc = "Max Speed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxs {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Max Speed Mode enabled"]
    Maximum = 1,
}
impl From<Maxs> for bool {
    #[inline(always)]
    fn from(variant: Maxs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS` reader - Max Speed Mode"]
pub type MaxsR = crate::BitReader<Maxs>;
impl MaxsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxs {
        match self.bits {
            false => Maxs::Normal,
            true => Maxs::Maximum,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Maxs::Normal
    }
    #[doc = "Max Speed Mode enabled"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Maxs::Maximum
    }
}
#[doc = "Field `MAXS` writer - Max Speed Mode"]
pub type MaxsW<'a, REG> = crate::BitWriter<'a, REG, Maxs>;
impl<'a, REG> MaxsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Maxs::Normal)
    }
    #[doc = "Max Speed Mode enabled"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Maxs::Maximum)
    }
}
#[doc = "Startup Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startup {
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
impl From<Startup> for u8 {
    #[inline(always)]
    fn from(variant: Startup) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startup {
    type Ux = u8;
}
impl crate::IsEnum for Startup {}
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub type StartupR = crate::FieldReader<Startup>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startup> {
        match self.bits {
            0 => Some(Startup::_0),
            1 => Some(Startup::_8),
            2 => Some(Startup::_16),
            3 => Some(Startup::_24),
            4 => Some(Startup::_64),
            5 => Some(Startup::_80),
            6 => Some(Startup::_96),
            7 => Some(Startup::_112),
            8 => Some(Startup::_512),
            9 => Some(Startup::_576),
            10 => Some(Startup::_640),
            11 => Some(Startup::_704),
            12 => Some(Startup::_768),
            13 => Some(Startup::_832),
            14 => Some(Startup::_896),
            15 => Some(Startup::_960),
            16 => Some(Startup::_1024),
            17 => Some(Startup::_1088),
            18 => Some(Startup::_1152),
            19 => Some(Startup::_1216),
            20 => Some(Startup::_1280),
            21 => Some(Startup::_1344),
            22 => Some(Startup::_1408),
            23 => Some(Startup::_1472),
            24 => Some(Startup::_1536),
            25 => Some(Startup::_1600),
            26 => Some(Startup::_1664),
            27 => Some(Startup::_1728),
            28 => Some(Startup::_1792),
            29 => Some(Startup::_1856),
            30 => Some(Startup::_1920),
            31 => Some(Startup::_1984),
            _ => None,
        }
    }
    #[doc = "0 periods of DACClock"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Startup::_0
    }
    #[doc = "8 periods of DACClock"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Startup::_8
    }
    #[doc = "16 periods of DACClock"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Startup::_16
    }
    #[doc = "24 periods of DACClock"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == Startup::_24
    }
    #[doc = "64 periods of DACClock"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Startup::_64
    }
    #[doc = "80 periods of DACClock"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == Startup::_80
    }
    #[doc = "96 periods of DACClock"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == Startup::_96
    }
    #[doc = "112 periods of DACClock"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        *self == Startup::_112
    }
    #[doc = "512 periods of DACClock"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Startup::_512
    }
    #[doc = "576 periods of DACClock"]
    #[inline(always)]
    pub fn is_576(&self) -> bool {
        *self == Startup::_576
    }
    #[doc = "640 periods of DACClock"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        *self == Startup::_640
    }
    #[doc = "704 periods of DACClock"]
    #[inline(always)]
    pub fn is_704(&self) -> bool {
        *self == Startup::_704
    }
    #[doc = "768 periods of DACClock"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == Startup::_768
    }
    #[doc = "832 periods of DACClock"]
    #[inline(always)]
    pub fn is_832(&self) -> bool {
        *self == Startup::_832
    }
    #[doc = "896 periods of DACClock"]
    #[inline(always)]
    pub fn is_896(&self) -> bool {
        *self == Startup::_896
    }
    #[doc = "960 periods of DACClock"]
    #[inline(always)]
    pub fn is_960(&self) -> bool {
        *self == Startup::_960
    }
    #[doc = "1024 periods of DACClock"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Startup::_1024
    }
    #[doc = "1088 periods of DACClock"]
    #[inline(always)]
    pub fn is_1088(&self) -> bool {
        *self == Startup::_1088
    }
    #[doc = "1152 periods of DACClock"]
    #[inline(always)]
    pub fn is_1152(&self) -> bool {
        *self == Startup::_1152
    }
    #[doc = "1216 periods of DACClock"]
    #[inline(always)]
    pub fn is_1216(&self) -> bool {
        *self == Startup::_1216
    }
    #[doc = "1280 periods of DACClock"]
    #[inline(always)]
    pub fn is_1280(&self) -> bool {
        *self == Startup::_1280
    }
    #[doc = "1344 periods of DACClock"]
    #[inline(always)]
    pub fn is_1344(&self) -> bool {
        *self == Startup::_1344
    }
    #[doc = "1408 periods of DACClock"]
    #[inline(always)]
    pub fn is_1408(&self) -> bool {
        *self == Startup::_1408
    }
    #[doc = "1472 periods of DACClock"]
    #[inline(always)]
    pub fn is_1472(&self) -> bool {
        *self == Startup::_1472
    }
    #[doc = "1536 periods of DACClock"]
    #[inline(always)]
    pub fn is_1536(&self) -> bool {
        *self == Startup::_1536
    }
    #[doc = "1600 periods of DACClock"]
    #[inline(always)]
    pub fn is_1600(&self) -> bool {
        *self == Startup::_1600
    }
    #[doc = "1664 periods of DACClock"]
    #[inline(always)]
    pub fn is_1664(&self) -> bool {
        *self == Startup::_1664
    }
    #[doc = "1728 periods of DACClock"]
    #[inline(always)]
    pub fn is_1728(&self) -> bool {
        *self == Startup::_1728
    }
    #[doc = "1792 periods of DACClock"]
    #[inline(always)]
    pub fn is_1792(&self) -> bool {
        *self == Startup::_1792
    }
    #[doc = "1856 periods of DACClock"]
    #[inline(always)]
    pub fn is_1856(&self) -> bool {
        *self == Startup::_1856
    }
    #[doc = "1920 periods of DACClock"]
    #[inline(always)]
    pub fn is_1920(&self) -> bool {
        *self == Startup::_1920
    }
    #[doc = "1984 periods of DACClock"]
    #[inline(always)]
    pub fn is_1984(&self) -> bool {
        *self == Startup::_1984
    }
}
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 6, Startup>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 periods of DACClock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_0)
    }
    #[doc = "8 periods of DACClock"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_8)
    }
    #[doc = "16 periods of DACClock"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_16)
    }
    #[doc = "24 periods of DACClock"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_24)
    }
    #[doc = "64 periods of DACClock"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_64)
    }
    #[doc = "80 periods of DACClock"]
    #[inline(always)]
    pub fn _80(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_80)
    }
    #[doc = "96 periods of DACClock"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_96)
    }
    #[doc = "112 periods of DACClock"]
    #[inline(always)]
    pub fn _112(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_112)
    }
    #[doc = "512 periods of DACClock"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_512)
    }
    #[doc = "576 periods of DACClock"]
    #[inline(always)]
    pub fn _576(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_576)
    }
    #[doc = "640 periods of DACClock"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_640)
    }
    #[doc = "704 periods of DACClock"]
    #[inline(always)]
    pub fn _704(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_704)
    }
    #[doc = "768 periods of DACClock"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_768)
    }
    #[doc = "832 periods of DACClock"]
    #[inline(always)]
    pub fn _832(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_832)
    }
    #[doc = "896 periods of DACClock"]
    #[inline(always)]
    pub fn _896(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_896)
    }
    #[doc = "960 periods of DACClock"]
    #[inline(always)]
    pub fn _960(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_960)
    }
    #[doc = "1024 periods of DACClock"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1024)
    }
    #[doc = "1088 periods of DACClock"]
    #[inline(always)]
    pub fn _1088(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1088)
    }
    #[doc = "1152 periods of DACClock"]
    #[inline(always)]
    pub fn _1152(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1152)
    }
    #[doc = "1216 periods of DACClock"]
    #[inline(always)]
    pub fn _1216(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1216)
    }
    #[doc = "1280 periods of DACClock"]
    #[inline(always)]
    pub fn _1280(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1280)
    }
    #[doc = "1344 periods of DACClock"]
    #[inline(always)]
    pub fn _1344(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1344)
    }
    #[doc = "1408 periods of DACClock"]
    #[inline(always)]
    pub fn _1408(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1408)
    }
    #[doc = "1472 periods of DACClock"]
    #[inline(always)]
    pub fn _1472(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1472)
    }
    #[doc = "1536 periods of DACClock"]
    #[inline(always)]
    pub fn _1536(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1536)
    }
    #[doc = "1600 periods of DACClock"]
    #[inline(always)]
    pub fn _1600(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1600)
    }
    #[doc = "1664 periods of DACClock"]
    #[inline(always)]
    pub fn _1664(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1664)
    }
    #[doc = "1728 periods of DACClock"]
    #[inline(always)]
    pub fn _1728(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1728)
    }
    #[doc = "1792 periods of DACClock"]
    #[inline(always)]
    pub fn _1792(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1792)
    }
    #[doc = "1856 periods of DACClock"]
    #[inline(always)]
    pub fn _1856(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1856)
    }
    #[doc = "1920 periods of DACClock"]
    #[inline(always)]
    pub fn _1920(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1920)
    }
    #[doc = "1984 periods of DACClock"]
    #[inline(always)]
    pub fn _1984(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::_1984)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TrgenR {
        TrgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WordR {
        WordR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FastwkupR {
        FastwkupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    pub fn refresh(&self) -> RefreshR {
        RefreshR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    pub fn user_sel(&self) -> UserSelR {
        UserSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline(always)]
    pub fn maxs(&self) -> MaxsR {
        MaxsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TrgenW<MrSpec> {
        TrgenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<MrSpec> {
        TrgselW::new(self, 1)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WordW<MrSpec> {
        WordW::new(self, 4)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<MrSpec> {
        SleepW::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fastwkup(&mut self) -> FastwkupW<MrSpec> {
        FastwkupW::new(self, 6)
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> RefreshW<MrSpec> {
        RefreshW::new(self, 8)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn user_sel(&mut self) -> UserSelW<MrSpec> {
        UserSelW::new(self, 16)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<MrSpec> {
        TagW::new(self, 20)
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn maxs(&mut self) -> MaxsW<MrSpec> {
        MaxsW::new(self, 21)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<MrSpec> {
        StartupW::new(self, 24)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
