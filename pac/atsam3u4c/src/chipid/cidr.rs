#[doc = "Register `CIDR` reader"]
pub type R = crate::R<CIDR_SPEC>;
#[doc = "Field `VERSION` reader - Version of the Device"]
pub type VERSION_R = crate::FieldReader;
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EPROC_R = crate::FieldReader<EPROC_A>;
#[doc = "Embedded Processor"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPROC_A {
    #[doc = "1: ARM946ES"]
    Arm946es = 1,
    #[doc = "2: ARM7TDMI"]
    Arm7tdmi = 2,
    #[doc = "3: Cortex-M3"]
    Cm3 = 3,
    #[doc = "4: ARM920T"]
    Arm920t = 4,
    #[doc = "5: ARM926EJS"]
    Arm926ejs = 5,
    #[doc = "6: Cortex-A5"]
    Ca5 = 6,
    #[doc = "7: Cortex-M4"]
    Cm4 = 7,
}
impl From<EPROC_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPROC_A {
    type Ux = u8;
}
impl EPROC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EPROC_A> {
        match self.bits {
            1 => Some(EPROC_A::Arm946es),
            2 => Some(EPROC_A::Arm7tdmi),
            3 => Some(EPROC_A::Cm3),
            4 => Some(EPROC_A::Arm920t),
            5 => Some(EPROC_A::Arm926ejs),
            6 => Some(EPROC_A::Ca5),
            7 => Some(EPROC_A::Cm4),
            _ => None,
        }
    }
    #[doc = "ARM946ES"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROC_A::Arm946es
    }
    #[doc = "ARM7TDMI"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROC_A::Arm7tdmi
    }
    #[doc = "Cortex-M3"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == EPROC_A::Cm3
    }
    #[doc = "ARM920T"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROC_A::Arm920t
    }
    #[doc = "ARM926EJS"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROC_A::Arm926ejs
    }
    #[doc = "Cortex-A5"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == EPROC_A::Ca5
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == EPROC_A::Cm4
    }
}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NVPSIZ_R = crate::FieldReader<NVPSIZ_A>;
#[doc = "Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ_A {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: 8 Kbytes"]
    _8k = 1,
    #[doc = "2: 16 Kbytes"]
    _16k = 2,
    #[doc = "3: 32 Kbytes"]
    _32k = 3,
    #[doc = "5: 64 Kbytes"]
    _64k = 5,
    #[doc = "7: 128 Kbytes"]
    _128k = 7,
    #[doc = "9: 256 Kbytes"]
    _256k = 9,
    #[doc = "10: 512 Kbytes"]
    _512k = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024k = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048k = 14,
}
impl From<NVPSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPSIZ_A {
    type Ux = u8;
}
impl NVPSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NVPSIZ_A> {
        match self.bits {
            0 => Some(NVPSIZ_A::None),
            1 => Some(NVPSIZ_A::_8k),
            2 => Some(NVPSIZ_A::_16k),
            3 => Some(NVPSIZ_A::_32k),
            5 => Some(NVPSIZ_A::_64k),
            7 => Some(NVPSIZ_A::_128k),
            9 => Some(NVPSIZ_A::_256k),
            10 => Some(NVPSIZ_A::_512k),
            12 => Some(NVPSIZ_A::_1024k),
            14 => Some(NVPSIZ_A::_2048k),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ_A::None
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ_A::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ_A::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ_A::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ_A::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ_A::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ_A::_256k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ_A::_512k
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ_A::_1024k
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ_A::_2048k
    }
}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type NVPSIZ2_R = crate::FieldReader<NVPSIZ2_A>;
#[doc = "Second Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ2_A {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: 8 Kbytes"]
    _8k = 1,
    #[doc = "2: 16 Kbytes"]
    _16k = 2,
    #[doc = "3: 32 Kbytes"]
    _32k = 3,
    #[doc = "5: 64 Kbytes"]
    _64k = 5,
    #[doc = "7: 128 Kbytes"]
    _128k = 7,
    #[doc = "9: 256 Kbytes"]
    _256k = 9,
    #[doc = "10: 512 Kbytes"]
    _512k = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024k = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048k = 14,
}
impl From<NVPSIZ2_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPSIZ2_A {
    type Ux = u8;
}
impl NVPSIZ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NVPSIZ2_A> {
        match self.bits {
            0 => Some(NVPSIZ2_A::None),
            1 => Some(NVPSIZ2_A::_8k),
            2 => Some(NVPSIZ2_A::_16k),
            3 => Some(NVPSIZ2_A::_32k),
            5 => Some(NVPSIZ2_A::_64k),
            7 => Some(NVPSIZ2_A::_128k),
            9 => Some(NVPSIZ2_A::_256k),
            10 => Some(NVPSIZ2_A::_512k),
            12 => Some(NVPSIZ2_A::_1024k),
            14 => Some(NVPSIZ2_A::_2048k),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2_A::None
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2_A::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2_A::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2_A::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2_A::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2_A::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2_A::_256k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2_A::_512k
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2_A::_1024k
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2_A::_2048k
    }
}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SRAMSIZ_R = crate::FieldReader<SRAMSIZ_A>;
#[doc = "Internal SRAM Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAMSIZ_A {
    #[doc = "0: 48 Kbytes"]
    _48k = 0,
    #[doc = "1: 192 Kbytes"]
    _192k = 1,
    #[doc = "2: 2 Kbytes"]
    _2k = 2,
    #[doc = "3: 6 Kbytes"]
    _6k = 3,
    #[doc = "4: 24 Kbytes"]
    _24k = 4,
    #[doc = "5: 4 Kbytes"]
    _4k = 5,
    #[doc = "6: 80 Kbytes"]
    _80k = 6,
    #[doc = "7: 160 Kbytes"]
    _160k = 7,
    #[doc = "8: 8 Kbytes"]
    _8k = 8,
    #[doc = "9: 16 Kbytes"]
    _16k = 9,
    #[doc = "10: 32 Kbytes"]
    _32k = 10,
    #[doc = "11: 64 Kbytes"]
    _64k = 11,
    #[doc = "12: 128 Kbytes"]
    _128k = 12,
    #[doc = "13: 256 Kbytes"]
    _256k = 13,
    #[doc = "14: 96 Kbytes"]
    _96k = 14,
    #[doc = "15: 512 Kbytes"]
    _512k = 15,
}
impl From<SRAMSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRAMSIZ_A {
    type Ux = u8;
}
impl SRAMSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAMSIZ_A {
        match self.bits {
            0 => SRAMSIZ_A::_48k,
            1 => SRAMSIZ_A::_192k,
            2 => SRAMSIZ_A::_2k,
            3 => SRAMSIZ_A::_6k,
            4 => SRAMSIZ_A::_24k,
            5 => SRAMSIZ_A::_4k,
            6 => SRAMSIZ_A::_80k,
            7 => SRAMSIZ_A::_160k,
            8 => SRAMSIZ_A::_8k,
            9 => SRAMSIZ_A::_16k,
            10 => SRAMSIZ_A::_32k,
            11 => SRAMSIZ_A::_64k,
            12 => SRAMSIZ_A::_128k,
            13 => SRAMSIZ_A::_256k,
            14 => SRAMSIZ_A::_96k,
            15 => SRAMSIZ_A::_512k,
            _ => unreachable!(),
        }
    }
    #[doc = "48 Kbytes"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZ_A::_48k
    }
    #[doc = "192 Kbytes"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZ_A::_192k
    }
    #[doc = "2 Kbytes"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == SRAMSIZ_A::_2k
    }
    #[doc = "6 Kbytes"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZ_A::_6k
    }
    #[doc = "24 Kbytes"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZ_A::_24k
    }
    #[doc = "4 Kbytes"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZ_A::_4k
    }
    #[doc = "80 Kbytes"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZ_A::_80k
    }
    #[doc = "160 Kbytes"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZ_A::_160k
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZ_A::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZ_A::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZ_A::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZ_A::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZ_A::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZ_A::_256k
    }
    #[doc = "96 Kbytes"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZ_A::_96k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZ_A::_512k
    }
}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ARCH_R = crate::FieldReader<ARCH_A>;
#[doc = "Architecture Identifier"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARCH_A {
    #[doc = "25: AT91SAM9xx Series"]
    At91sam9xx = 25,
    #[doc = "41: AT91SAM9XExx Series"]
    At91sam9xexx = 41,
    #[doc = "52: AT91x34 Series"]
    At91x34 = 52,
    #[doc = "55: CAP7 Series"]
    Cap7 = 55,
    #[doc = "57: CAP9 Series"]
    Cap9 = 57,
    #[doc = "59: CAP11 Series"]
    Cap11 = 59,
    #[doc = "64: AT91x40 Series"]
    At91x40 = 64,
    #[doc = "66: AT91x42 Series"]
    At91x42 = 66,
    #[doc = "69: AT91SAM4SH2 Series"]
    At91sam4sh2 = 69,
    #[doc = "85: AT91x55 Series"]
    At91x55 = 85,
    #[doc = "96: AT91SAM7Axx Series"]
    At91sam7axx = 96,
    #[doc = "97: AT91SAM7AQxx Series"]
    At91sam7aqxx = 97,
    #[doc = "99: AT91x63 Series"]
    At91x63 = 99,
    #[doc = "100: SAM4CxC Series (100-pin version)"]
    Sam4cxxC = 100,
    #[doc = "112: AT91SAM7Sxx Series"]
    At91sam7sxx = 112,
    #[doc = "113: AT91SAM7XCxx Series"]
    At91sam7xcxx = 113,
    #[doc = "114: AT91SAM7SExx Series"]
    At91sam7sexx = 114,
    #[doc = "115: AT91SAM7Lxx Series"]
    At91sam7lxx = 115,
    #[doc = "117: AT91SAM7Xxx Series"]
    At91sam7xxx = 117,
    #[doc = "118: AT91SAM7SLxx Series"]
    At91sam7slxx = 118,
    #[doc = "128: SAM3UxC Series (100-pin version)"]
    Sam3uxC = 128,
    #[doc = "129: SAM3UxE Series (144-pin version)"]
    Sam3uxE = 129,
    #[doc = "131: SAM3AxC Series (100-pin version)"]
    Sam3axC = 131,
    #[doc = "132: SAM3XxC Series (100-pin version)"]
    Sam3xxC = 132,
    #[doc = "133: SAM3XxE Series (144-pin version)"]
    Sam3xxE = 133,
    #[doc = "134: SAM3XxG Series (208/217-pin version)"]
    Sam3xxG = 134,
    #[doc = "146: AT91x92 Series"]
    At91x92 = 146,
    #[doc = "153: SAM3SDxB Series (64-pin version)"]
    Sam3sdxB = 153,
    #[doc = "154: SAM3SDxC Series (100-pin version)"]
    Sam3sdxC = 154,
    #[doc = "165: SAM5A"]
    Sam5a = 165,
    #[doc = "176: SAM4LxA Series (48-pin version)"]
    Sam4lxA = 176,
    #[doc = "177: SAM4LxB Series (64-pin version)"]
    Sam4lxB = 177,
    #[doc = "178: SAM4LxC Series (100-pin version)"]
    Sam4lxC = 178,
    #[doc = "240: AT75Cxx Series"]
    At75cxx = 240,
}
impl From<ARCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCH_A {
    type Ux = u8;
}
impl ARCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARCH_A> {
        match self.bits {
            25 => Some(ARCH_A::At91sam9xx),
            41 => Some(ARCH_A::At91sam9xexx),
            52 => Some(ARCH_A::At91x34),
            55 => Some(ARCH_A::Cap7),
            57 => Some(ARCH_A::Cap9),
            59 => Some(ARCH_A::Cap11),
            64 => Some(ARCH_A::At91x40),
            66 => Some(ARCH_A::At91x42),
            69 => Some(ARCH_A::At91sam4sh2),
            85 => Some(ARCH_A::At91x55),
            96 => Some(ARCH_A::At91sam7axx),
            97 => Some(ARCH_A::At91sam7aqxx),
            99 => Some(ARCH_A::At91x63),
            100 => Some(ARCH_A::Sam4cxxC),
            112 => Some(ARCH_A::At91sam7sxx),
            113 => Some(ARCH_A::At91sam7xcxx),
            114 => Some(ARCH_A::At91sam7sexx),
            115 => Some(ARCH_A::At91sam7lxx),
            117 => Some(ARCH_A::At91sam7xxx),
            118 => Some(ARCH_A::At91sam7slxx),
            128 => Some(ARCH_A::Sam3uxC),
            129 => Some(ARCH_A::Sam3uxE),
            131 => Some(ARCH_A::Sam3axC),
            132 => Some(ARCH_A::Sam3xxC),
            133 => Some(ARCH_A::Sam3xxE),
            134 => Some(ARCH_A::Sam3xxG),
            146 => Some(ARCH_A::At91x92),
            153 => Some(ARCH_A::Sam3sdxB),
            154 => Some(ARCH_A::Sam3sdxC),
            165 => Some(ARCH_A::Sam5a),
            176 => Some(ARCH_A::Sam4lxA),
            177 => Some(ARCH_A::Sam4lxB),
            178 => Some(ARCH_A::Sam4lxC),
            240 => Some(ARCH_A::At75cxx),
            _ => None,
        }
    }
    #[doc = "AT91SAM9xx Series"]
    #[inline(always)]
    pub fn is_at91sam9xx(&self) -> bool {
        *self == ARCH_A::At91sam9xx
    }
    #[doc = "AT91SAM9XExx Series"]
    #[inline(always)]
    pub fn is_at91sam9xexx(&self) -> bool {
        *self == ARCH_A::At91sam9xexx
    }
    #[doc = "AT91x34 Series"]
    #[inline(always)]
    pub fn is_at91x34(&self) -> bool {
        *self == ARCH_A::At91x34
    }
    #[doc = "CAP7 Series"]
    #[inline(always)]
    pub fn is_cap7(&self) -> bool {
        *self == ARCH_A::Cap7
    }
    #[doc = "CAP9 Series"]
    #[inline(always)]
    pub fn is_cap9(&self) -> bool {
        *self == ARCH_A::Cap9
    }
    #[doc = "CAP11 Series"]
    #[inline(always)]
    pub fn is_cap11(&self) -> bool {
        *self == ARCH_A::Cap11
    }
    #[doc = "AT91x40 Series"]
    #[inline(always)]
    pub fn is_at91x40(&self) -> bool {
        *self == ARCH_A::At91x40
    }
    #[doc = "AT91x42 Series"]
    #[inline(always)]
    pub fn is_at91x42(&self) -> bool {
        *self == ARCH_A::At91x42
    }
    #[doc = "AT91SAM4SH2 Series"]
    #[inline(always)]
    pub fn is_at91sam4sh2(&self) -> bool {
        *self == ARCH_A::At91sam4sh2
    }
    #[doc = "AT91x55 Series"]
    #[inline(always)]
    pub fn is_at91x55(&self) -> bool {
        *self == ARCH_A::At91x55
    }
    #[doc = "AT91SAM7Axx Series"]
    #[inline(always)]
    pub fn is_at91sam7axx(&self) -> bool {
        *self == ARCH_A::At91sam7axx
    }
    #[doc = "AT91SAM7AQxx Series"]
    #[inline(always)]
    pub fn is_at91sam7aqxx(&self) -> bool {
        *self == ARCH_A::At91sam7aqxx
    }
    #[doc = "AT91x63 Series"]
    #[inline(always)]
    pub fn is_at91x63(&self) -> bool {
        *self == ARCH_A::At91x63
    }
    #[doc = "SAM4CxC Series (100-pin version)"]
    #[inline(always)]
    pub fn is_sam4cxx_c(&self) -> bool {
        *self == ARCH_A::Sam4cxxC
    }
    #[doc = "AT91SAM7Sxx Series"]
    #[inline(always)]
    pub fn is_at91sam7sxx(&self) -> bool {
        *self == ARCH_A::At91sam7sxx
    }
    #[doc = "AT91SAM7XCxx Series"]
    #[inline(always)]
    pub fn is_at91sam7xcxx(&self) -> bool {
        *self == ARCH_A::At91sam7xcxx
    }
    #[doc = "AT91SAM7SExx Series"]
    #[inline(always)]
    pub fn is_at91sam7sexx(&self) -> bool {
        *self == ARCH_A::At91sam7sexx
    }
    #[doc = "AT91SAM7Lxx Series"]
    #[inline(always)]
    pub fn is_at91sam7lxx(&self) -> bool {
        *self == ARCH_A::At91sam7lxx
    }
    #[doc = "AT91SAM7Xxx Series"]
    #[inline(always)]
    pub fn is_at91sam7xxx(&self) -> bool {
        *self == ARCH_A::At91sam7xxx
    }
    #[doc = "AT91SAM7SLxx Series"]
    #[inline(always)]
    pub fn is_at91sam7slxx(&self) -> bool {
        *self == ARCH_A::At91sam7slxx
    }
    #[doc = "SAM3UxC Series (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3ux_c(&self) -> bool {
        *self == ARCH_A::Sam3uxC
    }
    #[doc = "SAM3UxE Series (144-pin version)"]
    #[inline(always)]
    pub fn is_sam3ux_e(&self) -> bool {
        *self == ARCH_A::Sam3uxE
    }
    #[doc = "SAM3AxC Series (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3ax_c(&self) -> bool {
        *self == ARCH_A::Sam3axC
    }
    #[doc = "SAM3XxC Series (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_c(&self) -> bool {
        *self == ARCH_A::Sam3xxC
    }
    #[doc = "SAM3XxE Series (144-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_e(&self) -> bool {
        *self == ARCH_A::Sam3xxE
    }
    #[doc = "SAM3XxG Series (208/217-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_g(&self) -> bool {
        *self == ARCH_A::Sam3xxG
    }
    #[doc = "AT91x92 Series"]
    #[inline(always)]
    pub fn is_at91x92(&self) -> bool {
        *self == ARCH_A::At91x92
    }
    #[doc = "SAM3SDxB Series (64-pin version)"]
    #[inline(always)]
    pub fn is_sam3sdx_b(&self) -> bool {
        *self == ARCH_A::Sam3sdxB
    }
    #[doc = "SAM3SDxC Series (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3sdx_c(&self) -> bool {
        *self == ARCH_A::Sam3sdxC
    }
    #[doc = "SAM5A"]
    #[inline(always)]
    pub fn is_sam5a(&self) -> bool {
        *self == ARCH_A::Sam5a
    }
    #[doc = "SAM4LxA Series (48-pin version)"]
    #[inline(always)]
    pub fn is_sam4lx_a(&self) -> bool {
        *self == ARCH_A::Sam4lxA
    }
    #[doc = "SAM4LxB Series (64-pin version)"]
    #[inline(always)]
    pub fn is_sam4lx_b(&self) -> bool {
        *self == ARCH_A::Sam4lxB
    }
    #[doc = "SAM4LxC Series (100-pin version)"]
    #[inline(always)]
    pub fn is_sam4lx_c(&self) -> bool {
        *self == ARCH_A::Sam4lxC
    }
    #[doc = "AT75Cxx Series"]
    #[inline(always)]
    pub fn is_at75cxx(&self) -> bool {
        *self == ARCH_A::At75cxx
    }
}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NVPTYP_R = crate::FieldReader<NVPTYP_A>;
#[doc = "Nonvolatile Program Memory Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPTYP_A {
    #[doc = "0: ROM"]
    Rom = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    Romless = 1,
    #[doc = "2: Embedded Flash Memory"]
    Flash = 2,
    #[doc = "3: ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    RomFlash = 3,
    #[doc = "4: SRAM emulating ROM"]
    Sram = 4,
}
impl From<NVPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPTYP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPTYP_A {
    type Ux = u8;
}
impl NVPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NVPTYP_A> {
        match self.bits {
            0 => Some(NVPTYP_A::Rom),
            1 => Some(NVPTYP_A::Romless),
            2 => Some(NVPTYP_A::Flash),
            3 => Some(NVPTYP_A::RomFlash),
            4 => Some(NVPTYP_A::Sram),
            _ => None,
        }
    }
    #[doc = "ROM"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYP_A::Rom
    }
    #[doc = "ROMless or on-chip Flash"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYP_A::Romless
    }
    #[doc = "Embedded Flash Memory"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYP_A::Flash
    }
    #[doc = "ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYP_A::RomFlash
    }
    #[doc = "SRAM emulating ROM"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYP_A::Sram
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub type EXT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr::R`](R) reader structure"]
impl crate::Readable for CIDR_SPEC {}
