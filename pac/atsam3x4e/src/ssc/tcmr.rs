#[doc = "Register `TCMR` reader"]
pub type R = crate::R<TcmrSpec>;
#[doc = "Register `TCMR` writer"]
pub type W = crate::W<TcmrSpec>;
#[doc = "Transmit Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "0: Divided Clock"]
    Mck = 0,
    #[doc = "1: RK Clock signal"]
    Rk = 1,
    #[doc = "2: TK pin"]
    Tk = 2,
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
#[doc = "Field `CKS` reader - Transmit Clock Selection"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cks> {
        match self.bits {
            0 => Some(Cks::Mck),
            1 => Some(Cks::Rk),
            2 => Some(Cks::Tk),
            _ => None,
        }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cks::Mck
    }
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == Cks::Rk
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == Cks::Tk
    }
}
#[doc = "Field `CKS` writer - Transmit Clock Selection"]
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
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Rk)
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Tk)
    }
}
#[doc = "Transmit Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cko {
    #[doc = "0: None, TK pin is an input"]
    None = 0,
    #[doc = "1: Continuous Transmit Clock, TK pin is an output"]
    Continuous = 1,
    #[doc = "2: Transmit Clock only during data transfers, TK pin is an output"]
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
#[doc = "Field `CKO` reader - Transmit Clock Output Mode Selection"]
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
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cko::None
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Cko::Continuous
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Cko::Transfer
    }
}
#[doc = "Field `CKO` writer - Transmit Clock Output Mode Selection"]
pub type CkoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cko>;
impl<'a, REG> CkoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::None)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::Continuous)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::Transfer)
    }
}
#[doc = "Field `CKI` reader - Transmit Clock Inversion"]
pub type CkiR = crate::BitReader;
#[doc = "Field `CKI` writer - Transmit Clock Inversion"]
pub type CkiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckg {
    #[doc = "0: None"]
    Continuous = 0,
    #[doc = "1: Transmit Clock enabled only if TF Low"]
    EnTfLow = 1,
    #[doc = "2: Transmit Clock enabled only if TF High"]
    EnTfHigh = 2,
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
#[doc = "Field `CKG` reader - Transmit Clock Gating Selection"]
pub type CkgR = crate::FieldReader<Ckg>;
impl CkgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckg> {
        match self.bits {
            0 => Some(Ckg::Continuous),
            1 => Some(Ckg::EnTfLow),
            2 => Some(Ckg::EnTfHigh),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ckg::Continuous
    }
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn is_en_tf_low(&self) -> bool {
        *self == Ckg::EnTfLow
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn is_en_tf_high(&self) -> bool {
        *self == Ckg::EnTfHigh
    }
}
#[doc = "Field `CKG` writer - Transmit Clock Gating Selection"]
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
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn en_tf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ckg::EnTfLow)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn en_tf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ckg::EnTfHigh)
    }
}
#[doc = "Transmit Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Start {
    #[doc = "0: Continuous, as soon as a word is written in the SSC_THR Register (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    Continuous = 0,
    #[doc = "1: Receive start"]
    Receive = 1,
    #[doc = "2: Detection of a low level on TF signal"]
    TfLow = 2,
    #[doc = "3: Detection of a high level on TF signal"]
    TfHigh = 3,
    #[doc = "4: Detection of a falling edge on TF signal"]
    TfFalling = 4,
    #[doc = "5: Detection of a rising edge on TF signal"]
    TfRising = 5,
    #[doc = "6: Detection of any level change on TF signal"]
    TfLevel = 6,
    #[doc = "7: Detection of any edge on TF signal"]
    TfEdge = 7,
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
#[doc = "Field `START` reader - Transmit Start Selection"]
pub type StartR = crate::FieldReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Start> {
        match self.bits {
            0 => Some(Start::Continuous),
            1 => Some(Start::Receive),
            2 => Some(Start::TfLow),
            3 => Some(Start::TfHigh),
            4 => Some(Start::TfFalling),
            5 => Some(Start::TfRising),
            6 => Some(Start::TfLevel),
            7 => Some(Start::TfEdge),
            _ => None,
        }
    }
    #[doc = "Continuous, as soon as a word is written in the SSC_THR Register (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Start::Continuous
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == Start::Receive
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn is_tf_low(&self) -> bool {
        *self == Start::TfLow
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn is_tf_high(&self) -> bool {
        *self == Start::TfHigh
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn is_tf_falling(&self) -> bool {
        *self == Start::TfFalling
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn is_tf_rising(&self) -> bool {
        *self == Start::TfRising
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn is_tf_level(&self) -> bool {
        *self == Start::TfLevel
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn is_tf_edge(&self) -> bool {
        *self == Start::TfEdge
    }
}
#[doc = "Field `START` writer - Transmit Start Selection"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 4, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous, as soon as a word is written in the SSC_THR Register (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Continuous)
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Receive)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn tf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Start::TfLow)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn tf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Start::TfHigh)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn tf_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Start::TfFalling)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn tf_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Start::TfRising)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn tf_level(self) -> &'a mut crate::W<REG> {
        self.variant(Start::TfLevel)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn tf_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Start::TfEdge)
    }
}
#[doc = "Field `STTDLY` reader - Transmit Start Delay"]
pub type SttdlyR = crate::FieldReader;
#[doc = "Field `STTDLY` writer - Transmit Start Delay"]
pub type SttdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERIOD` reader - Transmit Period Divider Selection"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Transmit Period Divider Selection"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CkoR {
        CkoR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CkiR {
        CkiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CkgR {
        CkgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> SttdlyR {
        SttdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CksW<TcmrSpec> {
        CksW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cko(&mut self) -> CkoW<TcmrSpec> {
        CkoW::new(self, 2)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn cki(&mut self) -> CkiW<TcmrSpec> {
        CkiW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckg(&mut self) -> CkgW<TcmrSpec> {
        CkgW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<TcmrSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sttdly(&mut self) -> SttdlyW<TcmrSpec> {
        SttdlyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<TcmrSpec> {
        PeriodW::new(self, 24)
    }
}
#[doc = "Transmit Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmrSpec;
impl crate::RegisterSpec for TcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcmr::R`](R) reader structure"]
impl crate::Readable for TcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcmr::W`](W) writer structure"]
impl crate::Writable for TcmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TcmrSpec {
    const RESET_VALUE: u32 = 0;
}
