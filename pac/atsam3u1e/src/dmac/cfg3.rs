#[doc = "Register `CFG3` reader"]
pub type R = crate::R<Cfg3Spec>;
#[doc = "Register `CFG3` writer"]
pub type W = crate::W<Cfg3Spec>;
#[doc = "Field `SRC_PER` reader - Source with Peripheral identifier"]
pub type SrcPerR = crate::FieldReader;
#[doc = "Field `SRC_PER` writer - Source with Peripheral identifier"]
pub type SrcPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DST_PER` reader - Destination with Peripheral identifier"]
pub type DstPerR = crate::FieldReader;
#[doc = "Field `DST_PER` writer - Destination with Peripheral identifier"]
pub type DstPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Software or Hardware Selection for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrcH2sel {
    #[doc = "0: Software handshaking interface is used to trigger a transfer request."]
    Sw = 0,
    #[doc = "1: Hardware handshaking interface is used to trigger a transfer request."]
    Hw = 1,
}
impl From<SrcH2sel> for bool {
    #[inline(always)]
    fn from(variant: SrcH2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_H2SEL` reader - Software or Hardware Selection for the Source"]
pub type SrcH2selR = crate::BitReader<SrcH2sel>;
impl SrcH2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrcH2sel {
        match self.bits {
            false => SrcH2sel::Sw,
            true => SrcH2sel::Hw,
        }
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == SrcH2sel::Sw
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == SrcH2sel::Hw
    }
}
#[doc = "Field `SRC_H2SEL` writer - Software or Hardware Selection for the Source"]
pub type SrcH2selW<'a, REG> = crate::BitWriter<'a, REG, SrcH2sel>;
impl<'a, REG> SrcH2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(SrcH2sel::Sw)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(SrcH2sel::Hw)
    }
}
#[doc = "Software or Hardware Selection for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DstH2sel {
    #[doc = "0: Software handshaking interface is used to trigger a transfer request."]
    Sw = 0,
    #[doc = "1: Hardware handshaking interface is used to trigger a transfer request."]
    Hw = 1,
}
impl From<DstH2sel> for bool {
    #[inline(always)]
    fn from(variant: DstH2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_H2SEL` reader - Software or Hardware Selection for the Destination"]
pub type DstH2selR = crate::BitReader<DstH2sel>;
impl DstH2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DstH2sel {
        match self.bits {
            false => DstH2sel::Sw,
            true => DstH2sel::Hw,
        }
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == DstH2sel::Sw
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == DstH2sel::Hw
    }
}
#[doc = "Field `DST_H2SEL` writer - Software or Hardware Selection for the Destination"]
pub type DstH2selW<'a, REG> = crate::BitWriter<'a, REG, DstH2sel>;
impl<'a, REG> DstH2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(DstH2sel::Sw)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(DstH2sel::Hw)
    }
}
#[doc = "Stop On Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sod {
    #[doc = "0: STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    Disable = 0,
    #[doc = "1: STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    Enable = 1,
}
impl From<Sod> for bool {
    #[inline(always)]
    fn from(variant: Sod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOD` reader - Stop On Done"]
pub type SodR = crate::BitReader<Sod>;
impl SodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sod {
        match self.bits {
            false => Sod::Disable,
            true => Sod::Enable,
        }
    }
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sod::Disable
    }
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sod::Enable
    }
}
#[doc = "Field `SOD` writer - Stop On Done"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG, Sod>;
impl<'a, REG> SodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sod::Disable)
    }
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sod::Enable)
    }
}
#[doc = "Interface Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockIf {
    #[doc = "0: Interface Lock capability is disabled"]
    Disable = 0,
    #[doc = "1: Interface Lock capability is enabled"]
    Enable = 1,
}
impl From<LockIf> for bool {
    #[inline(always)]
    fn from(variant: LockIf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_IF` reader - Interface Lock"]
pub type LockIfR = crate::BitReader<LockIf>;
impl LockIfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockIf {
        match self.bits {
            false => LockIf::Disable,
            true => LockIf::Enable,
        }
    }
    #[doc = "Interface Lock capability is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LockIf::Disable
    }
    #[doc = "Interface Lock capability is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LockIf::Enable
    }
}
#[doc = "Field `LOCK_IF` writer - Interface Lock"]
pub type LockIfW<'a, REG> = crate::BitWriter<'a, REG, LockIf>;
impl<'a, REG> LockIfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interface Lock capability is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LockIf::Disable)
    }
    #[doc = "Interface Lock capability is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LockIf::Enable)
    }
}
#[doc = "Bus Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockB {
    #[doc = "0: AHB Bus Locking capability is disabled."]
    Disable = 0,
}
impl From<LockB> for bool {
    #[inline(always)]
    fn from(variant: LockB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_B` reader - Bus Lock"]
pub type LockBR = crate::BitReader<LockB>;
impl LockBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockB> {
        match self.bits {
            false => Some(LockB::Disable),
            _ => None,
        }
    }
    #[doc = "AHB Bus Locking capability is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LockB::Disable
    }
}
#[doc = "Field `LOCK_B` writer - Bus Lock"]
pub type LockBW<'a, REG> = crate::BitWriter<'a, REG, LockB>;
impl<'a, REG> LockBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB Bus Locking capability is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LockB::Disable)
    }
}
#[doc = "Master Interface Arbiter Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockIfL {
    #[doc = "0: The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    Chunk = 0,
    #[doc = "1: The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    Buffer = 1,
}
impl From<LockIfL> for bool {
    #[inline(always)]
    fn from(variant: LockIfL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_IF_L` reader - Master Interface Arbiter Lock"]
pub type LockIfLR = crate::BitReader<LockIfL>;
impl LockIfLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockIfL {
        match self.bits {
            false => LockIfL::Chunk,
            true => LockIfL::Buffer,
        }
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    #[inline(always)]
    pub fn is_chunk(&self) -> bool {
        *self == LockIfL::Chunk
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    #[inline(always)]
    pub fn is_buffer(&self) -> bool {
        *self == LockIfL::Buffer
    }
}
#[doc = "Field `LOCK_IF_L` writer - Master Interface Arbiter Lock"]
pub type LockIfLW<'a, REG> = crate::BitWriter<'a, REG, LockIfL>;
impl<'a, REG> LockIfLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    #[inline(always)]
    pub fn chunk(self) -> &'a mut crate::W<REG> {
        self.variant(LockIfL::Chunk)
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    #[inline(always)]
    pub fn buffer(self) -> &'a mut crate::W<REG> {
        self.variant(LockIfL::Buffer)
    }
}
#[doc = "Field `AHB_PROT` reader - AHB Protection"]
pub type AhbProtR = crate::FieldReader;
#[doc = "Field `AHB_PROT` writer - AHB Protection"]
pub type AhbProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "FIFO Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifocfg {
    #[doc = "0: The largest defined length AHB burst is performed on the destination AHB interface."]
    AlapCfg = 0,
    #[doc = "1: When half FIFO size is available/filled, a source/destination request is serviced."]
    HalfCfg = 1,
    #[doc = "2: When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    AsapCfg = 2,
}
impl From<Fifocfg> for u8 {
    #[inline(always)]
    fn from(variant: Fifocfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifocfg {
    type Ux = u8;
}
impl crate::IsEnum for Fifocfg {}
#[doc = "Field `FIFOCFG` reader - FIFO Configuration"]
pub type FifocfgR = crate::FieldReader<Fifocfg>;
impl FifocfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fifocfg> {
        match self.bits {
            0 => Some(Fifocfg::AlapCfg),
            1 => Some(Fifocfg::HalfCfg),
            2 => Some(Fifocfg::AsapCfg),
            _ => None,
        }
    }
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    #[inline(always)]
    pub fn is_alap_cfg(&self) -> bool {
        *self == Fifocfg::AlapCfg
    }
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    #[inline(always)]
    pub fn is_half_cfg(&self) -> bool {
        *self == Fifocfg::HalfCfg
    }
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    #[inline(always)]
    pub fn is_asap_cfg(&self) -> bool {
        *self == Fifocfg::AsapCfg
    }
}
#[doc = "Field `FIFOCFG` writer - FIFO Configuration"]
pub type FifocfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fifocfg>;
impl<'a, REG> FifocfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    #[inline(always)]
    pub fn alap_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(Fifocfg::AlapCfg)
    }
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    #[inline(always)]
    pub fn half_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(Fifocfg::HalfCfg)
    }
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    #[inline(always)]
    pub fn asap_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(Fifocfg::AsapCfg)
    }
}
impl R {
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline(always)]
    pub fn src_per(&self) -> SrcPerR {
        SrcPerR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline(always)]
    pub fn dst_per(&self) -> DstPerR {
        DstPerR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline(always)]
    pub fn src_h2sel(&self) -> SrcH2selR {
        SrcH2selR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline(always)]
    pub fn dst_h2sel(&self) -> DstH2selR {
        DstH2selR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline(always)]
    pub fn lock_if(&self) -> LockIfR {
        LockIfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline(always)]
    pub fn lock_b(&self) -> LockBR {
        LockBR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline(always)]
    pub fn lock_if_l(&self) -> LockIfLR {
        LockIfLR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline(always)]
    pub fn ahb_prot(&self) -> AhbProtR {
        AhbProtR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline(always)]
    pub fn fifocfg(&self) -> FifocfgR {
        FifocfgR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn src_per(&mut self) -> SrcPerW<Cfg3Spec> {
        SrcPerW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn dst_per(&mut self) -> DstPerW<Cfg3Spec> {
        DstPerW::new(self, 4)
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline(always)]
    #[must_use]
    pub fn src_h2sel(&mut self) -> SrcH2selW<Cfg3Spec> {
        SrcH2selW::new(self, 9)
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline(always)]
    #[must_use]
    pub fn dst_h2sel(&mut self) -> DstH2selW<Cfg3Spec> {
        DstH2selW::new(self, 13)
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SodW<Cfg3Spec> {
        SodW::new(self, 16)
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_if(&mut self) -> LockIfW<Cfg3Spec> {
        LockIfW::new(self, 20)
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b(&mut self) -> LockBW<Cfg3Spec> {
        LockBW::new(self, 21)
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_if_l(&mut self) -> LockIfLW<Cfg3Spec> {
        LockIfLW::new(self, 22)
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_prot(&mut self) -> AhbProtW<Cfg3Spec> {
        AhbProtW::new(self, 24)
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fifocfg(&mut self) -> FifocfgW<Cfg3Spec> {
        FifocfgW::new(self, 28)
    }
}
#[doc = "DMAC Channel Configuration Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg3Spec;
impl crate::RegisterSpec for Cfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg3::R`](R) reader structure"]
impl crate::Readable for Cfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg3::W`](W) writer structure"]
impl crate::Writable for Cfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG3 to value 0x0100_0000"]
impl crate::Resettable for Cfg3Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
