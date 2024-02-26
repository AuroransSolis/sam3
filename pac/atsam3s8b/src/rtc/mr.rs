#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `HRMOD` reader - 12-/24-hour Mode"]
pub type HrmodR = crate::BitReader;
#[doc = "Field `HRMOD` writer - 12-/24-hour Mode"]
pub type HrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSIAN` reader - PERSIAN Calendar"]
pub type PersianR = crate::BitReader;
#[doc = "Field `PERSIAN` writer - PERSIAN Calendar"]
pub type PersianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEGPPM` reader - NEGative PPM Correction"]
pub type NegppmR = crate::BitReader;
#[doc = "Field `NEGPPM` writer - NEGative PPM Correction"]
pub type NegppmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRECTION` reader - Slow Clock Correction"]
pub type CorrectionR = crate::FieldReader;
#[doc = "Field `CORRECTION` writer - Slow Clock Correction"]
pub type CorrectionW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HIGHPPM` reader - HIGH PPM Correction"]
pub type HighppmR = crate::BitReader;
#[doc = "Field `HIGHPPM` writer - HIGH PPM Correction"]
pub type HighppmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RTCOUT0 Output Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out0 {
    #[doc = "0: no waveform, stuck at '0'"]
    NoWave = 0,
    #[doc = "1: 1 Hz square wave"]
    Freq1hz = 1,
    #[doc = "2: 32 Hz square wave"]
    Freq32hz = 2,
    #[doc = "3: 64 Hz square wave"]
    Freq64hz = 3,
    #[doc = "4: 512 Hz square wave"]
    Freq512hz = 4,
    #[doc = "5: output toggles when alarm flag rises"]
    AlarmToggle = 5,
    #[doc = "6: output is a copy of the alarm flag"]
    AlarmFlag = 6,
    #[doc = "7: duty cycle programmable pulse"]
    ProgPulse = 7,
}
impl From<Out0> for u8 {
    #[inline(always)]
    fn from(variant: Out0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out0 {
    type Ux = u8;
}
#[doc = "Field `OUT0` reader - RTCOUT0 Output Source Selection"]
pub type Out0R = crate::FieldReader<Out0>;
impl Out0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out0 {
        match self.bits {
            0 => Out0::NoWave,
            1 => Out0::Freq1hz,
            2 => Out0::Freq32hz,
            3 => Out0::Freq64hz,
            4 => Out0::Freq512hz,
            5 => Out0::AlarmToggle,
            6 => Out0::AlarmFlag,
            7 => Out0::ProgPulse,
            _ => unreachable!(),
        }
    }
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == Out0::NoWave
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == Out0::Freq1hz
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == Out0::Freq32hz
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == Out0::Freq64hz
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == Out0::Freq512hz
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == Out0::AlarmToggle
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == Out0::AlarmFlag
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == Out0::ProgPulse
    }
}
#[doc = "Field `OUT0` writer - RTCOUT0 Output Source Selection"]
pub type Out0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Out0>;
impl<'a, REG> Out0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::NoWave)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq1hz)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq32hz)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq64hz)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Freq512hz)
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::AlarmToggle)
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::AlarmFlag)
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::ProgPulse)
    }
}
#[doc = "RTCOUT1 Output Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out1 {
    #[doc = "0: no waveform, stuck at '0'"]
    NoWave = 0,
    #[doc = "1: 1 Hz square wave"]
    Freq1hz = 1,
    #[doc = "2: 32 Hz square wave"]
    Freq32hz = 2,
    #[doc = "3: 64 Hz square wave"]
    Freq64hz = 3,
    #[doc = "4: 512 Hz square wave"]
    Freq512hz = 4,
    #[doc = "5: output toggles when alarm flag rises"]
    AlarmToggle = 5,
    #[doc = "6: output is a copy of the alarm flag"]
    AlarmFlag = 6,
    #[doc = "7: duty cycle programmable pulse"]
    ProgPulse = 7,
}
impl From<Out1> for u8 {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out1 {
    type Ux = u8;
}
#[doc = "Field `OUT1` reader - RTCOUT1 Output Source Selection"]
pub type Out1R = crate::FieldReader<Out1>;
impl Out1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out1 {
        match self.bits {
            0 => Out1::NoWave,
            1 => Out1::Freq1hz,
            2 => Out1::Freq32hz,
            3 => Out1::Freq64hz,
            4 => Out1::Freq512hz,
            5 => Out1::AlarmToggle,
            6 => Out1::AlarmFlag,
            7 => Out1::ProgPulse,
            _ => unreachable!(),
        }
    }
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == Out1::NoWave
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == Out1::Freq1hz
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == Out1::Freq32hz
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == Out1::Freq64hz
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == Out1::Freq512hz
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == Out1::AlarmToggle
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == Out1::AlarmFlag
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == Out1::ProgPulse
    }
}
#[doc = "Field `OUT1` writer - RTCOUT1 Output Source Selection"]
pub type Out1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Out1>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::NoWave)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq1hz)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq32hz)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq64hz)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Freq512hz)
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::AlarmToggle)
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::AlarmFlag)
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::ProgPulse)
    }
}
#[doc = "High Duration of the Output Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Thigh {
    #[doc = "0: 31.2 ms"]
    H31ms = 0,
    #[doc = "1: 15.6 ms"]
    H16ms = 1,
    #[doc = "2: 3.91 ms"]
    H4ms = 2,
    #[doc = "3: 976 us"]
    H976us = 3,
    #[doc = "4: 488 us"]
    H488us = 4,
    #[doc = "5: 122 us"]
    H122us = 5,
    #[doc = "6: 30.5 us"]
    H30us = 6,
    #[doc = "7: 15.2 us"]
    H15us = 7,
}
impl From<Thigh> for u8 {
    #[inline(always)]
    fn from(variant: Thigh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Thigh {
    type Ux = u8;
}
#[doc = "Field `THIGH` reader - High Duration of the Output Pulse"]
pub type ThighR = crate::FieldReader<Thigh>;
impl ThighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thigh {
        match self.bits {
            0 => Thigh::H31ms,
            1 => Thigh::H16ms,
            2 => Thigh::H4ms,
            3 => Thigh::H976us,
            4 => Thigh::H488us,
            5 => Thigh::H122us,
            6 => Thigh::H30us,
            7 => Thigh::H15us,
            _ => unreachable!(),
        }
    }
    #[doc = "31.2 ms"]
    #[inline(always)]
    pub fn is_h_31ms(&self) -> bool {
        *self == Thigh::H31ms
    }
    #[doc = "15.6 ms"]
    #[inline(always)]
    pub fn is_h_16ms(&self) -> bool {
        *self == Thigh::H16ms
    }
    #[doc = "3.91 ms"]
    #[inline(always)]
    pub fn is_h_4ms(&self) -> bool {
        *self == Thigh::H4ms
    }
    #[doc = "976 us"]
    #[inline(always)]
    pub fn is_h_976us(&self) -> bool {
        *self == Thigh::H976us
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn is_h_488us(&self) -> bool {
        *self == Thigh::H488us
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn is_h_122us(&self) -> bool {
        *self == Thigh::H122us
    }
    #[doc = "30.5 us"]
    #[inline(always)]
    pub fn is_h_30us(&self) -> bool {
        *self == Thigh::H30us
    }
    #[doc = "15.2 us"]
    #[inline(always)]
    pub fn is_h_15us(&self) -> bool {
        *self == Thigh::H15us
    }
}
#[doc = "Field `THIGH` writer - High Duration of the Output Pulse"]
pub type ThighW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Thigh>;
impl<'a, REG> ThighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "31.2 ms"]
    #[inline(always)]
    pub fn h_31ms(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H31ms)
    }
    #[doc = "15.6 ms"]
    #[inline(always)]
    pub fn h_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H16ms)
    }
    #[doc = "3.91 ms"]
    #[inline(always)]
    pub fn h_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H4ms)
    }
    #[doc = "976 us"]
    #[inline(always)]
    pub fn h_976us(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H976us)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn h_488us(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H488us)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn h_122us(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H122us)
    }
    #[doc = "30.5 us"]
    #[inline(always)]
    pub fn h_30us(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H30us)
    }
    #[doc = "15.2 us"]
    #[inline(always)]
    pub fn h_15us(self) -> &'a mut crate::W<REG> {
        self.variant(Thigh::H15us)
    }
}
#[doc = "Period of the Output Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tperiod {
    #[doc = "0: 1 second"]
    P1s = 0,
    #[doc = "1: 500 ms"]
    P500ms = 1,
    #[doc = "2: 250 ms"]
    P250ms = 2,
    #[doc = "3: 125 ms"]
    P125ms = 3,
}
impl From<Tperiod> for u8 {
    #[inline(always)]
    fn from(variant: Tperiod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tperiod {
    type Ux = u8;
}
#[doc = "Field `TPERIOD` reader - Period of the Output Pulse"]
pub type TperiodR = crate::FieldReader<Tperiod>;
impl TperiodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tperiod {
        match self.bits {
            0 => Tperiod::P1s,
            1 => Tperiod::P500ms,
            2 => Tperiod::P250ms,
            3 => Tperiod::P125ms,
            _ => unreachable!(),
        }
    }
    #[doc = "1 second"]
    #[inline(always)]
    pub fn is_p_1s(&self) -> bool {
        *self == Tperiod::P1s
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn is_p_500ms(&self) -> bool {
        *self == Tperiod::P500ms
    }
    #[doc = "250 ms"]
    #[inline(always)]
    pub fn is_p_250ms(&self) -> bool {
        *self == Tperiod::P250ms
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn is_p_125ms(&self) -> bool {
        *self == Tperiod::P125ms
    }
}
#[doc = "Field `TPERIOD` writer - Period of the Output Pulse"]
pub type TperiodW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tperiod>;
impl<'a, REG> TperiodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 second"]
    #[inline(always)]
    pub fn p_1s(self) -> &'a mut crate::W<REG> {
        self.variant(Tperiod::P1s)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn p_500ms(self) -> &'a mut crate::W<REG> {
        self.variant(Tperiod::P500ms)
    }
    #[doc = "250 ms"]
    #[inline(always)]
    pub fn p_250ms(self) -> &'a mut crate::W<REG> {
        self.variant(Tperiod::P250ms)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn p_125ms(self) -> &'a mut crate::W<REG> {
        self.variant(Tperiod::P125ms)
    }
}
impl R {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HrmodR {
        HrmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PersianR {
        PersianR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NegppmR {
        NegppmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CorrectionR {
        CorrectionR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HighppmR {
        HighppmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - RTCOUT0 Output Source Selection"]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    pub fn thigh(&self) -> ThighR {
        ThighR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    pub fn tperiod(&self) -> TperiodR {
        TperiodR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrmod(&mut self) -> HrmodW<MrSpec> {
        HrmodW::new(self, 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn persian(&mut self) -> PersianW<MrSpec> {
        PersianW::new(self, 1)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn negppm(&mut self) -> NegppmW<MrSpec> {
        NegppmW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    #[must_use]
    pub fn correction(&mut self) -> CorrectionW<MrSpec> {
        CorrectionW::new(self, 8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn highppm(&mut self) -> HighppmW<MrSpec> {
        HighppmW::new(self, 15)
    }
    #[doc = "Bits 16:18 - RTCOUT0 Output Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<MrSpec> {
        Out0W::new(self, 16)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<MrSpec> {
        Out1W::new(self, 20)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn thigh(&mut self) -> ThighW<MrSpec> {
        ThighW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tperiod(&mut self) -> TperiodW<MrSpec> {
        TperiodW::new(self, 28)
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
