#[doc = "Register `RCMR` reader"]
pub type R = crate::R<RcmrSpec>;
#[doc = "Register `RCMR` writer"]
pub type W = crate::W<RcmrSpec>;
#[doc = "Receive Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "0: Divided Clock"]
    Mck = 0,
    #[doc = "1: TK Clock signal"]
    Tk = 1,
    #[doc = "2: RK pin"]
    Rk = 2,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - Receive Clock Selection"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cks> {
        match self.bits {
            0 => Some(Cks::Mck),
            1 => Some(Cks::Tk),
            2 => Some(Cks::Rk),
            _ => None,
        }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cks::Mck
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == Cks::Tk
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == Cks::Rk
    }
}
#[doc = "Field `CKS` writer - Receive Clock Selection"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cks>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Mck)
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Tk)
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Rk)
    }
}
#[doc = "Receive Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cko {
    #[doc = "0: None, RK pin is an input"]
    None = 0,
    #[doc = "1: Continuous Receive Clock, RK pin is an output"]
    Continuous = 1,
    #[doc = "2: Receive Clock only during data transfers, RK pin is an output"]
    Transfer = 2,
}
impl From<Cko> for u8 {
    #[inline(always)]
    fn from(variant: Cko) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cko {
    type Ux = u8;
}
impl crate::IsEnum for Cko {}
#[doc = "Field `CKO` reader - Receive Clock Output Mode Selection"]
pub type CkoR = crate::FieldReader<Cko>;
impl CkoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cko> {
        match self.bits {
            0 => Some(Cko::None),
            1 => Some(Cko::Continuous),
            2 => Some(Cko::Transfer),
            _ => None,
        }
    }
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cko::None
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Cko::Continuous
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Cko::Transfer
    }
}
#[doc = "Field `CKO` writer - Receive Clock Output Mode Selection"]
pub type CkoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cko>;
impl<'a, REG> CkoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::None)
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::Continuous)
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::Transfer)
    }
}
#[doc = "Field `CKI` reader - Receive Clock Inversion"]
pub type CkiR = crate::BitReader;
#[doc = "Field `CKI` writer - Receive Clock Inversion"]
pub type CkiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckg {
    #[doc = "0: None"]
    Continuous = 0,
    #[doc = "1: Receive Clock enabled only if RF Low"]
    EnRfLow = 1,
    #[doc = "2: Receive Clock enabled only if RF High"]
    EnRfHigh = 2,
}
impl From<Ckg> for u8 {
    #[inline(always)]
    fn from(variant: Ckg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckg {
    type Ux = u8;
}
impl crate::IsEnum for Ckg {}
#[doc = "Field `CKG` reader - Receive Clock Gating Selection"]
pub type CkgR = crate::FieldReader<Ckg>;
impl CkgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckg> {
        match self.bits {
            0 => Some(Ckg::Continuous),
            1 => Some(Ckg::EnRfLow),
            2 => Some(Ckg::EnRfHigh),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ckg::Continuous
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn is_en_rf_low(&self) -> bool {
        *self == Ckg::EnRfLow
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn is_en_rf_high(&self) -> bool {
        *self == Ckg::EnRfHigh
    }
}
#[doc = "Field `CKG` writer - Receive Clock Gating Selection"]
pub type CkgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckg>;
impl<'a, REG> CkgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Ckg::Continuous)
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn en_rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ckg::EnRfLow)
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn en_rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ckg::EnRfHigh)
    }
}
#[doc = "Receive Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Start {
    #[doc = "0: Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    Continuous = 0,
    #[doc = "1: Transmit start"]
    Transmit = 1,
    #[doc = "2: Detection of a low level on RF signal"]
    RfLow = 2,
    #[doc = "3: Detection of a high level on RF signal"]
    RfHigh = 3,
    #[doc = "4: Detection of a falling edge on RF signal"]
    RfFalling = 4,
    #[doc = "5: Detection of a rising edge on RF signal"]
    RfRising = 5,
    #[doc = "6: Detection of any level change on RF signal"]
    RfLevel = 6,
    #[doc = "7: Detection of any edge on RF signal"]
    RfEdge = 7,
    #[doc = "8: Compare 0"]
    Cmp0 = 8,
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Start {
    type Ux = u8;
}
impl crate::IsEnum for Start {}
#[doc = "Field `START` reader - Receive Start Selection"]
pub type StartR = crate::FieldReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Start> {
        match self.bits {
            0 => Some(Start::Continuous),
            1 => Some(Start::Transmit),
            2 => Some(Start::RfLow),
            3 => Some(Start::RfHigh),
            4 => Some(Start::RfFalling),
            5 => Some(Start::RfRising),
            6 => Some(Start::RfLevel),
            7 => Some(Start::RfEdge),
            8 => Some(Start::Cmp0),
            _ => None,
        }
    }
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Start::Continuous
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == Start::Transmit
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn is_rf_low(&self) -> bool {
        *self == Start::RfLow
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn is_rf_high(&self) -> bool {
        *self == Start::RfHigh
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_falling(&self) -> bool {
        *self == Start::RfFalling
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_rising(&self) -> bool {
        *self == Start::RfRising
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn is_rf_level(&self) -> bool {
        *self == Start::RfLevel
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_edge(&self) -> bool {
        *self == Start::RfEdge
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn is_cmp_0(&self) -> bool {
        *self == Start::Cmp0
    }
}
#[doc = "Field `START` writer - Receive Start Selection"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 4, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Continuous)
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Transmit)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Start::RfLow)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Start::RfHigh)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn rf_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Start::RfFalling)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn rf_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Start::RfRising)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn rf_level(self) -> &'a mut crate::W<REG> {
        self.variant(Start::RfLevel)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn rf_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Start::RfEdge)
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn cmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Cmp0)
    }
}
#[doc = "Field `STOP` reader - Receive Stop Selection"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Receive Stop Selection"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTDLY` reader - Receive Start Delay"]
pub type SttdlyR = crate::FieldReader;
#[doc = "Field `STTDLY` writer - Receive Start Delay"]
pub type SttdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERIOD` reader - Receive Period Divider Selection"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Receive Period Divider Selection"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CkoR {
        CkoR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CkiR {
        CkiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CkgR {
        CkgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> SttdlyR {
        SttdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CksW<RcmrSpec> {
        CksW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cko(&mut self) -> CkoW<RcmrSpec> {
        CkoW::new(self, 2)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn cki(&mut self) -> CkiW<RcmrSpec> {
        CkiW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckg(&mut self) -> CkgW<RcmrSpec> {
        CkgW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<RcmrSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<RcmrSpec> {
        StopW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sttdly(&mut self) -> SttdlyW<RcmrSpec> {
        SttdlyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<RcmrSpec> {
        PeriodW::new(self, 24)
    }
}
#[doc = "Receive Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcmrSpec;
impl crate::RegisterSpec for RcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcmr::R`](R) reader structure"]
impl crate::Readable for RcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcmr::W`](W) writer structure"]
impl crate::Writable for RcmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCMR to value 0"]
impl crate::Resettable for RcmrSpec {
    const RESET_VALUE: u32 = 0;
}
