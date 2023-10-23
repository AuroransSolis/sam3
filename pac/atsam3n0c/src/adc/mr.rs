#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader<TRGEN_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEN_A {
    #[doc = "0: Hardware triggers are disabled. Starting a conversion is only possible by software."]
    Dis = 0,
    #[doc = "1: Hardware trigger selected by TRGSEL field is enabled."]
    En = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::Dis,
            true => TRGEN_A::En,
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN_A::Dis
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN_A::En
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGEN_A>;
impl<'a, REG, const O: u8> TRGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TRGEN_A::Dis)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TRGEN_A::En)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader<TRGSEL_A>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: External trigger"]
    AdcTrig0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    AdcTrig1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    AdcTrig2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    AdcTrig3 = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGSEL_A {
    type Ux = u8;
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            0 => Some(TRGSEL_A::AdcTrig0),
            1 => Some(TRGSEL_A::AdcTrig1),
            2 => Some(TRGSEL_A::AdcTrig2),
            3 => Some(TRGSEL_A::AdcTrig3),
            _ => None,
        }
    }
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn is_adc_trig0(&self) -> bool {
        *self == TRGSEL_A::AdcTrig0
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn is_adc_trig1(&self) -> bool {
        *self == TRGSEL_A::AdcTrig1
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn is_adc_trig2(&self) -> bool {
        *self == TRGSEL_A::AdcTrig2
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn is_adc_trig3(&self) -> bool {
        *self == TRGSEL_A::AdcTrig3
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TRGSEL_A>;
impl<'a, REG, const O: u8> TRGSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn adc_trig0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::AdcTrig0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn adc_trig1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::AdcTrig1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn adc_trig2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::AdcTrig2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn adc_trig3(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::AdcTrig3)
    }
}
#[doc = "Field `LOWRES` reader - Resolution"]
pub type LOWRES_R = crate::BitReader<LOWRES_A>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOWRES_A {
    #[doc = "0: 10-bit resolution"]
    Bits10 = 0,
    #[doc = "1: 8-bit resolution"]
    Bits8 = 1,
}
impl From<LOWRES_A> for bool {
    #[inline(always)]
    fn from(variant: LOWRES_A) -> Self {
        variant as u8 != 0
    }
}
impl LOWRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOWRES_A {
        match self.bits {
            false => LOWRES_A::Bits10,
            true => LOWRES_A::Bits8,
        }
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        *self == LOWRES_A::Bits10
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == LOWRES_A::Bits8
    }
}
#[doc = "Field `LOWRES` writer - Resolution"]
pub type LOWRES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LOWRES_A>;
impl<'a, REG, const O: u8> LOWRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut crate::W<REG> {
        self.variant(LOWRES_A::Bits10)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut crate::W<REG> {
        self.variant(LOWRES_A::Bits8)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SLEEP_R = crate::BitReader<SLEEP_A>;
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_A {
    #[doc = "0: Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    Normal = 0,
    #[doc = "1: Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    Sleep = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::Normal,
            true => SLEEP_A::Sleep,
        }
    }
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLEEP_A::Normal
    }
    #[doc = "Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEP_A::Sleep
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLEEP_A>;
impl<'a, REG, const O: u8> SLEEP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::Normal)
    }
    #[doc = "Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::Sleep)
    }
}
#[doc = "Field `FWUP` reader - Fast Wake Up"]
pub type FWUP_R = crate::BitReader<FWUP_A>;
#[doc = "Fast Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWUP_A {
    #[doc = "0: Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    Off = 0,
    #[doc = "1: Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    On = 1,
}
impl From<FWUP_A> for bool {
    #[inline(always)]
    fn from(variant: FWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl FWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWUP_A {
        match self.bits {
            false => FWUP_A::Off,
            true => FWUP_A::On,
        }
    }
    #[doc = "Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FWUP_A::Off
    }
    #[doc = "Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FWUP_A::On
    }
}
#[doc = "Field `FWUP` writer - Fast Wake Up"]
pub type FWUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FWUP_A>;
impl<'a, REG, const O: u8> FWUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FWUP_A::Off)
    }
    #[doc = "Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(FWUP_A::On)
    }
}
#[doc = "Field `FREERUN` reader - Free Run Mode"]
pub type FREERUN_R = crate::BitReader<FREERUN_A>;
#[doc = "Free Run Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREERUN_A {
    #[doc = "0: Normal Mode"]
    Off = 0,
    #[doc = "1: Free Run Mode: Never wait for any trigger."]
    On = 1,
}
impl From<FREERUN_A> for bool {
    #[inline(always)]
    fn from(variant: FREERUN_A) -> Self {
        variant as u8 != 0
    }
}
impl FREERUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FREERUN_A {
        match self.bits {
            false => FREERUN_A::Off,
            true => FREERUN_A::On,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FREERUN_A::Off
    }
    #[doc = "Free Run Mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FREERUN_A::On
    }
}
#[doc = "Field `FREERUN` writer - Free Run Mode"]
pub type FREERUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FREERUN_A>;
impl<'a, REG, const O: u8> FREERUN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FREERUN_A::Off)
    }
    #[doc = "Free Run Mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(FREERUN_A::On)
    }
}
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PRESCAL_R = crate::FieldReader;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PRESCAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `STARTUP` reader - Start Up Time"]
pub type STARTUP_R = crate::FieldReader<STARTUP_A>;
#[doc = "Start Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of ADCClock"]
    Sut0 = 0,
    #[doc = "1: 8 periods of ADCClock"]
    Sut8 = 1,
    #[doc = "2: 16 periods of ADCClock"]
    Sut16 = 2,
    #[doc = "3: 24 periods of ADCClock"]
    Sut24 = 3,
    #[doc = "4: 64 periods of ADCClock"]
    Sut64 = 4,
    #[doc = "5: 80 periods of ADCClock"]
    Sut80 = 5,
    #[doc = "6: 96 periods of ADCClock"]
    Sut96 = 6,
    #[doc = "7: 112 periods of ADCClock"]
    Sut112 = 7,
    #[doc = "8: 512 periods of ADCClock"]
    Sut512 = 8,
    #[doc = "9: 576 periods of ADCClock"]
    Sut576 = 9,
    #[doc = "10: 640 periods of ADCClock"]
    Sut640 = 10,
    #[doc = "11: 704 periods of ADCClock"]
    Sut704 = 11,
    #[doc = "12: 768 periods of ADCClock"]
    Sut768 = 12,
    #[doc = "13: 832 periods of ADCClock"]
    Sut832 = 13,
    #[doc = "14: 896 periods of ADCClock"]
    Sut896 = 14,
    #[doc = "15: 960 periods of ADCClock"]
    Sut960 = 15,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTUP_A {
    type Ux = u8;
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::Sut0,
            1 => STARTUP_A::Sut8,
            2 => STARTUP_A::Sut16,
            3 => STARTUP_A::Sut24,
            4 => STARTUP_A::Sut64,
            5 => STARTUP_A::Sut80,
            6 => STARTUP_A::Sut96,
            7 => STARTUP_A::Sut112,
            8 => STARTUP_A::Sut512,
            9 => STARTUP_A::Sut576,
            10 => STARTUP_A::Sut640,
            11 => STARTUP_A::Sut704,
            12 => STARTUP_A::Sut768,
            13 => STARTUP_A::Sut832,
            14 => STARTUP_A::Sut896,
            15 => STARTUP_A::Sut960,
            _ => unreachable!(),
        }
    }
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == STARTUP_A::Sut0
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == STARTUP_A::Sut8
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == STARTUP_A::Sut16
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == STARTUP_A::Sut24
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == STARTUP_A::Sut64
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == STARTUP_A::Sut80
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == STARTUP_A::Sut96
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == STARTUP_A::Sut112
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == STARTUP_A::Sut512
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == STARTUP_A::Sut576
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == STARTUP_A::Sut640
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == STARTUP_A::Sut704
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == STARTUP_A::Sut768
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == STARTUP_A::Sut832
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == STARTUP_A::Sut896
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == STARTUP_A::Sut960
    }
}
#[doc = "Field `STARTUP` writer - Start Up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, STARTUP_A>;
impl<'a, REG, const O: u8> STARTUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut0)
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut8)
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut16)
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut24)
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut64)
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut80)
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut96)
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut112)
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut512)
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut576)
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut640)
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut704)
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut768)
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut832)
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut896)
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUP_A::Sut960)
    }
}
#[doc = "Field `TRACKTIM` reader - Tracking Time"]
pub type TRACKTIM_R = crate::FieldReader;
#[doc = "Field `TRACKTIM` writer - Tracking Time"]
pub type TRACKTIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USEQ` reader - Use Sequence Enable"]
pub type USEQ_R = crate::BitReader<USEQ_A>;
#[doc = "Use Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEQ_A {
    #[doc = "0: Normal Mode: The controller converts channels in a simple numeric order."]
    NumOrder = 0,
    #[doc = "1: User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers."]
    RegOrder = 1,
}
impl From<USEQ_A> for bool {
    #[inline(always)]
    fn from(variant: USEQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USEQ_A {
        match self.bits {
            false => USEQ_A::NumOrder,
            true => USEQ_A::RegOrder,
        }
    }
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == USEQ_A::NumOrder
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers."]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == USEQ_A::RegOrder
    }
}
#[doc = "Field `USEQ` writer - Use Sequence Enable"]
pub type USEQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USEQ_A>;
impl<'a, REG, const O: u8> USEQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut crate::W<REG> {
        self.variant(USEQ_A::NumOrder)
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut crate::W<REG> {
        self.variant(USEQ_A::RegOrder)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&self) -> LOWRES_R {
        LOWRES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    pub fn fwup(&self) -> FWUP_R {
        FWUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PRESCAL_R {
        PRESCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TRACKTIM_R {
        TRACKTIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> USEQ_R {
        USEQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<MR_SPEC, 0> {
        TRGEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<MR_SPEC, 1> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn lowres(&mut self) -> LOWRES_W<MR_SPEC, 4> {
        LOWRES_W::new(self)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<MR_SPEC, 5> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn fwup(&mut self) -> FWUP_W<MR_SPEC, 6> {
        FWUP_W::new(self)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<MR_SPEC, 7> {
        FREERUN_W::new(self)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescal(&mut self) -> PRESCAL_W<MR_SPEC, 8> {
        PRESCAL_W::new(self)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<MR_SPEC, 16> {
        STARTUP_W::new(self)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    #[must_use]
    pub fn tracktim(&mut self) -> TRACKTIM_W<MR_SPEC, 24> {
        TRACKTIM_W::new(self)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    #[must_use]
    pub fn useq(&mut self) -> USEQ_W<MR_SPEC, 31> {
        USEQ_W::new(self)
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
