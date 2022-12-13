#[doc = "Register `RFMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RFMR_SPEC>);
#[doc = "Register `RFMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RFMR_SPEC>);
#[doc = "Field `DATLEN` reader - Data Length"]
pub type DATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLEN` writer - Data Length"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFMR_SPEC, u8, u8, 5, O>;
#[doc = "Field `LOOP` reader - Loop Mode"]
pub type LOOP_R = crate::BitReader<bool>;
#[doc = "Field `LOOP` writer - Loop Mode"]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFMR_SPEC, bool, O>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader<bool>;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFMR_SPEC, bool, O>;
#[doc = "Field `DATNB` reader - Data Number per Frame"]
pub type DATNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATNB` writer - Data Number per Frame"]
pub type DATNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFMR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FSLEN` reader - Receive Frame Sync Length"]
pub type FSLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSLEN` writer - Receive Frame Sync Length"]
pub type FSLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFMR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FSOS` reader - Receive Frame Sync Output Selection"]
pub type FSOS_R = crate::FieldReader<u8, FSOS_A>;
#[doc = "Receive Frame Sync Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSOS_A {
    #[doc = "0: None, RF pin is an input"]
    None = 0,
    #[doc = "1: Negative Pulse, RF pin is an output"]
    Negative = 1,
    #[doc = "2: Positive Pulse, RF pin is an output"]
    Positive = 2,
    #[doc = "3: Driven Low during data transfer, RF pin is an output"]
    Low = 3,
    #[doc = "4: Driven High during data transfer, RF pin is an output"]
    High = 4,
    #[doc = "5: Toggling at each start of data transfer, RF pin is an output"]
    Toggling = 5,
}
impl From<FSOS_A> for u8 {
    #[inline(always)]
    fn from(variant: FSOS_A) -> Self {
        variant as _
    }
}
impl FSOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSOS_A> {
        match self.bits {
            0 => Some(FSOS_A::None),
            1 => Some(FSOS_A::Negative),
            2 => Some(FSOS_A::Positive),
            3 => Some(FSOS_A::Low),
            4 => Some(FSOS_A::High),
            5 => Some(FSOS_A::Toggling),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FSOS_A::None
    }
    #[doc = "Checks if the value of the field is `Negative`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSOS_A::Negative
    }
    #[doc = "Checks if the value of the field is `Positive`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSOS_A::Positive
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FSOS_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FSOS_A::High
    }
    #[doc = "Checks if the value of the field is `Toggling`"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == FSOS_A::Toggling
    }
}
#[doc = "Field `FSOS` writer - Receive Frame Sync Output Selection"]
pub type FSOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFMR_SPEC, u8, FSOS_A, 3, O>;
impl<'a, const O: u8> FSOS_W<'a, O> {
    #[doc = "None, RF pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FSOS_A::None)
    }
    #[doc = "Negative Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSOS_A::Negative)
    }
    #[doc = "Positive Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSOS_A::Positive)
    }
    #[doc = "Driven Low during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FSOS_A::Low)
    }
    #[doc = "Driven High during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FSOS_A::High)
    }
    #[doc = "Toggling at each start of data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut W {
        self.variant(FSOS_A::Toggling)
    }
}
#[doc = "Field `FSEDGE` reader - Frame Sync Edge Detection"]
pub type FSEDGE_R = crate::BitReader<FSEDGE_A>;
#[doc = "Frame Sync Edge Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSEDGE_A {
    #[doc = "0: Positive Edge Detection"]
    Positive = 0,
    #[doc = "1: Negative Edge Detection"]
    Negative = 1,
}
impl From<FSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: FSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl FSEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEDGE_A {
        match self.bits {
            false => FSEDGE_A::Positive,
            true => FSEDGE_A::Negative,
        }
    }
    #[doc = "Checks if the value of the field is `Positive`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSEDGE_A::Positive
    }
    #[doc = "Checks if the value of the field is `Negative`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSEDGE_A::Negative
    }
}
#[doc = "Field `FSEDGE` writer - Frame Sync Edge Detection"]
pub type FSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFMR_SPEC, FSEDGE_A, O>;
impl<'a, const O: u8> FSEDGE_W<'a, O> {
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSEDGE_A::Positive)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSEDGE_A::Negative)
    }
}
#[doc = "Field `FSLEN_EXT` reader - FSLEN Field Extension"]
pub type FSLEN_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSLEN_EXT` writer - FSLEN Field Extension"]
pub type FSLEN_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFMR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DATNB_R {
        DATNB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FSLEN_R {
        FSLEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FSOS_R {
        FSOS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&self) -> FSEDGE_R {
        FSEDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&self) -> FSLEN_EXT_R {
        FSLEN_EXT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<0> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<5> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<7> {
        MSBF_W::new(self)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    #[must_use]
    pub fn datnb(&mut self) -> DATNB_W<8> {
        DATNB_W::new(self)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    #[must_use]
    pub fn fslen(&mut self) -> FSLEN_W<16> {
        FSLEN_W::new(self)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsos(&mut self) -> FSOS_W<20> {
        FSOS_W::new(self)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn fsedge(&mut self) -> FSEDGE_W<24> {
        FSEDGE_W::new(self)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fslen_ext(&mut self) -> FSLEN_EXT_W<28> {
        FSLEN_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfmr](index.html) module"]
pub struct RFMR_SPEC;
impl crate::RegisterSpec for RFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfmr::R](R) reader structure"]
impl crate::Readable for RFMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfmr::W](W) writer structure"]
impl crate::Writable for RFMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFMR to value 0"]
impl crate::Resettable for RFMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
