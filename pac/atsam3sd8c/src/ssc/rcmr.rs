#[doc = "Register `RCMR` reader"]
pub type R = crate::R<RCMR_SPEC>;
#[doc = "Register `RCMR` writer"]
pub type W = crate::W<RCMR_SPEC>;
#[doc = "Field `CKS` reader - Receive Clock Selection"]
pub type CKS_R = crate::FieldReader<CKS_A>;
#[doc = "Receive Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: Divided Clock"]
    Mck = 0,
    #[doc = "1: TK Clock signal"]
    Tk = 1,
    #[doc = "2: RK pin"]
    Rk = 2,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKS_A {
    type Ux = u8;
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKS_A> {
        match self.bits {
            0 => Some(CKS_A::Mck),
            1 => Some(CKS_A::Tk),
            2 => Some(CKS_A::Rk),
            _ => None,
        }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKS_A::Mck
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKS_A::Tk
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKS_A::Rk
    }
}
#[doc = "Field `CKS` writer - Receive Clock Selection"]
pub type CKS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CKS_A>;
impl<'a, REG, const O: u8> CKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::Mck)
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::Tk)
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::Rk)
    }
}
#[doc = "Field `CKO` reader - Receive Clock Output Mode Selection"]
pub type CKO_R = crate::FieldReader<CKO_A>;
#[doc = "Receive Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKO_A {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Continuous Receive Clock"]
    Continuous = 1,
    #[doc = "2: Receive Clock only during data transfers"]
    Transfer = 2,
}
impl From<CKO_A> for u8 {
    #[inline(always)]
    fn from(variant: CKO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKO_A {
    type Ux = u8;
}
impl CKO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKO_A> {
        match self.bits {
            0 => Some(CKO_A::None),
            1 => Some(CKO_A::Continuous),
            2 => Some(CKO_A::Transfer),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKO_A::None
    }
    #[doc = "Continuous Receive Clock"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKO_A::Continuous
    }
    #[doc = "Receive Clock only during data transfers"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKO_A::Transfer
    }
}
#[doc = "Field `CKO` writer - Receive Clock Output Mode Selection"]
pub type CKO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CKO_A>;
impl<'a, REG, const O: u8> CKO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CKO_A::None)
    }
    #[doc = "Continuous Receive Clock"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CKO_A::Continuous)
    }
    #[doc = "Receive Clock only during data transfers"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(CKO_A::Transfer)
    }
}
#[doc = "Field `CKI` reader - Receive Clock Inversion"]
pub type CKI_R = crate::BitReader;
#[doc = "Field `CKI` writer - Receive Clock Inversion"]
pub type CKI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKG` reader - Receive Clock Gating Selection"]
pub type CKG_R = crate::FieldReader<CKG_A>;
#[doc = "Receive Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKG_A {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Continuous Receive Clock"]
    Continuous = 1,
    #[doc = "2: Receive Clock only during data transfers"]
    Transfer = 2,
}
impl From<CKG_A> for u8 {
    #[inline(always)]
    fn from(variant: CKG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKG_A {
    type Ux = u8;
}
impl CKG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKG_A> {
        match self.bits {
            0 => Some(CKG_A::None),
            1 => Some(CKG_A::Continuous),
            2 => Some(CKG_A::Transfer),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKG_A::None
    }
    #[doc = "Continuous Receive Clock"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKG_A::Continuous
    }
    #[doc = "Receive Clock only during data transfers"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKG_A::Transfer
    }
}
#[doc = "Field `CKG` writer - Receive Clock Gating Selection"]
pub type CKG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CKG_A>;
impl<'a, REG, const O: u8> CKG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CKG_A::None)
    }
    #[doc = "Continuous Receive Clock"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CKG_A::Continuous)
    }
    #[doc = "Receive Clock only during data transfers"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(CKG_A::Transfer)
    }
}
#[doc = "Field `START` reader - Receive Start Selection"]
pub type START_R = crate::FieldReader<START_A>;
#[doc = "Receive Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START_A {
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
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for START_A {
    type Ux = u8;
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<START_A> {
        match self.bits {
            0 => Some(START_A::Continuous),
            1 => Some(START_A::Transmit),
            2 => Some(START_A::RfLow),
            3 => Some(START_A::RfHigh),
            4 => Some(START_A::RfFalling),
            5 => Some(START_A::RfRising),
            6 => Some(START_A::RfLevel),
            7 => Some(START_A::RfEdge),
            8 => Some(START_A::Cmp0),
            _ => None,
        }
    }
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == START_A::Continuous
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == START_A::Transmit
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn is_rf_low(&self) -> bool {
        *self == START_A::RfLow
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn is_rf_high(&self) -> bool {
        *self == START_A::RfHigh
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_falling(&self) -> bool {
        *self == START_A::RfFalling
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_rising(&self) -> bool {
        *self == START_A::RfRising
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn is_rf_level(&self) -> bool {
        *self == START_A::RfLevel
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_edge(&self) -> bool {
        *self == START_A::RfEdge
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn is_cmp_0(&self) -> bool {
        *self == START_A::Cmp0
    }
}
#[doc = "Field `START` writer - Receive Start Selection"]
pub type START_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::Continuous)
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::Transmit)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::RfLow)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::RfHigh)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn rf_falling(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::RfFalling)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn rf_rising(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::RfRising)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn rf_level(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::RfLevel)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn rf_edge(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::RfEdge)
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn cmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::Cmp0)
    }
}
#[doc = "Field `STOP` reader - Receive Stop Selection"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Receive Stop Selection"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STTDLY` reader - Receive Start Delay"]
pub type STTDLY_R = crate::FieldReader;
#[doc = "Field `STTDLY` writer - Receive Start Delay"]
pub type STTDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PERIOD` reader - Receive Period Divider Selection"]
pub type PERIOD_R = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Receive Period Divider Selection"]
pub type PERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<RCMR_SPEC, 0> {
        CKS_W::new(self)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cko(&mut self) -> CKO_W<RCMR_SPEC, 2> {
        CKO_W::new(self)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn cki(&mut self) -> CKI_W<RCMR_SPEC, 5> {
        CKI_W::new(self)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckg(&mut self) -> CKG_W<RCMR_SPEC, 6> {
        CKG_W::new(self)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<RCMR_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<RCMR_SPEC, 12> {
        STOP_W::new(self)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sttdly(&mut self) -> STTDLY_W<RCMR_SPEC, 16> {
        STTDLY_W::new(self)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<RCMR_SPEC, 24> {
        PERIOD_W::new(self)
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
#[doc = "Receive Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCMR_SPEC;
impl crate::RegisterSpec for RCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcmr::R`](R) reader structure"]
impl crate::Readable for RCMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcmr::W`](W) writer structure"]
impl crate::Writable for RCMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCMR to value 0"]
impl crate::Resettable for RCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
