#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "FWUP Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwups {
    #[doc = "0: no wake-up due to the assertion of the FWUP pin has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: at least one wake-up due to the assertion of the FWUP pin has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Fwups> for bool {
    #[inline(always)]
    fn from(variant: Fwups) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWUPS` reader - FWUP Wake-up Status"]
pub type FwupsR = crate::BitReader<Fwups>;
impl FwupsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwups {
        match self.bits {
            false => Fwups::No,
            true => Fwups::Present,
        }
    }
    #[doc = "no wake-up due to the assertion of the FWUP pin has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Fwups::No
    }
    #[doc = "at least one wake-up due to the assertion of the FWUP pin has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Fwups::Present
    }
}
#[doc = "WKUP Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkups {
    #[doc = "0: no wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: at least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Wkups> for bool {
    #[inline(always)]
    fn from(variant: Wkups) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPS` reader - WKUP Wake-up Status"]
pub type WkupsR = crate::BitReader<Wkups>;
impl WkupsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkups {
        match self.bits {
            false => Wkups::No,
            true => Wkups::Present,
        }
    }
    #[doc = "no wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Wkups::No
    }
    #[doc = "at least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Wkups::Present
    }
}
#[doc = "Supply Monitor Detection Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smws {
    #[doc = "0: no wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: at least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Smws> for bool {
    #[inline(always)]
    fn from(variant: Smws) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMWS` reader - Supply Monitor Detection Wake-up Status"]
pub type SmwsR = crate::BitReader<Smws>;
impl SmwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smws {
        match self.bits {
            false => Smws::No,
            true => Smws::Present,
        }
    }
    #[doc = "no wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Smws::No
    }
    #[doc = "at least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Smws::Present
    }
}
#[doc = "Brownout Detector Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodrsts {
    #[doc = "0: no core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    No = 0,
    #[doc = "1: at least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    Present = 1,
}
impl From<Bodrsts> for bool {
    #[inline(always)]
    fn from(variant: Bodrsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTS` reader - Brownout Detector Reset Status"]
