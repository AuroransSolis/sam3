#[doc = "Register `WUMR` reader"]
pub type R = crate::R<WumrSpec>;
#[doc = "Register `WUMR` writer"]
pub type W = crate::W<WumrSpec>;
#[doc = "Supply Monitor Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smen {
    #[doc = "0: the supply monitor detection has no wake up effect."]
    NotEnable = 0,
    #[doc = "1: the supply monitor detection forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Smen> for bool {
    #[inline(always)]
    fn from(variant: Smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake Up Enable"]
pub type SmenR = crate::BitReader<Smen>;
impl SmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smen {
        match self.bits {
            false => Smen::NotEnable,
            true => Smen::Enable,
        }
    }
    #[doc = "the supply monitor detection has no wake up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smen::NotEnable
    }
    #[doc = "the supply monitor detection forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smen::Enable
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake Up Enable"]
pub type SmenW<'a, REG> = crate::BitWriter<'a, REG, Smen>;
impl<'a, REG> SmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the supply monitor detection has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smen::NotEnable)
    }
    #[doc = "the supply monitor detection forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smen::Enable)
    }
}
#[doc = "Real Time Timer Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtten {
    #[doc = "0: the RTT alarm signal has no wake up effect."]
    NotEnable = 0,
    #[doc = "1: the RTT alarm signal forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Rtten> for bool {
    #[inline(always)]
    fn from(variant: Rtten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTTEN` reader - Real Time Timer Wake Up Enable"]
pub type RttenR = crate::BitReader<Rtten>;
impl RttenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtten {
        match self.bits {
            false => Rtten::NotEnable,
            true => Rtten::Enable,
        }
    }
    #[doc = "the RTT alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Rtten::NotEnable
    }
    #[doc = "the RTT alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtten::Enable
    }
}
#[doc = "Field `RTTEN` writer - Real Time Timer Wake Up Enable"]
pub type RttenW<'a, REG> = crate::BitWriter<'a, REG, Rtten>;
impl<'a, REG> RttenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTT alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtten::NotEnable)
    }
    #[doc = "the RTT alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtten::Enable)
    }
}
#[doc = "Real Time Clock Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcen {
    #[doc = "0: the RTC alarm signal has no wake up effect."]
    NotEnable = 0,
    #[doc = "1: the RTC alarm signal forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Rtcen> for bool {
    #[inline(always)]
    fn from(variant: Rtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - Real Time Clock Wake Up Enable"]
pub type RtcenR = crate::BitReader<Rtcen>;
impl RtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcen {
        match self.bits {
            false => Rtcen::NotEnable,
            true => Rtcen::Enable,
        }
    }
    #[doc = "the RTC alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Rtcen::NotEnable
    }
    #[doc = "the RTC alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtcen::Enable
    }
}
#[doc = "Field `RTCEN` writer - Real Time Clock Wake Up Enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcen>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTC alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::NotEnable)
    }
    #[doc = "the RTC alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::Enable)
    }
}
#[doc = "Low power Debouncer ENable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcen0 {
    #[doc = "0: the WKUP0 input pin is not connected with low power debouncer."]
    NotEnable = 0,
    #[doc = "1: the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    Enable = 1,
}
impl From<Lpdbcen0> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN0` reader - Low power Debouncer ENable WKUP0"]
pub type Lpdbcen0R = crate::BitReader<Lpdbcen0>;
impl Lpdbcen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcen0 {
        match self.bits {
            false => Lpdbcen0::NotEnable,
            true => Lpdbcen0::Enable,
        }
    }
    #[doc = "the WKUP0 input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Lpdbcen0::NotEnable
    }
    #[doc = "the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpdbcen0::Enable
    }
}
#[doc = "Field `LPDBCEN0` writer - Low power Debouncer ENable WKUP0"]
pub type Lpdbcen0W<'a, REG> = crate::BitWriter<'a, REG, Lpdbcen0>;
impl<'a, REG> Lpdbcen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the WKUP0 input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen0::NotEnable)
    }
    #[doc = "the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen0::Enable)
    }
}
#[doc = "Low power Debouncer ENable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcen1 {
    #[doc = "0: the WKUP1input pin is not connected with low power debouncer."]
    NotEnable = 0,
    #[doc = "1: the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    Enable = 1,
}
impl From<Lpdbcen1> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN1` reader - Low power Debouncer ENable WKUP1"]
pub type Lpdbcen1R = crate::BitReader<Lpdbcen1>;
impl Lpdbcen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcen1 {
        match self.bits {
            false => Lpdbcen1::NotEnable,
            true => Lpdbcen1::Enable,
        }
    }
    #[doc = "the WKUP1input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Lpdbcen1::NotEnable
    }
    #[doc = "the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpdbcen1::Enable
    }
}
#[doc = "Field `LPDBCEN1` writer - Low power Debouncer ENable WKUP1"]
pub type Lpdbcen1W<'a, REG> = crate::BitWriter<'a, REG, Lpdbcen1>;
impl<'a, REG> Lpdbcen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the WKUP1input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen1::NotEnable)
    }
    #[doc = "the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen1::Enable)
    }
}
#[doc = "Low power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcclr {
    #[doc = "0: a low power debounce event does not create an immediate clear on first half GPBR registers."]
    NotEnable = 0,
    #[doc = "1: a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    Enable = 1,
}
impl From<Lpdbcclr> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCCLR` reader - Low power Debouncer Clear"]
pub type LpdbcclrR = crate::BitReader<Lpdbcclr>;
impl LpdbcclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcclr {
        match self.bits {
            false => Lpdbcclr::NotEnable,
            true => Lpdbcclr::Enable,
        }
    }
    #[doc = "a low power debounce event does not create an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Lpdbcclr::NotEnable
    }
    #[doc = "a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpdbcclr::Enable
    }
}
#[doc = "Field `LPDBCCLR` writer - Low power Debouncer Clear"]
pub type LpdbcclrW<'a, REG> = crate::BitWriter<'a, REG, Lpdbcclr>;
impl<'a, REG> LpdbcclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low power debounce event does not create an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcclr::NotEnable)
    }
    #[doc = "a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcclr::Enable)
    }
}
#[doc = "Wake Up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wkupdbc {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    Immediate = 0,
    #[doc = "1: WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3Sclk = 1,
    #[doc = "2: WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32Sclk = 2,
    #[doc = "3: WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512Sclk = 3,
    #[doc = "4: WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096Sclk = 4,
    #[doc = "5: WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768Sclk = 5,
}
impl From<Wkupdbc> for u8 {
    #[inline(always)]
    fn from(variant: Wkupdbc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wkupdbc {
    type Ux = u8;
}
impl crate::IsEnum for Wkupdbc {}
#[doc = "Field `WKUPDBC` reader - Wake Up Inputs Debouncer Period"]
pub type WkupdbcR = crate::FieldReader<Wkupdbc>;
impl WkupdbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wkupdbc> {
        match self.bits {
            0 => Some(Wkupdbc::Immediate),
            1 => Some(Wkupdbc::_3Sclk),
            2 => Some(Wkupdbc::_32Sclk),
            3 => Some(Wkupdbc::_512Sclk),
            4 => Some(Wkupdbc::_4096Sclk),
            5 => Some(Wkupdbc::_32768Sclk),
            _ => None,
        }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Wkupdbc::Immediate
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn is_3_sclk(&self) -> bool {
        *self == Wkupdbc::_3Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32_sclk(&self) -> bool {
        *self == Wkupdbc::_32Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn is_512_sclk(&self) -> bool {
        *self == Wkupdbc::_512Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn is_4096_sclk(&self) -> bool {
        *self == Wkupdbc::_4096Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn is_32768_sclk(&self) -> bool {
        *self == Wkupdbc::_32768Sclk
    }
}
#[doc = "Field `WKUPDBC` writer - Wake Up Inputs Debouncer Period"]
pub type WkupdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wkupdbc>;
impl<'a, REG> WkupdbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbc::Immediate)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbc::_3Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbc::_32Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbc::_512Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbc::_4096Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbc::_32768Sclk)
    }
}
#[doc = "Low Power DeBounCer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpdbc {
    #[doc = "0: Disable the low power debouncer."]
    Disable = 0,
    #[doc = "1: WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    _2Rtcout0 = 1,
    #[doc = "2: WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    _3Rtcout0 = 2,
    #[doc = "3: WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    _4Rtcout0 = 3,
    #[doc = "4: WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    _5Rtcout0 = 4,
    #[doc = "5: WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    _6Rtcout0 = 5,
    #[doc = "6: WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    _7Rtcout0 = 6,
    #[doc = "7: WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    _8Rtcout0 = 7,
}
impl From<Lpdbc> for u8 {
    #[inline(always)]
    fn from(variant: Lpdbc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpdbc {
    type Ux = u8;
}
impl crate::IsEnum for Lpdbc {}
#[doc = "Field `LPDBC` reader - Low Power DeBounCer Period"]
pub type LpdbcR = crate::FieldReader<Lpdbc>;
impl LpdbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbc {
        match self.bits {
            0 => Lpdbc::Disable,
            1 => Lpdbc::_2Rtcout0,
            2 => Lpdbc::_3Rtcout0,
            3 => Lpdbc::_4Rtcout0,
            4 => Lpdbc::_5Rtcout0,
            5 => Lpdbc::_6Rtcout0,
            6 => Lpdbc::_7Rtcout0,
            7 => Lpdbc::_8Rtcout0,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable the low power debouncer."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lpdbc::Disable
    }
    #[doc = "WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_2_rtcout0(&self) -> bool {
        *self == Lpdbc::_2Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_3_rtcout0(&self) -> bool {
        *self == Lpdbc::_3Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_4_rtcout0(&self) -> bool {
        *self == Lpdbc::_4Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_5_rtcout0(&self) -> bool {
        *self == Lpdbc::_5Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_6_rtcout0(&self) -> bool {
        *self == Lpdbc::_6Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_7_rtcout0(&self) -> bool {
        *self == Lpdbc::_7Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_8_rtcout0(&self) -> bool {
        *self == Lpdbc::_8Rtcout0
    }
}
#[doc = "Field `LPDBC` writer - Low Power DeBounCer Period"]
pub type LpdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lpdbc, crate::Safe>;
impl<'a, REG> LpdbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the low power debouncer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::Disable)
    }
    #[doc = "WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _2_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_2Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _3_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_3Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _4_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_4Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _5_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_5Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _6_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_6Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _7_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_7Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _8_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbc::_8Rtcout0)
    }
}
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake Up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SmenR {
        SmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Timer Wake Up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RttenR {
        RttenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Wake Up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Low power Debouncer ENable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> Lpdbcen0R {
        Lpdbcen0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low power Debouncer ENable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> Lpdbcen1R {
        Lpdbcen1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LpdbcclrR {
        LpdbcclrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Wake Up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WkupdbcR {
        WkupdbcR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Low Power DeBounCer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LpdbcR {
        LpdbcR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SmenW<WumrSpec> {
        SmenW::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Timer Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtten(&mut self) -> RttenW<WumrSpec> {
        RttenW::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RtcenW<WumrSpec> {
        RtcenW::new(self, 3)
    }
    #[doc = "Bit 5 - Low power Debouncer ENable WKUP0"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcen0(&mut self) -> Lpdbcen0W<WumrSpec> {
        Lpdbcen0W::new(self, 5)
    }
    #[doc = "Bit 6 - Low power Debouncer ENable WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcen1(&mut self) -> Lpdbcen1W<WumrSpec> {
        Lpdbcen1W::new(self, 6)
    }
    #[doc = "Bit 7 - Low power Debouncer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcclr(&mut self) -> LpdbcclrW<WumrSpec> {
        LpdbcclrW::new(self, 7)
    }
    #[doc = "Bits 12:14 - Wake Up Inputs Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn wkupdbc(&mut self) -> WkupdbcW<WumrSpec> {
        WkupdbcW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Low Power DeBounCer Period"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbc(&mut self) -> LpdbcW<WumrSpec> {
        LpdbcW::new(self, 16)
    }
}
#[doc = "Supply Controller Wake Up Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wumr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wumr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WumrSpec;
impl crate::RegisterSpec for WumrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wumr::R`](R) reader structure"]
impl crate::Readable for WumrSpec {}
#[doc = "`write(|w| ..)` method takes [`wumr::W`](W) writer structure"]
impl crate::Writable for WumrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUMR to value 0"]
impl crate::Resettable for WumrSpec {
    const RESET_VALUE: u32 = 0;
}
