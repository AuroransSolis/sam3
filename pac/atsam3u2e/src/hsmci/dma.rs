#[doc = "Register `DMA` reader"]
pub struct R(crate::R<DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA` writer"]
pub struct W(crate::W<DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SPEC>;
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
impl From<crate::W<DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - DMA Write Buffer Offset"]
pub struct OFFSET_R(crate::FieldReader<u8, u8>);
impl OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` writer - DMA Write Buffer Offset"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CHKSIZE` reader - DMA Channel Read and Write Chunk Size"]
pub struct CHKSIZE_R(crate::FieldReader<u8, u8>);
impl CHKSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHKSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHKSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKSIZE` writer - DMA Channel Read and Write Chunk Size"]
pub struct CHKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DMAEN` reader - DMA Hardware Handshaking Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Hardware Handshaking Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ROPT` reader - Read Optimization with padding"]
pub struct ROPT_R(crate::FieldReader<bool, bool>);
impl ROPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROPT` writer - Read Optimization with padding"]
pub struct ROPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ROPT_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - DMA Write Buffer Offset"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> CHKSIZE_R {
        CHKSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read Optimization with padding"]
    #[inline(always)]
    pub fn ropt(&self) -> ROPT_R {
        ROPT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Write Buffer Offset"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&mut self) -> CHKSIZE_W {
        CHKSIZE_W { w: self }
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 12 - Read Optimization with padding"]
    #[inline(always)]
    pub fn ropt(&mut self) -> ROPT_W {
        ROPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma::R](R) reader structure"]
impl crate::Readable for DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma::W](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
