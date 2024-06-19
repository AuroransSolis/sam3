#[doc = "Register `CMR0_WAVEFORM_MODE` reader"]
pub type R = crate::R<WaveformModeCmr0WaveformModeSpec>;
#[doc = "Register `CMR0_WAVEFORM_MODE` writer"]
pub type W = crate::W<WaveformModeCmr0WaveformModeSpec>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcclks {
    #[doc = "0: Clock selected: internal MCK/2 clock signal (from PMC)"]
    TimerClock1 = 0,
    #[doc = "1: Clock selected: internal MCK/8 clock signal (from PMC)"]
    TimerClock2 = 1,
    #[doc = "2: Clock selected: internal MCK/32 clock signal (from PMC)"]
    TimerClock3 = 2,
    #[doc = "3: Clock selected: internal MCK/128 clock signal (from PMC)"]
    TimerClock4 = 3,
    #[doc = "4: Clock selected: internal SLCK clock signal (from PMC)"]
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
impl crate::IsEnum for Tcclks {}
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
    #[doc = "Clock selected: internal MCK/2 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == Tcclks::TimerClock1
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == Tcclks::TimerClock2
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == Tcclks::TimerClock3
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == Tcclks::TimerClock4
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
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
pub type TcclksW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tcclks, crate::Safe>;
impl<'a, REG> TcclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock selected: internal MCK/2 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclks::TimerClock4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
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
impl crate::IsEnum for Burst {}
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
pub type BurstW<'a, REG> = crate::FieldWriter<'a, REG, 2, Burst, crate::Safe>;
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
#[doc = "Field `CPCSTOP` reader - Counter Clock Stopped with RC Compare"]
pub type CpcstopR = crate::BitReader;
#[doc = "Field `CPCSTOP` writer - Counter Clock Stopped with RC Compare"]
pub type CpcstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCDIS` reader - Counter Clock Disable with RC Compare"]
pub type CpcdisR = crate::BitReader;
#[doc = "Field `CPCDIS` writer - Counter Clock Disable with RC Compare"]
pub type CpcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Event Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eevtedg {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Each edge"]
    Edge = 3,
}
impl From<Eevtedg> for u8 {
    #[inline(always)]
    fn from(variant: Eevtedg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eevtedg {
    type Ux = u8;
}
impl crate::IsEnum for Eevtedg {}
#[doc = "Field `EEVTEDG` reader - External Event Edge Selection"]
pub type EevtedgR = crate::FieldReader<Eevtedg>;
impl EevtedgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eevtedg {
        match self.bits {
            0 => Eevtedg::None,
            1 => Eevtedg::Rising,
            2 => Eevtedg::Falling,
            3 => Eevtedg::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Eevtedg::None
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Eevtedg::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Eevtedg::Falling
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Eevtedg::Edge
    }
}
#[doc = "Field `EEVTEDG` writer - External Event Edge Selection"]
pub type EevtedgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eevtedg, crate::Safe>;
impl<'a, REG> EevtedgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedg::None)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedg::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedg::Falling)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedg::Edge)
    }
}
#[doc = "External Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eevt {
    #[doc = "0: TIOB"]
    Tiob = 0,
    #[doc = "1: XC0"]
    Xc0 = 1,
    #[doc = "2: XC1"]
    Xc1 = 2,
    #[doc = "3: XC2"]
    Xc2 = 3,
}
impl From<Eevt> for u8 {
    #[inline(always)]
    fn from(variant: Eevt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eevt {
    type Ux = u8;
}
impl crate::IsEnum for Eevt {}
#[doc = "Field `EEVT` reader - External Event Selection"]
pub type EevtR = crate::FieldReader<Eevt>;
impl EevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eevt {
        match self.bits {
            0 => Eevt::Tiob,
            1 => Eevt::Xc0,
            2 => Eevt::Xc1,
            3 => Eevt::Xc2,
            _ => unreachable!(),
        }
    }
    #[doc = "TIOB"]
    #[inline(always)]
    pub fn is_tiob(&self) -> bool {
        *self == Eevt::Tiob
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == Eevt::Xc0
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == Eevt::Xc1
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == Eevt::Xc2
    }
}
#[doc = "Field `EEVT` writer - External Event Selection"]
pub type EevtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eevt, crate::Safe>;
impl<'a, REG> EevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIOB"]
    #[inline(always)]
    pub fn tiob(self) -> &'a mut crate::W<REG> {
        self.variant(Eevt::Tiob)
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut crate::W<REG> {
        self.variant(Eevt::Xc0)
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut crate::W<REG> {
        self.variant(Eevt::Xc1)
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut crate::W<REG> {
        self.variant(Eevt::Xc2)
    }
}
#[doc = "Field `ENETRG` reader - External Event Trigger Enable"]
pub type EnetrgR = crate::BitReader;
#[doc = "Field `ENETRG` writer - External Event Trigger Enable"]
pub type EnetrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Waveform Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavsel {
    #[doc = "0: UP mode without automatic trigger on RC Compare"]
    Up = 0,
    #[doc = "1: UPDOWN mode without automatic trigger on RC Compare"]
    Updown = 1,
    #[doc = "2: UP mode with automatic trigger on RC Compare"]
    UpRc = 2,
    #[doc = "3: UPDOWN mode with automatic trigger on RC Compare"]
    UpdownRc = 3,
}
impl From<Wavsel> for u8 {
    #[inline(always)]
    fn from(variant: Wavsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wavsel {
    type Ux = u8;
}
impl crate::IsEnum for Wavsel {}
#[doc = "Field `WAVSEL` reader - Waveform Selection"]
pub type WavselR = crate::FieldReader<Wavsel>;
impl WavselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wavsel {
        match self.bits {
            0 => Wavsel::Up,
            1 => Wavsel::Updown,
            2 => Wavsel::UpRc,
            3 => Wavsel::UpdownRc,
            _ => unreachable!(),
        }
    }
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Wavsel::Up
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Wavsel::Updown
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_up_rc(&self) -> bool {
        *self == Wavsel::UpRc
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_updown_rc(&self) -> bool {
        *self == Wavsel::UpdownRc
    }
}
#[doc = "Field `WAVSEL` writer - Waveform Selection"]
pub type WavselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wavsel, crate::Safe>;
impl<'a, REG> WavselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Wavsel::Up)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Wavsel::Updown)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_rc(self) -> &'a mut crate::W<REG> {
        self.variant(Wavsel::UpRc)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_rc(self) -> &'a mut crate::W<REG> {
        self.variant(Wavsel::UpdownRc)
    }
}
#[doc = "Field `WAVE` reader - Waveform Mode"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Mode"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RA Compare Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acpa {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Acpa> for u8 {
    #[inline(always)]
    fn from(variant: Acpa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acpa {
    type Ux = u8;
}
impl crate::IsEnum for Acpa {}
#[doc = "Field `ACPA` reader - RA Compare Effect on TIOA"]
pub type AcpaR = crate::FieldReader<Acpa>;
impl AcpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acpa {
        match self.bits {
            0 => Acpa::None,
            1 => Acpa::Set,
            2 => Acpa::Clear,
            3 => Acpa::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Acpa::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Acpa::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Acpa::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Acpa::Toggle
    }
}
#[doc = "Field `ACPA` writer - RA Compare Effect on TIOA"]
pub type AcpaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acpa, crate::Safe>;
impl<'a, REG> AcpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Acpa::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Acpa::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Acpa::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Acpa::Toggle)
    }
}
#[doc = "RC Compare Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acpc {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Acpc> for u8 {
    #[inline(always)]
    fn from(variant: Acpc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acpc {
    type Ux = u8;
}
impl crate::IsEnum for Acpc {}
#[doc = "Field `ACPC` reader - RC Compare Effect on TIOA"]
pub type AcpcR = crate::FieldReader<Acpc>;
impl AcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acpc {
        match self.bits {
            0 => Acpc::None,
            1 => Acpc::Set,
            2 => Acpc::Clear,
            3 => Acpc::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Acpc::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Acpc::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Acpc::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Acpc::Toggle
    }
}
#[doc = "Field `ACPC` writer - RC Compare Effect on TIOA"]
pub type AcpcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acpc, crate::Safe>;
impl<'a, REG> AcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Acpc::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Acpc::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Acpc::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Acpc::Toggle)
    }
}
#[doc = "External Event Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aeevt {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Aeevt> for u8 {
    #[inline(always)]
    fn from(variant: Aeevt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aeevt {
    type Ux = u8;
}
impl crate::IsEnum for Aeevt {}
#[doc = "Field `AEEVT` reader - External Event Effect on TIOA"]
pub type AeevtR = crate::FieldReader<Aeevt>;
impl AeevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aeevt {
        match self.bits {
            0 => Aeevt::None,
            1 => Aeevt::Set,
            2 => Aeevt::Clear,
            3 => Aeevt::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Aeevt::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Aeevt::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Aeevt::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Aeevt::Toggle
    }
}
#[doc = "Field `AEEVT` writer - External Event Effect on TIOA"]
pub type AeevtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aeevt, crate::Safe>;
impl<'a, REG> AeevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevt::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevt::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevt::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevt::Toggle)
    }
}
#[doc = "Software Trigger Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aswtrg {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Aswtrg> for u8 {
    #[inline(always)]
    fn from(variant: Aswtrg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aswtrg {
    type Ux = u8;
}
impl crate::IsEnum for Aswtrg {}
#[doc = "Field `ASWTRG` reader - Software Trigger Effect on TIOA"]
pub type AswtrgR = crate::FieldReader<Aswtrg>;
impl AswtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aswtrg {
        match self.bits {
            0 => Aswtrg::None,
            1 => Aswtrg::Set,
            2 => Aswtrg::Clear,
            3 => Aswtrg::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Aswtrg::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Aswtrg::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Aswtrg::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Aswtrg::Toggle
    }
}
#[doc = "Field `ASWTRG` writer - Software Trigger Effect on TIOA"]
pub type AswtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aswtrg, crate::Safe>;
impl<'a, REG> AswtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrg::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrg::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrg::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrg::Toggle)
    }
}
#[doc = "RB Compare Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcpb {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Bcpb> for u8 {
    #[inline(always)]
    fn from(variant: Bcpb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcpb {
    type Ux = u8;
}
impl crate::IsEnum for Bcpb {}
#[doc = "Field `BCPB` reader - RB Compare Effect on TIOB"]
pub type BcpbR = crate::FieldReader<Bcpb>;
impl BcpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcpb {
        match self.bits {
            0 => Bcpb::None,
            1 => Bcpb::Set,
            2 => Bcpb::Clear,
            3 => Bcpb::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Bcpb::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bcpb::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Bcpb::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Bcpb::Toggle
    }
}
#[doc = "Field `BCPB` writer - RB Compare Effect on TIOB"]
pub type BcpbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bcpb, crate::Safe>;
impl<'a, REG> BcpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpb::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpb::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpb::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpb::Toggle)
    }
}
#[doc = "RC Compare Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcpc {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Bcpc> for u8 {
    #[inline(always)]
    fn from(variant: Bcpc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcpc {
    type Ux = u8;
}
impl crate::IsEnum for Bcpc {}
#[doc = "Field `BCPC` reader - RC Compare Effect on TIOB"]
pub type BcpcR = crate::FieldReader<Bcpc>;
impl BcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcpc {
        match self.bits {
            0 => Bcpc::None,
            1 => Bcpc::Set,
            2 => Bcpc::Clear,
            3 => Bcpc::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Bcpc::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bcpc::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Bcpc::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Bcpc::Toggle
    }
}
#[doc = "Field `BCPC` writer - RC Compare Effect on TIOB"]
pub type BcpcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bcpc, crate::Safe>;
impl<'a, REG> BcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpc::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpc::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpc::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpc::Toggle)
    }
}
#[doc = "External Event Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Beevt {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Beevt> for u8 {
    #[inline(always)]
    fn from(variant: Beevt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Beevt {
    type Ux = u8;
}
impl crate::IsEnum for Beevt {}
#[doc = "Field `BEEVT` reader - External Event Effect on TIOB"]
pub type BeevtR = crate::FieldReader<Beevt>;
impl BeevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Beevt {
        match self.bits {
            0 => Beevt::None,
            1 => Beevt::Set,
            2 => Beevt::Clear,
            3 => Beevt::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Beevt::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Beevt::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Beevt::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Beevt::Toggle
    }
}
#[doc = "Field `BEEVT` writer - External Event Effect on TIOB"]
pub type BeevtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Beevt, crate::Safe>;
impl<'a, REG> BeevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Beevt::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Beevt::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Beevt::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Beevt::Toggle)
    }
}
#[doc = "Software Trigger Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bswtrg {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "2: Clear"]
    Clear = 2,
    #[doc = "3: Toggle"]
    Toggle = 3,
}
impl From<Bswtrg> for u8 {
    #[inline(always)]
    fn from(variant: Bswtrg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bswtrg {
    type Ux = u8;
}
impl crate::IsEnum for Bswtrg {}
#[doc = "Field `BSWTRG` reader - Software Trigger Effect on TIOB"]
pub type BswtrgR = crate::FieldReader<Bswtrg>;
impl BswtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bswtrg {
        match self.bits {
            0 => Bswtrg::None,
            1 => Bswtrg::Set,
            2 => Bswtrg::Clear,
            3 => Bswtrg::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Bswtrg::None
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bswtrg::Set
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Bswtrg::Clear
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Bswtrg::Toggle
    }
}
#[doc = "Field `BSWTRG` writer - Software Trigger Effect on TIOB"]
pub type BswtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bswtrg, crate::Safe>;
impl<'a, REG> BswtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrg::None)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrg::Set)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrg::Clear)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrg::Toggle)
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
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&self) -> CpcstopR {
        CpcstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline(always)]
    pub fn cpcdis(&self) -> CpcdisR {
        CpcdisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&self) -> EevtedgR {
        EevtedgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&self) -> EevtR {
        EevtR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&self) -> EnetrgR {
        EnetrgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&self) -> WavselR {
        WavselR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpa(&self) -> AcpaR {
        AcpaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpc(&self) -> AcpcR {
        AcpcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline(always)]
    pub fn aeevt(&self) -> AeevtR {
        AeevtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline(always)]
    pub fn aswtrg(&self) -> AswtrgR {
        AswtrgR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpb(&self) -> BcpbR {
        BcpbR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpc(&self) -> BcpcR {
        BcpcR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline(always)]
    pub fn beevt(&self) -> BeevtR {
        BeevtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline(always)]
    pub fn bswtrg(&self) -> BswtrgR {
        BswtrgR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tcclks(&mut self) -> TcclksW<WaveformModeCmr0WaveformModeSpec> {
        TcclksW::new(self, 0)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    #[must_use]
    pub fn clki(&mut self) -> ClkiW<WaveformModeCmr0WaveformModeSpec> {
        ClkiW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BurstW<WaveformModeCmr0WaveformModeSpec> {
        BurstW::new(self, 4)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcstop(&mut self) -> CpcstopW<WaveformModeCmr0WaveformModeSpec> {
        CpcstopW::new(self, 6)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcdis(&mut self) -> CpcdisW<WaveformModeCmr0WaveformModeSpec> {
        CpcdisW::new(self, 7)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn eevtedg(&mut self) -> EevtedgW<WaveformModeCmr0WaveformModeSpec> {
        EevtedgW::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn eevt(&mut self) -> EevtW<WaveformModeCmr0WaveformModeSpec> {
        EevtW::new(self, 10)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enetrg(&mut self) -> EnetrgW<WaveformModeCmr0WaveformModeSpec> {
        EnetrgW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wavsel(&mut self) -> WavselW<WaveformModeCmr0WaveformModeSpec> {
        WavselW::new(self, 13)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WaveW<WaveformModeCmr0WaveformModeSpec> {
        WaveW::new(self, 15)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn acpa(&mut self) -> AcpaW<WaveformModeCmr0WaveformModeSpec> {
        AcpaW::new(self, 16)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn acpc(&mut self) -> AcpcW<WaveformModeCmr0WaveformModeSpec> {
        AcpcW::new(self, 18)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn aeevt(&mut self) -> AeevtW<WaveformModeCmr0WaveformModeSpec> {
        AeevtW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn aswtrg(&mut self) -> AswtrgW<WaveformModeCmr0WaveformModeSpec> {
        AswtrgW::new(self, 22)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn bcpb(&mut self) -> BcpbW<WaveformModeCmr0WaveformModeSpec> {
        BcpbW::new(self, 24)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn bcpc(&mut self) -> BcpcW<WaveformModeCmr0WaveformModeSpec> {
        BcpcW::new(self, 26)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn beevt(&mut self) -> BeevtW<WaveformModeCmr0WaveformModeSpec> {
        BeevtW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn bswtrg(&mut self) -> BswtrgW<WaveformModeCmr0WaveformModeSpec> {
        BswtrgW::new(self, 30)
    }
}
#[doc = "Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`waveform_mode_cmr0_waveform_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`waveform_mode_cmr0_waveform_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaveformModeCmr0WaveformModeSpec;
impl crate::RegisterSpec for WaveformModeCmr0WaveformModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waveform_mode_cmr0_waveform_mode::R`](R) reader structure"]
impl crate::Readable for WaveformModeCmr0WaveformModeSpec {}
#[doc = "`write(|w| ..)` method takes [`waveform_mode_cmr0_waveform_mode::W`](W) writer structure"]
impl crate::Writable for WaveformModeCmr0WaveformModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR0_WAVEFORM_MODE to value 0"]
impl crate::Resettable for WaveformModeCmr0WaveformModeSpec {
    const RESET_VALUE: u32 = 0;
}
