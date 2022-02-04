#[doc = "Register `TIMINGS6` reader"]
pub struct R(crate::R<TIMINGS6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGS6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGS6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGS6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMINGS6` writer"]
pub struct W(crate::W<TIMINGS6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGS6_SPEC>;
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
impl From<crate::W<TIMINGS6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGS6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCLR` reader - CLE to REN Low Delay"]
pub struct TCLR_R(crate::FieldReader<u8, u8>);
impl TCLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLR` writer - CLE to REN Low Delay"]
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TADL` reader - ALE to Data Start"]
pub struct TADL_R(crate::FieldReader<u8, u8>);
impl TADL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TADL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TADL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TADL` writer - ALE to Data Start"]
pub struct TADL_W<'a> {
    w: &'a mut W,
}
impl<'a> TADL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TAR` reader - ALE to REN Low Delay"]
pub struct TAR_R(crate::FieldReader<u8, u8>);
impl TAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAR` writer - ALE to REN Low Delay"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `OCMS` reader - Off Chip Memory Scrambling Enable"]
pub struct OCMS_R(crate::FieldReader<bool, bool>);
impl OCMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OCMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCMS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCMS` writer - Off Chip Memory Scrambling Enable"]
pub struct OCMS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCMS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TRR` reader - Ready to REN Low Delay"]
pub struct TRR_R(crate::FieldReader<u8, u8>);
impl TRR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRR` writer - Ready to REN Low Delay"]
pub struct TRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TWB` reader - WEN High to REN to Busy"]
pub struct TWB_R(crate::FieldReader<u8, u8>);
impl TWB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TWB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWB` writer - WEN High to REN to Busy"]
pub struct TWB_W<'a> {
    w: &'a mut W,
}
impl<'a> TWB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RBNSEL` reader - Ready/Busy Line Selection"]
pub struct RBNSEL_R(crate::FieldReader<u8, u8>);
impl RBNSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBNSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBNSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBNSEL` writer - Ready/Busy Line Selection"]
pub struct RBNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RBNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `NFSEL` reader - NAND Flash Selection"]
pub struct NFSEL_R(crate::FieldReader<bool, bool>);
impl NFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFSEL` writer - NAND Flash Selection"]
pub struct NFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NFSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&self) -> TADL_R {
        TADL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&self) -> OCMS_R {
        OCMS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&self) -> RBNSEL_R {
        RBNSEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&self) -> NFSEL_R {
        NFSEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&mut self) -> TADL_W {
        TADL_W { w: self }
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&mut self) -> OCMS_W {
        OCMS_W { w: self }
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&mut self) -> TRR_W {
        TRR_W { w: self }
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&mut self) -> TWB_W {
        TWB_W { w: self }
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&mut self) -> RBNSEL_W {
        RBNSEL_W { w: self }
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&mut self) -> NFSEL_W {
        NFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Timings Register (CS_number = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings6](index.html) module"]
pub struct TIMINGS6_SPEC;
impl crate::RegisterSpec for TIMINGS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timings6::R](R) reader structure"]
impl crate::Readable for TIMINGS6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timings6::W](W) writer structure"]
impl crate::Writable for TIMINGS6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMINGS6 to value 0"]
impl crate::Resettable for TIMINGS6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