pub type BodrstsR = crate::BitReader<Bodrsts>;
impl BodrstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrsts {
        match self.bits {
            false => Bodrsts::No,
            true => Bodrsts::Present,
        }
    }
    #[doc = "no core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Bodrsts::No
    }
    #[doc = "at least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Bodrsts::Present
    }
}
#[doc = "Supply Monitor Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smrsts {
    #[doc = "0: no supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    No = 0,
    #[doc = "1: at least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    Present = 1,
}
impl From<Smrsts> for bool {
    #[inline(always)]
    fn from(variant: Smrsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTS` reader - Supply Monitor Reset Status"]
pub type SmrstsR = crate::BitReader<Smrsts>;
impl SmrstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smrsts {
        match self.bits {
            false => Smrsts::No,
            true => Smrsts::Present,
        }
    }
    #[doc = "no supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Smrsts::No
    }
    #[doc = "at least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Smrsts::Present
    }
}
#[doc = "Supply Monitor Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sms {
    #[doc = "0: no supply monitor detection since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: at least one supply monitor detection since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Sms> for bool {
    #[inline(always)]
    fn from(variant: Sms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS` reader - Supply Monitor Status"]
pub type SmsR = crate::BitReader<Sms>;
impl SmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sms {
        match self.bits {
            false => Sms::No,
            true => Sms::Present,
        }
    }
    #[doc = "no supply monitor detection since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Sms::No
    }
    #[doc = "at least one supply monitor detection since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Sms::Present
    }
}
#[doc = "Supply Monitor Output Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smos {
    #[doc = "0: the supply monitor detected VDDUTMI higher than its threshold at its last measurement."]
    High = 0,
    #[doc = "1: the supply monitor detected VDDUTMI lower than its threshold at its last measurement."]
    Low = 1,
}
impl From<Smos> for bool {
    #[inline(always)]
    fn from(variant: Smos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMOS` reader - Supply Monitor Output Status"]
pub type SmosR = crate::BitReader<Smos>;
impl SmosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smos {
        match self.bits {
            false => Smos::High,
            true => Smos::Low,
        }
    }
    #[doc = "the supply monitor detected VDDUTMI higher than its threshold at its last measurement."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Smos::High
    }
    #[doc = "the supply monitor detected VDDUTMI lower than its threshold at its last measurement."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Smos::Low
    }
}
#[doc = "32-kHz Oscillator Selection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscsel {
    #[doc = "0: the slow clock, SLCK is generated by the embedded 32-kHz RC oscillator."]
    Rc = 0,
    #[doc = "1: the slow clock, SLCK is generated by the 32-kHz crystal oscillator."]
    Cryst = 1,
}
impl From<Oscsel> for bool {
    #[inline(always)]
    fn from(variant: Oscsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSEL` reader - 32-kHz Oscillator Selection Status"]
pub type OscselR = crate::BitReader<Oscsel>;
impl OscselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscsel {
        match self.bits {
            false => Oscsel::Rc,
            true => Oscsel::Cryst,
        }
    }
    #[doc = "the slow clock, SLCK is generated by the embedded 32-kHz RC oscillator."]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Oscsel::Rc
    }
    #[doc = "the slow clock, SLCK is generated by the 32-kHz crystal oscillator."]
    #[inline(always)]
    pub fn is_cryst(&self) -> bool {
        *self == Oscsel::Cryst
    }
}
#[doc = "FWUP Input Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwupis {
    #[doc = "0: FWUP input is tied low."]
    Low = 0,
    #[doc = "1: FWUP input is tied high."]
    High = 1,
}
impl From<Fwupis> for bool {
    #[inline(always)]
    fn from(variant: Fwupis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWUPIS` reader - FWUP Input Status"]
pub type FwupisR = crate::BitReader<Fwupis>;
impl FwupisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwupis {
        match self.bits {
            false => Fwupis::Low,
            true => Fwupis::High,
        }
    }
    #[doc = "FWUP input is tied low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Fwupis::Low
    }
    #[doc = "FWUP input is tied high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Fwupis::High
    }
}
#[doc = "WKUP Input Status 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis0 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis0> for bool {
    #[inline(always)]
    fn from(variant: Wkupis0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS0` reader - WKUP Input Status 0"]
pub type Wkupis0R = crate::BitReader<Wkupis0>;
impl Wkupis0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis0 {
        match self.bits {
            false => Wkupis0::Dis,
            true => Wkupis0::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis0::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis0::En
    }
}
#[doc = "WKUP Input Status 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis1 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis1> for bool {
    #[inline(always)]
    fn from(variant: Wkupis1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS1` reader - WKUP Input Status 1"]
pub type Wkupis1R = crate::BitReader<Wkupis1>;
impl Wkupis1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis1 {
        match self.bits {
            false => Wkupis1::Dis,
            true => Wkupis1::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis1::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis1::En
    }
}
#[doc = "WKUP Input Status 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis2 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis2> for bool {
    #[inline(always)]
    fn from(variant: Wkupis2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS2` reader - WKUP Input Status 2"]
pub type Wkupis2R = crate::BitReader<Wkupis2>;
impl Wkupis2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis2 {
        match self.bits {
            false => Wkupis2::Dis,
            true => Wkupis2::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis2::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis2::En
    }
}
#[doc = "WKUP Input Status 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis3 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis3> for bool {
    #[inline(always)]
    fn from(variant: Wkupis3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS3` reader - WKUP Input Status 3"]
pub type Wkupis3R = crate::BitReader<Wkupis3>;
impl Wkupis3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis3 {
        match self.bits {
            false => Wkupis3::Dis,
            true => Wkupis3::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis3::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis3::En
    }
}
#[doc = "WKUP Input Status 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis4 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis4> for bool {
    #[inline(always)]
    fn from(variant: Wkupis4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS4` reader - WKUP Input Status 4"]
pub type Wkupis4R = crate::BitReader<Wkupis4>;
impl Wkupis4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis4 {
        match self.bits {
            false => Wkupis4::Dis,
            true => Wkupis4::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis4::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis4::En
    }
}
#[doc = "WKUP Input Status 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis5 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis5> for bool {
    #[inline(always)]
    fn from(variant: Wkupis5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS5` reader - WKUP Input Status 5"]
pub type Wkupis5R = crate::BitReader<Wkupis5>;
impl Wkupis5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis5 {
        match self.bits {
            false => Wkupis5::Dis,
            true => Wkupis5::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis5::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis5::En
    }
}
#[doc = "WKUP Input Status 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis6 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis6> for bool {
    #[inline(always)]
    fn from(variant: Wkupis6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS6` reader - WKUP Input Status 6"]
pub type Wkupis6R = crate::BitReader<Wkupis6>;
impl Wkupis6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis6 {
        match self.bits {
            false => Wkupis6::Dis,
            true => Wkupis6::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis6::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis6::En
    }
}
#[doc = "WKUP Input Status 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis7 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis7> for bool {
    #[inline(always)]
    fn from(variant: Wkupis7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS7` reader - WKUP Input Status 7"]
pub type Wkupis7R = crate::BitReader<Wkupis7>;
impl Wkupis7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis7 {
        match self.bits {
            false => Wkupis7::Dis,
            true => Wkupis7::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis7::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis7::En
    }
}
#[doc = "WKUP Input Status 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis8 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis8> for bool {
    #[inline(always)]
    fn from(variant: Wkupis8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS8` reader - WKUP Input Status 8"]
pub type Wkupis8R = crate::BitReader<Wkupis8>;
impl Wkupis8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis8 {
        match self.bits {
            false => Wkupis8::Dis,
            true => Wkupis8::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis8::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis8::En
    }
}
#[doc = "WKUP Input Status 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis9 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis9> for bool {
    #[inline(always)]
    fn from(variant: Wkupis9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS9` reader - WKUP Input Status 9"]
pub type Wkupis9R = crate::BitReader<Wkupis9>;
impl Wkupis9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis9 {
        match self.bits {
            false => Wkupis9::Dis,
            true => Wkupis9::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis9::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis9::En
    }
}
#[doc = "WKUP Input Status 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis10 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis10> for bool {
    #[inline(always)]
    fn from(variant: Wkupis10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS10` reader - WKUP Input Status 10"]
pub type Wkupis10R = crate::BitReader<Wkupis10>;
impl Wkupis10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis10 {
        match self.bits {
            false => Wkupis10::Dis,
            true => Wkupis10::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis10::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis10::En
    }
}
#[doc = "WKUP Input Status 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis11 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis11> for bool {
    #[inline(always)]
    fn from(variant: Wkupis11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS11` reader - WKUP Input Status 11"]
pub type Wkupis11R = crate::BitReader<Wkupis11>;
impl Wkupis11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis11 {
        match self.bits {
            false => Wkupis11::Dis,
            true => Wkupis11::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis11::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis11::En
    }
}
#[doc = "WKUP Input Status 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis12 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis12> for bool {
    #[inline(always)]
    fn from(variant: Wkupis12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS12` reader - WKUP Input Status 12"]
pub type Wkupis12R = crate::BitReader<Wkupis12>;
impl Wkupis12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis12 {
        match self.bits {
            false => Wkupis12::Dis,
            true => Wkupis12::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis12::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis12::En
    }
}
#[doc = "WKUP Input Status 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis13 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis13> for bool {
    #[inline(always)]
    fn from(variant: Wkupis13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS13` reader - WKUP Input Status 13"]
pub type Wkupis13R = crate::BitReader<Wkupis13>;
impl Wkupis13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis13 {
        match self.bits {
            false => Wkupis13::Dis,
            true => Wkupis13::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis13::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis13::En
    }
}
#[doc = "WKUP Input Status 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis14 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis14> for bool {
    #[inline(always)]
    fn from(variant: Wkupis14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS14` reader - WKUP Input Status 14"]
pub type Wkupis14R = crate::BitReader<Wkupis14>;
impl Wkupis14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis14 {
        match self.bits {
            false => Wkupis14::Dis,
            true => Wkupis14::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis14::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis14::En
    }
}
#[doc = "WKUP Input Status 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis15 {
    #[doc = "0: the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    En = 1,
}
impl From<Wkupis15> for bool {
    #[inline(always)]
    fn from(variant: Wkupis15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS15` reader - WKUP Input Status 15"]
pub type Wkupis15R = crate::BitReader<Wkupis15>;
impl Wkupis15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis15 {
        match self.bits {
            false => Wkupis15::Dis,
            true => Wkupis15::En,
        }
    }
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis15::Dis
    }
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis15::En
    }
}
impl R {
    #[doc = "Bit 0 - FWUP Wake-up Status"]
    #[inline(always)]
    pub fn fwups(&self) -> FwupsR {
        FwupsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUP Wake-up Status"]
    #[inline(always)]
    pub fn wkups(&self) -> WkupsR {
        WkupsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wake-up Status"]
    #[inline(always)]
    pub fn smws(&self) -> SmwsR {
        SmwsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status"]
    #[inline(always)]
    pub fn bodrsts(&self) -> BodrstsR {
        BodrstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status"]
    #[inline(always)]
    pub fn smrsts(&self) -> SmrstsR {
        SmrstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Supply Monitor Status"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline(always)]
    pub fn smos(&self) -> SmosR {
        SmosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline(always)]
    pub fn oscsel(&self) -> OscselR {
        OscselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - FWUP Input Status"]
    #[inline(always)]
    pub fn fwupis(&self) -> FwupisR {
        FwupisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUP Input Status 0"]
    #[inline(always)]
    pub fn wkupis0(&self) -> Wkupis0R {
        Wkupis0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WKUP Input Status 1"]
    #[inline(always)]
    pub fn wkupis1(&self) -> Wkupis1R {
        Wkupis1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WKUP Input Status 2"]
    #[inline(always)]
    pub fn wkupis2(&self) -> Wkupis2R {
        Wkupis2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WKUP Input Status 3"]
    #[inline(always)]
    pub fn wkupis3(&self) -> Wkupis3R {
        Wkupis3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUP Input Status 4"]
    #[inline(always)]
    pub fn wkupis4(&self) -> Wkupis4R {
        Wkupis4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WKUP Input Status 5"]
    #[inline(always)]
    pub fn wkupis5(&self) -> Wkupis5R {
        Wkupis5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - WKUP Input Status 6"]
    #[inline(always)]
    pub fn wkupis6(&self) -> Wkupis6R {
        Wkupis6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WKUP Input Status 7"]
    #[inline(always)]
    pub fn wkupis7(&self) -> Wkupis7R {
        Wkupis7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - WKUP Input Status 8"]
    #[inline(always)]
    pub fn wkupis8(&self) -> Wkupis8R {
        Wkupis8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WKUP Input Status 9"]
    #[inline(always)]
    pub fn wkupis9(&self) -> Wkupis9R {
        Wkupis9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WKUP Input Status 10"]
    #[inline(always)]
    pub fn wkupis10(&self) -> Wkupis10R {
        Wkupis10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - WKUP Input Status 11"]
    #[inline(always)]
    pub fn wkupis11(&self) -> Wkupis11R {
        Wkupis11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WKUP Input Status 12"]
    #[inline(always)]
    pub fn wkupis12(&self) -> Wkupis12R {
        Wkupis12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - WKUP Input Status 13"]
    #[inline(always)]
    pub fn wkupis13(&self) -> Wkupis13R {
        Wkupis13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - WKUP Input Status 14"]
    #[inline(always)]
    pub fn wkupis14(&self) -> Wkupis14R {
        Wkupis14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - WKUP Input Status 15"]
    #[inline(always)]
    pub fn wkupis15(&self) -> Wkupis15R {
        Wkupis15R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Supply Controller Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
