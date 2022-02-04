#[doc = "Register `DMANXTDSC0` reader"]
pub struct R(crate::R<DMANXTDSC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMANXTDSC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMANXTDSC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMANXTDSC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMANXTDSC0` writer"]
pub struct W(crate::W<DMANXTDSC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMANXTDSC0_SPEC>;
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
impl From<crate::W<DMANXTDSC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMANXTDSC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub struct NXT_DSC_ADD_R(crate::FieldReader<u32, u32>);
impl NXT_DSC_ADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NXT_DSC_ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NXT_DSC_ADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub struct NXT_DSC_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> NXT_DSC_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NXT_DSC_ADD_R {
        NXT_DSC_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W {
        NXT_DSC_ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmanxtdsc0](index.html) module"]
pub struct DMANXTDSC0_SPEC;
impl crate::RegisterSpec for DMANXTDSC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmanxtdsc0::R](R) reader structure"]
impl crate::Readable for DMANXTDSC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmanxtdsc0::W](W) writer structure"]
impl crate::Writable for DMANXTDSC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMANXTDSC0 to value 0"]
impl crate::Resettable for DMANXTDSC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
