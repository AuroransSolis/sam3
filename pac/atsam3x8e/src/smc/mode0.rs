#[doc = "Register `MODE0` reader"]
pub type R = crate::R<Mode0Spec>;
#[doc = "Register `MODE0` writer"]
pub type W = crate::W<Mode0Spec>;
#[doc = "Selection of the Control Signal for Read Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadMode {
    #[doc = "0: The Read operation is controlled by the NCS signal."]
    NcsCtrl = 0,
    #[doc = "1: The Read operation is controlled by the NRD signal."]
    NrdCtrl = 1,
}
impl From<ReadMode> for bool {
    #[inline(always)]
    fn from(variant: ReadMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_MODE` reader - Selection of the Control Signal for Read Operation"]
pub type ReadModeR = crate::BitReader<ReadMode>;
impl ReadModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadMode {
        match self.bits {
            false => ReadMode::NcsCtrl,
            true => ReadMode::NrdCtrl,
        }
    }
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == ReadMode::NcsCtrl
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn is_nrd_ctrl(&self) -> bool {
        *self == ReadMode::NrdCtrl
    }
}
#[doc = "Field `READ_MODE` writer - Selection of the Control Signal for Read Operation"]
pub type ReadModeW<'a, REG> = crate::BitWriter<'a, REG, ReadMode>;
impl<'a, REG> ReadModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::NcsCtrl)
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn nrd_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::NrdCtrl)
    }
}
#[doc = "Selection of the Control Signal for Write Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteMode {
    #[doc = "0: The Write operation is controller by the NCS signal."]
    NcsCtrl = 0,
    #[doc = "1: The Write operation is controlled by the NWE signal."]
    NweCtrl = 1,
}
impl From<WriteMode> for bool {
    #[inline(always)]
    fn from(variant: WriteMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_MODE` reader - Selection of the Control Signal for Write Operation"]
pub type WriteModeR = crate::BitReader<WriteMode>;
impl WriteModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteMode {
        match self.bits {
            false => WriteMode::NcsCtrl,
            true => WriteMode::NweCtrl,
        }
    }
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == WriteMode::NcsCtrl
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn is_nwe_ctrl(&self) -> bool {
        *self == WriteMode::NweCtrl
    }
}
#[doc = "Field `WRITE_MODE` writer - Selection of the Control Signal for Write Operation"]
pub type WriteModeW<'a, REG> = crate::BitWriter<'a, REG, WriteMode>;
impl<'a, REG> WriteModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMode::NcsCtrl)
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn nwe_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMode::NweCtrl)
    }
}
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
#[doc = "Field `BAT` reader - Byte Access Type"]
pub type BatR = crate::BitReader;
#[doc = "Field `BAT` writer - Byte Access Type"]
pub type BatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbw {
    #[doc = "0: 8-bit bus"]
    Bit8 = 0,
    #[doc = "1: 16-bit bus"]
    Bit16 = 1,
}
impl From<Dbw> for bool {
    #[inline(always)]
    fn from(variant: Dbw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::BitReader<Dbw>;
impl DbwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbw {
        match self.bits {
            false => Dbw::Bit8,
            true => Dbw::Bit16,
        }
    }
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == Dbw::Bit8
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        *self == Dbw::Bit16
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::BitWriter<'a, REG, Dbw>;
impl<'a, REG> DbwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dbw::Bit8)
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dbw::Bit16)
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
impl R {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    pub fn write_mode(&self) -> WriteModeR {
        WriteModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> ExnwModeR {
        ExnwModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BatR {
        BatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 12) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> ReadModeW<Mode0Spec> {
        ReadModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
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
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    #[must_use]
    pub fn bat(&mut self) -> BatW<Mode0Spec> {
        BatW::new(self, 8)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DbwW<Mode0Spec> {
        DbwW::new(self, 12)
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
}
#[doc = "SMC Mode Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
