#[doc = "Register `BR` reader"]
pub struct R(crate::R<BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BR` writer"]
pub struct W(crate::W<BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE2` reader - Phase 2 segment"]
pub struct PHASE2_R(crate::FieldReader<u8, u8>);
impl PHASE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHASE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE2` writer - Phase 2 segment"]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `PHASE1` reader - Phase 1 segment"]
pub struct PHASE1_R(crate::FieldReader<u8, u8>);
impl PHASE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHASE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE1` writer - Phase 1 segment"]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `PROPAG` reader - Programming time segment"]
pub struct PROPAG_R(crate::FieldReader<u8, u8>);
impl PROPAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROPAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROPAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROPAG` writer - Programming time segment"]
pub struct PROPAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROPAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SJW` reader - Re-synchronization jump width"]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - Re-synchronization jump width"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `BRP` reader - Baudrate Prescaler."]
pub struct BRP_R(crate::FieldReader<u8, u8>);
impl BRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - Baudrate Prescaler."]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Sampling Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_A {
    #[doc = "0: The incoming bit stream is sampled once at sample point."]
    ONCE = 0,
    #[doc = "1: The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    THREE = 1,
}
impl From<SMP_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMP` reader - Sampling Mode"]
pub struct SMP_R(crate::FieldReader<bool, SMP_A>);
impl SMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            false => SMP_A::ONCE,
            true => SMP_A::THREE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        **self == SMP_A::ONCE
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        **self == SMP_A::THREE
    }
}
impl core::ops::Deref for SMP_R {
    type Target = crate::FieldReader<bool, SMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP` writer - Sampling Mode"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The incoming bit stream is sampled once at sample point."]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(SMP_A::ONCE)
    }
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(SMP_A::THREE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    pub fn propag(&self) -> PROPAG_R {
        PROPAG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    pub fn propag(&mut self) -> PROPAG_W {
        PROPAG_W { w: self }
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baudrate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](index.html) module"]
pub struct BR_SPEC;
impl crate::RegisterSpec for BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [br::R](R) reader structure"]
impl crate::Readable for BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [br::W](W) writer structure"]
impl crate::Writable for BR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BR to value 0"]
impl crate::Resettable for BR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
