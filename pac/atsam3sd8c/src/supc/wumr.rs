#[doc = "Register `WUMR` reader"]
pub type R = crate::R<WUMR_SPEC>;
#[doc = "Register `WUMR` writer"]
pub type W = crate::W<WUMR_SPEC>;
#[doc = "Field `SMEN` reader - Supply Monitor Wake Up Enable"]
pub type SMEN_R = crate::BitReader<SMEN_A>;
#[doc = "Supply Monitor Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMEN_A {
    #[doc = "0: the supply monitor detection has no wake up effect."]
    NotEnable = 0,
    #[doc = "1: the supply monitor detection forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMEN_A {
        match self.bits {
            false => SMEN_A::NotEnable,
            true => SMEN_A::Enable,
        }
    }
    #[doc = "the supply monitor detection has no wake up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMEN_A::NotEnable
    }
    #[doc = "the supply monitor detection forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMEN_A::Enable
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake Up Enable"]
pub type SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SMEN_A>;
impl<'a, REG> SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the supply monitor detection has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMEN_A::NotEnable)
    }
    #[doc = "the supply monitor detection forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMEN_A::Enable)
    }
}
#[doc = "Field `RTTEN` reader - Real Time Timer Wake Up Enable"]
pub type RTTEN_R = crate::BitReader<RTTEN_A>;
#[doc = "Real Time Timer Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTTEN_A {
    #[doc = "0: the RTT alarm signal has no wake up effect."]
    NotEnable = 0,
    #[doc = "1: the RTT alarm signal forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<RTTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTTEN_A {
        match self.bits {
            false => RTTEN_A::NotEnable,
            true => RTTEN_A::Enable,
        }
    }
    #[doc = "the RTT alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTEN_A::NotEnable
    }
    #[doc = "the RTT alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTEN_A::Enable
    }
}
#[doc = "Field `RTTEN` writer - Real Time Timer Wake Up Enable"]
pub type RTTEN_W<'a, REG> = crate::BitWriter<'a, REG, RTTEN_A>;
impl<'a, REG> RTTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTT alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTTEN_A::NotEnable)
    }
    #[doc = "the RTT alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTTEN_A::Enable)
    }
}
#[doc = "Field `RTCEN` reader - Real Time Clock Wake Up Enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "Real Time Clock Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: the RTC alarm signal has no wake up effect."]
    NotEnable = 0,
    #[doc = "1: the RTC alarm signal forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::NotEnable,
            true => RTCEN_A::Enable,
        }
    }
    #[doc = "the RTC alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCEN_A::NotEnable
    }
    #[doc = "the RTC alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCEN_A::Enable
    }
}
#[doc = "Field `RTCEN` writer - Real Time Clock Wake Up Enable"]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN_A>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTC alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::NotEnable)
    }
    #[doc = "the RTC alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::Enable)
    }
}
#[doc = "Field `LPDBCEN0` reader - Low power Debouncer ENable WKUP0"]
pub type LPDBCEN0_R = crate::BitReader<LPDBCEN0_A>;
#[doc = "Low power Debouncer ENable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCEN0_A {
    #[doc = "0: the WKUP0 input pin is not connected with low power debouncer."]
    NotEnable = 0,
    #[doc = "1: the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    Enable = 1,
}
impl From<LPDBCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPDBCEN0_A {
        match self.bits {
            false => LPDBCEN0_A::NotEnable,
            true => LPDBCEN0_A::Enable,
        }
    }
    #[doc = "the WKUP0 input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN0_A::NotEnable
    }
    #[doc = "the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN0_A::Enable
    }
}
#[doc = "Field `LPDBCEN0` writer - Low power Debouncer ENable WKUP0"]
pub type LPDBCEN0_W<'a, REG> = crate::BitWriter<'a, REG, LPDBCEN0_A>;
impl<'a, REG> LPDBCEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the WKUP0 input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBCEN0_A::NotEnable)
    }
    #[doc = "the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBCEN0_A::Enable)
    }
}
#[doc = "Field `LPDBCEN1` reader - Low power Debouncer ENable WKUP1"]
pub type LPDBCEN1_R = crate::BitReader<LPDBCEN1_A>;
#[doc = "Low power Debouncer ENable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCEN1_A {
    #[doc = "0: the WKUP1input pin is not connected with low power debouncer."]
    NotEnable = 0,
    #[doc = "1: the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    Enable = 1,
}
impl From<LPDBCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPDBCEN1_A {
        match self.bits {
            false => LPDBCEN1_A::NotEnable,
            true => LPDBCEN1_A::Enable,
        }
    }
    #[doc = "the WKUP1input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN1_A::NotEnable
    }
    #[doc = "the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN1_A::Enable
    }
}
#[doc = "Field `LPDBCEN1` writer - Low power Debouncer ENable WKUP1"]
pub type LPDBCEN1_W<'a, REG> = crate::BitWriter<'a, REG, LPDBCEN1_A>;
impl<'a, REG> LPDBCEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the WKUP1input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBCEN1_A::NotEnable)
    }
    #[doc = "the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBCEN1_A::Enable)
    }
}
#[doc = "Field `LPDBCCLR` reader - Low power Debouncer Clear"]
pub type LPDBCCLR_R = crate::BitReader<LPDBCCLR_A>;
#[doc = "Low power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCCLR_A {
    #[doc = "0: a low power debounce event does not create an immediate clear on first half GPBR registers."]
    NotEnable = 0,
    #[doc = "1: a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    Enable = 1,
}
impl From<LPDBCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPDBCCLR_A {
        match self.bits {
            false => LPDBCCLR_A::NotEnable,
            true => LPDBCCLR_A::Enable,
        }
    }
    #[doc = "a low power debounce event does not create an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCCLR_A::NotEnable
    }
    #[doc = "a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCCLR_A::Enable
    }
}
#[doc = "Field `LPDBCCLR` writer - Low power Debouncer Clear"]
pub type LPDBCCLR_W<'a, REG> = crate::BitWriter<'a, REG, LPDBCCLR_A>;
impl<'a, REG> LPDBCCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low power debounce event does not create an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBCCLR_A::NotEnable)
    }
    #[doc = "a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBCCLR_A::Enable)
    }
}
#[doc = "Field `WKUPDBC` reader - Wake Up Inputs Debouncer Period"]
pub type WKUPDBC_R = crate::FieldReader<WKUPDBC_A>;
#[doc = "Wake Up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WKUPDBC_A {
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
impl From<WKUPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: WKUPDBC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WKUPDBC_A {
    type Ux = u8;
}
impl WKUPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WKUPDBC_A> {
        match self.bits {
            0 => Some(WKUPDBC_A::Immediate),
            1 => Some(WKUPDBC_A::_3Sclk),
            2 => Some(WKUPDBC_A::_32Sclk),
            3 => Some(WKUPDBC_A::_512Sclk),
            4 => Some(WKUPDBC_A::_4096Sclk),
            5 => Some(WKUPDBC_A::_32768Sclk),
            _ => None,
        }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBC_A::Immediate
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn is_3_sclk(&self) -> bool {
        *self == WKUPDBC_A::_3Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32_sclk(&self) -> bool {
        *self == WKUPDBC_A::_32Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn is_512_sclk(&self) -> bool {
        *self == WKUPDBC_A::_512Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn is_4096_sclk(&self) -> bool {
        *self == WKUPDBC_A::_4096Sclk
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn is_32768_sclk(&self) -> bool {
        *self == WKUPDBC_A::_32768Sclk
    }
}
#[doc = "Field `WKUPDBC` writer - Wake Up Inputs Debouncer Period"]
pub type WKUPDBC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WKUPDBC_A>;
impl<'a, REG> WKUPDBC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPDBC_A::Immediate)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPDBC_A::_3Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPDBC_A::_32Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPDBC_A::_512Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPDBC_A::_4096Sclk)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPDBC_A::_32768Sclk)
    }
}
#[doc = "Field `LPDBC` reader - Low Power DeBounCer Period"]
pub type LPDBC_R = crate::FieldReader<LPDBC_A>;
#[doc = "Low Power DeBounCer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPDBC_A {
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
impl From<LPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPDBC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPDBC_A {
    type Ux = u8;
}
impl LPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPDBC_A {
        match self.bits {
            0 => LPDBC_A::Disable,
            1 => LPDBC_A::_2Rtcout0,
            2 => LPDBC_A::_3Rtcout0,
            3 => LPDBC_A::_4Rtcout0,
            4 => LPDBC_A::_5Rtcout0,
            5 => LPDBC_A::_6Rtcout0,
            6 => LPDBC_A::_7Rtcout0,
            7 => LPDBC_A::_8Rtcout0,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable the low power debouncer."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPDBC_A::Disable
    }
    #[doc = "WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_2_rtcout0(&self) -> bool {
        *self == LPDBC_A::_2Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_3_rtcout0(&self) -> bool {
        *self == LPDBC_A::_3Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_4_rtcout0(&self) -> bool {
        *self == LPDBC_A::_4Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_5_rtcout0(&self) -> bool {
        *self == LPDBC_A::_5Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_6_rtcout0(&self) -> bool {
        *self == LPDBC_A::_6Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_7_rtcout0(&self) -> bool {
        *self == LPDBC_A::_7Rtcout0
    }
    #[doc = "WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    #[inline(always)]
    pub fn is_8_rtcout0(&self) -> bool {
        *self == LPDBC_A::_8Rtcout0
    }
}
#[doc = "Field `LPDBC` writer - Low Power DeBounCer Period"]
pub type LPDBC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, LPDBC_A>;
impl<'a, REG> LPDBC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the low power debouncer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::Disable)
    }
    #[doc = "WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _2_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_2Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _3_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_3Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _4_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_4Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _5_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_5Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _6_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_6Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _7_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_7Rtcout0)
    }
    #[doc = "WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _8_rtcout0(self) -> &'a mut crate::W<REG> {
        self.variant(LPDBC_A::_8Rtcout0)
    }
}
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake Up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Timer Wake Up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Wake Up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Low power Debouncer ENable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> LPDBCEN0_R {
        LPDBCEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low power Debouncer ENable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> LPDBCEN1_R {
        LPDBCEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LPDBCCLR_R {
        LPDBCCLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Wake Up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Low Power DeBounCer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LPDBC_R {
        LPDBC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<WUMR_SPEC> {
        SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Timer Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtten(&mut self) -> RTTEN_W<WUMR_SPEC> {
        RTTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<WUMR_SPEC> {
        RTCEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - Low power Debouncer ENable WKUP0"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcen0(&mut self) -> LPDBCEN0_W<WUMR_SPEC> {
        LPDBCEN0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Low power Debouncer ENable WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcen1(&mut self) -> LPDBCEN1_W<WUMR_SPEC> {
        LPDBCEN1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Low power Debouncer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcclr(&mut self) -> LPDBCCLR_W<WUMR_SPEC> {
        LPDBCCLR_W::new(self, 7)
    }
    #[doc = "Bits 12:14 - Wake Up Inputs Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W<WUMR_SPEC> {
        WKUPDBC_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Low Power DeBounCer Period"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbc(&mut self) -> LPDBC_W<WUMR_SPEC> {
        LPDBC_W::new(self, 16)
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
#[doc = "Supply Controller Wake Up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wumr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wumr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUMR_SPEC;
impl crate::RegisterSpec for WUMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wumr::R`](R) reader structure"]
impl crate::Readable for WUMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wumr::W`](W) writer structure"]
impl crate::Writable for WUMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUMR to value 0"]
impl crate::Resettable for WUMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
