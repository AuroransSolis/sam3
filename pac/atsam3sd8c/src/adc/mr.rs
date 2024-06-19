#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgen {
    #[doc = "0: Hardware triggers are disabled. Starting a conversion is only possible by software."]
    Dis = 0,
    #[doc = "1: Hardware trigger selected by TRGSEL field is enabled."]
    En = 1,
}
impl From<Trgen> for bool {
    #[inline(always)]
    fn from(variant: Trgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TrgenR = crate::BitReader<Trgen>;
impl TrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgen {
        match self.bits {
            false => Trgen::Dis,
            true => Trgen::En,
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgen::Dis
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Trgen::En
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TrgenW<'a, REG> = crate::BitWriter<'a, REG, Trgen>;
impl<'a, REG> TrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen::Dis)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen::En)
    }
}
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsel {
    #[doc = "0: External trigger"]
    AdcTrig0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    AdcTrig1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    AdcTrig2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    AdcTrig3 = 3,
    #[doc = "4: PWM Event Line 0"]
    AdcTrig4 = 4,
    #[doc = "5: PWM Event Line 1"]
    AdcTrig5 = 5,
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(variant: Trgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsel {
    type Ux = u8;
}
impl crate::IsEnum for Trgsel {}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TrgselR = crate::FieldReader<Trgsel>;
impl TrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgsel> {
        match self.bits {
            0 => Some(Trgsel::AdcTrig0),
            1 => Some(Trgsel::AdcTrig1),
            2 => Some(Trgsel::AdcTrig2),
            3 => Some(Trgsel::AdcTrig3),
            4 => Some(Trgsel::AdcTrig4),
            5 => Some(Trgsel::AdcTrig5),
            _ => None,
        }
    }
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn is_adc_trig0(&self) -> bool {
        *self == Trgsel::AdcTrig0
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn is_adc_trig1(&self) -> bool {
        *self == Trgsel::AdcTrig1
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn is_adc_trig2(&self) -> bool {
        *self == Trgsel::AdcTrig2
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn is_adc_trig3(&self) -> bool {
        *self == Trgsel::AdcTrig3
    }
    #[doc = "PWM Event Line 0"]
    #[inline(always)]
    pub fn is_adc_trig4(&self) -> bool {
        *self == Trgsel::AdcTrig4
    }
    #[doc = "PWM Event Line 1"]
    #[inline(always)]
    pub fn is_adc_trig5(&self) -> bool {
        *self == Trgsel::AdcTrig5
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgsel>;
impl<'a, REG> TrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn adc_trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::AdcTrig0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn adc_trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::AdcTrig1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn adc_trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::AdcTrig2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn adc_trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::AdcTrig3)
    }
    #[doc = "PWM Event Line 0"]
    #[inline(always)]
    pub fn adc_trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::AdcTrig4)
    }
    #[doc = "PWM Event Line 1"]
    #[inline(always)]
    pub fn adc_trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::AdcTrig5)
    }
}
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowres {
    #[doc = "0: 12-bit resolution"]
    Bits12 = 0,
    #[doc = "1: 10-bit resolution"]
    Bits10 = 1,
}
impl From<Lowres> for bool {
    #[inline(always)]
    fn from(variant: Lowres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWRES` reader - Resolution"]
pub type LowresR = crate::BitReader<Lowres>;
impl LowresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowres {
        match self.bits {
            false => Lowres::Bits12,
            true => Lowres::Bits10,
        }
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_bits_12(&self) -> bool {
        *self == Lowres::Bits12
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        *self == Lowres::Bits10
    }
}
#[doc = "Field `LOWRES` writer - Resolution"]
pub type LowresW<'a, REG> = crate::BitWriter<'a, REG, Lowres>;
impl<'a, REG> LowresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn bits_12(self) -> &'a mut crate::W<REG> {
        self.variant(Lowres::Bits12)
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut crate::W<REG> {
        self.variant(Lowres::Bits10)
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleep {
    #[doc = "0: Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    Normal = 0,
    #[doc = "1: Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    Sleep = 1,
}
impl From<Sleep> for bool {
    #[inline(always)]
    fn from(variant: Sleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SleepR = crate::BitReader<Sleep>;
impl SleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleep {
        match self.bits {
            false => Sleep::Normal,
            true => Sleep::Sleep,
        }
    }
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sleep::Normal
    }
    #[doc = "Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Sleep::Sleep
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG, Sleep>;
impl<'a, REG> SleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::Normal)
    }
    #[doc = "Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::Sleep)
    }
}
#[doc = "Fast Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwup {
    #[doc = "0: Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    Off = 0,
    #[doc = "1: Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    On = 1,
}
impl From<Fwup> for bool {
    #[inline(always)]
    fn from(variant: Fwup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWUP` reader - Fast Wake Up"]
pub type FwupR = crate::BitReader<Fwup>;
impl FwupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwup {
        match self.bits {
            false => Fwup::Off,
            true => Fwup::On,
        }
    }
    #[doc = "Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Fwup::Off
    }
    #[doc = "Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Fwup::On
    }
}
#[doc = "Field `FWUP` writer - Fast Wake Up"]
pub type FwupW<'a, REG> = crate::BitWriter<'a, REG, Fwup>;
impl<'a, REG> FwupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Fwup::Off)
    }
    #[doc = "Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Fwup::On)
    }
}
#[doc = "Free Run Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Freerun {
    #[doc = "0: Normal Mode"]
    Off = 0,
    #[doc = "1: Free Run Mode: Never wait for any trigger."]
    On = 1,
}
impl From<Freerun> for bool {
    #[inline(always)]
    fn from(variant: Freerun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREERUN` reader - Free Run Mode"]
pub type FreerunR = crate::BitReader<Freerun>;
impl FreerunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freerun {
        match self.bits {
            false => Freerun::Off,
            true => Freerun::On,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Freerun::Off
    }
    #[doc = "Free Run Mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Freerun::On
    }
}
#[doc = "Field `FREERUN` writer - Free Run Mode"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG, Freerun>;
impl<'a, REG> FreerunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Freerun::Off)
    }
    #[doc = "Free Run Mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Freerun::On)
    }
}
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PrescalR = crate::FieldReader;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PrescalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Start Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startup {
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
impl From<Startup> for u8 {
    #[inline(always)]
    fn from(variant: Startup) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startup {
    type Ux = u8;
}
impl crate::IsEnum for Startup {}
#[doc = "Field `STARTUP` reader - Start Up Time"]
pub type StartupR = crate::FieldReader<Startup>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startup {
        match self.bits {
            0 => Startup::Sut0,
            1 => Startup::Sut8,
            2 => Startup::Sut16,
            3 => Startup::Sut24,
            4 => Startup::Sut64,
            5 => Startup::Sut80,
            6 => Startup::Sut96,
            7 => Startup::Sut112,
            8 => Startup::Sut512,
            9 => Startup::Sut576,
            10 => Startup::Sut640,
            11 => Startup::Sut704,
            12 => Startup::Sut768,
            13 => Startup::Sut832,
            14 => Startup::Sut896,
            15 => Startup::Sut960,
            _ => unreachable!(),
        }
    }
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == Startup::Sut0
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == Startup::Sut8
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == Startup::Sut16
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == Startup::Sut24
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == Startup::Sut64
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == Startup::Sut80
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == Startup::Sut96
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == Startup::Sut112
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == Startup::Sut512
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == Startup::Sut576
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == Startup::Sut640
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == Startup::Sut704
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == Startup::Sut768
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == Startup::Sut832
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == Startup::Sut896
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == Startup::Sut960
    }
}
#[doc = "Field `STARTUP` writer - Start Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startup, crate::Safe>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut0)
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut8)
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut16)
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut24)
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut64)
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut80)
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut96)
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut112)
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut512)
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut576)
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut640)
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut704)
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut768)
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut832)
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut896)
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut crate::W<REG> {
        self.variant(Startup::Sut960)
    }
}
#[doc = "Analog Settling Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Settling {
    #[doc = "0: 3 periods of ADCClock"]
    Ast3 = 0,
    #[doc = "1: 5 periods of ADCClock"]
    Ast5 = 1,
    #[doc = "2: 9 periods of ADCClock"]
    Ast9 = 2,
    #[doc = "3: 17 periods of ADCClock"]
    Ast17 = 3,
}
impl From<Settling> for u8 {
    #[inline(always)]
    fn from(variant: Settling) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Settling {
    type Ux = u8;
}
impl crate::IsEnum for Settling {}
#[doc = "Field `SETTLING` reader - Analog Settling Time"]
pub type SettlingR = crate::FieldReader<Settling>;
impl SettlingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Settling {
        match self.bits {
            0 => Settling::Ast3,
            1 => Settling::Ast5,
            2 => Settling::Ast9,
            3 => Settling::Ast17,
            _ => unreachable!(),
        }
    }
    #[doc = "3 periods of ADCClock"]
    #[inline(always)]
    pub fn is_ast3(&self) -> bool {
        *self == Settling::Ast3
    }
    #[doc = "5 periods of ADCClock"]
    #[inline(always)]
    pub fn is_ast5(&self) -> bool {
        *self == Settling::Ast5
    }
    #[doc = "9 periods of ADCClock"]
    #[inline(always)]
    pub fn is_ast9(&self) -> bool {
        *self == Settling::Ast9
    }
    #[doc = "17 periods of ADCClock"]
    #[inline(always)]
    pub fn is_ast17(&self) -> bool {
        *self == Settling::Ast17
    }
}
#[doc = "Field `SETTLING` writer - Analog Settling Time"]
pub type SettlingW<'a, REG> = crate::FieldWriter<'a, REG, 2, Settling, crate::Safe>;
impl<'a, REG> SettlingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3 periods of ADCClock"]
    #[inline(always)]
    pub fn ast3(self) -> &'a mut crate::W<REG> {
        self.variant(Settling::Ast3)
    }
    #[doc = "5 periods of ADCClock"]
    #[inline(always)]
    pub fn ast5(self) -> &'a mut crate::W<REG> {
        self.variant(Settling::Ast5)
    }
    #[doc = "9 periods of ADCClock"]
    #[inline(always)]
    pub fn ast9(self) -> &'a mut crate::W<REG> {
        self.variant(Settling::Ast9)
    }
    #[doc = "17 periods of ADCClock"]
    #[inline(always)]
    pub fn ast17(self) -> &'a mut crate::W<REG> {
        self.variant(Settling::Ast17)
    }
}
#[doc = "Analog Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anach {
    #[doc = "0: No analog change on channel switching: DIFF0, GAIN0 and OFF0 are used for all channels"]
    None = 0,
    #[doc = "1: Allows different analog settings for each channel. See ADC_CGR and ADC_COR Registers"]
    Allowed = 1,
}
impl From<Anach> for bool {
    #[inline(always)]
    fn from(variant: Anach) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACH` reader - Analog Change"]
pub type AnachR = crate::BitReader<Anach>;
impl AnachR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anach {
        match self.bits {
            false => Anach::None,
            true => Anach::Allowed,
        }
    }
    #[doc = "No analog change on channel switching: DIFF0, GAIN0 and OFF0 are used for all channels"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Anach::None
    }
    #[doc = "Allows different analog settings for each channel. See ADC_CGR and ADC_COR Registers"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Anach::Allowed
    }
}
#[doc = "Field `ANACH` writer - Analog Change"]
pub type AnachW<'a, REG> = crate::BitWriter<'a, REG, Anach>;
impl<'a, REG> AnachW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog change on channel switching: DIFF0, GAIN0 and OFF0 are used for all channels"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Anach::None)
    }
    #[doc = "Allows different analog settings for each channel. See ADC_CGR and ADC_COR Registers"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Anach::Allowed)
    }
}
#[doc = "Field `TRACKTIM` reader - Tracking Time"]
pub type TracktimR = crate::FieldReader;
#[doc = "Field `TRACKTIM` writer - Tracking Time"]
pub type TracktimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRANSFER` reader - Transfer Period"]
pub type TransferR = crate::FieldReader;
#[doc = "Field `TRANSFER` writer - Transfer Period"]
pub type TransferW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Use Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Useq {
    #[doc = "0: Normal Mode: The controller converts channels in a simple numeric order."]
    NumOrder = 0,
    #[doc = "1: User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers."]
    RegOrder = 1,
}
impl From<Useq> for bool {
    #[inline(always)]
    fn from(variant: Useq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEQ` reader - Use Sequence Enable"]
