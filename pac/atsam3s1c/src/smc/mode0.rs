#[doc = "Register `MODE0` reader"]
pub type R = crate::R<Mode0Spec>;
#[doc = "Register `MODE0` writer"]
pub type W = crate::W<Mode0Spec>;
#[doc = "Field `READ_MODE` reader - "]
pub type ReadModeR = crate::BitReader;
#[doc = "Field `READ_MODE` writer - "]
pub type ReadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MODE` reader - "]
pub type WriteModeR = crate::BitReader;
#[doc = "Field `WRITE_MODE` writer - "]
pub type WriteModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExnwMode {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "2: Frozen Mode"]
    Frozen = 2,
    #[doc = "3: Ready Mode"]
    Ready = 3,
}
impl From<ExnwMode> for u8 {
    #[inline(always)]
    fn from(variant: ExnwMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ExnwMode {
    type Ux = u8;
}
impl crate::IsEnum for ExnwMode {}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type ExnwModeR = crate::FieldReader<ExnwMode>;
impl ExnwModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ExnwMode> {
        match self.bits {
            0 => Some(ExnwMode::Disabled),
            2 => Some(ExnwMode::Frozen),
            3 => Some(ExnwMode::Ready),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ExnwMode::Disabled
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == ExnwMode::Frozen
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ExnwMode::Ready
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type ExnwModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ExnwMode>;
impl<'a, REG> ExnwModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwMode::Disabled)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwMode::Frozen)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwMode::Ready)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TdfCyclesR = crate::FieldReader;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TdfCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TdfModeR = crate::BitReader;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TdfModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub type PmenR = crate::BitReader;
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub type PmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Page Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: 4-byte page"]
    _4Byte = 0,
    #[doc = "1: 8-byte page"]
    _8Byte = 1,
    #[doc = "2: 16-byte page"]
    _16Byte = 2,
    #[doc = "3: 32-byte page"]
    _32Byte = 3,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Page Size"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::_4Byte,
            1 => Ps::_8Byte,
            2 => Ps::_16Byte,
            3 => Ps::_32Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == Ps::_4Byte
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Ps::_8Byte
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Ps::_16Byte
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Ps::_32Byte
    }
}
#[doc = "Field `PS` writer - Page Size"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_4Byte)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_8Byte)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_16Byte)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_32Byte)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&self) -> WriteModeR {
        WriteModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> ExnwModeR {
        ExnwModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TdfCyclesR {
        TdfCyclesR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TdfModeR {
        TdfModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PmenR {
        PmenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> ReadModeW<Mode0Spec> {
        ReadModeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn write_mode(&mut self) -> WriteModeW<Mode0Spec> {
        WriteModeW::new(self, 1)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn exnw_mode(&mut self) -> ExnwModeW<Mode0Spec> {
        ExnwModeW::new(self, 4)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_cycles(&mut self) -> TdfCyclesW<Mode0Spec> {
        TdfCyclesW::new(self, 16)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_mode(&mut self) -> TdfModeW<Mode0Spec> {
        TdfModeW::new(self, 20)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pmen(&mut self) -> PmenW<Mode0Spec> {
        PmenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<Mode0Spec> {
        PsW::new(self, 28)
    }
}
#[doc = "SMC Mode Register (CS_number = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mode0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mode0Spec;
impl crate::RegisterSpec for Mode0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode0::R`](R) reader structure"]
impl crate::Readable for Mode0Spec {}
#[doc = "`write(|w| ..)` method takes [`mode0::W`](W) writer structure"]
impl crate::Writable for Mode0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE0 to value 0x1000_0003"]
impl crate::Resettable for Mode0Spec {
    const RESET_VALUE: u32 = 0x1000_0003;
}
