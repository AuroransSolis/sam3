#[doc = "Register `FRO` reader"]
pub struct R(crate::R<FRO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO` writer"]
pub struct W(crate::W<FRO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO_SPEC>;
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
impl From<crate::W<FRO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FROK` reader - Frames Received OK"]
pub struct FROK_R(crate::FieldReader<u32, u32>);
impl FROK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FROK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FROK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FROK` writer - Frames Received OK"]
pub struct FROK_W<'a> {
    w: &'a mut W,
}
impl<'a> FROK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&mut self) -> FROK_W {
        FROK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frames Received Ok Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro](index.html) module"]
pub struct FRO_SPEC;
impl crate::RegisterSpec for FRO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro::R](R) reader structure"]
impl crate::Readable for FRO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro::W](W) writer structure"]
impl crate::Writable for FRO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRO to value 0"]
impl crate::Resettable for FRO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
