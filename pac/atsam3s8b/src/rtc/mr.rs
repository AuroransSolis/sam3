#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `HRMOD` reader - 12-/24-hour Mode"]
pub type HRMOD_R = crate::BitReader;
#[doc = "Field `HRMOD` writer - 12-/24-hour Mode"]
pub type HRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSIAN` reader - PERSIAN Calendar"]
pub type PERSIAN_R = crate::BitReader;
#[doc = "Field `PERSIAN` writer - PERSIAN Calendar"]
pub type PERSIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEGPPM` reader - NEGative PPM Correction"]
pub type NEGPPM_R = crate::BitReader;
#[doc = "Field `NEGPPM` writer - NEGative PPM Correction"]
pub type NEGPPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRECTION` reader - Slow Clock Correction"]
pub type CORRECTION_R = crate::FieldReader;
#[doc = "Field `CORRECTION` writer - Slow Clock Correction"]
pub type CORRECTION_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HIGHPPM` reader - HIGH PPM Correction"]
pub type HIGHPPM_R = crate::BitReader;
#[doc = "Field `HIGHPPM` writer - HIGH PPM Correction"]
pub type HIGHPPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT0` reader - RTCOUT0 Output Source Selection"]
pub type OUT0_R = crate::FieldReader<OUT0_A>;
#[doc = "RTCOUT0 Output Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT0_A {
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
impl From<OUT0_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUT0_A {
    type Ux = u8;
}
impl OUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUT0_A {
        match self.bits {
            0 => OUT0_A::NoWave,
            1 => OUT0_A::Freq1hz,
            2 => OUT0_A::Freq32hz,
            3 => OUT0_A::Freq64hz,
            4 => OUT0_A::Freq512hz,
            5 => OUT0_A::AlarmToggle,
            6 => OUT0_A::AlarmFlag,
            7 => OUT0_A::ProgPulse,
            _ => unreachable!(),
        }
    }
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == OUT0_A::NoWave
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == OUT0_A::Freq1hz
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == OUT0_A::Freq32hz
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == OUT0_A::Freq64hz
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == OUT0_A::Freq512hz
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == OUT0_A::AlarmToggle
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == OUT0_A::AlarmFlag
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == OUT0_A::ProgPulse
    }
}
#[doc = "Field `OUT0` writer - RTCOUT0 Output Source Selection"]
pub type OUT0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OUT0_A>;
impl<'a, REG> OUT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::NoWave)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::Freq1hz)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::Freq32hz)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::Freq64hz)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::Freq512hz)
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::AlarmToggle)
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::AlarmFlag)
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(OUT0_A::ProgPulse)
    }
}
#[doc = "Field `OUT1` reader - RTCOUT1 Output Source Selection"]
pub type OUT1_R = crate::FieldReader<OUT1_A>;
#[doc = "RTCOUT1 Output Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT1_A {
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
impl From<OUT1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUT1_A {
    type Ux = u8;
}
impl OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUT1_A {
        match self.bits {
            0 => OUT1_A::NoWave,
            1 => OUT1_A::Freq1hz,
            2 => OUT1_A::Freq32hz,
            3 => OUT1_A::Freq64hz,
            4 => OUT1_A::Freq512hz,
            5 => OUT1_A::AlarmToggle,
            6 => OUT1_A::AlarmFlag,
            7 => OUT1_A::ProgPulse,
            _ => unreachable!(),
        }
    }
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == OUT1_A::NoWave
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == OUT1_A::Freq1hz
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == OUT1_A::Freq32hz
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == OUT1_A::Freq64hz
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == OUT1_A::Freq512hz
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == OUT1_A::AlarmToggle
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == OUT1_A::AlarmFlag
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == OUT1_A::ProgPulse
    }
}
#[doc = "Field `OUT1` writer - RTCOUT1 Output Source Selection"]
pub type OUT1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OUT1_A>;
impl<'a, REG> OUT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::NoWave)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::Freq1hz)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::Freq32hz)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::Freq64hz)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::Freq512hz)
    }
    #[doc = "output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::AlarmToggle)
    }
    #[doc = "output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::AlarmFlag)
    }
    #[doc = "duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(OUT1_A::ProgPulse)
    }
}
#[doc = "Field `THIGH` reader - High Duration of the Output Pulse"]
pub type THIGH_R = crate::FieldReader<THIGH_A>;
#[doc = "High Duration of the Output Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THIGH_A {
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
impl From<THIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: THIGH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for THIGH_A {
    type Ux = u8;
}
impl THIGH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THIGH_A {
        match self.bits {
            0 => THIGH_A::H31ms,
            1 => THIGH_A::H16ms,
            2 => THIGH_A::H4ms,
            3 => THIGH_A::H976us,
            4 => THIGH_A::H488us,
            5 => THIGH_A::H122us,
            6 => THIGH_A::H30us,
            7 => THIGH_A::H15us,
            _ => unreachable!(),
        }
    }
    #[doc = "31.2 ms"]
    #[inline(always)]
    pub fn is_h_31ms(&self) -> bool {
        *self == THIGH_A::H31ms
    }
    #[doc = "15.6 ms"]
    #[inline(always)]
    pub fn is_h_16ms(&self) -> bool {
        *self == THIGH_A::H16ms
    }
    #[doc = "3.91 ms"]
    #[inline(always)]
    pub fn is_h_4ms(&self) -> bool {
        *self == THIGH_A::H4ms
    }
    #[doc = "976 us"]
    #[inline(always)]
    pub fn is_h_976us(&self) -> bool {
        *self == THIGH_A::H976us
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn is_h_488us(&self) -> bool {
        *self == THIGH_A::H488us
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn is_h_122us(&self) -> bool {
        *self == THIGH_A::H122us
    }
    #[doc = "30.5 us"]
    #[inline(always)]
    pub fn is_h_30us(&self) -> bool {
        *self == THIGH_A::H30us
    }
    #[doc = "15.2 us"]
    #[inline(always)]
    pub fn is_h_15us(&self) -> bool {
        *self == THIGH_A::H15us
    }
}
#[doc = "Field `THIGH` writer - High Duration of the Output Pulse"]
pub type THIGH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, THIGH_A>;
impl<'a, REG> THIGH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "31.2 ms"]
    #[inline(always)]
    pub fn h_31ms(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H31ms)
    }
    #[doc = "15.6 ms"]
    #[inline(always)]
    pub fn h_16ms(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H16ms)
    }
    #[doc = "3.91 ms"]
    #[inline(always)]
    pub fn h_4ms(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H4ms)
    }
    #[doc = "976 us"]
    #[inline(always)]
    pub fn h_976us(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H976us)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn h_488us(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H488us)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn h_122us(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H122us)
    }
    #[doc = "30.5 us"]
    #[inline(always)]
    pub fn h_30us(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H30us)
    }
    #[doc = "15.2 us"]
    #[inline(always)]
    pub fn h_15us(self) -> &'a mut crate::W<REG> {
        self.variant(THIGH_A::H15us)
    }
}
#[doc = "Field `TPERIOD` reader - Period of the Output Pulse"]
pub type TPERIOD_R = crate::FieldReader<TPERIOD_A>;
#[doc = "Period of the Output Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPERIOD_A {
    #[doc = "0: 1 second"]
    P1s = 0,
    #[doc = "1: 500 ms"]
    P500ms = 1,
    #[doc = "2: 250 ms"]
    P250ms = 2,
    #[doc = "3: 125 ms"]
    P125ms = 3,
}
impl From<TPERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TPERIOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPERIOD_A {
    type Ux = u8;
}
impl TPERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPERIOD_A {
        match self.bits {
            0 => TPERIOD_A::P1s,
            1 => TPERIOD_A::P500ms,
            2 => TPERIOD_A::P250ms,
            3 => TPERIOD_A::P125ms,
            _ => unreachable!(),
        }
    }
    #[doc = "1 second"]
    #[inline(always)]
    pub fn is_p_1s(&self) -> bool {
        *self == TPERIOD_A::P1s
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn is_p_500ms(&self) -> bool {
        *self == TPERIOD_A::P500ms
    }
    #[doc = "250 ms"]
    #[inline(always)]
    pub fn is_p_250ms(&self) -> bool {
        *self == TPERIOD_A::P250ms
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn is_p_125ms(&self) -> bool {
        *self == TPERIOD_A::P125ms
    }
}
#[doc = "Field `TPERIOD` writer - Period of the Output Pulse"]
pub type TPERIOD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TPERIOD_A>;
impl<'a, REG> TPERIOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 second"]
    #[inline(always)]
    pub fn p_1s(self) -> &'a mut crate::W<REG> {
        self.variant(TPERIOD_A::P1s)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn p_500ms(self) -> &'a mut crate::W<REG> {
        self.variant(TPERIOD_A::P500ms)
    }
    #[doc = "250 ms"]
    #[inline(always)]
    pub fn p_250ms(self) -> &'a mut crate::W<REG> {
        self.variant(TPERIOD_A::P250ms)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn p_125ms(self) -> &'a mut crate::W<REG> {
        self.variant(TPERIOD_A::P125ms)
    }
}
impl R {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HRMOD_R {
        HRMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PERSIAN_R {
        PERSIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NEGPPM_R {
        NEGPPM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CORRECTION_R {
        CORRECTION_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HIGHPPM_R {
        HIGHPPM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - RTCOUT0 Output Source Selection"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    pub fn thigh(&self) -> THIGH_R {
        THIGH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    pub fn tperiod(&self) -> TPERIOD_R {
        TPERIOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrmod(&mut self) -> HRMOD_W<MR_SPEC> {
        HRMOD_W::new(self, 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn persian(&mut self) -> PERSIAN_W<MR_SPEC> {
        PERSIAN_W::new(self, 1)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn negppm(&mut self) -> NEGPPM_W<MR_SPEC> {
        NEGPPM_W::new(self, 4)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    #[must_use]
    pub fn correction(&mut self) -> CORRECTION_W<MR_SPEC> {
        CORRECTION_W::new(self, 8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn highppm(&mut self) -> HIGHPPM_W<MR_SPEC> {
        HIGHPPM_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - RTCOUT0 Output Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> OUT0_W<MR_SPEC> {
        OUT0_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> OUT1_W<MR_SPEC> {
        OUT1_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn thigh(&mut self) -> THIGH_W<MR_SPEC> {
        THIGH_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tperiod(&mut self) -> TPERIOD_W<MR_SPEC> {
        TPERIOD_W::new(self, 28)
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
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
