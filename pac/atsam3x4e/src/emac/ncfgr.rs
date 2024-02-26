#[doc = "Register `NCFGR` reader"]
pub type R = crate::R<NcfgrSpec>;
#[doc = "Register `NCFGR` writer"]
pub type W = crate::W<NcfgrSpec>;
#[doc = "Field `SPD` reader - Speed"]
pub type SpdR = crate::BitReader;
#[doc = "Field `SPD` writer - Speed"]
pub type SpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FD` reader - Full Duplex"]
pub type FdR = crate::BitReader;
#[doc = "Field `FD` writer - Full Duplex"]
pub type FdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JFRAME` reader - Jumbo Frames"]
pub type JframeR = crate::BitReader;
#[doc = "Field `JFRAME` writer - Jumbo Frames"]
pub type JframeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAF` reader - Copy All Frames"]
pub type CafR = crate::BitReader;
#[doc = "Field `CAF` writer - Copy All Frames"]
pub type CafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBC` reader - No Broadcast"]
pub type NbcR = crate::BitReader;
#[doc = "Field `NBC` writer - No Broadcast"]
pub type NbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTI` reader - Multicast Hash Enable"]
pub type MtiR = crate::BitReader;
#[doc = "Field `MTI` writer - Multicast Hash Enable"]
pub type MtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNI` reader - Unicast Hash Enable"]
pub type UniR = crate::BitReader;
#[doc = "Field `UNI` writer - Unicast Hash Enable"]
pub type UniW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIG` reader - Receive 1536 bytes frames"]
pub type BigR = crate::BitReader;
#[doc = "Field `BIG` writer - Receive 1536 bytes frames"]
pub type BigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MDC clock divider\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clk {
    #[doc = "0: MCK divided by 8 (MCK up to 20 MHz)."]
    Mck8 = 0,
    #[doc = "1: MCK divided by 16 (MCK up to 40 MHz)."]
    Mck16 = 1,
    #[doc = "2: MCK divided by 32 (MCK up to 80 MHz)."]
    Mck32 = 2,
    #[doc = "3: MCK divided by 64 (MCK up to 160 MHz)."]
    Mck64 = 3,
}
impl From<Clk> for u8 {
    #[inline(always)]
    fn from(variant: Clk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clk {
    type Ux = u8;
}
#[doc = "Field `CLK` reader - MDC clock divider"]
pub type ClkR = crate::FieldReader<Clk>;
impl ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clk {
        match self.bits {
            0 => Clk::Mck8,
            1 => Clk::Mck16,
            2 => Clk::Mck32,
            3 => Clk::Mck64,
            _ => unreachable!(),
        }
    }
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline(always)]
    pub fn is_mck_8(&self) -> bool {
        *self == Clk::Mck8
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline(always)]
    pub fn is_mck_16(&self) -> bool {
        *self == Clk::Mck16
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline(always)]
    pub fn is_mck_32(&self) -> bool {
        *self == Clk::Mck32
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline(always)]
    pub fn is_mck_64(&self) -> bool {
        *self == Clk::Mck64
    }
}
#[doc = "Field `CLK` writer - MDC clock divider"]
pub type ClkW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Clk>;
impl<'a, REG> ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline(always)]
    pub fn mck_8(self) -> &'a mut crate::W<REG> {
        self.variant(Clk::Mck8)
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline(always)]
    pub fn mck_16(self) -> &'a mut crate::W<REG> {
        self.variant(Clk::Mck16)
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline(always)]
    pub fn mck_32(self) -> &'a mut crate::W<REG> {
        self.variant(Clk::Mck32)
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline(always)]
    pub fn mck_64(self) -> &'a mut crate::W<REG> {
        self.variant(Clk::Mck64)
    }
}
#[doc = "Field `RTY` reader - Retry test"]
pub type RtyR = crate::BitReader;
#[doc = "Field `RTY` writer - Retry test"]
pub type RtyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAE` reader - Pause Enable"]
pub type PaeR = crate::BitReader;
#[doc = "Field `PAE` writer - Pause Enable"]
pub type PaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive Buffer Offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rbof {
    #[doc = "0: No offset from start of receive buffer."]
    Offset0 = 0,
    #[doc = "1: One-byte offset from start of receive buffer."]
    Offset1 = 1,
    #[doc = "2: Two-byte offset from start of receive buffer."]
    Offset2 = 2,
    #[doc = "3: Three-byte offset from start of receive buffer."]
    Offset3 = 3,
}
impl From<Rbof> for u8 {
    #[inline(always)]
    fn from(variant: Rbof) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rbof {
    type Ux = u8;
}
#[doc = "Field `RBOF` reader - Receive Buffer Offset"]
pub type RbofR = crate::FieldReader<Rbof>;
impl RbofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbof {
        match self.bits {
            0 => Rbof::Offset0,
            1 => Rbof::Offset1,
            2 => Rbof::Offset2,
            3 => Rbof::Offset3,
            _ => unreachable!(),
        }
    }
    #[doc = "No offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_0(&self) -> bool {
        *self == Rbof::Offset0
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_1(&self) -> bool {
        *self == Rbof::Offset1
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_2(&self) -> bool {
        *self == Rbof::Offset2
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_3(&self) -> bool {
        *self == Rbof::Offset3
    }
}
#[doc = "Field `RBOF` writer - Receive Buffer Offset"]
pub type RbofW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rbof>;
impl<'a, REG> RbofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rbof::Offset0)
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rbof::Offset1)
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rbof::Offset2)
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rbof::Offset3)
    }
}
#[doc = "Field `RLCE` reader - Receive Length field Checking Enable"]
pub type RlceR = crate::BitReader;
#[doc = "Field `RLCE` writer - Receive Length field Checking Enable"]
pub type RlceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRFCS` reader - Discard Receive FCS"]
pub type DrfcsR = crate::BitReader;
#[doc = "Field `DRFCS` writer - Discard Receive FCS"]
pub type DrfcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFRHD` reader - "]
pub type EfrhdR = crate::BitReader;
#[doc = "Field `EFRHD` writer - "]
pub type EfrhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRXFCS` reader - Ignore RX FCS"]
pub type IrxfcsR = crate::BitReader;
#[doc = "Field `IRXFCS` writer - Ignore RX FCS"]
pub type IrxfcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FdR {
        FdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    pub fn jframe(&self) -> JframeR {
        JframeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CafR {
        CafR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NbcR {
        NbcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MtiR {
        MtiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn uni(&self) -> UniR {
        UniR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    pub fn big(&self) -> BigR {
        BigR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn rty(&self) -> RtyR {
        RtyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pae(&self) -> PaeR {
        PaeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rbof(&self) -> RbofR {
        RbofR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    pub fn rlce(&self) -> RlceR {
        RlceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    pub fn drfcs(&self) -> DrfcsR {
        DrfcsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn efrhd(&self) -> EfrhdR {
        EfrhdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IrxfcsR {
        IrxfcsR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SpdW<NcfgrSpec> {
        SpdW::new(self, 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FdW<NcfgrSpec> {
        FdW::new(self, 1)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    #[must_use]
    pub fn jframe(&mut self) -> JframeW<NcfgrSpec> {
        JframeW::new(self, 3)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn caf(&mut self) -> CafW<NcfgrSpec> {
        CafW::new(self, 4)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn nbc(&mut self) -> NbcW<NcfgrSpec> {
        NbcW::new(self, 5)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mti(&mut self) -> MtiW<NcfgrSpec> {
        MtiW::new(self, 6)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uni(&mut self) -> UniW<NcfgrSpec> {
        UniW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    #[must_use]
    pub fn big(&mut self) -> BigW<NcfgrSpec> {
        BigW::new(self, 8)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> ClkW<NcfgrSpec> {
        ClkW::new(self, 10)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    #[must_use]
    pub fn rty(&mut self) -> RtyW<NcfgrSpec> {
        RtyW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pae(&mut self) -> PaeW<NcfgrSpec> {
        PaeW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    #[must_use]
    pub fn rbof(&mut self) -> RbofW<NcfgrSpec> {
        RbofW::new(self, 14)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlce(&mut self) -> RlceW<NcfgrSpec> {
        RlceW::new(self, 16)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    #[must_use]
    pub fn drfcs(&mut self) -> DrfcsW<NcfgrSpec> {
        DrfcsW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn efrhd(&mut self) -> EfrhdW<NcfgrSpec> {
        EfrhdW::new(self, 18)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    #[must_use]
    pub fn irxfcs(&mut self) -> IrxfcsW<NcfgrSpec> {
        IrxfcsW::new(self, 19)
    }
}
#[doc = "Network Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcfgrSpec;
impl crate::RegisterSpec for NcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncfgr::R`](R) reader structure"]
impl crate::Readable for NcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncfgr::W`](W) writer structure"]
impl crate::Writable for NcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCFGR to value 0x0800"]
impl crate::Resettable for NcfgrSpec {
    const RESET_VALUE: u32 = 0x0800;
}