pub type UseqR = crate::BitReader<Useq>;
impl UseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Useq {
        match self.bits {
            false => Useq::NumOrder,
            true => Useq::RegOrder,
        }
    }
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == Useq::NumOrder
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers."]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == Useq::RegOrder
    }
}
#[doc = "Field `USEQ` writer - Use Sequence Enable"]
pub type UseqW<'a, REG> = crate::BitWriter<'a, REG, Useq>;
impl<'a, REG> UseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut crate::W<REG> {
        self.variant(Useq::NumOrder)
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut crate::W<REG> {
        self.variant(Useq::RegOrder)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TrgenR {
        TrgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&self) -> LowresR {
        LowresR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    pub fn fwup(&self) -> FwupR {
        FwupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PrescalR {
        PrescalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Analog Settling Time"]
    #[inline(always)]
    pub fn settling(&self) -> SettlingR {
        SettlingR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Analog Change"]
    #[inline(always)]
    pub fn anach(&self) -> AnachR {
        AnachR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TracktimR {
        TracktimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&self) -> TransferR {
        TransferR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> UseqR {
        UseqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TrgenW<MrSpec> {
        TrgenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<MrSpec> {
        TrgselW::new(self, 1)
    }
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn lowres(&mut self) -> LowresW<MrSpec> {
        LowresW::new(self, 4)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<MrSpec> {
        SleepW::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn fwup(&mut self) -> FwupW<MrSpec> {
        FwupW::new(self, 6)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FreerunW<MrSpec> {
        FreerunW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescal(&mut self) -> PrescalW<MrSpec> {
        PrescalW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<MrSpec> {
        StartupW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Analog Settling Time"]
    #[inline(always)]
    #[must_use]
    pub fn settling(&mut self) -> SettlingW<MrSpec> {
        SettlingW::new(self, 20)
    }
    #[doc = "Bit 23 - Analog Change"]
    #[inline(always)]
    #[must_use]
    pub fn anach(&mut self) -> AnachW<MrSpec> {
        AnachW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    #[must_use]
    pub fn tracktim(&mut self) -> TracktimW<MrSpec> {
        TracktimW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    #[must_use]
    pub fn transfer(&mut self) -> TransferW<MrSpec> {
        TransferW::new(self, 28)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    #[must_use]
    pub fn useq(&mut self) -> UseqW<MrSpec> {
        UseqW::new(self, 31)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
