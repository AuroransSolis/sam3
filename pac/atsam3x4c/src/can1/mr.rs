#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `CANEN` reader - CAN Controller Enable"]
pub type CanenR = crate::BitReader;
#[doc = "Field `CANEN` writer - CAN Controller Enable"]
pub type CanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM` reader - Disable/Enable Low Power Mode"]
pub type LpmR = crate::BitReader;
#[doc = "Field `LPM` writer - Disable/Enable Low Power Mode"]
pub type LpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABM` reader - Disable/Enable Autobaud/Listen mode"]
pub type AbmR = crate::BitReader;
#[doc = "Field `ABM` writer - Disable/Enable Autobaud/Listen mode"]
pub type AbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVL` reader - Disable/Enable Overload Frame"]
pub type OvlR = crate::BitReader;
#[doc = "Field `OVL` writer - Disable/Enable Overload Frame"]
pub type OvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEOF` reader - Timestamp messages at each end of Frame"]
pub type TeofR = crate::BitReader;
#[doc = "Field `TEOF` writer - Timestamp messages at each end of Frame"]
pub type TeofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTM` reader - Disable/Enable Time Triggered Mode"]
pub type TtmR = crate::BitReader;
#[doc = "Field `TTM` writer - Disable/Enable Time Triggered Mode"]
pub type TtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMFRZ` reader - Enable Timer Freeze"]
pub type TimfrzR = crate::BitReader;
#[doc = "Field `TIMFRZ` writer - Enable Timer Freeze"]
pub type TimfrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRPT` reader - Disable Repeat"]
pub type DrptR = crate::BitReader;
#[doc = "Field `DRPT` writer - Disable Repeat"]
pub type DrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reception Synchronization Stage (not readable)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxsync {
    #[doc = "0: Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DoublePp = 0,
    #[doc = "1: Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DoublePn = 1,
    #[doc = "2: Rx Signal with Single Synchro Stage (Positive Edge)"]
    SingleP = 2,
    #[doc = "3: Rx Signal with No Synchro Stage"]
    None = 3,
}
impl From<Rxsync> for u8 {
    #[inline(always)]
    fn from(variant: Rxsync) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxsync {
    type Ux = u8;
}
#[doc = "Field `RXSYNC` reader - Reception Synchronization Stage (not readable)"]
pub type RxsyncR = crate::FieldReader<Rxsync>;
impl RxsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxsync> {
        match self.bits {
            0 => Some(Rxsync::DoublePp),
            1 => Some(Rxsync::DoublePn),
            2 => Some(Rxsync::SingleP),
            3 => Some(Rxsync::None),
            _ => None,
        }
    }
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn is_double_pp(&self) -> bool {
        *self == Rxsync::DoublePp
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn is_double_pn(&self) -> bool {
        *self == Rxsync::DoublePn
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn is_single_p(&self) -> bool {
        *self == Rxsync::SingleP
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rxsync::None
    }
}
#[doc = "Field `RXSYNC` writer - Reception Synchronization Stage (not readable)"]
pub type RxsyncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxsync>;
impl<'a, REG> RxsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn double_pp(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsync::DoublePp)
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn double_pn(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsync::DoublePn)
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn single_p(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsync::SingleP)
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsync::None)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CanenR {
        CanenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LpmR {
        LpmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&self) -> AbmR {
        AbmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&self) -> OvlR {
        OvlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&self) -> TeofR {
        TeofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&self) -> TtmR {
        TtmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&self) -> TimfrzR {
        TimfrzR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&self) -> DrptR {
        DrptR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&self) -> RxsyncR {
        RxsyncR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CanenW<MrSpec> {
        CanenW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LpmW<MrSpec> {
        LpmW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn abm(&mut self) -> AbmW<MrSpec> {
        AbmW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ovl(&mut self) -> OvlW<MrSpec> {
        OvlW::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn teof(&mut self) -> TeofW<MrSpec> {
        TeofW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ttm(&mut self) -> TtmW<MrSpec> {
        TtmW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn timfrz(&mut self) -> TimfrzW<MrSpec> {
        TimfrzW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    #[must_use]
    pub fn drpt(&mut self) -> DrptW<MrSpec> {
        DrptW::new(self, 7)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    #[must_use]
    pub fn rxsync(&mut self) -> RxsyncW<MrSpec> {
        RxsyncW::new(self, 24)
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
