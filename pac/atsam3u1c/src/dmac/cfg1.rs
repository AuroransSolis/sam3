#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `SRC_PER` reader - Source with Peripheral identifier"]
pub type SRC_PER_R = crate::FieldReader;
#[doc = "Field `SRC_PER` writer - Source with Peripheral identifier"]
pub type SRC_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DST_PER` reader - Destination with Peripheral identifier"]
pub type DST_PER_R = crate::FieldReader;
#[doc = "Field `DST_PER` writer - Destination with Peripheral identifier"]
pub type DST_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRC_H2SEL` reader - Software or Hardware Selection for the Source"]
pub type SRC_H2SEL_R = crate::BitReader<SRC_H2SEL_A>;
#[doc = "Software or Hardware Selection for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_H2SEL_A {
    #[doc = "0: Software handshaking interface is used to trigger a transfer request."]
    Sw = 0,
    #[doc = "1: Hardware handshaking interface is used to trigger a transfer request."]
    Hw = 1,
}
impl From<SRC_H2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_H2SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_H2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRC_H2SEL_A {
        match self.bits {
            false => SRC_H2SEL_A::Sw,
            true => SRC_H2SEL_A::Hw,
        }
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == SRC_H2SEL_A::Sw
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == SRC_H2SEL_A::Hw
    }
}
#[doc = "Field `SRC_H2SEL` writer - Software or Hardware Selection for the Source"]
pub type SRC_H2SEL_W<'a, REG> = crate::BitWriter<'a, REG, SRC_H2SEL_A>;
impl<'a, REG> SRC_H2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_H2SEL_A::Sw)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_H2SEL_A::Hw)
    }
}
#[doc = "Field `DST_H2SEL` reader - Software or Hardware Selection for the Destination"]
pub type DST_H2SEL_R = crate::BitReader<DST_H2SEL_A>;
#[doc = "Software or Hardware Selection for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DST_H2SEL_A {
    #[doc = "0: Software handshaking interface is used to trigger a transfer request."]
    Sw = 0,
    #[doc = "1: Hardware handshaking interface is used to trigger a transfer request."]
    Hw = 1,
}
impl From<DST_H2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DST_H2SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_H2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DST_H2SEL_A {
        match self.bits {
            false => DST_H2SEL_A::Sw,
            true => DST_H2SEL_A::Hw,
        }
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == DST_H2SEL_A::Sw
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == DST_H2SEL_A::Hw
    }
}
#[doc = "Field `DST_H2SEL` writer - Software or Hardware Selection for the Destination"]
pub type DST_H2SEL_W<'a, REG> = crate::BitWriter<'a, REG, DST_H2SEL_A>;
impl<'a, REG> DST_H2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(DST_H2SEL_A::Sw)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(DST_H2SEL_A::Hw)
    }
}
#[doc = "Field `SOD` reader - Stop On Done"]
pub type SOD_R = crate::BitReader<SOD_A>;
#[doc = "Stop On Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOD_A {
    #[doc = "0: STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    Disable = 0,
    #[doc = "1: STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    Enable = 1,
}
impl From<SOD_A> for bool {
    #[inline(always)]
    fn from(variant: SOD_A) -> Self {
        variant as u8 != 0
    }
}
impl SOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOD_A {
        match self.bits {
            false => SOD_A::Disable,
            true => SOD_A::Enable,
        }
    }
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOD_A::Disable
    }
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOD_A::Enable
    }
}
#[doc = "Field `SOD` writer - Stop On Done"]
pub type SOD_W<'a, REG> = crate::BitWriter<'a, REG, SOD_A>;
impl<'a, REG> SOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SOD_A::Disable)
    }
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SOD_A::Enable)
    }
}
#[doc = "Field `LOCK_IF` reader - Interface Lock"]
pub type LOCK_IF_R = crate::BitReader<LOCK_IF_A>;
#[doc = "Interface Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_IF_A {
    #[doc = "0: Interface Lock capability is disabled"]
    Disable = 0,
    #[doc = "1: Interface Lock capability is enabled"]
    Enable = 1,
}
impl From<LOCK_IF_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_IF_A {
        match self.bits {
            false => LOCK_IF_A::Disable,
            true => LOCK_IF_A::Enable,
        }
    }
    #[doc = "Interface Lock capability is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_IF_A::Disable
    }
    #[doc = "Interface Lock capability is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOCK_IF_A::Enable
    }
}
#[doc = "Field `LOCK_IF` writer - Interface Lock"]
pub type LOCK_IF_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_IF_A>;
impl<'a, REG> LOCK_IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interface Lock capability is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_IF_A::Disable)
    }
    #[doc = "Interface Lock capability is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_IF_A::Enable)
    }
}
#[doc = "Field `LOCK_B` reader - Bus Lock"]
pub type LOCK_B_R = crate::BitReader<LOCK_B_A>;
#[doc = "Bus Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_B_A {
    #[doc = "0: AHB Bus Locking capability is disabled."]
    Disable = 0,
}
impl From<LOCK_B_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_B_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCK_B_A> {
        match self.bits {
            false => Some(LOCK_B_A::Disable),
            _ => None,
        }
    }
    #[doc = "AHB Bus Locking capability is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_B_A::Disable
    }
}
#[doc = "Field `LOCK_B` writer - Bus Lock"]
pub type LOCK_B_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_B_A>;
impl<'a, REG> LOCK_B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB Bus Locking capability is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_B_A::Disable)
    }
}
#[doc = "Field `LOCK_IF_L` reader - Master Interface Arbiter Lock"]
pub type LOCK_IF_L_R = crate::BitReader<LOCK_IF_L_A>;
#[doc = "Master Interface Arbiter Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_IF_L_A {
    #[doc = "0: The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    Chunk = 0,
    #[doc = "1: The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    Buffer = 1,
}
impl From<LOCK_IF_L_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_IF_L_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_IF_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_IF_L_A {
        match self.bits {
            false => LOCK_IF_L_A::Chunk,
            true => LOCK_IF_L_A::Buffer,
        }
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    #[inline(always)]
    pub fn is_chunk(&self) -> bool {
        *self == LOCK_IF_L_A::Chunk
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    #[inline(always)]
    pub fn is_buffer(&self) -> bool {
        *self == LOCK_IF_L_A::Buffer
    }
}
#[doc = "Field `LOCK_IF_L` writer - Master Interface Arbiter Lock"]
pub type LOCK_IF_L_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_IF_L_A>;
impl<'a, REG> LOCK_IF_L_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    #[inline(always)]
    pub fn chunk(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_IF_L_A::Chunk)
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    #[inline(always)]
    pub fn buffer(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_IF_L_A::Buffer)
    }
}
#[doc = "Field `AHB_PROT` reader - AHB Protection"]
pub type AHB_PROT_R = crate::FieldReader;
#[doc = "Field `AHB_PROT` writer - AHB Protection"]
pub type AHB_PROT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIFOCFG` reader - FIFO Configuration"]
pub type FIFOCFG_R = crate::FieldReader<FIFOCFG_A>;
#[doc = "FIFO Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFOCFG_A {
    #[doc = "0: The largest defined length AHB burst is performed on the destination AHB interface."]
    AlapCfg = 0,
    #[doc = "1: When half FIFO size is available/filled, a source/destination request is serviced."]
    HalfCfg = 1,
    #[doc = "2: When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    AsapCfg = 2,
}
impl From<FIFOCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FIFOCFG_A {
    type Ux = u8;
}
impl FIFOCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FIFOCFG_A> {
        match self.bits {
            0 => Some(FIFOCFG_A::AlapCfg),
            1 => Some(FIFOCFG_A::HalfCfg),
            2 => Some(FIFOCFG_A::AsapCfg),
            _ => None,
        }
    }
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    #[inline(always)]
    pub fn is_alap_cfg(&self) -> bool {
        *self == FIFOCFG_A::AlapCfg
    }
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    #[inline(always)]
    pub fn is_half_cfg(&self) -> bool {
        *self == FIFOCFG_A::HalfCfg
    }
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    #[inline(always)]
    pub fn is_asap_cfg(&self) -> bool {
        *self == FIFOCFG_A::AsapCfg
    }
}
#[doc = "Field `FIFOCFG` writer - FIFO Configuration"]
pub type FIFOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FIFOCFG_A>;
impl<'a, REG> FIFOCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    #[inline(always)]
    pub fn alap_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOCFG_A::AlapCfg)
    }
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    #[inline(always)]
    pub fn half_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOCFG_A::HalfCfg)
    }
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    #[inline(always)]
    pub fn asap_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOCFG_A::AsapCfg)
    }
}
impl R {
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline(always)]
    pub fn dst_per(&self) -> DST_PER_R {
        DST_PER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline(always)]
    pub fn src_h2sel(&self) -> SRC_H2SEL_R {
        SRC_H2SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline(always)]
    pub fn dst_h2sel(&self) -> DST_H2SEL_R {
        DST_H2SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline(always)]
    pub fn lock_if(&self) -> LOCK_IF_R {
        LOCK_IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline(always)]
    pub fn lock_b(&self) -> LOCK_B_R {
        LOCK_B_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline(always)]
    pub fn lock_if_l(&self) -> LOCK_IF_L_R {
        LOCK_IF_L_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline(always)]
    pub fn ahb_prot(&self) -> AHB_PROT_R {
        AHB_PROT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline(always)]
    pub fn fifocfg(&self) -> FIFOCFG_R {
        FIFOCFG_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn src_per(&mut self) -> SRC_PER_W<CFG1_SPEC> {
        SRC_PER_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn dst_per(&mut self) -> DST_PER_W<CFG1_SPEC> {
        DST_PER_W::new(self, 4)
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline(always)]
    #[must_use]
    pub fn src_h2sel(&mut self) -> SRC_H2SEL_W<CFG1_SPEC> {
        SRC_H2SEL_W::new(self, 9)
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline(always)]
    #[must_use]
    pub fn dst_h2sel(&mut self) -> DST_H2SEL_W<CFG1_SPEC> {
        DST_H2SEL_W::new(self, 13)
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SOD_W<CFG1_SPEC> {
        SOD_W::new(self, 16)
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_if(&mut self) -> LOCK_IF_W<CFG1_SPEC> {
        LOCK_IF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b(&mut self) -> LOCK_B_W<CFG1_SPEC> {
        LOCK_B_W::new(self, 21)
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_if_l(&mut self) -> LOCK_IF_L_W<CFG1_SPEC> {
        LOCK_IF_L_W::new(self, 22)
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_prot(&mut self) -> AHB_PROT_W<CFG1_SPEC> {
        AHB_PROT_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fifocfg(&mut self) -> FIFOCFG_W<CFG1_SPEC> {
        FIFOCFG_W::new(self, 28)
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
#[doc = "DMAC Channel Configuration Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0x0100_0000"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
