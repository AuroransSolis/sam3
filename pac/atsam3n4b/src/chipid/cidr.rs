#[doc = "Register `CIDR` reader"]
pub struct R(crate::R<CIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version of the Device"]
pub struct VERSION_R(crate::FieldReader<u8, u8>);
impl VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Embedded Processor"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPROC_A {
    #[doc = "1: ARM946ES"]
    ARM946ES = 1,
    #[doc = "2: ARM7TDMI"]
    ARM7TDMI = 2,
    #[doc = "3: Cortex-M3"]
    CM3 = 3,
    #[doc = "4: ARM920T"]
    ARM920T = 4,
    #[doc = "5: ARM926EJS"]
    ARM926EJS = 5,
    #[doc = "6: Cortex-A5"]
    CA5 = 6,
    #[doc = "7: Cortex-M4"]
    CM4 = 7,
}
impl From<EPROC_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub struct EPROC_R(crate::FieldReader<u8, EPROC_A>);
impl EPROC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPROC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPROC_A> {
        match self.bits {
            1 => Some(EPROC_A::ARM946ES),
            2 => Some(EPROC_A::ARM7TDMI),
            3 => Some(EPROC_A::CM3),
            4 => Some(EPROC_A::ARM920T),
            5 => Some(EPROC_A::ARM926EJS),
            6 => Some(EPROC_A::CA5),
            7 => Some(EPROC_A::CM4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        **self == EPROC_A::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        **self == EPROC_A::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        **self == EPROC_A::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        **self == EPROC_A::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        **self == EPROC_A::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        **self == EPROC_A::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        **self == EPROC_A::CM4
    }
}
impl core::ops::Deref for EPROC_R {
    type Target = crate::FieldReader<u8, EPROC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NVPSIZ_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8K bytes"]
    _8K = 1,
    #[doc = "2: 16K bytes"]
    _16K = 2,
    #[doc = "3: 32K bytes"]
    _32K = 3,
    #[doc = "5: 64K bytes"]
    _64K = 5,
    #[doc = "7: 128K bytes"]
    _128K = 7,
    #[doc = "9: 256K bytes"]
    _256K = 9,
    #[doc = "10: 512K bytes"]
    _512K = 10,
    #[doc = "12: 1024K bytes"]
    _1024K = 12,
    #[doc = "14: 2048K bytes"]
    _2048K = 14,
}
impl From<NVPSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub struct NVPSIZ_R(crate::FieldReader<u8, NVPSIZ_A>);
impl NVPSIZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NVPSIZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ_A> {
        match self.bits {
            0 => Some(NVPSIZ_A::NONE),
            1 => Some(NVPSIZ_A::_8K),
            2 => Some(NVPSIZ_A::_16K),
            3 => Some(NVPSIZ_A::_32K),
            5 => Some(NVPSIZ_A::_64K),
            7 => Some(NVPSIZ_A::_128K),
            9 => Some(NVPSIZ_A::_256K),
            10 => Some(NVPSIZ_A::_512K),
            12 => Some(NVPSIZ_A::_1024K),
            14 => Some(NVPSIZ_A::_2048K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == NVPSIZ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        **self == NVPSIZ_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        **self == NVPSIZ_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        **self == NVPSIZ_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        **self == NVPSIZ_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        **self == NVPSIZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        **self == NVPSIZ_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        **self == NVPSIZ_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        **self == NVPSIZ_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        **self == NVPSIZ_A::_2048K
    }
}
impl core::ops::Deref for NVPSIZ_R {
    type Target = crate::FieldReader<u8, NVPSIZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Second Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NVPSIZ2_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8K bytes"]
    _8K = 1,
    #[doc = "2: 16K bytes"]
    _16K = 2,
    #[doc = "3: 32K bytes"]
    _32K = 3,
    #[doc = "5: 64K bytes"]
    _64K = 5,
    #[doc = "7: 128K bytes"]
    _128K = 7,
    #[doc = "9: 256K bytes"]
    _256K = 9,
    #[doc = "10: 512K bytes"]
    _512K = 10,
    #[doc = "12: 1024K bytes"]
    _1024K = 12,
    #[doc = "14: 2048K bytes"]
    _2048K = 14,
}
impl From<NVPSIZ2_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub struct NVPSIZ2_R(crate::FieldReader<u8, NVPSIZ2_A>);
impl NVPSIZ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NVPSIZ2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ2_A> {
        match self.bits {
            0 => Some(NVPSIZ2_A::NONE),
            1 => Some(NVPSIZ2_A::_8K),
            2 => Some(NVPSIZ2_A::_16K),
            3 => Some(NVPSIZ2_A::_32K),
            5 => Some(NVPSIZ2_A::_64K),
            7 => Some(NVPSIZ2_A::_128K),
            9 => Some(NVPSIZ2_A::_256K),
            10 => Some(NVPSIZ2_A::_512K),
            12 => Some(NVPSIZ2_A::_1024K),
            14 => Some(NVPSIZ2_A::_2048K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == NVPSIZ2_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        **self == NVPSIZ2_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        **self == NVPSIZ2_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        **self == NVPSIZ2_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        **self == NVPSIZ2_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        **self == NVPSIZ2_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        **self == NVPSIZ2_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        **self == NVPSIZ2_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        **self == NVPSIZ2_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        **self == NVPSIZ2_A::_2048K
    }
}
impl core::ops::Deref for NVPSIZ2_R {
    type Target = crate::FieldReader<u8, NVPSIZ2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal SRAM Size"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMSIZ_A {
    #[doc = "0: 48K bytes"]
    _48K = 0,
    #[doc = "1: 1K bytes"]
    _1K = 1,
    #[doc = "2: 2K bytes"]
    _2K = 2,
    #[doc = "3: 6K bytes"]
    _6K = 3,
    #[doc = "4: 24K bytes"]
    _24K = 4,
    #[doc = "5: 4K bytes"]
    _4K = 5,
    #[doc = "6: 80K bytes"]
    _80K = 6,
    #[doc = "7: 160K bytes"]
    _160K = 7,
    #[doc = "8: 8K bytes"]
    _8K = 8,
    #[doc = "9: 16K bytes"]
    _16K = 9,
    #[doc = "10: 32K bytes"]
    _32K = 10,
    #[doc = "11: 64K bytes"]
    _64K = 11,
    #[doc = "12: 128K bytes"]
    _128K = 12,
    #[doc = "13: 256K bytes"]
    _256K = 13,
    #[doc = "14: 96K bytes"]
    _96K = 14,
    #[doc = "15: 512K bytes"]
    _512K = 15,
}
impl From<SRAMSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub struct SRAMSIZ_R(crate::FieldReader<u8, SRAMSIZ_A>);
impl SRAMSIZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAMSIZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSIZ_A {
        match self.bits {
            0 => SRAMSIZ_A::_48K,
            1 => SRAMSIZ_A::_1K,
            2 => SRAMSIZ_A::_2K,
            3 => SRAMSIZ_A::_6K,
            4 => SRAMSIZ_A::_24K,
            5 => SRAMSIZ_A::_4K,
            6 => SRAMSIZ_A::_80K,
            7 => SRAMSIZ_A::_160K,
            8 => SRAMSIZ_A::_8K,
            9 => SRAMSIZ_A::_16K,
            10 => SRAMSIZ_A::_32K,
            11 => SRAMSIZ_A::_64K,
            12 => SRAMSIZ_A::_128K,
            13 => SRAMSIZ_A::_256K,
            14 => SRAMSIZ_A::_96K,
            15 => SRAMSIZ_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        **self == SRAMSIZ_A::_48K
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        **self == SRAMSIZ_A::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        **self == SRAMSIZ_A::_2K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        **self == SRAMSIZ_A::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        **self == SRAMSIZ_A::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        **self == SRAMSIZ_A::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        **self == SRAMSIZ_A::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        **self == SRAMSIZ_A::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        **self == SRAMSIZ_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        **self == SRAMSIZ_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        **self == SRAMSIZ_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        **self == SRAMSIZ_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        **self == SRAMSIZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        **self == SRAMSIZ_A::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        **self == SRAMSIZ_A::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        **self == SRAMSIZ_A::_512K
    }
}
impl core::ops::Deref for SRAMSIZ_R {
    type Target = crate::FieldReader<u8, SRAMSIZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Architecture Identifier"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARCH_A {
    #[doc = "25: AT91SAM9xx Series"]
    AT91SAM9XX = 25,
    #[doc = "41: AT91SAM9XExx Series"]
    AT91SAM9XEXX = 41,
    #[doc = "52: AT91x34 Series"]
    AT91X34 = 52,
    #[doc = "55: CAP7 Series"]
    CAP7 = 55,
    #[doc = "57: CAP9 Series"]
    CAP9 = 57,
    #[doc = "59: CAP11 Series"]
    CAP11 = 59,
    #[doc = "64: AT91x40 Series"]
    AT91X40 = 64,
    #[doc = "66: AT91x42 Series"]
    AT91X42 = 66,
    #[doc = "85: AT91x55 Series"]
    AT91X55 = 85,
    #[doc = "96: AT91SAM7Axx Series"]
    AT91SAM7AXX = 96,
    #[doc = "97: AT91SAM7AQxx Series"]
    AT91SAM7AQXX = 97,
    #[doc = "99: AT91x63 Series"]
    AT91X63 = 99,
    #[doc = "112: AT91SAM7Sxx Series"]
    AT91SAM7SXX = 112,
    #[doc = "113: AT91SAM7XCxx Series"]
    AT91SAM7XCXX = 113,
    #[doc = "114: AT91SAM7SExx Series"]
    AT91SAM7SEXX = 114,
    #[doc = "115: AT91SAM7Lxx Series"]
    AT91SAM7LXX = 115,
    #[doc = "117: AT91SAM7Xxx Series"]
    AT91SAM7XXX = 117,
    #[doc = "118: AT91SAM7SLxx Series"]
    AT91SAM7SLXX = 118,
    #[doc = "128: SAM3UxC Series (100-pin version)"]
    SAM3UXC = 128,
    #[doc = "129: SAM3UxE Series (144-pin version)"]
    SAM3UXE = 129,
    #[doc = "131: SAM3AxC Series (100-pin version)"]
    SAM3AXC = 131,
    #[doc = "131: SAM4AxC Series (100-pin version)"]
    SAM4AXC = 131,
    #[doc = "132: SAM3XxC Series (100-pin version)"]
    SAM3XXC = 132,
    #[doc = "132: SAM4XxC Series (100-pin version)"]
    SAM4XXC = 132,
    #[doc = "133: SAM3XxE Series (144-pin version)"]
    SAM3XXE = 133,
    #[doc = "133: SAM4XxE Series (144-pin version)"]
    SAM4XXE = 133,
    #[doc = "134: SAM3XxG Series (208/217-pin version)"]
    SAM3XXG = 134,
    #[doc = "134: SAM4XxG Series (208/217-pin version)"]
    SAM4XXG = 134,
    #[doc = "136: SAM3SxASeries (48-pin version)"]
    SAM3SXA = 136,
    #[doc = "136: SAM4SxA Series (48-pin version)"]
    SAM4SXA = 136,
    #[doc = "137: SAM3SxB Series (64-pin version)"]
    SAM3SXB = 137,
    #[doc = "137: SAM4SxB Series (64-pin version)"]
    SAM4SXB = 137,
    #[doc = "138: SAM3SxC Series (100-pin version)"]
    SAM3SXC = 138,
    #[doc = "138: SAM4SxC Series (100-pin version)"]
    SAM4SXC = 138,
    #[doc = "146: AT91x92 Series"]
    AT91X92 = 146,
    #[doc = "147: SAM3NxA Series (48-pin version)"]
    SAM3NXA = 147,
    #[doc = "148: SAM3NxB Series (64-pin version)"]
    SAM3NXB = 148,
    #[doc = "149: SAM3NxC Series (100-pin version)"]
    SAM3NXC = 149,
    #[doc = "153: SAM3SDxB Series (64-pin version)"]
    SAM3SDXB = 153,
    #[doc = "154: SAM3SDxC Series (100-pin version)"]
    SAM3SDXC = 154,
    #[doc = "165: SAM5A"]
    SAM5A = 165,
    #[doc = "240: AT75Cxx Series"]
    AT75CXX = 240,
}
impl From<ARCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub struct ARCH_R(crate::FieldReader<u8, ARCH_A>);
impl ARCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ARCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARCH_A> {
        match self.bits {
            25 => Some(ARCH_A::AT91SAM9XX),
            41 => Some(ARCH_A::AT91SAM9XEXX),
            52 => Some(ARCH_A::AT91X34),
            55 => Some(ARCH_A::CAP7),
            57 => Some(ARCH_A::CAP9),
            59 => Some(ARCH_A::CAP11),
            64 => Some(ARCH_A::AT91X40),
            66 => Some(ARCH_A::AT91X42),
            85 => Some(ARCH_A::AT91X55),
            96 => Some(ARCH_A::AT91SAM7AXX),
            97 => Some(ARCH_A::AT91SAM7AQXX),
            99 => Some(ARCH_A::AT91X63),
            112 => Some(ARCH_A::AT91SAM7SXX),
            113 => Some(ARCH_A::AT91SAM7XCXX),
            114 => Some(ARCH_A::AT91SAM7SEXX),
            115 => Some(ARCH_A::AT91SAM7LXX),
            117 => Some(ARCH_A::AT91SAM7XXX),
            118 => Some(ARCH_A::AT91SAM7SLXX),
            128 => Some(ARCH_A::SAM3UXC),
            129 => Some(ARCH_A::SAM3UXE),
            131 => Some(ARCH_A::SAM3AXC),
            131 => Some(ARCH_A::SAM4AXC),
            132 => Some(ARCH_A::SAM3XXC),
            132 => Some(ARCH_A::SAM4XXC),
            133 => Some(ARCH_A::SAM3XXE),
            133 => Some(ARCH_A::SAM4XXE),
            134 => Some(ARCH_A::SAM3XXG),
            134 => Some(ARCH_A::SAM4XXG),
            136 => Some(ARCH_A::SAM3SXA),
            136 => Some(ARCH_A::SAM4SXA),
            137 => Some(ARCH_A::SAM3SXB),
            137 => Some(ARCH_A::SAM4SXB),
            138 => Some(ARCH_A::SAM3SXC),
            138 => Some(ARCH_A::SAM4SXC),
            146 => Some(ARCH_A::AT91X92),
            147 => Some(ARCH_A::SAM3NXA),
            148 => Some(ARCH_A::SAM3NXB),
            149 => Some(ARCH_A::SAM3NXC),
            153 => Some(ARCH_A::SAM3SDXB),
            154 => Some(ARCH_A::SAM3SDXC),
            165 => Some(ARCH_A::SAM5A),
            240 => Some(ARCH_A::AT75CXX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AT91SAM9XX`"]
    #[inline(always)]
    pub fn is_at91sam9xx(&self) -> bool {
        **self == ARCH_A::AT91SAM9XX
    }
    #[doc = "Checks if the value of the field is `AT91SAM9XEXX`"]
    #[inline(always)]
    pub fn is_at91sam9xexx(&self) -> bool {
        **self == ARCH_A::AT91SAM9XEXX
    }
    #[doc = "Checks if the value of the field is `AT91X34`"]
    #[inline(always)]
    pub fn is_at91x34(&self) -> bool {
        **self == ARCH_A::AT91X34
    }
    #[doc = "Checks if the value of the field is `CAP7`"]
    #[inline(always)]
    pub fn is_cap7(&self) -> bool {
        **self == ARCH_A::CAP7
    }
    #[doc = "Checks if the value of the field is `CAP9`"]
    #[inline(always)]
    pub fn is_cap9(&self) -> bool {
        **self == ARCH_A::CAP9
    }
    #[doc = "Checks if the value of the field is `CAP11`"]
    #[inline(always)]
    pub fn is_cap11(&self) -> bool {
        **self == ARCH_A::CAP11
    }
    #[doc = "Checks if the value of the field is `AT91X40`"]
    #[inline(always)]
    pub fn is_at91x40(&self) -> bool {
        **self == ARCH_A::AT91X40
    }
    #[doc = "Checks if the value of the field is `AT91X42`"]
    #[inline(always)]
    pub fn is_at91x42(&self) -> bool {
        **self == ARCH_A::AT91X42
    }
    #[doc = "Checks if the value of the field is `AT91X55`"]
    #[inline(always)]
    pub fn is_at91x55(&self) -> bool {
        **self == ARCH_A::AT91X55
    }
    #[doc = "Checks if the value of the field is `AT91SAM7AXX`"]
    #[inline(always)]
    pub fn is_at91sam7axx(&self) -> bool {
        **self == ARCH_A::AT91SAM7AXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7AQXX`"]
    #[inline(always)]
    pub fn is_at91sam7aqxx(&self) -> bool {
        **self == ARCH_A::AT91SAM7AQXX
    }
    #[doc = "Checks if the value of the field is `AT91X63`"]
    #[inline(always)]
    pub fn is_at91x63(&self) -> bool {
        **self == ARCH_A::AT91X63
    }
    #[doc = "Checks if the value of the field is `AT91SAM7SXX`"]
    #[inline(always)]
    pub fn is_at91sam7sxx(&self) -> bool {
        **self == ARCH_A::AT91SAM7SXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7XCXX`"]
    #[inline(always)]
    pub fn is_at91sam7xcxx(&self) -> bool {
        **self == ARCH_A::AT91SAM7XCXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7SEXX`"]
    #[inline(always)]
    pub fn is_at91sam7sexx(&self) -> bool {
        **self == ARCH_A::AT91SAM7SEXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7LXX`"]
    #[inline(always)]
    pub fn is_at91sam7lxx(&self) -> bool {
        **self == ARCH_A::AT91SAM7LXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7XXX`"]
    #[inline(always)]
    pub fn is_at91sam7xxx(&self) -> bool {
        **self == ARCH_A::AT91SAM7XXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7SLXX`"]
    #[inline(always)]
    pub fn is_at91sam7slxx(&self) -> bool {
        **self == ARCH_A::AT91SAM7SLXX
    }
    #[doc = "Checks if the value of the field is `SAM3UXC`"]
    #[inline(always)]
    pub fn is_sam3ux_c(&self) -> bool {
        **self == ARCH_A::SAM3UXC
    }
    #[doc = "Checks if the value of the field is `SAM3UXE`"]
    #[inline(always)]
    pub fn is_sam3ux_e(&self) -> bool {
        **self == ARCH_A::SAM3UXE
    }
    #[doc = "Checks if the value of the field is `SAM3AXC`"]
    #[inline(always)]
    pub fn is_sam3ax_c(&self) -> bool {
        **self == ARCH_A::SAM3AXC
    }
    #[doc = "Checks if the value of the field is `SAM4AXC`"]
    #[inline(always)]
    pub fn is_sam4ax_c(&self) -> bool {
        **self == ARCH_A::SAM4AXC
    }
    #[doc = "Checks if the value of the field is `SAM3XXC`"]
    #[inline(always)]
    pub fn is_sam3xx_c(&self) -> bool {
        **self == ARCH_A::SAM3XXC
    }
    #[doc = "Checks if the value of the field is `SAM4XXC`"]
    #[inline(always)]
    pub fn is_sam4xx_c(&self) -> bool {
        **self == ARCH_A::SAM4XXC
    }
    #[doc = "Checks if the value of the field is `SAM3XXE`"]
    #[inline(always)]
    pub fn is_sam3xx_e(&self) -> bool {
        **self == ARCH_A::SAM3XXE
    }
    #[doc = "Checks if the value of the field is `SAM4XXE`"]
    #[inline(always)]
    pub fn is_sam4xx_e(&self) -> bool {
        **self == ARCH_A::SAM4XXE
    }
    #[doc = "Checks if the value of the field is `SAM3XXG`"]
    #[inline(always)]
    pub fn is_sam3xx_g(&self) -> bool {
        **self == ARCH_A::SAM3XXG
    }
    #[doc = "Checks if the value of the field is `SAM4XXG`"]
    #[inline(always)]
    pub fn is_sam4xx_g(&self) -> bool {
        **self == ARCH_A::SAM4XXG
    }
    #[doc = "Checks if the value of the field is `SAM3SXA`"]
    #[inline(always)]
    pub fn is_sam3sx_a(&self) -> bool {
        **self == ARCH_A::SAM3SXA
    }
    #[doc = "Checks if the value of the field is `SAM4SXA`"]
    #[inline(always)]
    pub fn is_sam4sx_a(&self) -> bool {
        **self == ARCH_A::SAM4SXA
    }
    #[doc = "Checks if the value of the field is `SAM3SXB`"]
    #[inline(always)]
    pub fn is_sam3sx_b(&self) -> bool {
        **self == ARCH_A::SAM3SXB
    }
    #[doc = "Checks if the value of the field is `SAM4SXB`"]
    #[inline(always)]
    pub fn is_sam4sx_b(&self) -> bool {
        **self == ARCH_A::SAM4SXB
    }
    #[doc = "Checks if the value of the field is `SAM3SXC`"]
    #[inline(always)]
    pub fn is_sam3sx_c(&self) -> bool {
        **self == ARCH_A::SAM3SXC
    }
    #[doc = "Checks if the value of the field is `SAM4SXC`"]
    #[inline(always)]
    pub fn is_sam4sx_c(&self) -> bool {
        **self == ARCH_A::SAM4SXC
    }
    #[doc = "Checks if the value of the field is `AT91X92`"]
    #[inline(always)]
    pub fn is_at91x92(&self) -> bool {
        **self == ARCH_A::AT91X92
    }
    #[doc = "Checks if the value of the field is `SAM3NXA`"]
    #[inline(always)]
    pub fn is_sam3nx_a(&self) -> bool {
        **self == ARCH_A::SAM3NXA
    }
    #[doc = "Checks if the value of the field is `SAM3NXB`"]
    #[inline(always)]
    pub fn is_sam3nx_b(&self) -> bool {
        **self == ARCH_A::SAM3NXB
    }
    #[doc = "Checks if the value of the field is `SAM3NXC`"]
    #[inline(always)]
    pub fn is_sam3nx_c(&self) -> bool {
        **self == ARCH_A::SAM3NXC
    }
    #[doc = "Checks if the value of the field is `SAM3SDXB`"]
    #[inline(always)]
    pub fn is_sam3sdx_b(&self) -> bool {
        **self == ARCH_A::SAM3SDXB
    }
    #[doc = "Checks if the value of the field is `SAM3SDXC`"]
    #[inline(always)]
    pub fn is_sam3sdx_c(&self) -> bool {
        **self == ARCH_A::SAM3SDXC
    }
    #[doc = "Checks if the value of the field is `SAM5A`"]
    #[inline(always)]
    pub fn is_sam5a(&self) -> bool {
        **self == ARCH_A::SAM5A
    }
    #[doc = "Checks if the value of the field is `AT75CXX`"]
    #[inline(always)]
    pub fn is_at75cxx(&self) -> bool {
        **self == ARCH_A::AT75CXX
    }
}
impl core::ops::Deref for ARCH_R {
    type Target = crate::FieldReader<u8, ARCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Nonvolatile Program Memory Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NVPTYP_A {
    #[doc = "0: ROM"]
    ROM = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    ROMLESS = 1,
    #[doc = "2: Embedded Flash Memory"]
    FLASH = 2,
    #[doc = "3: ROM and Embedded Flash MemoryNVPSIZ is ROM size NVPSIZ2 is Flash size"]
    ROM_FLASH = 3,
    #[doc = "4: SRAM emulating ROM"]
    SRAM = 4,
}
impl From<NVPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub struct NVPTYP_R(crate::FieldReader<u8, NVPTYP_A>);
impl NVPTYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NVPTYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPTYP_A> {
        match self.bits {
            0 => Some(NVPTYP_A::ROM),
            1 => Some(NVPTYP_A::ROMLESS),
            2 => Some(NVPTYP_A::FLASH),
            3 => Some(NVPTYP_A::ROM_FLASH),
            4 => Some(NVPTYP_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        **self == NVPTYP_A::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        **self == NVPTYP_A::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        **self == NVPTYP_A::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        **self == NVPTYP_A::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == NVPTYP_A::SRAM
    }
}
impl core::ops::Deref for NVPTYP_R {
    type Target = crate::FieldReader<u8, NVPTYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub struct EXT_R(crate::FieldReader<bool, bool>);
impl EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 0x07) as u8)
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
        NVPTYP_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](index.html) module"]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr::R](R) reader structure"]
impl crate::Readable for CIDR_SPEC {
    type Reader = R;
}
