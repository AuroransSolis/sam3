#[doc = "Register `CIDR` reader"]
pub type R = crate::R<CidrSpec>;
#[doc = "Field `VERSION` reader - Version of the Device"]
pub type VersionR = crate::FieldReader;
#[doc = "Embedded Processor"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eproc {
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
impl From<Eproc> for u8 {
    #[inline(always)]
    fn from(variant: Eproc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eproc {
    type Ux = u8;
}
impl crate::IsEnum for Eproc {}
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EprocR = crate::FieldReader<Eproc>;
impl EprocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eproc> {
        match self.bits {
            1 => Some(Eproc::Arm946es),
            2 => Some(Eproc::Arm7tdmi),
            3 => Some(Eproc::Cm3),
            4 => Some(Eproc::Arm920t),
            5 => Some(Eproc::Arm926ejs),
            6 => Some(Eproc::Ca5),
            7 => Some(Eproc::Cm4),
            _ => None,
        }
    }
    #[doc = "ARM946ES"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == Eproc::Arm946es
    }
    #[doc = "ARM7TDMI"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == Eproc::Arm7tdmi
    }
    #[doc = "Cortex-M3"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == Eproc::Cm3
    }
    #[doc = "ARM920T"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == Eproc::Arm920t
    }
    #[doc = "ARM926EJS"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == Eproc::Arm926ejs
    }
    #[doc = "Cortex-A5"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == Eproc::Ca5
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == Eproc::Cm4
    }
}
#[doc = "Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvpsiz {
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
impl From<Nvpsiz> for u8 {
    #[inline(always)]
    fn from(variant: Nvpsiz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvpsiz {
    type Ux = u8;
}
impl crate::IsEnum for Nvpsiz {}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NvpsizR = crate::FieldReader<Nvpsiz>;
impl NvpsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvpsiz> {
        match self.bits {
            0 => Some(Nvpsiz::None),
            1 => Some(Nvpsiz::_8k),
            2 => Some(Nvpsiz::_16k),
            3 => Some(Nvpsiz::_32k),
            5 => Some(Nvpsiz::_64k),
            7 => Some(Nvpsiz::_128k),
            9 => Some(Nvpsiz::_256k),
            10 => Some(Nvpsiz::_512k),
            12 => Some(Nvpsiz::_1024k),
            14 => Some(Nvpsiz::_2048k),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Nvpsiz::None
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Nvpsiz::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Nvpsiz::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Nvpsiz::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Nvpsiz::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Nvpsiz::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Nvpsiz::_256k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Nvpsiz::_512k
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == Nvpsiz::_1024k
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == Nvpsiz::_2048k
    }
}
#[doc = "Second Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvpsiz2 {
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
impl From<Nvpsiz2> for u8 {
    #[inline(always)]
    fn from(variant: Nvpsiz2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvpsiz2 {
    type Ux = u8;
}
impl crate::IsEnum for Nvpsiz2 {}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type Nvpsiz2R = crate::FieldReader<Nvpsiz2>;
impl Nvpsiz2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvpsiz2> {
        match self.bits {
            0 => Some(Nvpsiz2::None),
            1 => Some(Nvpsiz2::_8k),
            2 => Some(Nvpsiz2::_16k),
            3 => Some(Nvpsiz2::_32k),
            5 => Some(Nvpsiz2::_64k),
            7 => Some(Nvpsiz2::_128k),
            9 => Some(Nvpsiz2::_256k),
            10 => Some(Nvpsiz2::_512k),
            12 => Some(Nvpsiz2::_1024k),
            14 => Some(Nvpsiz2::_2048k),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Nvpsiz2::None
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Nvpsiz2::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Nvpsiz2::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Nvpsiz2::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Nvpsiz2::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Nvpsiz2::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Nvpsiz2::_256k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Nvpsiz2::_512k
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == Nvpsiz2::_1024k
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == Nvpsiz2::_2048k
    }
}
#[doc = "Internal SRAM Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramsiz {
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
impl From<Sramsiz> for u8 {
    #[inline(always)]
    fn from(variant: Sramsiz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramsiz {
    type Ux = u8;
}
impl crate::IsEnum for Sramsiz {}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SramsizR = crate::FieldReader<Sramsiz>;
impl SramsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsiz {
        match self.bits {
            0 => Sramsiz::_48k,
            1 => Sramsiz::_192k,
            2 => Sramsiz::_2k,
            3 => Sramsiz::_6k,
            4 => Sramsiz::_24k,
            5 => Sramsiz::_4k,
            6 => Sramsiz::_80k,
            7 => Sramsiz::_160k,
            8 => Sramsiz::_8k,
            9 => Sramsiz::_16k,
            10 => Sramsiz::_32k,
            11 => Sramsiz::_64k,
            12 => Sramsiz::_128k,
            13 => Sramsiz::_256k,
            14 => Sramsiz::_96k,
            15 => Sramsiz::_512k,
            _ => unreachable!(),
        }
    }
    #[doc = "48 Kbytes"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == Sramsiz::_48k
    }
    #[doc = "192 Kbytes"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == Sramsiz::_192k
    }
    #[doc = "2 Kbytes"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == Sramsiz::_2k
    }
    #[doc = "6 Kbytes"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == Sramsiz::_6k
    }
    #[doc = "24 Kbytes"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == Sramsiz::_24k
    }
    #[doc = "4 Kbytes"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == Sramsiz::_4k
    }
    #[doc = "80 Kbytes"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == Sramsiz::_80k
    }
    #[doc = "160 Kbytes"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == Sramsiz::_160k
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == Sramsiz::_8k
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == Sramsiz::_16k
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Sramsiz::_32k
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == Sramsiz::_64k
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == Sramsiz::_128k
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == Sramsiz::_256k
    }
    #[doc = "96 Kbytes"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == Sramsiz::_96k
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Sramsiz::_512k
    }
}
#[doc = "Architecture Identifier"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arch {
    #[doc = "131: SAM3AxC (100-pin version)"]
    Sam3axC = 131,
    #[doc = "132: SAM3XxC (100-pin version)"]
    Sam3xxC = 132,
    #[doc = "133: SAM3XxE (144-pin version)"]
    Sam3xxE = 133,
    #[doc = "134: SAM3XxG (208/217-pin version)"]
    Sam3xxG = 134,
}
impl From<Arch> for u8 {
    #[inline(always)]
    fn from(variant: Arch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arch {
    type Ux = u8;
}
impl crate::IsEnum for Arch {}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ArchR = crate::FieldReader<Arch>;
impl ArchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Arch> {
        match self.bits {
            131 => Some(Arch::Sam3axC),
            132 => Some(Arch::Sam3xxC),
            133 => Some(Arch::Sam3xxE),
            134 => Some(Arch::Sam3xxG),
            _ => None,
        }
    }
    #[doc = "SAM3AxC (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3ax_c(&self) -> bool {
        *self == Arch::Sam3axC
    }
    #[doc = "SAM3XxC (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_c(&self) -> bool {
        *self == Arch::Sam3xxC
    }
    #[doc = "SAM3XxE (144-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_e(&self) -> bool {
        *self == Arch::Sam3xxE
    }
    #[doc = "SAM3XxG (208/217-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_g(&self) -> bool {
        *self == Arch::Sam3xxG
    }
}
#[doc = "Nonvolatile Program Memory Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nvptyp {
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
impl From<Nvptyp> for u8 {
    #[inline(always)]
    fn from(variant: Nvptyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nvptyp {
    type Ux = u8;
}
impl crate::IsEnum for Nvptyp {}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NvptypR = crate::FieldReader<Nvptyp>;
impl NvptypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nvptyp> {
        match self.bits {
            0 => Some(Nvptyp::Rom),
            1 => Some(Nvptyp::Romless),
            2 => Some(Nvptyp::Flash),
            3 => Some(Nvptyp::RomFlash),
            4 => Some(Nvptyp::Sram),
            _ => None,
        }
    }
    #[doc = "ROM"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == Nvptyp::Rom
    }
    #[doc = "ROMless or on-chip Flash"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == Nvptyp::Romless
    }
    #[doc = "Embedded Flash Memory"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == Nvptyp::Flash
    }
    #[doc = "ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == Nvptyp::RomFlash
    }
    #[doc = "SRAM emulating ROM"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == Nvptyp::Sram
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub type ExtR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EprocR {
        EprocR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NvpsizR {
        NvpsizR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> Nvpsiz2R {
        Nvpsiz2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SramsizR {
        SramsizR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ArchR {
        ArchR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NvptypR {
        NvptypR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Chip ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CidrSpec;
impl crate::RegisterSpec for CidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr::R`](R) reader structure"]
impl crate::Readable for CidrSpec {}
