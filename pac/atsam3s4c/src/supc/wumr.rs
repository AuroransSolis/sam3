#[doc = "Register `WUMR` reader"]
pub type R = crate::R<WUMR_SPEC>;
#[doc = "Register `WUMR` writer"]
pub type W = crate::W<WUMR_SPEC>;
#[doc = "Field `SMEN` reader - Supply Monitor Wake-up Enable"]
pub type SMEN_R = crate::BitReader<SMEN_A>;
#[doc = "Supply Monitor Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMEN_A {
    #[doc = "0: the supply monitor detection has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the supply monitor detection forces the wake-up of the core power supply."]
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
    #[doc = "the supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMEN_A::NotEnable
    }
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMEN_A::Enable
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake-up Enable"]
pub type SMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMEN_A>;
impl<'a, REG, const O: u8> SMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMEN_A::NotEnable)
    }
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMEN_A::Enable)
    }
}
#[doc = "Field `RTTEN` reader - Real Time Timer Wake-up Enable"]
pub type RTTEN_R = crate::BitReader<RTTEN_A>;
#[doc = "Real Time Timer Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTTEN_A {
    #[doc = "0: the RTT alarm signal has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the RTT alarm signal forces the wake-up of the core power supply."]
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
    #[doc = "the RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTEN_A::NotEnable
    }
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTEN_A::Enable
    }
}
#[doc = "Field `RTTEN` writer - Real Time Timer Wake-up Enable"]
pub type RTTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTTEN_A>;
impl<'a, REG, const O: u8> RTTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTTEN_A::NotEnable)
    }
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTTEN_A::Enable)
    }
}
#[doc = "Field `RTCEN` reader - Real Time Clock Wake-up Enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "Real Time Clock Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: the RTC alarm signal has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: the RTC alarm signal forces the wake-up of the core power supply."]
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
    #[doc = "the RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCEN_A::NotEnable
    }
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCEN_A::Enable
    }
}
#[doc = "Field `RTCEN` writer - Real Time Clock Wake-up Enable"]
pub type RTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCEN_A>;
impl<'a, REG, const O: u8> RTCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::NotEnable)
    }
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN_A::Enable)
    }
}
#[doc = "Field `WKUPDBC` reader - Wake-up Inputs Debouncer Period"]
pub type WKUPDBC_R = crate::FieldReader<WKUPDBC_A>;
#[doc = "Wake-up Inputs Debouncer Period\n\nValue on reset: 0"]
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
#[doc = "Field `WKUPDBC` writer - Wake-up Inputs Debouncer Period"]
pub type WKUPDBC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WKUPDBC_A>;
impl<'a, REG, const O: u8> WKUPDBC_W<'a, REG, O>
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
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<WUMR_SPEC, 1> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtten(&mut self) -> RTTEN_W<WUMR_SPEC, 2> {
        RTTEN_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<WUMR_SPEC, 3> {
        RTCEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W<WUMR_SPEC, 12> {
        WKUPDBC_W::new(self)
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
#[doc = "Supply Controller Wake-up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wumr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wumr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
