#[doc = "Register `RFMR` reader"]
pub type R = crate::R<RFMR_SPEC>;
#[doc = "Register `RFMR` writer"]
pub type W = crate::W<RFMR_SPEC>;
#[doc = "Field `DATLEN` reader - Data Length"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data Length"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LOOP` reader - Loop Mode"]
pub type LOOP_R = crate::BitReader;
#[doc = "Field `LOOP` writer - Loop Mode"]
pub type LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATNB` reader - Data Number per Frame"]
pub type DATNB_R = crate::FieldReader;
#[doc = "Field `DATNB` writer - Data Number per Frame"]
pub type DATNB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FSLEN` reader - Receive Frame Sync Length"]
pub type FSLEN_R = crate::FieldReader;
#[doc = "Field `FSLEN` writer - Receive Frame Sync Length"]
pub type FSLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FSOS` reader - Receive Frame Sync Output Selection"]
pub type FSOS_R = crate::FieldReader<FSOS_A>;
#[doc = "Receive Frame Sync Output Selection\n\nValue on reset: 0"]
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
    pub const fn variant(&self) -> Option<FSOS_A> {
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
#[doc = "Field `FSOS` writer - Receive Frame Sync Output Selection"]
pub type FSOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FSOS_A>;
impl<'a, REG> FSOS_W<'a, REG>
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
    pub const fn variant(&self) -> FSEDGE_A {
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
pub type FSEDGE_W<'a, REG> = crate::BitWriter<'a, REG, FSEDGE_A>;
impl<'a, REG> FSEDGE_W<'a, REG>
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
pub type FSLEN_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    pub fn datlen(&mut self) -> DATLEN_W<RFMR_SPEC> {
        DATLEN_W::new(self, 0)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<RFMR_SPEC> {
        LOOP_W::new(self, 5)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<RFMR_SPEC> {
        MSBF_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    #[must_use]
    pub fn datnb(&mut self) -> DATNB_W<RFMR_SPEC> {
        DATNB_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    #[must_use]
    pub fn fslen(&mut self) -> FSLEN_W<RFMR_SPEC> {
        FSLEN_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsos(&mut self) -> FSOS_W<RFMR_SPEC> {
        FSOS_W::new(self, 20)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn fsedge(&mut self) -> FSEDGE_W<RFMR_SPEC> {
        FSEDGE_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fslen_ext(&mut self) -> FSLEN_EXT_W<RFMR_SPEC> {
        FSLEN_EXT_W::new(self, 28)
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
#[doc = "Receive Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFMR_SPEC;
impl crate::RegisterSpec for RFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfmr::R`](R) reader structure"]
impl crate::Readable for RFMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfmr::W`](W) writer structure"]
impl crate::Writable for RFMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFMR to value 0"]
impl crate::Resettable for RFMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
