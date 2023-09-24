#[doc = "Register `TFMR` reader"]
pub type R = crate::R<TFMR_SPEC>;
#[doc = "Register `TFMR` writer"]
pub type W = crate::W<TFMR_SPEC>;
#[doc = "Field `DATLEN` reader - Data Length"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data Length"]
pub type DATLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DATDEF` reader - Data Default Value"]
pub type DATDEF_R = crate::BitReader;
#[doc = "Field `DATDEF` writer - Data Default Value"]
pub type DATDEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATNB` reader - Data Number per frame"]
pub type DATNB_R = crate::FieldReader;
#[doc = "Field `DATNB` writer - Data Number per frame"]
pub type DATNB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FSLEN` reader - Transmit Frame Sync Length"]
pub type FSLEN_R = crate::FieldReader;
#[doc = "Field `FSLEN` writer - Transmit Frame Sync Length"]
pub type FSLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FSOS` reader - Transmit Frame Sync Output Selection"]
pub type FSOS_R = crate::FieldReader<FSOS_A>;
#[doc = "Transmit Frame Sync Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSOS_A {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Negative Pulse"]
    Negative = 1,
    #[doc = "2: Positive Pulse"]
    Positive = 2,
    #[doc = "3: Driven Low during data transfer"]
    Low = 3,
    #[doc = "4: Driven High during data transfer"]
    High = 4,
    #[doc = "5: Toggling at each start of data transfer"]
    Toggling = 5,
}
impl From<FSOS_A> for u8 {
    #[inline(always)]
    fn from(variant: FSOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSOS_A {
    type Ux = u8;
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
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FSOS_A::None
    }
    #[doc = "Negative Pulse"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSOS_A::Negative
    }
    #[doc = "Positive Pulse"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSOS_A::Positive
    }
    #[doc = "Driven Low during data transfer"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FSOS_A::Low
    }
    #[doc = "Driven High during data transfer"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FSOS_A::High
    }
    #[doc = "Toggling at each start of data transfer"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == FSOS_A::Toggling
    }
}
#[doc = "Field `FSOS` writer - Transmit Frame Sync Output Selection"]
pub type FSOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, FSOS_A>;
impl<'a, REG, const O: u8> FSOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(FSOS_A::None)
    }
    #[doc = "Negative Pulse"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(FSOS_A::Negative)
    }
    #[doc = "Positive Pulse"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(FSOS_A::Positive)
    }
    #[doc = "Driven Low during data transfer"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(FSOS_A::Low)
    }
    #[doc = "Driven High during data transfer"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(FSOS_A::High)
    }
    #[doc = "Toggling at each start of data transfer"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut crate::W<REG> {
        self.variant(FSOS_A::Toggling)
    }
}
#[doc = "Field `FSDEN` reader - Frame Sync Data Enable"]
pub type FSDEN_R = crate::BitReader;
#[doc = "Field `FSDEN` writer - Frame Sync Data Enable"]
pub type FSDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSEDGE_A::Positive
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSEDGE_A::Negative
    }
}
#[doc = "Field `FSEDGE` writer - Frame Sync Edge Detection"]
pub type FSEDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FSEDGE_A>;
impl<'a, REG, const O: u8> FSEDGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(FSEDGE_A::Positive)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(FSEDGE_A::Negative)
    }
}
#[doc = "Field `FSLEN_EXT` reader - FSLEN Field Extension"]
pub type FSLEN_EXT_R = crate::FieldReader;
#[doc = "Field `FSLEN_EXT` writer - FSLEN Field Extension"]
pub type FSLEN_EXT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline(always)]
    pub fn datdef(&self) -> DATDEF_R {
        DATDEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DATNB_R {
        DATNB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FSLEN_R {
        FSLEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FSOS_R {
        FSOS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline(always)]
    pub fn fsden(&self) -> FSDEN_R {
        FSDEN_R::new(((self.bits >> 23) & 1) != 0)
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
    pub fn datlen(&mut self) -> DATLEN_W<TFMR_SPEC, 0> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn datdef(&mut self) -> DATDEF_W<TFMR_SPEC, 5> {
        DATDEF_W::new(self)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<TFMR_SPEC, 7> {
        MSBF_W::new(self)
    }
    #[doc = "Bits 8:11 - Data Number per frame"]
    #[inline(always)]
    #[must_use]
    pub fn datnb(&mut self) -> DATNB_W<TFMR_SPEC, 8> {
        DATNB_W::new(self)
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline(always)]
    #[must_use]
    pub fn fslen(&mut self) -> FSLEN_W<TFMR_SPEC, 16> {
        FSLEN_W::new(self)
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsos(&mut self) -> FSOS_W<TFMR_SPEC, 20> {
        FSOS_W::new(self)
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsden(&mut self) -> FSDEN_W<TFMR_SPEC, 23> {
        FSDEN_W::new(self)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn fsedge(&mut self) -> FSEDGE_W<TFMR_SPEC, 24> {
        FSEDGE_W::new(self)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fslen_ext(&mut self) -> FSLEN_EXT_W<TFMR_SPEC, 28> {
        FSLEN_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFMR_SPEC;
impl crate::RegisterSpec for TFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfmr::R`](R) reader structure"]
impl crate::Readable for TFMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfmr::W`](W) writer structure"]
impl crate::Writable for TFMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFMR to value 0"]
impl crate::Resettable for TFMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
