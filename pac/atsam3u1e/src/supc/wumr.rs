#[doc = "Register `WUMR` reader"]
pub type R = crate::R<WumrSpec>;
#[doc = "Register `WUMR` writer"]
pub type W = crate::W<WumrSpec>;
#[doc = "Force Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwupen {
    #[doc = "0: the Force Wake-up pin has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the Force Wake-up pin low forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Fwupen> for bool {
    #[inline(always)]
    fn from(variant: Fwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWUPEN` reader - Force Wake-up Enable"]
pub type FwupenR = crate::BitReader<Fwupen>;
impl FwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwupen {
        match self.bits {
            false => Fwupen::NotEnable,
            true => Fwupen::Enable,
        }
    }
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Fwupen::NotEnable
    }
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fwupen::Enable
    }
}
#[doc = "Field `FWUPEN` writer - Force Wake-up Enable"]
pub type FwupenW<'a, REG> = crate::BitWriter<'a, REG, Fwupen>;
impl<'a, REG> FwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupen::NotEnable)
    }
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupen::Enable)
    }
}
#[doc = "Supply Monitor Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smen {
    #[doc = "0: the supply monitor detection has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the supply monitor detection forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Smen> for bool {
    #[inline(always)]
    fn from(variant: Smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake-up Enable"]
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
    #[doc = "the supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smen::NotEnable
    }
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smen::Enable
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake-up Enable"]
pub type SmenW<'a, REG> = crate::BitWriter<'a, REG, Smen>;
impl<'a, REG> SmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smen::NotEnable)
    }
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smen::Enable)
    }
}
#[doc = "Real Time Timer Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtten {
    #[doc = "0: the RTT alarm signal has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the RTT alarm signal forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Rtten> for bool {
    #[inline(always)]
    fn from(variant: Rtten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTTEN` reader - Real Time Timer Wake-up Enable"]
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
    #[doc = "the RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Rtten::NotEnable
    }
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtten::Enable
    }
}
#[doc = "Field `RTTEN` writer - Real Time Timer Wake-up Enable"]
pub type RttenW<'a, REG> = crate::BitWriter<'a, REG, Rtten>;
impl<'a, REG> RttenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtten::NotEnable)
    }
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtten::Enable)
    }
}
#[doc = "Real Time Clock Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcen {
    #[doc = "0: the RTC alarm signal has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the RTC alarm signal forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Rtcen> for bool {
    #[inline(always)]
    fn from(variant: Rtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - Real Time Clock Wake-up Enable"]
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
    #[doc = "the RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Rtcen::NotEnable
    }
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtcen::Enable
    }
}
#[doc = "Field `RTCEN` writer - Real Time Clock Wake-up Enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcen>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::NotEnable)
    }
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::Enable)
    }
}
#[doc = "Force Wake-up Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fwupdbc {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    Immediate = 0,
    #[doc = "1: FWUP shall be low for at least 3 SLCK periods"]
    _3Sclk = 1,
    #[doc = "2: FWUP shall be low for at least 32 SLCK periods"]
    _32Sclk = 2,
    #[doc = "3: FWUP shall be low for at least 512 SLCK periods"]
    _512Sclk = 3,
    #[doc = "4: FWUP shall be low for at least 4,096 SLCK periods"]
    _4096Sclk = 4,
    #[doc = "5: FWUP shall be low for at least 32,768 SLCK periods"]
    _32768Sclk = 5,
}
impl From<Fwupdbc> for u8 {
    #[inline(always)]
    fn from(variant: Fwupdbc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fwupdbc {
    type Ux = u8;
}
#[doc = "Field `FWUPDBC` reader - Force Wake-up Debouncer Period"]
pub type FwupdbcR = crate::FieldReader<Fwupdbc>;
impl FwupdbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fwupdbc> {
        match self.bits {
            0 => Some(Fwupdbc::Immediate),
            1 => Some(Fwupdbc::_3Sclk),
            2 => Some(Fwupdbc::_32Sclk),
            3 => Some(Fwupdbc::_512Sclk),
            4 => Some(Fwupdbc::_4096Sclk),
            5 => Some(Fwupdbc::_32768Sclk),
            _ => None,
        }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Fwupdbc::Immediate
    }
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn is_3_sclk(&self) -> bool {
        *self == Fwupdbc::_3Sclk
    }
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32_sclk(&self) -> bool {
        *self == Fwupdbc::_32Sclk
    }
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn is_512_sclk(&self) -> bool {
        *self == Fwupdbc::_512Sclk
    }
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn is_4096_sclk(&self) -> bool {
        *self == Fwupdbc::_4096Sclk
    }
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn is_32768_sclk(&self) -> bool {
        *self == Fwupdbc::_32768Sclk
    }
}
#[doc = "Field `FWUPDBC` writer - Force Wake-up Debouncer Period"]
pub type FwupdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fwupdbc>;
impl<'a, REG> FwupdbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupdbc::Immediate)
    }
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupdbc::_3Sclk)
    }
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupdbc::_32Sclk)
    }
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupdbc::_512Sclk)
    }
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupdbc::_4096Sclk)
    }
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupdbc::_32768Sclk)
    }
}
#[doc = "Wake-up Inputs Debouncer Period\n\nValue on reset: 0"]
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
#[doc = "Field `WKUPDBC` reader - Wake-up Inputs Debouncer Period"]
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
#[doc = "Field `WKUPDBC` writer - Wake-up Inputs Debouncer Period"]
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
impl R {
    #[doc = "Bit 0 - Force Wake-up Enable"]
    #[inline(always)]
    pub fn fwupen(&self) -> FwupenR {
        FwupenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SmenR {
        SmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RttenR {
        RttenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Force Wake-up Debouncer Period"]
    #[inline(always)]
    pub fn fwupdbc(&self) -> FwupdbcR {
        FwupdbcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WkupdbcR {
        WkupdbcR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwupen(&mut self) -> FwupenW<WumrSpec> {
        FwupenW::new(self, 0)
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SmenW<WumrSpec> {
        SmenW::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtten(&mut self) -> RttenW<WumrSpec> {
        RttenW::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RtcenW<WumrSpec> {
        RtcenW::new(self, 3)
    }
    #[doc = "Bits 8:10 - Force Wake-up Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn fwupdbc(&mut self) -> FwupdbcW<WumrSpec> {
        FwupdbcW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn wkupdbc(&mut self) -> WkupdbcW<WumrSpec> {
        WkupdbcW::new(self, 12)
    }
}
#[doc = "Supply Controller Wake-up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wumr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wumr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
