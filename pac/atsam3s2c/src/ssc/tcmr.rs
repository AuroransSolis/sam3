#[doc = "Register `TCMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<TCMR_SPEC>);
#[doc = "Register `TCMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<TCMR_SPEC>);
#[doc = "Field `CKS` reader - Transmit Clock Selection"]
pub type CKS_R = crate::FieldReader<u8, CKS_A>;
#[doc = "Transmit Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: Divided Clock"]
    Mck = 0,
    #[doc = "1: RK Clock signal"]
    Rk = 1,
    #[doc = "2: TK pin"]
    Tk = 2,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKS_A> {
        match self.bits {
            0 => Some(CKS_A::Mck),
            1 => Some(CKS_A::Rk),
            2 => Some(CKS_A::Tk),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Mck`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKS_A::Mck
    }
    #[doc = "Checks if the value of the field is `Rk`"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKS_A::Rk
    }
    #[doc = "Checks if the value of the field is `Tk`"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKS_A::Tk
    }
}
#[doc = "Field `CKS` writer - Transmit Clock Selection"]
pub type CKS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, CKS_A, 2, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CKS_A::Mck)
    }
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKS_A::Rk)
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKS_A::Tk)
    }
}
#[doc = "Field `CKO` reader - Transmit Clock Output Mode Selection"]
pub type CKO_R = crate::FieldReader<u8, CKO_A>;
#[doc = "Transmit Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKO_A {
    #[doc = "0: None, TK pin is an input"]
    None = 0,
    #[doc = "1: Continuous Transmit Clock, TK pin is an output"]
    Continuous = 1,
    #[doc = "2: Transmit Clock only during data transfers, TK pin is an output"]
    Transfer = 2,
}
impl From<CKO_A> for u8 {
    #[inline(always)]
    fn from(variant: CKO_A) -> Self {
        variant as _
    }
}
impl CKO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKO_A> {
        match self.bits {
            0 => Some(CKO_A::None),
            1 => Some(CKO_A::Continuous),
            2 => Some(CKO_A::Transfer),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKO_A::None
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKO_A::Continuous
    }
    #[doc = "Checks if the value of the field is `Transfer`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKO_A::Transfer
    }
}
#[doc = "Field `CKO` writer - Transmit Clock Output Mode Selection"]
pub type CKO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, CKO_A, 3, O>;
impl<'a, const O: u8> CKO_W<'a, O> {
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKO_A::None)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKO_A::Continuous)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(CKO_A::Transfer)
    }
}
#[doc = "Field `CKI` reader - Transmit Clock Inversion"]
pub type CKI_R = crate::BitReader<bool>;
#[doc = "Field `CKI` writer - Transmit Clock Inversion"]
pub type CKI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCMR_SPEC, bool, O>;
#[doc = "Field `CKG` reader - Transmit Clock Gating Selection"]
pub type CKG_R = crate::FieldReader<u8, CKG_A>;
#[doc = "Transmit Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKG_A {
    #[doc = "0: None"]
    Continuous = 0,
    #[doc = "1: Transmit Clock enabled only if TF Low"]
    EnTfLow = 1,
    #[doc = "2: Transmit Clock enabled only if TF High"]
    EnTfHigh = 2,
}
impl From<CKG_A> for u8 {
    #[inline(always)]
    fn from(variant: CKG_A) -> Self {
        variant as _
    }
}
impl CKG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKG_A> {
        match self.bits {
            0 => Some(CKG_A::Continuous),
            1 => Some(CKG_A::EnTfLow),
            2 => Some(CKG_A::EnTfHigh),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKG_A::Continuous
    }
    #[doc = "Checks if the value of the field is `EnTfLow`"]
    #[inline(always)]
    pub fn is_en_tf_low(&self) -> bool {
        *self == CKG_A::EnTfLow
    }
    #[doc = "Checks if the value of the field is `EnTfHigh`"]
    #[inline(always)]
    pub fn is_en_tf_high(&self) -> bool {
        *self == CKG_A::EnTfHigh
    }
}
#[doc = "Field `CKG` writer - Transmit Clock Gating Selection"]
pub type CKG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, CKG_A, 2, O>;
impl<'a, const O: u8> CKG_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKG_A::Continuous)
    }
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn en_tf_low(self) -> &'a mut W {
        self.variant(CKG_A::EnTfLow)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn en_tf_high(self) -> &'a mut W {
        self.variant(CKG_A::EnTfHigh)
    }
}
#[doc = "Field `START` reader - Transmit Start Selection"]
pub type START_R = crate::FieldReader<u8, START_A>;
#[doc = "Transmit Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START_A {
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
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            0 => Some(START_A::Continuous),
            1 => Some(START_A::Receive),
            2 => Some(START_A::TfLow),
            3 => Some(START_A::TfHigh),
            4 => Some(START_A::TfFalling),
            5 => Some(START_A::TfRising),
            6 => Some(START_A::TfLevel),
            7 => Some(START_A::TfEdge),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == START_A::Continuous
    }
    #[doc = "Checks if the value of the field is `Receive`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == START_A::Receive
    }
    #[doc = "Checks if the value of the field is `TfLow`"]
    #[inline(always)]
    pub fn is_tf_low(&self) -> bool {
        *self == START_A::TfLow
    }
    #[doc = "Checks if the value of the field is `TfHigh`"]
    #[inline(always)]
    pub fn is_tf_high(&self) -> bool {
        *self == START_A::TfHigh
    }
    #[doc = "Checks if the value of the field is `TfFalling`"]
    #[inline(always)]
    pub fn is_tf_falling(&self) -> bool {
        *self == START_A::TfFalling
    }
    #[doc = "Checks if the value of the field is `TfRising`"]
    #[inline(always)]
    pub fn is_tf_rising(&self) -> bool {
        *self == START_A::TfRising
    }
    #[doc = "Checks if the value of the field is `TfLevel`"]
    #[inline(always)]
    pub fn is_tf_level(&self) -> bool {
        *self == START_A::TfLevel
    }
    #[doc = "Checks if the value of the field is `TfEdge`"]
    #[inline(always)]
    pub fn is_tf_edge(&self) -> bool {
        *self == START_A::TfEdge
    }
}
#[doc = "Field `START` writer - Transmit Start Selection"]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, START_A, 4, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Continuous, as soon as a word is written in the SSC_THR Register (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(START_A::Continuous)
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(START_A::Receive)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn tf_low(self) -> &'a mut W {
        self.variant(START_A::TfLow)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn tf_high(self) -> &'a mut W {
        self.variant(START_A::TfHigh)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn tf_falling(self) -> &'a mut W {
        self.variant(START_A::TfFalling)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn tf_rising(self) -> &'a mut W {
        self.variant(START_A::TfRising)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn tf_level(self) -> &'a mut W {
        self.variant(START_A::TfLevel)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn tf_edge(self) -> &'a mut W {
        self.variant(START_A::TfEdge)
    }
}
#[doc = "Field `STTDLY` reader - Transmit Start Delay"]
pub type STTDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STTDLY` writer - Transmit Start Delay"]
pub type STTDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PERIOD` reader - Transmit Period Divider Selection"]
pub type PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD` writer - Transmit Period Divider Selection"]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<0> {
        CKS_W::new(self)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cko(&mut self) -> CKO_W<2> {
        CKO_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn cki(&mut self) -> CKI_W<5> {
        CKI_W::new(self)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckg(&mut self) -> CKG_W<6> {
        CKG_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sttdly(&mut self) -> STTDLY_W<16> {
        STTDLY_W::new(self)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<24> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](index.html) module"]
pub struct TCMR_SPEC;
impl crate::RegisterSpec for TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcmr::R](R) reader structure"]
impl crate::Readable for TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcmr::W](W) writer structure"]
impl crate::Writable for TCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
