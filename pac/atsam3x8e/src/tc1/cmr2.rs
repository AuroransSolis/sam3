#[doc = "Register `CMR2` reader"]
pub type R = crate::R<Cmr2Spec>;
#[doc = "Register `CMR2` writer"]
pub type W = crate::W<Cmr2Spec>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcclks {
    #[doc = "0: Clock selected: TCLK1"]
    TimerClock1 = 0,
    #[doc = "1: Clock selected: TCLK2"]
    TimerClock2 = 1,
    #[doc = "2: Clock selected: TCLK3"]
    TimerClock3 = 2,
    #[doc = "3: Clock selected: TCLK4"]
    TimerClock4 = 3,
    #[doc = "4: Clock selected: TCLK5"]
    TimerClock5 = 4,
    #[doc = "5: Clock selected: XC0"]
    Xc0 = 5,
    #[doc = "6: Clock selected: XC1"]
    Xc1 = 6,
    #[doc = "7: Clock selected: XC2"]
    Xc2 = 7,
}
impl From<Tcclks> for u8 {
    #[inline(always)]
    fn from(variant: Tcclks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcclks {
    type Ux = u8;
}
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub type TcclksR = crate::FieldReader<Tcclks>;
impl TcclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcclks {
        match self.bits {
            0 => Tcclks::TimerClock1,
            1 => Tcclks::TimerClock2,
            2 => Tcclks::TimerClock3,
            3 => Tcclks::TimerClock4,
            4 => Tcclks::TimerClock5,
            5 => Tcclks::Xc0,
            6 => Tcclks::Xc1,
            7 => Tcclks::Xc2,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock selected: TCLK1"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == Tcclks::TimerClock1
    }
    #[doc = "Clock selected: TCLK2"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == Tcclks::TimerClock2
    }
    #[doc = "Clock selected: TCLK3"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == Tcclks::TimerClock3
    }
    #[doc = "Clock selected: TCLK4"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == Tcclks::TimerClock4
    }
    #[doc = "Clock selected: TCLK5"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == Tcclks::TimerClock5
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == Tcclks::Xc0
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == Tcclks::Xc1
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == Tcclks::Xc2
    }
}
#[doc = "Field `TCCLKS` writer - Clock Selection"]
pub type TcclksW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Tcclks>;
impl<'a, REG> TcclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock selected: TCLK1"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock1)
    }
    #[doc = "Clock selected: TCLK2"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock2)
    }
    #[doc = "Clock selected: TCLK3"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock3)
    }
    #[doc = "Clock selected: TCLK4"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock4)
    }
    #[doc = "Clock selected: TCLK5"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::Xc0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::Xc1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::Xc2)
    }
}
#[doc = "Field `CLKI` reader - Clock Invert"]
pub type ClkiR = crate::BitReader;
#[doc = "Field `CLKI` writer - Clock Invert"]
pub type ClkiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burst {
    #[doc = "0: The clock is not gated by an external signal."]
    None = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    Xc0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    Xc1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    Xc2 = 3,
}
impl From<Burst> for u8 {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burst {
    type Ux = u8;
}
#[doc = "Field `BURST` reader - Burst Signal Selection"]
pub type BurstR = crate::FieldReader<Burst>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            0 => Burst::None,
            1 => Burst::Xc0,
            2 => Burst::Xc1,
            3 => Burst::Xc2,
            _ => unreachable!(),
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Burst::None
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == Burst::Xc0
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == Burst::Xc1
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == Burst::Xc2
    }
}
#[doc = "Field `BURST` writer - Burst Signal Selection"]
pub type BurstW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Burst>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::None)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Xc0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Xc1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Xc2)
    }
}
#[doc = "Field `LDBSTOP` reader - Counter Clock Stopped with RB Loading"]
pub type LdbstopR = crate::BitReader;
#[doc = "Field `LDBSTOP` writer - Counter Clock Stopped with RB Loading"]
pub type LdbstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDBDIS` reader - Counter Clock Disable with RB Loading"]
pub type LdbdisR = crate::BitReader;
#[doc = "Field `LDBDIS` writer - Counter Clock Disable with RB Loading"]
pub type LdbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etrgedg {
    #[doc = "0: The clock is not gated by an external signal."]
    None = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Each edge"]
    Edge = 3,
}
impl From<Etrgedg> for u8 {
    #[inline(always)]
    fn from(variant: Etrgedg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etrgedg {
    type Ux = u8;
}
#[doc = "Field `ETRGEDG` reader - External Trigger Edge Selection"]
pub type EtrgedgR = crate::FieldReader<Etrgedg>;
impl EtrgedgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etrgedg {
        match self.bits {
            0 => Etrgedg::None,
            1 => Etrgedg::Rising,
            2 => Etrgedg::Falling,
            3 => Etrgedg::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Etrgedg::None
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Etrgedg::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Etrgedg::Falling
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Etrgedg::Edge
    }
}
#[doc = "Field `ETRGEDG` writer - External Trigger Edge Selection"]
pub type EtrgedgW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Etrgedg>;
impl<'a, REG> EtrgedgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedg::None)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedg::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedg::Falling)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedg::Edge)
    }
}
#[doc = "Field `ABETRG` reader - TIOA or TIOB External Trigger Selection"]
pub type AbetrgR = crate::BitReader;
#[doc = "Field `ABETRG` writer - TIOA or TIOB External Trigger Selection"]
pub type AbetrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCTRG` reader - RC Compare Trigger Enable"]
pub type CpctrgR = crate::BitReader;
#[doc = "Field `CPCTRG` writer - RC Compare Trigger Enable"]
pub type CpctrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - Waveform Mode"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Mode"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RA Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldra {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rising edge of TIOA"]
    Rising = 1,
    #[doc = "2: Falling edge of TIOA"]
    Falling = 2,
    #[doc = "3: Each edge of TIOA"]
    Edge = 3,
}
impl From<Ldra> for u8 {
    #[inline(always)]
    fn from(variant: Ldra) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldra {
    type Ux = u8;
}
#[doc = "Field `LDRA` reader - RA Loading Edge Selection"]
pub type LdraR = crate::FieldReader<Ldra>;
impl LdraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldra {
        match self.bits {
            0 => Ldra::None,
            1 => Ldra::Rising,
            2 => Ldra::Falling,
            3 => Ldra::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ldra::None
    }
    #[doc = "Rising edge of TIOA"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Ldra::Rising
    }
    #[doc = "Falling edge of TIOA"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Ldra::Falling
    }
    #[doc = "Each edge of TIOA"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Ldra::Edge
    }
}
#[doc = "Field `LDRA` writer - RA Loading Edge Selection"]
pub type LdraW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ldra>;
impl<'a, REG> LdraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ldra::None)
    }
    #[doc = "Rising edge of TIOA"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ldra::Rising)
    }
    #[doc = "Falling edge of TIOA"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ldra::Falling)
    }
    #[doc = "Each edge of TIOA"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ldra::Edge)
    }
}
#[doc = "RB Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldrb {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rising edge of TIOA"]
    Rising = 1,
    #[doc = "2: Falling edge of TIOA"]
    Falling = 2,
    #[doc = "3: Each edge of TIOA"]
    Edge = 3,
}
impl From<Ldrb> for u8 {
    #[inline(always)]
    fn from(variant: Ldrb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldrb {
    type Ux = u8;
}
#[doc = "Field `LDRB` reader - RB Loading Edge Selection"]
pub type LdrbR = crate::FieldReader<Ldrb>;
impl LdrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldrb {
        match self.bits {
            0 => Ldrb::None,
            1 => Ldrb::Rising,
            2 => Ldrb::Falling,
            3 => Ldrb::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ldrb::None
    }
    #[doc = "Rising edge of TIOA"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Ldrb::Rising
    }
    #[doc = "Falling edge of TIOA"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Ldrb::Falling
    }
    #[doc = "Each edge of TIOA"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Ldrb::Edge
    }
}
#[doc = "Field `LDRB` writer - RB Loading Edge Selection"]
pub type LdrbW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ldrb>;
impl<'a, REG> LdrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrb::None)
    }
    #[doc = "Rising edge of TIOA"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrb::Rising)
    }
    #[doc = "Falling edge of TIOA"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrb::Falling)
    }
    #[doc = "Each edge of TIOA"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrb::Edge)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TcclksR {
        TcclksR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> ClkiR {
        ClkiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LdbstopR {
        LdbstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LdbdisR {
        LdbdisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> EtrgedgR {
        EtrgedgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> AbetrgR {
        AbetrgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CpctrgR {
        CpctrgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LdraR {
        LdraR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LdrbR {
        LdrbR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tcclks(&mut self) -> TcclksW<Cmr2Spec> {
        TcclksW::new(self, 0)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    #[must_use]
    pub fn clki(&mut self) -> ClkiW<Cmr2Spec> {
        ClkiW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BurstW<Cmr2Spec> {
        BurstW::new(self, 4)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldbstop(&mut self) -> LdbstopW<Cmr2Spec> {
        LdbstopW::new(self, 6)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldbdis(&mut self) -> LdbdisW<Cmr2Spec> {
        LdbdisW::new(self, 7)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrgedg(&mut self) -> EtrgedgW<Cmr2Spec> {
        EtrgedgW::new(self, 8)
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn abetrg(&mut self) -> AbetrgW<Cmr2Spec> {
        AbetrgW::new(self, 10)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpctrg(&mut self) -> CpctrgW<Cmr2Spec> {
        CpctrgW::new(self, 14)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WaveW<Cmr2Spec> {
        WaveW::new(self, 15)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldra(&mut self) -> LdraW<Cmr2Spec> {
        LdraW::new(self, 16)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldrb(&mut self) -> LdrbW<Cmr2Spec> {
        LdrbW::new(self, 18)
    }
}
#[doc = "Channel Mode Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmr2Spec;
impl crate::RegisterSpec for Cmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr2::R`](R) reader structure"]
impl crate::Readable for Cmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cmr2::W`](W) writer structure"]
impl crate::Writable for Cmr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR2 to value 0"]
impl crate::Resettable for Cmr2Spec {
    const RESET_VALUE: u32 = 0;
}
