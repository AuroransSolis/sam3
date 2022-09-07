#[doc = "Register `WUMR` reader"]
pub struct R(crate::R<WUMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUMR` writer"]
pub struct W(crate::W<WUMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WUMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake Up Enable"]
pub type SMEN_R = crate::BitReader<SMEN_A>;
#[doc = "Supply Monitor Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMEN_A {
    #[doc = "0: the supply monitor detection has no wake up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: the supply monitor detection forces the wake up of the core power supply."]
    ENABLE = 1,
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
    pub fn variant(&self) -> SMEN_A {
        match self.bits {
            false => SMEN_A::NOT_ENABLE,
            true => SMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMEN_A::ENABLE
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake Up Enable"]
pub type SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, SMEN_A, O>;
impl<'a, const O: u8> SMEN_W<'a, O> {
    #[doc = "the supply monitor detection has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMEN_A::NOT_ENABLE)
    }
    #[doc = "the supply monitor detection forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMEN_A::ENABLE)
    }
}
#[doc = "Field `RTTEN` reader - Real Time Timer Wake Up Enable"]
pub type RTTEN_R = crate::BitReader<RTTEN_A>;
#[doc = "Real Time Timer Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTEN_A {
    #[doc = "0: the RTT alarm signal has no wake up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: the RTT alarm signal forces the wake up of the core power supply."]
    ENABLE = 1,
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
    pub fn variant(&self) -> RTTEN_A {
        match self.bits {
            false => RTTEN_A::NOT_ENABLE,
            true => RTTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTEN_A::ENABLE
    }
}
#[doc = "Field `RTTEN` writer - Real Time Timer Wake Up Enable"]
pub type RTTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, RTTEN_A, O>;
impl<'a, const O: u8> RTTEN_W<'a, O> {
    #[doc = "the RTT alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTEN_A::NOT_ENABLE)
    }
    #[doc = "the RTT alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTEN_A::ENABLE)
    }
}
#[doc = "Field `RTCEN` reader - Real Time Clock Wake Up Enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "Real Time Clock Wake Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: the RTC alarm signal has no wake up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: the RTC alarm signal forces the wake up of the core power supply."]
    ENABLE = 1,
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
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::NOT_ENABLE,
            true => RTCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCEN_A::ENABLE
    }
}
#[doc = "Field `RTCEN` writer - Real Time Clock Wake Up Enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, RTCEN_A, O>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "the RTC alarm signal has no wake up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCEN_A::NOT_ENABLE)
    }
    #[doc = "the RTC alarm signal forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLE)
    }
}
#[doc = "Field `LPDBCEN0` reader - Low power Debouncer ENable WKUP0"]
pub type LPDBCEN0_R = crate::BitReader<LPDBCEN0_A>;
#[doc = "Low power Debouncer ENable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0_A {
    #[doc = "0: the WKUP0 input pin is not connected with low power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    ENABLE = 1,
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
    pub fn variant(&self) -> LPDBCEN0_A {
        match self.bits {
            false => LPDBCEN0_A::NOT_ENABLE,
            true => LPDBCEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN0_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN0_A::ENABLE
    }
}
#[doc = "Field `LPDBCEN0` writer - Low power Debouncer ENable WKUP0"]
pub type LPDBCEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, LPDBCEN0_A, O>;
impl<'a, const O: u8> LPDBCEN0_W<'a, O> {
    #[doc = "the WKUP0 input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN0_A::NOT_ENABLE)
    }
    #[doc = "the WKUP0 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN0_A::ENABLE)
    }
}
#[doc = "Field `LPDBCEN1` reader - Low power Debouncer ENable WKUP1"]
pub type LPDBCEN1_R = crate::BitReader<LPDBCEN1_A>;
#[doc = "Low power Debouncer ENable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1_A {
    #[doc = "0: the WKUP1input pin is not connected with low power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    ENABLE = 1,
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
    pub fn variant(&self) -> LPDBCEN1_A {
        match self.bits {
            false => LPDBCEN1_A::NOT_ENABLE,
            true => LPDBCEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN1_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN1_A::ENABLE
    }
}
#[doc = "Field `LPDBCEN1` writer - Low power Debouncer ENable WKUP1"]
pub type LPDBCEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, LPDBCEN1_A, O>;
impl<'a, const O: u8> LPDBCEN1_W<'a, O> {
    #[doc = "the WKUP1input pin is not connected with low power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN1_A::NOT_ENABLE)
    }
    #[doc = "the WKUP1 input pin is connected with low power debouncer and can force a core wake up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN1_A::ENABLE)
    }
}
#[doc = "Field `LPDBCCLR` reader - Low power Debouncer Clear"]
pub type LPDBCCLR_R = crate::BitReader<LPDBCCLR_A>;
#[doc = "Low power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLR_A {
    #[doc = "0: a low power debounce event does not create an immediate clear on first half GPBR registers."]
    NOT_ENABLE = 0,
    #[doc = "1: a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    ENABLE = 1,
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
    pub fn variant(&self) -> LPDBCCLR_A {
        match self.bits {
            false => LPDBCCLR_A::NOT_ENABLE,
            true => LPDBCCLR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCCLR_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCCLR_A::ENABLE
    }
}
#[doc = "Field `LPDBCCLR` writer - Low power Debouncer Clear"]
pub type LPDBCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, LPDBCCLR_A, O>;
impl<'a, const O: u8> LPDBCCLR_W<'a, O> {
    #[doc = "a low power debounce event does not create an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCCLR_A::NOT_ENABLE)
    }
    #[doc = "a low power debounce event on WKUP0 or WKUP1 generates an immediate clear on first half GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCCLR_A::ENABLE)
    }
}
#[doc = "Field `WKUPDBC` reader - Wake Up Inputs Debouncer Period"]
pub type WKUPDBC_R = crate::FieldReader<u8, WKUPDBC_A>;
#[doc = "Wake Up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WKUPDBC_A {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE = 0,
    #[doc = "1: WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SCLK = 1,
    #[doc = "2: WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SCLK = 2,
    #[doc = "3: WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SCLK = 3,
    #[doc = "4: WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SCLK = 4,
    #[doc = "5: WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SCLK = 5,
}
impl From<WKUPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: WKUPDBC_A) -> Self {
        variant as _
    }
}
impl WKUPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKUPDBC_A> {
        match self.bits {
            0 => Some(WKUPDBC_A::IMMEDIATE),
            1 => Some(WKUPDBC_A::_3_SCLK),
            2 => Some(WKUPDBC_A::_32_SCLK),
            3 => Some(WKUPDBC_A::_512_SCLK),
            4 => Some(WKUPDBC_A::_4096_SCLK),
            5 => Some(WKUPDBC_A::_32768_SCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBC_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SCLK`"]
    #[inline(always)]
    pub fn is_3_sclk(&self) -> bool {
        *self == WKUPDBC_A::_3_SCLK
    }
    #[doc = "Checks if the value of the field is `_32_SCLK`"]
    #[inline(always)]
    pub fn is_32_sclk(&self) -> bool {
        *self == WKUPDBC_A::_32_SCLK
    }
    #[doc = "Checks if the value of the field is `_512_SCLK`"]
    #[inline(always)]
    pub fn is_512_sclk(&self) -> bool {
        *self == WKUPDBC_A::_512_SCLK
    }
    #[doc = "Checks if the value of the field is `_4096_SCLK`"]
    #[inline(always)]
    pub fn is_4096_sclk(&self) -> bool {
        *self == WKUPDBC_A::_4096_SCLK
    }
    #[doc = "Checks if the value of the field is `_32768_SCLK`"]
    #[inline(always)]
    pub fn is_32768_sclk(&self) -> bool {
        *self == WKUPDBC_A::_32768_SCLK
    }
}
#[doc = "Field `WKUPDBC` writer - Wake Up Inputs Debouncer Period"]
pub type WKUPDBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUMR_SPEC, u8, WKUPDBC_A, 3, O>;
impl<'a, const O: u8> WKUPDBC_W<'a, O> {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBC_A::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_3_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_512_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_4096_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32768_SCLK)
    }
}
#[doc = "Field `LPDBC` reader - Low Power DeBounCer Period"]
pub type LPDBC_R = crate::FieldReader<u8, LPDBC_A>;
#[doc = "Low Power DeBounCer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPDBC_A {
    #[doc = "0: Disable the low power debouncer."]
    DISABLE = 0,
    #[doc = "1: WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    _2_RTCOUT0 = 1,
    #[doc = "2: WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    _3_RTCOUT0 = 2,
    #[doc = "3: WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    _4_RTCOUT0 = 3,
    #[doc = "4: WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    _5_RTCOUT0 = 4,
    #[doc = "5: WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    _6_RTCOUT0 = 5,
    #[doc = "6: WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    _7_RTCOUT0 = 6,
    #[doc = "7: WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    _8_RTCOUT0 = 7,
}
impl From<LPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPDBC_A) -> Self {
        variant as _
    }
}
impl LPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBC_A {
        match self.bits {
            0 => LPDBC_A::DISABLE,
            1 => LPDBC_A::_2_RTCOUT0,
            2 => LPDBC_A::_3_RTCOUT0,
            3 => LPDBC_A::_4_RTCOUT0,
            4 => LPDBC_A::_5_RTCOUT0,
            5 => LPDBC_A::_6_RTCOUT0,
            6 => LPDBC_A::_7_RTCOUT0,
            7 => LPDBC_A::_8_RTCOUT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPDBC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_2_RTCOUT0`"]
    #[inline(always)]
    pub fn is_2_rtcout0(&self) -> bool {
        *self == LPDBC_A::_2_RTCOUT0
    }
    #[doc = "Checks if the value of the field is `_3_RTCOUT0`"]
    #[inline(always)]
    pub fn is_3_rtcout0(&self) -> bool {
        *self == LPDBC_A::_3_RTCOUT0
    }
    #[doc = "Checks if the value of the field is `_4_RTCOUT0`"]
    #[inline(always)]
    pub fn is_4_rtcout0(&self) -> bool {
        *self == LPDBC_A::_4_RTCOUT0
    }
    #[doc = "Checks if the value of the field is `_5_RTCOUT0`"]
    #[inline(always)]
    pub fn is_5_rtcout0(&self) -> bool {
        *self == LPDBC_A::_5_RTCOUT0
    }
    #[doc = "Checks if the value of the field is `_6_RTCOUT0`"]
    #[inline(always)]
    pub fn is_6_rtcout0(&self) -> bool {
        *self == LPDBC_A::_6_RTCOUT0
    }
    #[doc = "Checks if the value of the field is `_7_RTCOUT0`"]
    #[inline(always)]
    pub fn is_7_rtcout0(&self) -> bool {
        *self == LPDBC_A::_7_RTCOUT0
    }
    #[doc = "Checks if the value of the field is `_8_RTCOUT0`"]
    #[inline(always)]
    pub fn is_8_rtcout0(&self) -> bool {
        *self == LPDBC_A::_8_RTCOUT0
    }
}
#[doc = "Field `LPDBC` writer - Low Power DeBounCer Period"]
pub type LPDBC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WUMR_SPEC, u8, LPDBC_A, 3, O>;
impl<'a, const O: u8> LPDBC_W<'a, O> {
    #[doc = "Disable the low power debouncer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPDBC_A::DISABLE)
    }
    #[doc = "WKUP0/1 in its active state for at least 2 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _2_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_2_RTCOUT0)
    }
    #[doc = "WKUP0/1 in its active state for at least 3 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _3_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_3_RTCOUT0)
    }
    #[doc = "WKUP0/1 in its active state for at least 4 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _4_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_4_RTCOUT0)
    }
    #[doc = "WKUP0/1 in its active state for at least 5 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _5_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_5_RTCOUT0)
    }
    #[doc = "WKUP0/1 in its active state for at least 6 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _6_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_6_RTCOUT0)
    }
    #[doc = "WKUP0/1 in its active state for at least 7 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _7_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_7_RTCOUT0)
    }
    #[doc = "WKUP0/1 in its active state for at least 8 RTCOUT0 periods"]
    #[inline(always)]
    pub fn _8_rtcout0(self) -> &'a mut W {
        self.variant(LPDBC_A::_8_RTCOUT0)
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
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Timer Wake Up Enable"]
    #[inline(always)]
    pub fn rtten(&mut self) -> RTTEN_W<2> {
        RTTEN_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock Wake Up Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<3> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 5 - Low power Debouncer ENable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&mut self) -> LPDBCEN0_W<5> {
        LPDBCEN0_W::new(self)
    }
    #[doc = "Bit 6 - Low power Debouncer ENable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&mut self) -> LPDBCEN1_W<6> {
        LPDBCEN1_W::new(self)
    }
    #[doc = "Bit 7 - Low power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&mut self) -> LPDBCCLR_W<7> {
        LPDBCCLR_W::new(self)
    }
    #[doc = "Bits 12:14 - Wake Up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W<12> {
        WKUPDBC_W::new(self)
    }
    #[doc = "Bits 16:18 - Low Power DeBounCer Period"]
    #[inline(always)]
    pub fn lpdbc(&mut self) -> LPDBC_W<16> {
        LPDBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Wake Up Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wumr](index.html) module"]
pub struct WUMR_SPEC;
impl crate::RegisterSpec for WUMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wumr::R](R) reader structure"]
impl crate::Readable for WUMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wumr::W](W) writer structure"]
impl crate::Writable for WUMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUMR to value 0"]
impl crate::Resettable for WUMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
